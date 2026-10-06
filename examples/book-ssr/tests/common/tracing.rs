use tracing_subscriber::{
    Layer, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

pub fn init_subscriber() {
    let log_filter = tracing_subscriber::filter::Targets::new()
        .with_default(tracing::Level::INFO)
        .with_target("tokio", tracing::Level::WARN)
        .with_target("runtime", tracing::Level::WARN)
        // `BROWSER_TEST_LOG_STEPS=1`: log every `browser_test::step` with its duration.
        .with_target(
            "browser_test::step",
            if std::env::var("BROWSER_TEST_LOG_STEPS").is_ok_and(|v| v == "1") {
                tracing::Level::DEBUG
            } else {
                tracing::Level::INFO
            },
        );

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
}
