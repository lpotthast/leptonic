//! The `Disclosure` alone (`#perf-disclosure`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/disclosure.rs`): its trigger (`#perf-trigger`, a `Button` in a
//! `DisclosureTrigger`) and its panel (`.leptonic-DisclosurePanel`) with a button
//! (`#perf-panel-button`), between two plain buttons (`#perf-before`, `#perf-after`) to tab from
//! and to, mounted at first, collapsed. Its expanded state is the app's (`is_expanded` +
//! `set_expanded`). Its controls ([`Control`]) mount and unmount it, expand and collapse it (the
//! app's state), disable and enable it, and read how often it reported a change (`changes`).

use leptonic::atoms::{
    button::Button,
    disclosure::{Disclosure, DisclosurePanel, DisclosureTrigger},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let expanded = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let changes = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("expand", move || expanded.set(true)),
        Control::action("collapse", move || expanded.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("changes", move || f64::from(changes.get_value())),
    ];
    view! {
        <h1>"Disclosure performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Disclosure
                attr:id="perf-disclosure"
                is_expanded=expanded
                set_expanded=expanded
                is_disabled=disabled
                on_expanded_change=move |_| changes.update_value(|count| *count += 1)
            >
                <DisclosureTrigger>
                    <Button id="perf-trigger">"Details"</Button>
                </DisclosureTrigger>
                <DisclosurePanel>
                    <button id="perf-panel-button">
                        "Content"
                    </button>
                </DisclosurePanel>
            </Disclosure>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
