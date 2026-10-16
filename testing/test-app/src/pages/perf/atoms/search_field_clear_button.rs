//! The `SearchFieldClearButton` ("Clear") of a `SearchField` (its input `#perf-input`, labelled
//! "Search", value "leptos" at first), for its performance test
//! (`leptonic/tests/perf_tests/atoms/search_field_clear_button.rs`): mounted at first, between
//! two plain buttons (`#perf-before`, `#perf-after`). Its controls ([`Control`]) mount and
//! unmount the button (the field stays), disable and enable the field, and read how often the
//! field cleared (`clears`).

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
    let clears = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("clears", move || f64::from(clears.get_value())),
    ];
    view! {
        <h1>"SearchFieldClearButton performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <SearchField
            id="perf-input"
            default_value="leptos"
            is_disabled=disabled
            on_clear=move |()| clears.update_value(|clears| *clears += 1)
        >
            <Label>"Search"</Label>
            <Input />
            <Show when=move || mounted.get()>
                <SearchFieldClearButton>"Clear"</SearchFieldClearButton>
            </Show>
        </SearchField>
        <button id="perf-after">"After"</button>
    }
}
