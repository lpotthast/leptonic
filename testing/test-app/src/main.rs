/// Number of panics on the server. Browser tests read it through `/__test/server-panics`, so that
/// a panic during server-side rendering fails the test run even if the page still loads.
#[cfg(feature = "ssr")]
static SERVER_PANICS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

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
        SERVER_PANICS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
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
                    .load(std::sync::atomic::Ordering::SeqCst)
                    .to_string()
            }),
        )
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        .layer(axum::middleware::from_fn(cache_hashed_files));

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
