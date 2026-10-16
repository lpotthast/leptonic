//! The `ProgressBarValueText` (`#perf-progress-bar-value-text`) in a `ProgressBar` (at 25 of 0 to
//! 100, formatted as a percentage), for its performance test
//! (`leptonic/tests/perf_tests/atoms/progress_bar_value_text.rs`), mounted at first. Its controls
//! ([`Control`]) mount and unmount the text (the `ProgressBar` stays), and change the value (25 or
//! 75, or none: indeterminate) and the format (a percentage or a decimal).

use leptonic::{
    NumberFormatOptions, NumberStyle,
    atoms::progress_bar::{ProgressBar, ProgressBarValueText},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The default format: a percentage.
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
    let format = RwSignal::new(percent());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("raise", move || value.set(Some(75.0))),
        Control::action("lower", move || value.set(Some(25.0))),
        Control::action("make-indeterminate", move || value.set(None)),
        Control::action("format-decimal", move || {
            format.set(NumberFormatOptions::default());
        }),
        Control::action("format-percent", move || format.set(percent())),
    ];
    view! {
        <h1>"ProgressBarValueText performance"</h1>
        <PerfControls controls />
        <ProgressBar value=value format_options=format aria_label="Progress">
            <Show when=move || mounted.get()>
                <ProgressBarValueText attr:id="perf-progress-bar-value-text" />
            </Show>
        </ProgressBar>
    }
}
