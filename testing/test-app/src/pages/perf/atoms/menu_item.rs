//! The `MenuItem` alone ("Item 1", key `1`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/menu_item.rs`): between two other items ("Item 0", "Item 2")
//! of a `Menu` (`role="menu"`, not in a popover), next to a plain button (`#perf-before`) to tab
//! from, mounted at first, without selection. Its controls ([`Control`]) mount and unmount it (the
//! menu stays), switch the menu to single selection and back, disable and enable it, and read how
//! many actions the menu reported (`actions`).

use std::collections::HashSet;

use leptonic::{
    atoms::menu::{Menu, MenuItem},
    hooks::collections::{Key, SelectionMode, use_collection},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let selection_mode = RwSignal::new(SelectionMode::None);
    let disabled_keys = RwSignal::new(HashSet::<Key>::new());
    let actions = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("single-selection", move || {
            selection_mode.set(SelectionMode::Single);
        }),
        Control::action("no-selection", move || {
            selection_mode.set(SelectionMode::None);
        }),
        Control::action("disable", move || {
            disabled_keys.set(HashSet::from([Key::from(1)]));
        }),
        Control::action("enable", move || disabled_keys.set(HashSet::new())),
        Control::read("actions", move || f64::from(actions.get_value())),
    ];
    let items = use_collection(|b| {
        for i in 0..3 {
            b.item(i, format!("Item {i}"));
        }
    });
    view! {
        <h1>"MenuItem performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Menu
            collection=items
            aria_label="Actions"
            selection_mode=selection_mode
            disabled_keys=disabled_keys
            on_action=move |_| actions.update_value(|count| *count += 1)
        >
            <MenuItem key=0>"Item 0"</MenuItem>
            <Show when=move || mounted.get()>
                <MenuItem key=1>"Item 1"</MenuItem>
            </Show>
            <MenuItem key=2>"Item 2"</MenuItem>
        </Menu>
    }
}
