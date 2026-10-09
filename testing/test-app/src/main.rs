/// The panics on the server, one line each. Browser tests read them through
/// `/__test/server-panics`, so that a panic during server-side rendering fails the test run even if
/// the page still loads.
#[cfg(feature = "ssr")]
static SERVER_PANICS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

#[cfg(feature = "ssr")]
tokio::task_local! {
    /// The path (and query) of the request a task serves, for the panic report.
    static REQUEST_PATH: String;
}

/// Runs the request's handling with its path in [`REQUEST_PATH`]: the handler, and the response
/// body, which Leptos streams (rendering continues while the server sends it).
#[cfg(feature = "ssr")]
async fn record_request_path(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let path = request
        .uri()
        .path_and_query()
        .map_or_else(|| request.uri().path().to_owned(), ToString::to_string);
    let response = REQUEST_PATH.scope(path.clone(), next.run(request)).await;
    response.map(|body| axum::body::Body::new(BodyOfRequest { path, body }))
}

/// A response body polled with its request's path in [`REQUEST_PATH`].
#[cfg(feature = "ssr")]
struct BodyOfRequest {
    path: String,
    body: axum::body::Body,
}

#[cfg(feature = "ssr")]
impl axum::body::HttpBody for BodyOfRequest {
    type Data = axum::body::Bytes;
    type Error = axum::Error;

    fn poll_frame(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<http_body::Frame<Self::Data>, Self::Error>>> {
        let this = self.get_mut();
        REQUEST_PATH.sync_scope(this.path.clone(), || {
            std::pin::Pin::new(&mut this.body).poll_frame(cx)
        })
    }

    fn is_end_stream(&self) -> bool {
        self.body.is_end_stream()
    }

    fn size_hint(&self) -> http_body::SizeHint {
        self.body.size_hint()
    }
}

/// One line describing a panic: the request it happened in (if it happened in a request's task),
/// where, and its message (quoted, so that it stays on one line).
#[cfg(feature = "ssr")]
fn describe_panic(info: &std::panic::PanicHookInfo<'_>) -> String {
    let request = REQUEST_PATH
        .try_with(Clone::clone)
        .unwrap_or_else(|_| "no request (a task of its own)".to_owned());
    let location = info
        .location()
        .map_or_else(|| "an unknown location".to_owned(), ToString::to_string);
    let message = info.payload_as_str().unwrap_or("a non-string payload");
    format!("{request}: panicked at {location}: {message:?}")
}

/// Lets browsers cache the files under `/pkg/` for good, as a production server would: their names
/// carry their content's hash (`hash-files` in `Cargo.toml`), so a changed file has a new name.
#[cfg(feature = "ssr")]
async fn cache_hashed_files(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let hashed = request.uri().path().starts_with("/pkg/");
    let mut response = next.run(request).await;
    if hashed && response.status().is_success() {
        response.headers_mut().insert(
            axum::http::header::CACHE_CONTROL,
            axum::http::HeaderValue::from_static("public, max-age=31536000, immutable"),
        );
    }
    response
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptonic_test_app::app::*;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use tracing_subscriber::{
        Layer, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
    };

    let log_filter = tracing_subscriber::filter::Targets::new()
        .with_default(tracing::Level::INFO)
        .with_target("tokio", tracing::Level::WARN)
        .with_target("runtime", tracing::Level::WARN);

    let fmt_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .with_thread_names(false)
        .with_thread_ids(false);

    let fmt_layer_filtered = fmt_layer.with_filter(log_filter);

    tracing_subscriber::Registry::default()
        .with(fmt_layer_filtered)
        .init();

    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let panic = describe_panic(info);
        SERVER_PANICS
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push(panic);
        default_hook(info);
    }));

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let routes = generate_route_list(App);

    let app = Router::new()
        .route(
            "/__test/server-panics",
            axum::routing::get(|| async {
                SERVER_PANICS
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .join("\n")
            }),
        )
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        .layer(axum::middleware::from_fn(cache_hashed_files))
        .layer(axum::middleware::from_fn(record_request_path));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server to start successfully");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // see lib.rs for hydration function instead
}
