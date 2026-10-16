//! The `NumberFieldGroup` of a `NumberField<i32>` (labelled "Quantity", value 5), holding its
//! input (`#perf-input`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/number_field_group.rs`): mounted at first, between two
//! plain buttons (`#perf-before`, `#perf-after`). Its controls ([`Control`]) mount and unmount
//! the group (the field stays) and disable, enable, invalidate and uninvalidate the field.

use leptonic::atoms::{
    field::Label,
    input::Input,
    number_field::{NumberField, NumberFieldGroup},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("invalidate", move || invalid.set(true)),
        Control::action("uninvalidate", move || invalid.set(false)),
    ];
    view! {
        <h1>"NumberFieldGroup performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <NumberField
            id="perf-input"
            default_value=5_i32
            is_disabled=disabled
            is_invalid=invalid
        >
            <Label>"Quantity"</Label>
            <Show when=move || mounted.get()>
                <NumberFieldGroup>
                    <Input />
                </NumberFieldGroup>
            </Show>
        </NumberField>
        <button id="perf-after">"After"</button>
    }
}
