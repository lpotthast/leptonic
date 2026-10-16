//! `TabPanels` (`#perf-tabs .leptonic-TabPanels`) around the panels of three tabs ("Panel 1"
//! one line high, "Panel 2" five, "Panel 3" ten: the selection changes their size) in their
//! `Tabs` (in `#perf-tabs`, the selected key bound), for its performance test
//! (`leptonic/tests/perf_tests/atoms/tab_panels.rs`), mounted at first. Its controls
//! ([`Control`]) mount and unmount it (`use-transition` while unmounted: with a CSS transition of
//! its height, of zero duration, so that it animates its size without waiting for the
//! animation), and select the second tab and the first again.

use leptonic::{
    atoms::tabs::{Tab, TabList, TabPanel, TabPanels, Tabs},
    hooks::collections::{Key, use_collection},
    leptos_styles::Styles,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The lines of the panel of tab `i`.
fn lines(i: usize) -> Vec<String> {
    let count = match i {
        1 => 1,
        2 => 5,
        _ => 10,
    };
    (1..=count)
        .map(|line| format!("Panel {i}, line {line}"))
        .collect()
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let transition = RwSignal::new(false);
    let selected = RwSignal::new(Key::from("tab-1"));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("use-transition", move || transition.set(true)),
        Control::action("select-second", move || selected.set(Key::from("tab-2"))),
        Control::action("select-first", move || selected.set(Key::from("tab-1"))),
    ];
    let collection = use_collection(|b| {
        for i in 1..=3 {
            b.item(format!("tab-{i}"), format!("Tab {i}"));
        }
    });
    let panels_styles = move || {
        if transition.get_untracked() {
            Styles::new().add_unchecked("transition", "height 0s")
        } else {
            Styles::new()
        }
    };
    view! {
        <h1>"TabPanels performance"</h1>
        <PerfControls controls />
        <div id="perf-tabs">
            <Tabs collection=collection selected_key=selected set_selected_key=selected>
                <TabList aria_label="Tabs">
                    {(1..=3)
                        .map(|i| view! { <Tab key=format!("tab-{i}")>{format!("Tab {i}")}</Tab> })
                        .collect_view()}
                </TabList>
                <Show when=move || mounted.get()>
                    <TabPanels styles=panels_styles()>
                        {(1..=3)
                            .map(|i| {
                                view! {
                                    <TabPanel key=format!("tab-{i}")>
                                        {lines(i)
                                            .into_iter()
                                            .map(|line| view! { <p>{line}</p> })
                                            .collect_view()}
                                    </TabPanel>
                                }
                            })
                            .collect_view()}
                    </TabPanels>
                </Show>
            </Tabs>
        </div>
    }
}
