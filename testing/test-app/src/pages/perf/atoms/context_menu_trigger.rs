//! The `ContextMenuTrigger` alone, for its performance test
//! (`leptonic/tests/perf_tests/atoms/context_menu_trigger.rs`): around a `GridList` of three rows
//! (Documents, Pictures, Music) and a menu (Open, Rename, Delete) in a `Popover`, next to a plain
//! button (`#perf-before`) to tab from, mounted at first, closed. Its controls ([`Control`]) mount
//! and unmount it, disable and enable it, and read how many actions the menu reported
//! (`actions`).

use leptonic::{
    atoms::{
        grid_list::{GridList, GridListItem},
        menu::{ContextMenuTrigger, Menu, MenuItems},
        popover::Popover,
    },
    hooks::collections::{Key, UseListCollectionInput, use_collection, use_list_collection},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

const FILES: [&str; 3] = ["Documents", "Pictures", "Music"];

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let actions = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("actions", move || f64::from(actions.get_value())),
    ];
    view! {
        <h1>"ContextMenuTrigger performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Subject disabled=disabled.into() actions />
        </Show>
    }
}

/// The trigger around the list, with its menu.
#[component]
fn Subject(disabled: Signal<bool>, actions: StoredValue<u32>) -> impl IntoView {
    let files = use_list_collection(UseListCollectionInput {
        items: Signal::stored(FILES.to_vec()),
        key: |file| Key::from(*file),
        text_value: |file| (*file).to_owned(),
    });
    let items = use_collection(|b| {
        for action in ["Open", "Rename", "Delete"] {
            b.item(action, action);
        }
    });
    view! {
        <ContextMenuTrigger is_disabled=disabled>
            <GridList collection=files aria_label="Files">
                {FILES
                    .map(|file| view! { <GridListItem key=file>{file}</GridListItem> })
                    .collect_view()}
            </GridList>
            <Popover>
                <Menu
                    collection=items
                    on_action=move |_| actions.update_value(|count| *count += 1)
                >
                    <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
                </Menu>
            </Popover>
        </ContextMenuTrigger>
    }
}
