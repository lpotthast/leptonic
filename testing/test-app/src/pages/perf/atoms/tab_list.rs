//! A `TabList` (in `#perf-tabs`) of [`FEW`] tabs (or [`MANY`], control `use-many` while
//! unmounted; "Tab 1", ... with the keys `tab-1`, ...) in its `Tabs`, with a panel per tab, for
//! its performance test (`leptonic/tests/perf_tests/atoms/tab_list.rs`): between two plain
//! buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, and set and unset its label (`aria_label`).

use leptonic::{
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
    let label = RwSignal::new(None::<String>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("use-many", move || count.set(MANY)),
        Control::action("set-label", move || label.set(Some("Sections".to_owned()))),
        Control::action("unset-label", move || label.set(None)),
    ];
    let collection = use_collection(move |b| {
        for i in 1..=count.get() {
            b.item(key(i), format!("Tab {i}"));
        }
    });
    view! {
        <h1>"TabList performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div id="perf-tabs">
            <Tabs collection=collection>
                <Show when=move || mounted.get()>
                    <TabList aria_label=label>
                        {(1..=count.get_untracked())
                            .map(|i| view! { <Tab key=key(i)>{format!("Tab {i}")}</Tab> })
                            .collect_view()}
                    </TabList>
                </Show>
                <For each=move || 1..=count.get() key=|i| *i let:i>
                    <TabPanel key=key(i)>{format!("Panel {i}")}</TabPanel>
                </For>
            </Tabs>
        </div>
        <button id="perf-after">"After"</button>
    }
}
