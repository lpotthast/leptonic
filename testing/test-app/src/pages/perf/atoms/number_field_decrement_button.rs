//! The `NumberFieldDecrementButton` ("-") of a `NumberField<i32>` (its input `#perf-input`,
//! labelled "Quantity", 0 to 10), for its performance test
//! (`leptonic/tests/perf_tests/atoms/number_field_decrement_button.rs`): mounted at first, the
//! field's value the page's (5 at first), between two plain buttons (`#perf-before`,
//! `#perf-after`). Its controls ([`Control`]) mount and unmount the button (the field stays),
//! set the value to the minimum (`set-minimum`) and read it (`value`).

use leptonic::atoms::{
    field::Label,
    input::Input,
    number_field::{NumberField, NumberFieldDecrementButton, NumberFieldGroup},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(Some(5_i32));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-minimum", move || value.set(Some(0))),
        Control::read("value", move || {
            f64::from(value.get_untracked().unwrap_or(-1))
        }),
    ];
    view! {
        <h1>"NumberFieldDecrementButton performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <NumberField id="perf-input" value=value set_value=value min_value=0 max_value=10>
            <Label>"Quantity"</Label>
            <NumberFieldGroup>
                <Show when=move || mounted.get()>
                    <NumberFieldDecrementButton>"-"</NumberFieldDecrementButton>
                </Show>
                <Input />
            </NumberFieldGroup>
        </NumberField>
        <button id="perf-after">"After"</button>
    }
}
