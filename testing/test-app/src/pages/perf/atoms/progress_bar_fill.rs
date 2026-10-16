//! The `ProgressBarFill` (`#perf-progress-bar-fill`) in a `ProgressBar` (at 25 of 0 to 100), in a
//! 200px track, for its performance test (`leptonic/tests/perf_tests/atoms/progress_bar_fill.rs`),
//! mounted at first. Its controls ([`Control`]) mount and unmount the fill (the `ProgressBar`
//! stays) and change the value (25 or 75, or none: indeterminate).

use leptonic::atoms::progress_bar::{ProgressBar, ProgressBarFill};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(Some(25.0_f64));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("raise", move || value.set(Some(75.0))),
        Control::action("lower", move || value.set(Some(25.0))),
        Control::action("make-indeterminate", move || value.set(None)),
    ];
    view! {
        <h1>"ProgressBarFill performance"</h1>
        <PerfControls controls />
        <ProgressBar value=value aria_label="Progress">
            <div style="width: 200px; height: 8px;">
                <Show when=move || mounted.get()>
                    <ProgressBarFill attr:id="perf-progress-bar-fill" />
                </Show>
            </div>
        </ProgressBar>
    }
}
