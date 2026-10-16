//! The `ProgressBar` (`#perf-progress-bar`, named by `aria_label`, without parts), for its
//! performance test (`leptonic/tests/perf_tests/atoms/progress_bar.rs`): at 25 of 0 to 100,
//! formatted as a percentage, mounted at first. Its controls ([`Control`]) mount and unmount it
//! and change its reactive props: the value (25, 75 or none: indeterminate), the maximum (100 or
//! 200), the format (a percentage or a decimal) and the value label.

use leptonic::{NumberFormatOptions, NumberStyle, atoms::progress_bar::ProgressBar};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The progress bar's default format: a percentage.
fn percent() -> NumberFormatOptions {
    NumberFormatOptions {
        style: NumberStyle::Percent,
        ..NumberFormatOptions::default()
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
        <h1>"ProgressBar performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <ProgressBar
                id="perf-progress-bar"
                value=value
                max_value=max
                format_options=format
                value_label=value_label
                aria_label="Loading"
            />
        </Show>
    }
}
