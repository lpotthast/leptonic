//! The `MenuItemShortcut` alone (`.leptonic-MenuItemShortcut`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/menu_item_shortcut.rs`): the keyboard shortcut of the item
//! "Open" of a `Menu` (`role="menu"`, not in a popover), mounted at first. Its controls
//! ([`Control`]) mount and unmount it (the item stays).

use leptonic::{
    atoms::menu::{Menu, MenuItem, MenuItemDescription, MenuItemLabel, MenuItemShortcut},
    hooks::collections::use_collection,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
    ];
    let items = use_collection(|b| {
        b.item("open", "Open");
    });
    view! {
        <h1>"MenuItemShortcut performance"</h1>
        <PerfControls controls />
        <Menu collection=items aria_label="File">
            <MenuItem key="open">
                <MenuItemLabel>"Open"</MenuItemLabel>
                <MenuItemDescription>"Open a file"</MenuItemDescription>
                <Show when=move || mounted.get()>
                    <MenuItemShortcut>"Ctrl+O"</MenuItemShortcut>
                </Show>
            </MenuItem>
        </Menu>
    }
}
