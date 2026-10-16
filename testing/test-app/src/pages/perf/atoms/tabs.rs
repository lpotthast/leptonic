//! The `Tabs` alone (in `#perf-tabs`), for their performance test
//! (`leptonic/tests/perf_tests/atoms/tabs.rs`): [`FEW`] tabs (or [`MANY`], control `use-many`
//! while unmounted, "Tab 1", ... with the keys `tab-1`, ...) in a `TabList`, each with its
//! `TabPanel`, the selected key bound (`selected_key` + `set_selected_key`), between two plain
//! buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount them, change their reactive props (disabled, orientation,
//! disabled keys, the selected key), and read how often the selection changed (`changes`, its
//! `on_selected_key_change`).

use std::collections::HashSet;

use leptonic::{
    Orientation,
    atoms::tabs::{Tab, TabList, TabPanel, Tabs},
    hooks::collections::{Key, use_collection},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The tabs of the small tab list.
const FEW: usize = 3;
/// The tabs of the large tab list.
const MANY: usize = 100;

fn key(i: usize) -> Key {
    Key::from(format!("tab-{i}"))
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let count = RwSignal::new(FEW);
    let selected = RwSignal::new(key(1));
    let disabled = RwSignal::new(false);
    let orientation = RwSignal::new(Orientation::Horizontal);
    let disabled_keys = RwSignal::new(HashSet::<Key>::new());
    let change_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("use-many", move || count.set(MANY)),
        Control::action("select-second", move || selected.set(key(2))),
        Control::action("select-first", move || selected.set(key(1))),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("make-vertical", move || {
            orientation.set(Orientation::Vertical);
        }),
        Control::action("make-horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
        Control::action("disable-second", move || {
            disabled_keys.set(HashSet::from([key(2)]));
        }),
        Control::action("enable-second", move || disabled_keys.set(HashSet::new())),
        Control::read("changes", move || f64::from(change_count.get_value())),
    ];
    let collection = use_collection(move |b| {
        for i in 1..=count.get() {
            b.item(key(i), format!("Tab {i}"));
        }
    });
    view! {
        <h1>"Tabs performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div id="perf-tabs">
            <Show when=move || mounted.get()>
                <Tabs
                    collection=collection
                    selected_key=selected
                    set_selected_key=selected
                    on_selected_key_change=move |_| change_count.update_value(|count| *count += 1)
                    is_disabled=disabled
                    orientation=orientation
                    disabled_keys=disabled_keys
                >
                    <TabList aria_label="Tabs">
                        {(1..=count.get_untracked())
                            .map(|i| view! { <Tab key=key(i)>{format!("Tab {i}")}</Tab> })
                            .collect_view()}
                    </TabList>
                    {(1..=count.get_untracked())
                        .map(|i| view! { <TabPanel key=key(i)>{format!("Panel {i}")}</TabPanel> })
                        .collect_view()}
                </Tabs>
            </Show>
        </div>
        <button id="perf-after">"After"</button>
    }
}
