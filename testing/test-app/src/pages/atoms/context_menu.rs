use leptonic::{
    atoms::{
        grid_list::{GridList, GridListItem},
        menu::{ContextMenuTrigger, Menu, MenuItems, use_context_menu_target},
        popover::Popover,
    },
    hooks::collections::{Key, UseListCollectionInput, use_collection, use_list_collection},
};
use leptos::prelude::*;

const FILES: [&str; 3] = ["Documents", "Pictures", "Music"];

/// Context menus on the rows of a grid list: a `ContextMenuTrigger` around the list and a menu
/// (Open, Rename, Delete). Each action is appended to `#test-cm-actions` as "Action Row".
#[component]
pub fn PageAtomContextMenu() -> impl IntoView {
    let files = use_list_collection(UseListCollectionInput {
        items: Signal::stored(FILES.to_vec()),
        key: |file| Key::from(*file),
        text_value: |file| (*file).to_owned(),
    });
    let actions = RwSignal::new(Vec::<String>::new());
    view! {
        <h1>"Context menus"</h1>
        <button id="test-cm-before">"Before"</button>
        <ContextMenuTrigger>
            <GridList collection=files aria_label="Files">
                {FILES.map(|file| view! { <GridListItem key=file>{file}</GridListItem> }).collect_view()}
            </GridList>
            <FileMenu actions=actions />
        </ContextMenuTrigger>
        <p id="test-cm-actions">{move || actions.get().join(", ")}</p>
    }
}

/// The menu, telling the action and the row it was opened on.
#[component]
fn FileMenu(actions: RwSignal<Vec<String>>) -> impl IntoView {
    let target = use_context_menu_target();
    let items = use_collection(|b| {
        for action in ["Open", "Rename", "Delete"] {
            b.item(action, action);
        }
    });
    view! {
        <Popover>
            <Menu
                collection=items
                on_action=move |key: Key| {
                    let row = target.get_untracked().map(|row| row.to_string()).unwrap_or_default();
                    actions.update(|actions| actions.push(format!("{key} {row}")));
                }
            >
                <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
            </Menu>
        </Popover>
    }
}
