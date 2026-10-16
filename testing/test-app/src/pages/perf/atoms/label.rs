//! The `Label` of a `TextField` (its text "Name"), for its performance test
//! (`leptonic/tests/perf_tests/atoms/label.rs`): mounted at first, in a text field whose input
//! (`#perf-input`) also has an `aria-label`, so that the field stays labelled without it. A
//! plain `Label` outside a field ("Plain") is unmounted at first. Between two plain buttons
//! (`#perf-before`, `#perf-after`). Its controls ([`Control`]) mount and unmount both.

use leptonic::atoms::{field::Label, input::Input, text_field::TextField};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let plain_mounted = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("mount-plain", move || plain_mounted.set(true)),
        Control::action("unmount-plain", move || plain_mounted.set(false)),
    ];
    view! {
        <h1>"Label performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <TextField id="perf-input" aria_label="Name">
            <Show when=move || mounted.get()>
                <Label>"Name"</Label>
            </Show>
            <Input />
        </TextField>
        <Show when=move || plain_mounted.get()>
            <Label>"Plain"</Label>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
