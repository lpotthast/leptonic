use tower_http::compression::CompressionLayer;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use book_ssr::app::*;

    use tracing_subscriber::{
        prelude::__tracing_subscriber_SubscriberExt,
        util::SubscriberInitExt,
        Layer,
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
        let conf = get_configuration(None).unwrap();
        let addr = conf.leptos_options.site_addr;
        let leptos_options = conf.leptos_options;
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);

        let app = Router::new()
            .leptos_routes(&leptos_options, routes, {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            })
            .fallback(leptos_axum::file_and_error_handler(shell))
            .layer(
                CompressionLayer::new()
                    .gzip(true)
                    .br(true)
                    .deflate(true)
                    .quality(tower_http::CompressionLevel::Default),
            )
            .with_state(leptos_options);
        
        tracing::info!("Loading certs...");

        let working_dir = std::env::current_dir().expect("Could not determine working directory.");

        let mut cert_path = working_dir.clone();
        cert_path.push(std::env::var("TLS_CERT_PATH").unwrap_or(String::from("certs/ssl_cert.pem")));
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
        shutdown_send.send(()).unwrap();
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
