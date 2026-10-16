//! A `Tab` (`#perf-tabs [role=tablist] > :nth-child(2)`, "Tab 2") in its `Tabs` (in
//! `#perf-tabs`), for its performance test (`leptonic/tests/perf_tests/atoms/tab.rs`): after
//! the first tab ("Tab 1", selected at first), with a panel per tab, manual activation (a focused
//! tab is selected by Enter or Space, not by focus), between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first. Its controls ([`Control`]) mount and
//! unmount it, disable and enable it (its `is_disabled`), and read how often the selection
//! changed (`changes`).

use leptonic::{
    atoms::tabs::{Tab, TabList, TabPanel, Tabs},
    hooks::{
        collections::{Key, use_collection},
        tabs::KeyboardActivation,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let change_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("changes", move || f64::from(change_count.get_value())),
    ];
    let collection = use_collection(|b| {
        b.item("tab-1", "Tab 1");
        b.item("tab-2", "Tab 2");
    });
    view! {
        <h1>"Tab performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div id="perf-tabs">
            <Tabs
                collection=collection
                keyboard_activation=KeyboardActivation::Manual
                on_selected_key_change=move |_: Key| change_count.update_value(|count| *count += 1)
            >
                <TabList aria_label="Tabs">
                    <Tab key="tab-1">"Tab 1"</Tab>
                    <Show when=move || mounted.get()>
                        <Tab key="tab-2" is_disabled=disabled>
                            "Tab 2"
                        </Tab>
                    </Show>
                </TabList>
                <TabPanel key="tab-1">"Panel 1"</TabPanel>
                <TabPanel key="tab-2">"Panel 2"</TabPanel>
            </Tabs>
        </div>
        <button id="perf-after">"After"</button>
    }
}
