use leptonic::{
    atoms::{
        grid_list::{GridList, GridListItem},
        prelude::{ContextMenuTrigger, Menu, MenuItems, Popover, use_context_menu_target},
    },
    hooks::collections::{Key, use_collection, use_list_collection},
};
use leptos::prelude::*;

const FILES: [(&str, &str); 3] = [
    ("report", "Report.pdf"),
    ("photo", "Photo.jpg"),
    ("budget", "Budget.xlsx"),
];

#[component]
pub fn MenuContextRowsDemo() -> impl IntoView {
    let files = use_list_collection(
        Signal::stored(FILES.to_vec()),
        |(key, _)| Key::from(*key),
        |(_, name)| (*name).to_owned(),
    );
    let last_action = RwSignal::new(None::<String>);

    view! {
        // Right click a row, or focus it and press Shift+F10 (or the context menu key): the menu opens for that row.
        <ContextMenuTrigger>
            <GridList collection=files aria_label="Files" classes="demo-grid-list">
                {FILES
                    .map(|(key, name)| view! { <GridListItem key=key classes="demo-grid-list-item">{name}</GridListItem> })
                    .collect_view()}
            </GridList>
            <FileMenu last_action=last_action/>
        </ContextMenuTrigger>
        <p class="demo-status">
            {move || last_action.get().unwrap_or_else(|| "No action yet.".to_owned())}
        </p>
    }
}

/// The menu of a row. It is a component of its own: `use_context_menu_target` reads the context of the
/// `ContextMenuTrigger` around it.
#[component]
fn FileMenu(last_action: RwSignal<Option<String>>) -> impl IntoView {
    let target = use_context_menu_target();
    let actions = use_collection(|b| {
        b.item("open", "Open");
        b.item("rename", "Rename");
        b.item("delete", "Delete");
    });

    view! {
        <Popover>
            <Menu
                collection=actions
                on_action=move |action: Key| {
                    // The row the menu was opened on, by its key.
                    let file = target.get_untracked().and_then(|key| {
                        FILES.iter().find(|(file, _)| key == Key::from(*file)).map(|(_, name)| *name)
                    });
                    last_action.set(Some(format!("Last action: {action} {}.", file.unwrap_or("?"))));
                }
                classes="demo-menu-list"
            >
                <MenuItems classes="demo-menu-atom-item" let:node>
                    {node.text_value.to_string()}
                </MenuItems>
            </Menu>
        </Popover>
    }
}
