//! The `ToggleButton` alone: uncontrolled (`#perf-toggle`) and controlled
//! (`#perf-toggle-controlled`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/toggle_button.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, both mounted at first. Its controls
//! ([`Control`]) mount and unmount the uncontrolled one, disable and enable it, select and
//! deselect the controlled one, and read how often the uncontrolled one's selection changed
//! (`changes`).

use leptonic::atoms::toggle_button::ToggleButton;
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let controlled = RwSignal::new(false);
    let change_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("select-controlled", move || controlled.set(true)),
        Control::action("deselect-controlled", move || controlled.set(false)),
        Control::read("changes", move || f64::from(change_count.get_value())),
    ];
    view! {
        <h1>"ToggleButton performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <ToggleButton
                attr:id="perf-toggle"
                is_disabled=disabled
                on_change=move |_| change_count.update_value(|count| *count += 1)
            >
                "Toggle me"
            </ToggleButton>
        </Show>
        <button id="perf-after">"After"</button>
        <ToggleButton
            attr:id="perf-toggle-controlled"
            is_selected=controlled
            set_selected=controlled
        >
            "Controlled"
        </ToggleButton>
    }
}
