//! The `CheckboxButton` (`#perf-checkbox`: the `<label>` with its hidden input) in its
//! `CheckboxField`, for its performance test (`leptonic/tests/perf_tests/atoms/checkbox_button.rs`):
//! unchecked and uncontrolled, between two plain buttons (`#perf-before`, `#perf-after`) to tab
//! from and to, mounted at first. Its controls ([`Control`]) mount and unmount it, disable it and
//! make it read-only (for the presses it ignores), and read whether it is checked (`selected`) and
//! how often it changed (`changes`).

use leptonic::atoms::checkbox::{CheckboxButton, CheckboxField};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let selected = StoredValue::new(false);
    let changes = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("make-read-only", move || read_only.set(true)),
        Control::read("selected", move || {
            f64::from(u8::from(selected.get_value()))
        }),
        Control::read("changes", move || f64::from(changes.get_value())),
    ];
    let on_change = move |is_selected: bool| {
        selected.set_value(is_selected);
        changes.update_value(|count| *count += 1);
    };
    view! {
        <h1>"CheckboxButton performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <CheckboxField is_disabled=disabled is_read_only=read_only on_change>
                <CheckboxButton attr:id="perf-checkbox">"Check me"</CheckboxButton>
            </CheckboxField>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
