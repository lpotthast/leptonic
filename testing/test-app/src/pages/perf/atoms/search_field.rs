//! A `SearchField` (its input `#perf-input`, labelled "Search", value "leptos" at first) with a
//! clear button, for its performance test (`leptonic/tests/perf_tests/atoms/search_field.rs`):
//! mounted at first, between two plain buttons (`#perf-before`, `#perf-after`). Its controls
//! ([`Control`]) mount and unmount it, disable and enable it, and read how often it submitted
//! (`submits`) and cleared (`clears`).

use leptonic::atoms::{
    field::Label,
    input::Input,
    search_field::{SearchField, SearchFieldClearButton},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let submits = StoredValue::new(0_u32);
    let clears = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("submits", move || f64::from(submits.get_value())),
        Control::read("clears", move || f64::from(clears.get_value())),
    ];
    view! {
        <h1>"SearchField performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <SearchField
                id="perf-input"
                default_value="leptos"
                is_disabled=disabled
                on_submit=move |_| submits.update_value(|submits| *submits += 1)
                on_clear=move |()| clears.update_value(|clears| *clears += 1)
            >
                <Label>"Search"</Label>
                <Input />
                <SearchFieldClearButton>"Clear"</SearchFieldClearButton>
            </SearchField>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
