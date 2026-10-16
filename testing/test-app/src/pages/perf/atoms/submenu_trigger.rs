//! The `SubmenuTrigger` alone, for its performance test
//! (`leptonic/tests/perf_tests/atoms/submenu_trigger.rs`): the item "Share" (key `share`, first)
//! of a `Menu` (`role="menu"`, not in a popover; then "Copy" and "Delete") opening a submenu
//! (Email, Link) in a `Popover`, without delay, next to a plain button (`#perf-before`) to tab
//! from, mounted at first, closed. Its controls ([`Control`]) mount and unmount it (the menu
//! stays).

use std::time::Duration;

use leptonic::{
    atoms::{
        menu::{Menu, MenuItem, MenuItems, SubmenuTrigger},
        popover::Popover,
    },
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
        b.item("share", "Share");
        b.item("copy", "Copy");
        b.item("delete", "Delete");
    });
    let share_items = use_collection(|b| {
        b.item("email", "Email");
        b.item("link", "Link");
    });
    view! {
        <h1>"SubmenuTrigger performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div style="width: 200px;">
            <Menu collection=items aria_label="Actions">
                <Show when=move || mounted.get()>
                    <SubmenuTrigger key="share" delay=Duration::ZERO>
                        <MenuItem key="share">"Share"</MenuItem>
                        <Popover>
                            <Menu collection=share_items>
                                <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
                            </Menu>
                        </Popover>
                    </SubmenuTrigger>
                </Show>
                <MenuItem key="copy">"Copy"</MenuItem>
                <MenuItem key="delete">"Delete"</MenuItem>
            </Menu>
        </div>
    }
}
