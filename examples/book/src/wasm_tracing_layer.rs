/*
 * NOTE: This was copied (at 2023-01-03) from the unmaintained repository: https://github.com/old-storyai/tracing-wasm/blob/master/src/lib.rs
 */
use core::fmt::{self, Write};
use core::sync::atomic::AtomicUsize;

use tracing::Subscriber;
use tracing::{
    dispatcher::SetGlobalDefaultError,
    field::{Field, Visit},
};
use tracing_subscriber::layer::*;
use tracing_subscriber::registry::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance)]
    fn mark(a: &str);
    #[wasm_bindgen(catch, js_namespace = performance)]
    fn measure(name: String, startMark: String) -> Result<(), JsValue>;
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log1(message: String);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log2(message1: &str, message2: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log3(message1: &str, message2: &str, message3: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log4(message1: String, message2: &str, message3: &str, message4: &str);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_built_config() {
        let builder = WASMLayerConfigBuilder::new();

        let config = builder.build();

        assert_eq!(
            config,
            WASMLayerConfig {
                report_logs_in_timings: true,
                report_logs_in_console: true,
                use_console_color: true,
                max_level: tracing::Level::TRACE,
            }
        )
    }

    #[test]
    fn test_set_report_logs_in_timings() {
        let mut builder = WASMLayerConfigBuilder::new();
        builder.set_report_logs_in_timings(false);

        let config = builder.build();

        assert_eq!(config.report_logs_in_timings, false);
    }

    #[test]
    fn test_set_console_config_no_reporting() {
        let mut builder = WASMLayerConfigBuilder::new();
        builder.set_console_config(ConsoleConfig::NoReporting);

        let config = builder.build();

        assert_eq!(config.report_logs_in_console, false);
        assert_eq!(config.use_console_color, false);
    }

    #[test]
    fn test_set_console_config_without_color() {
        let mut builder = WASMLayerConfigBuilder::new();
        builder.set_console_config(ConsoleConfig::ReportWithoutConsoleColor);

        let config = builder.build();

        assert_eq!(config.report_logs_in_console, true);
        assert_eq!(config.use_console_color, false);
    }

    #[test]
    fn test_set_console_config_with_color() {
        let mut builder = WASMLayerConfigBuilder::new();
        builder.set_console_config(ConsoleConfig::ReportWithConsoleColor);

        let config = builder.build();

        assert_eq!(config.report_logs_in_console, true);
        assert_eq!(config.use_console_color, true);
    }

    #[test]
    fn test_default_config_log_level() {
        let builder = WASMLayerConfigBuilder::new();

        let config = builder.build();

        assert_eq!(config.max_level, tracing::Level::TRACE);
    }

    #[test]
    fn test_set_config_log_level_warn() {
        let mut builder = WASMLayerConfigBuilder::new();
        builder.set_max_level(tracing::Level::WARN);

        let config = builder.build();

        assert_eq!(config.max_level, tracing::Level::WARN);
    }
}

#[allow(unused)]
pub enum ConsoleConfig {
    NoReporting,
    ReportWithoutConsoleColor,
    ReportWithConsoleColor,
}

pub struct WASMLayerConfigBuilder {
    /// Log events will be marked and measured so they appear in performance Timings
    report_logs_in_timings: bool,
    /// Log events will be logged to the browser console
    report_logs_in_console: bool,
    /// Only relevant if report_logs_in_console is true, this will use color style strings in the console.
    use_console_color: bool,
    /// Log events will be reported from this level -- Default is ALL (TRACE)
    max_level: tracing::Level,
}

impl WASMLayerConfigBuilder {
    #[allow(unused)]
    pub fn new() -> WASMLayerConfigBuilder {
        WASMLayerConfigBuilder::default()
    }

    /// Set whether events should appear in performance Timings
    #[allow(unused)]
    pub fn set_report_logs_in_timings(
        &mut self,
        report_logs_in_timings: bool,
    ) -> &mut WASMLayerConfigBuilder {
        self.report_logs_in_timings = report_logs_in_timings;
        self
    }

    /// Set the maximal level on which events should be displayed
    #[allow(unused)]
    pub fn set_max_level(&mut self, max_level: tracing::Level) -> &mut WASMLayerConfigBuilder {
        self.max_level = max_level;
        self
    }

    /// Set if and how events should be displayed in the browser console
    #[allow(unused)]
    pub fn set_console_config(
        &mut self,
        console_config: ConsoleConfig,
    ) -> &mut WASMLayerConfigBuilder {
        match console_config {
            ConsoleConfig::NoReporting => {
                self.report_logs_in_console = false;
                self.use_console_color = false;
            }
            ConsoleConfig::ReportWithoutConsoleColor => {
                self.report_logs_in_console = true;
                self.use_console_color = false;
            }
            ConsoleConfig::ReportWithConsoleColor => {
                self.report_logs_in_console = true;
                self.use_console_color = true;
            }
        }

        self
    }

    /// Build the WASMLayerConfig
    #[allow(unused)]
    pub fn build(&self) -> WASMLayerConfig {
        WASMLayerConfig {
            report_logs_in_timings: self.report_logs_in_timings,
            report_logs_in_console: self.report_logs_in_console,
            use_console_color: self.use_console_color,
            max_level: self.max_level,
        }
    }
}

impl Default for WASMLayerConfigBuilder {
    fn default() -> WASMLayerConfigBuilder {
        WASMLayerConfigBuilder {
            report_logs_in_timings: true,
            report_logs_in_console: true,
            use_console_color: true,
            max_level: tracing::Level::TRACE,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct WASMLayerConfig {
    report_logs_in_timings: bool,
    report_logs_in_console: bool,
    use_console_color: bool,
    max_level: tracing::Level,
}

impl core::default::Default for WASMLayerConfig {
    fn default() -> Self {
        WASMLayerConfig {
            report_logs_in_timings: true,
            report_logs_in_console: true,
            use_console_color: true,
            max_level: tracing::Level::TRACE,
        }
    }
}

/// Implements [tracing_subscriber::layer::Layer] which uses [wasm_bindgen] for marking and measuring with `window.performance`
pub struct WASMLayer {
    last_event_id: AtomicUsize,
    config: WASMLayerConfig,
}

impl WASMLayer {
    pub fn new(config: WASMLayerConfig) -> Self {
        WASMLayer {
            last_event_id: AtomicUsize::new(0),
            config,
        }
    }
}

impl core::default::Default for WASMLayer {
    fn default() -> Self {
        WASMLayer::new(WASMLayerConfig::default())
    }
}

#[cfg(not(feature = "mark-with-rayon-thread-index"))]
#[inline]
fn thread_display_suffix() -> &'static str {
    ""
}
#[cfg(feature = "mark-with-rayon-thread-index")]
fn thread_display_suffix() -> String {
    let mut message = " #".to_string();
    match rayon::current_thread_index() {
        Some(idx) => message.push_str(&format!("{}", idx)),
        None => message.push_str("main"),
    }
    message
}

#[cfg(not(feature = "mark-with-rayon-thread-index"))]
fn mark_name(id: &tracing::Id) -> String {
    format!("t{:x}", id.into_u64())
}
#[cfg(feature = "mark-with-rayon-thread-index")]
fn mark_name(id: &tracing::Id) -> String {
    format!(
        "t{:x}-{}",
        id.into_u64(),
        rayon::current_thread_index().unwrap_or(999)
    )
}

impl<S: Subscriber + for<'a> LookupSpan<'a>> Layer<S> for WASMLayer {
    fn enabled(&self, metadata: &tracing::Metadata<'_>, _: Context<'_, S>) -> bool {
        let level = metadata.level();
        level <= &self.config.max_level
    }

    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::Id,
        ctx: Context<'_, S>,
    ) {
        let mut new_debug_record = StringRecorder::new();
        attrs.record(&mut new_debug_record);

        if let Some(span_ref) = ctx.span(id) {
            span_ref
                .extensions_mut()
                .insert::<StringRecorder>(new_debug_record);
        }
    }

    /// doc: Notifies this layer that a span with the given Id recorded the given values.
    fn on_record(&self, id: &tracing::Id, values: &tracing::span::Record<'_>, ctx: Context<'_, S>) {
        if let Some(span_ref) = ctx.span(id) {
            if let Some(debug_record) = span_ref.extensions_mut().get_mut::<StringRecorder>() {
                values.record(debug_record);
            }
        }
    }

    // /// doc: Notifies this layer that a span with the ID span recorded that it follows from the span with the ID follows.
    // fn on_follows_from(&self, _span: &tracing::Id, _follows: &tracing::Id, ctx: Context<'_, S>) {}
    /// doc: Notifies this layer that an event has occurred.
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        if self.config.report_logs_in_timings || self.config.report_logs_in_console {
            let mut recorder = StringRecorder::new();
            event.record(&mut recorder);
            let meta = event.metadata();
            let level = meta.level();
            if self.config.report_logs_in_console {
                let origin = meta
                    .file()
                    .and_then(|file| meta.line().map(|ln| format!("{}:{}", file, ln)))
                    .unwrap_or_default();

                if self.config.use_console_color {
                    log4(
                        format!(
                            "%c{}%c {}{}%c{}",
                            level,
                            origin,
                            thread_display_suffix(),
                            recorder,
                        ),
                        match *level {
                            tracing::Level::TRACE => "color: dodgerblue; background: #444",
                            tracing::Level::DEBUG => "color: lawngreen; background: #444",
                            tracing::Level::INFO => "color: whitesmoke; background: #444",
                            tracing::Level::WARN => "color: orange; background: #444",
                            tracing::Level::ERROR => "color: red; background: #444",
                        },
                        "color: gray; font-style: italic",
                        "color: inherit",
                    );
                } else {
                    log1(format!(
                        "{} {}{} {}",
                        level,
                        origin,
                        thread_display_suffix(),
                        recorder,
                    ));
                }
            }
            if self.config.report_logs_in_timings {
                let mark_name = format!(
                    "c{:x}",
                    self.last_event_id
                        .fetch_add(1, core::sync::atomic::Ordering::Relaxed)
                );
                // mark and measure so you can see a little blip in the profile
                mark(&mark_name);
                let _ = measure(
                    format!(
                        "{} {}{} {}",
                        level,
                        meta.module_path().unwrap_or("..."),
                        thread_display_suffix(),
                        recorder,
                    ),
                    mark_name,
                );
            }
        }
    }
    /// doc: Notifies this layer that a span with the given ID was entered.
    fn on_enter(&self, id: &tracing::Id, _ctx: Context<'_, S>) {
        mark(&mark_name(id));
    }
    /// doc: Notifies this layer that the span with the given ID was exited.
    fn on_exit(&self, id: &tracing::Id, ctx: Context<'_, S>) {
        if let Some(span_ref) = ctx.span(id) {
            let meta = span_ref.metadata();
            if let Some(debug_record) = span_ref.extensions().get::<StringRecorder>() {
                let _ = measure(
                    format!(
                        "\"{}\"{} {} {}",
                        meta.name(),
                        thread_display_suffix(),
                        meta.module_path().unwrap_or("..."),
                        debug_record,
                    ),
                    mark_name(id),
                );
            } else {
                let _ = measure(
                    format!(
                        "\"{}\"{} {}",
                        meta.name(),
                        thread_display_suffix(),
                        meta.module_path().unwrap_or("..."),
                    ),
                    mark_name(id),
                );
            }
        }
    }
    // /// doc: Notifies this layer that the span with the given ID has been closed.
    // /// We can dispose of any data for the span we might have here...
    // fn on_close(&self, _id: tracing::Id, ctx: Context<'_, S>) {}
    // /// doc: Notifies this layer that a span ID has been cloned, and that the subscriber returned a different ID.
    // /// I'm not sure if I need to do something here...
    // fn on_id_change(&self, _old: &tracing::Id, _new: &tracing::Id, ctx: Context<'_, S>) {}
}

const NOT_ALREADY_SET: &str = "No global default to have already been set. Global default can only be set once!";

/// Set the global default with [tracing::subscriber::set_global_default] and expect the operation to succeed.
/// This might panic if a global default was already set.
#[allow(unused)]
pub fn set_as_global_default() {
    try_set_as_global_default().expect(NOT_ALREADY_SET);
}

/// Set the global default with [tracing::subscriber::set_global_default].
#[allow(unused)]
pub fn try_set_as_global_default() -> Result<(), SetGlobalDefaultError> {
    tracing::subscriber::set_global_default(
        Registry::default().with(WASMLayer::new(WASMLayerConfig::default())),
    )
}

/// Set the global default with [tracing::subscriber::set_global_default] and expect the operation to succeed.
/// This might panic if a global default was already set.
#[allow(unused)]
pub fn set_as_global_default_with_config(config: WASMLayerConfig) {
    try_set_as_global_default_with_config(config).expect(NOT_ALREADY_SET);
}

/// Set the global default with [tracing::subscriber::set_global_default].
#[allow(unused)]
pub fn try_set_as_global_default_with_config(config: WASMLayerConfig) -> Result<(), SetGlobalDefaultError> {
    tracing::subscriber::set_global_default(Registry::default().with(WASMLayer::new(config)))
}

struct StringRecorder {
    display: String,
    is_following_args: bool,
}
impl StringRecorder {
    fn new() -> Self {
        StringRecorder {
            display: String::new(),
            is_following_args: false,
        }
    }
}

impl Visit for StringRecorder {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            if !self.display.is_empty() {
                self.display = format!("{:?}\n{}", value, self.display)
            } else {
                self.display = format!("{:?}", value)
            }
        } else {
            if self.is_following_args {
                // following args
                writeln!(self.display).unwrap();
            } else {
                // first arg
                write!(self.display, " ").unwrap();
                self.is_following_args = true;
            }
            write!(self.display, "{} = {:?};", field.name(), value).unwrap();
        }
    }
}

impl core::fmt::Display for StringRecorder {
    fn fmt(&self, mut f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if !self.display.is_empty() {
            write!(&mut f, " {}", self.display)
        } else {
            Ok(())
        }
    }
}

impl core::default::Default for StringRecorder {
    fn default() -> Self {
        StringRecorder::new()
    }
}
