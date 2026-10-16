//! The `MenuSection` alone (`.leptonic-MenuSection`, heading "Edit"), for its performance test
//! (`leptonic/tests/perf_tests/atoms/menu_section.rs`): the section "edit" (Cut, Copy, Paste) of a
//! `Menu` (`role="menu"`, not in a popover, without selection) after the section "file" (Open),
//! with a multiple selection of its own, mounted at first. Its controls ([`Control`]) mount and
//! unmount it (the menu stays).

use leptonic::{
    atoms::menu::{Menu, MenuItem, MenuSection},
    hooks::collections::{SelectionMode, use_collection},
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
        b.section("file", |s| {
            s.header("file-header", "File");
            s.item("Open", "Open");
        });
        b.section("edit", |s| {
            s.header("edit-header", "Edit");
            for item in ["Cut", "Copy", "Paste"] {
                s.item(item, item);
            }
        });
    });
    view! {
        <h1>"MenuSection performance"</h1>
        <PerfControls controls />
        <Menu collection=items aria_label="Actions">
            <MenuSection key="file">
                <MenuItem key="Open">"Open"</MenuItem>
            </MenuSection>
            <Show when=move || mounted.get()>
                <MenuSection key="edit" selection_mode=SelectionMode::Multiple>
                    <MenuItem key="Cut">"Cut"</MenuItem>
                    <MenuItem key="Copy">"Copy"</MenuItem>
                    <MenuItem key="Paste">"Paste"</MenuItem>
                </MenuSection>
            </Show>
        </Menu>
    }
}
