//! The `MenuItemLabel` alone (`.leptonic-MenuItemLabel`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/menu_item_label.rs`): the label of the item "Open" of a `Menu`
//! (`role="menu"`, not in a popover), mounted at first. Its controls ([`Control`]) mount and
//! unmount it (the item stays).

use leptonic::{
    atoms::menu::{Menu, MenuItem, MenuItemDescription, MenuItemLabel},
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
        <h1>"MenuItemLabel performance"</h1>
        <PerfControls controls />
        <Menu collection=items aria_label="File">
            <MenuItem key="open">
                <Show when=move || mounted.get()>
                    <MenuItemLabel>"Open"</MenuItemLabel>
                </Show>
                <MenuItemDescription>"Open a file"</MenuItemDescription>

            </MenuItem>
        </Menu>
    }
}
