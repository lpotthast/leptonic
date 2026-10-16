//! `use_progress_bar` on a plain `<div>` (`#perf-progress-bar`, named by `aria_label`) with its
//! value text (`#perf-progress-bar-text`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_progress_bar.rs`): at 25 of 0 to 100, formatted as a
//! percentage, mounted at first. Its controls ([`Control`]) mount and unmount it and change the
//! hook's inputs: the value (25 or 75, or none: indeterminate), the maximum (100 or 200), the
//! format (a percentage or a decimal) and the value label.

use leptonic::{
    IntoAttrs, NumberFormatOptions, NumberStyle,
    hooks::progress::{UseProgressBarInput, use_progress_bar},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The hook's default format: a percentage.
fn percent() -> NumberFormatOptions {
    NumberFormatOptions {
        style: NumberStyle::Percent,
        ..NumberFormatOptions::default()
    }
}

/// The progress bar: `use_progress_bar` on a `<div>`, which shows the value text.
#[component]
fn Subject(
    #[prop(into)] value: Signal<Option<f64>>,
    #[prop(into)] max: Signal<f64>,
    #[prop(into)] format: Signal<NumberFormatOptions>,
    #[prop(into)] value_label: Signal<Option<String>>,
) -> impl IntoView {
    let progress = use_progress_bar(UseProgressBarInput {
        value,
        max_value: max,
        format_options: format,
        value_label: value_label.into(),
        id: Some("perf-progress-bar".to_owned()),
        aria_label: "Storage".into(),
        ..UseProgressBarInput::default()
    });
    view! {
        <div {..progress.props.into_attrs()}>
            <span id="perf-progress-bar-text">{progress.value_text}</span>
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(Some(25.0_f64));
    let max = RwSignal::new(100.0_f64);
    let format = RwSignal::new(percent());
    let value_label = RwSignal::new(None::<String>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("raise", move || value.set(Some(75.0))),
        Control::action("lower", move || value.set(Some(25.0))),
        Control::action("make-indeterminate", move || value.set(None)),
        Control::action("widen", move || max.set(200.0)),
        Control::action("narrow", move || max.set(100.0)),
        Control::action("format-decimal", move || {
            format.set(NumberFormatOptions::default());
        }),
        Control::action("format-percent", move || format.set(percent())),
        Control::action("set-value-label", move || {
            value_label.set(Some("A quarter".to_owned()));
        }),
        Control::action("unset-value-label", move || value_label.set(None)),
    ];
    view! {
        <h1>"use_progress_bar performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Subject value max format value_label />
        </Show>
    }
}
