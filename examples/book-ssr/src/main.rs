/// Lets browsers cache the files under `/pkg/` for good: their names carry their content's hash
/// (`hash-files` in `Cargo.toml`), so a changed file has a new name.
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
    use book_ssr::{app::*, markdown};
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use tower_http::compression::predicate::{DefaultPredicate, NotForContentType, Predicate};
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

    let (shutdown_send, mut shutdown_recv) = tokio::sync::mpsc::unbounded_channel::<()>();

    let _app_jh = tokio::spawn(async move {
        let conf =
            get_configuration(None).expect("Leptos configuration in Cargo.toml or the environment");
        let addr = conf.leptos_options.site_addr;
        let leptos_options = conf.leptos_options;
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);

        let doc_paths: Vec<String> = routes
            .iter()
            .map(|r| r.path().to_owned())
            .filter(|p| p.starts_with("/doc/"))
            .collect();

        let md_cache = markdown::MarkdownCache::default();

        let md_cache_for_ctx = md_cache.clone();
        let warmup_cache = md_cache.clone();
        let pages = Router::new()
            .leptos_routes_with_context(
                &leptos_options,
                routes,
                move || leptos::prelude::provide_context(md_cache_for_ctx.clone()),
                {
                    let leptos_options = leptos_options.clone();
                    move || shell(leptos_options.clone())
                },
            )
            .fallback(leptos_axum::file_and_error_handler(shell))
            .with_state(leptos_options);

        // The Markdown middleware rewrites `/doc/x.md` to `/doc/x`, so it must run before routing. Middleware added to
        // a router with `layer` runs after routing, therefore the pages are wrapped in an outer router.
        let app = Router::new()
            .fallback_service(pages)
            .layer(axum::middleware::from_fn(cache_hashed_files))
            .layer(axum::middleware::from_fn_with_state(
                md_cache,
                markdown::markdown_middleware,
            ))
            .layer(
                tower_http::compression::CompressionLayer::new()
                    .gzip(true)
                    .br(true)
                    .deflate(true)
                    .quality(tower_http::CompressionLevel::Default)
                    // Not the WebAssembly bundle: compressing the 43 MB development bundle takes 3.4 s of CPU per
                    // request (seconds before the page hydrates, on every reload, and longer when browser tests load
                    // pages in parallel). A release build serves it precompressed (`precompress.sh`), which this layer
                    // leaves alone.
                    .compress_when(
                        DefaultPredicate::new()
                            .and(NotForContentType::const_new("application/wasm")),
                    ),
            );

        let warmup_app = app.clone();
        tokio::spawn(async move {
            markdown::warm_markdown_cache(warmup_app, warmup_cache, &doc_paths).await;
        });

        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("no other crypto provider is installed yet");
        tracing::info!("Loading certs...");

        let working_dir = std::env::current_dir().expect("Could not determine working directory.");

        let mut cert_path = working_dir.clone();
        cert_path
            .push(std::env::var("TLS_CERT_PATH").unwrap_or(String::from("certs/ssl_cert.pem")));
        tracing::info!("Using crt path: {cert_path:?}");

        let mut key_path = working_dir.clone();
        key_path.push(std::env::var("TLS_KEY_PATH").unwrap_or(String::from("certs/ssl_key.pem")));
        tracing::info!("Using key path: {key_path:?}");

        let config = axum_server::tls_rustls::RustlsConfig::from_pem_file(cert_path, key_path)
            .await
            .expect("Could not load certificates");

        tracing::info!("listening on https://{}", &addr);

        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await
            .expect("Server to start successfully");

        tracing::info!("Exiting...");
        // The receiver only goes away once the process shuts down anyway.
        let _ = shutdown_send.send(());
    });

    tracing::info!("Waiting for Ctrl-C...");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Received Ctrl-C signal");
        },
        _ = shutdown_recv.recv() => {
            tracing::info!("Received application shutdown signal");
        },
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
