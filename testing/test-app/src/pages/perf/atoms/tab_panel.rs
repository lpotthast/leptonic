//! A `TabPanel` (`#perf-subject > .leptonic-TabPanel`, "Panel 1") of the first of two tabs, in
//! its `Tabs` (in `#perf-tabs`, the selected key bound), for its performance test
//! (`leptonic/tests/perf_tests/atoms/tab_panel.rs`): selected at first, the second tab's panel
//! after it, between two plain buttons (`#perf-before`, `#perf-after`) to tab from and to,
//! mounted at first. Its controls ([`Control`]) mount and unmount it (`use-force-mount` while
//! unmounted: mounted while not selected, `should_force_mount`), select the second tab and the
//! first again, and set and unset its label (`aria_label`).

use leptonic::{
    atoms::tabs::{Tab, TabList, TabPanel, Tabs},
    hooks::collections::{Key, use_collection},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let force_mount = RwSignal::new(false);
    let selected = RwSignal::new(Key::from("tab-1"));
    let label = RwSignal::new(None::<String>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("use-force-mount", move || force_mount.set(true)),
        Control::action("select-second", move || selected.set(Key::from("tab-2"))),
        Control::action("select-first", move || selected.set(Key::from("tab-1"))),
        Control::action("set-label", move || label.set(Some("First".to_owned()))),
        Control::action("unset-label", move || label.set(None)),
    ];
    let collection = use_collection(|b| {
        b.item("tab-1", "Tab 1");
        b.item("tab-2", "Tab 2");
    });
    view! {
        <h1>"TabPanel performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div id="perf-tabs">
            <Tabs collection=collection selected_key=selected set_selected_key=selected>
                <TabList aria_label="Tabs">
                    <Tab key="tab-1">"Tab 1"</Tab>
                    <Tab key="tab-2">"Tab 2"</Tab>
                </TabList>
                <div id="perf-subject">
                    <Show when=move || mounted.get()>
                        <TabPanel
                            key="tab-1"
                            should_force_mount=force_mount.get_untracked()
                            aria_label=label
                        >
                            "Panel 1"
                        </TabPanel>
                    </Show>
                </div>
                <TabPanel key="tab-2">"Panel 2"</TabPanel>
            </Tabs>
        </div>
        <button id="perf-after">"After"</button>
    }
}
