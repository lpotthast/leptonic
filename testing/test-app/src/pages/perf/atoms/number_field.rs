//! A `NumberField<i32>` (its input `#perf-input`, labelled "Quantity", 0 to 100) with its group
//! and stepper buttons, for its performance test
//! (`leptonic/tests/perf_tests/atoms/number_field.rs`): mounted at first, its value the page's
//! (5 at first), between two plain buttons (`#perf-before`, `#perf-after`). Its controls
//! ([`Control`]) mount and unmount it, change its reactive props (disabled, read-only, the
//! maximum: `limit-to-5`) and its value (`set-value`: 42), and read the value (`value`, -1:
//! empty).

use leptonic::atoms::{
    field::Label,
    input::Input,
    number_field::{
        NumberField, NumberFieldDecrementButton, NumberFieldGroup, NumberFieldIncrementButton,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(Some(5_i32));
    let max_value = RwSignal::new(Some(100_i32));
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("set-read-only", move || read_only.set(true)),
        Control::action("unset-read-only", move || read_only.set(false)),
        Control::action("limit-to-5", move || max_value.set(Some(5))),
        Control::action("unlimit", move || max_value.set(Some(100))),
        Control::action("set-value", move || value.set(Some(42))),
        Control::read("value", move || {
            f64::from(value.get_untracked().unwrap_or(-1))
        }),
    ];
    view! {
        <h1>"NumberField performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <NumberField
                id="perf-input"
                value=value
                set_value=value
                min_value=0
                max_value=max_value
                is_disabled=disabled
                is_read_only=read_only
            >
                <Label>"Quantity"</Label>
                <NumberFieldGroup>
                    <NumberFieldDecrementButton>"-"</NumberFieldDecrementButton>
                    <Input />
                    <NumberFieldIncrementButton>"+"</NumberFieldIncrementButton>
                </NumberFieldGroup>
            </NumberField>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
