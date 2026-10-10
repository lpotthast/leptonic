use std::str::FromStr;

use leptos_browser_test::{Report, report};
use tracing_subscriber::{
    Layer, filter::Targets, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

/// The log filter when `BROWSER_TEST_LOG` is unset or empty: the run's milestones, warnings and errors. A failing test
/// is explained by its failure report, not by logs.
const DEFAULT_LOG_FILTER: &str = "info,tokio=warn,runtime=warn";

/// Logs one line per event, filtered by `BROWSER_TEST_LOG`: a level (`debug`) or levels per target
/// (`info,browser_test::step=debug` logs every step with its duration), as `tracing-subscriber`'s `Targets` parses
/// them. A plain level also applies to tokio's events; `debug,tokio=warn,runtime=warn` keeps them out.
///
/// # Errors
///
/// Returns an error if `BROWSER_TEST_LOG` is no such filter.
pub fn init_subscriber() -> Result<(), Report> {
    let directives = match std::env::var("BROWSER_TEST_LOG") {
        Ok(directives) if !directives.is_empty() => directives,
        _ => DEFAULT_LOG_FILTER.to_owned(),
    };
    let log_filter = Targets::from_str(&directives).map_err(|error| {
        report!(
            "BROWSER_TEST_LOG is {directives:?}, expected a level (`debug`) or levels per target \
             (`info,browser_test::step=debug`): {error}"
        )
        .into_dynamic()
    })?;

    let fmt_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .with_thread_names(false)
        .with_thread_ids(false);

    tracing_subscriber::Registry::default()
        .with(fmt_layer.with_filter(log_filter))
        .init();
    Ok(())
}
