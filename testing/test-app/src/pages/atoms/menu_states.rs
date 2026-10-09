use std::collections::HashSet;

use leptonic::{
    atoms::menu::{Menu, MenuItem, MenuItems, MenuSection},
    hooks::collections::{Key, Selection, SelectionMode, use_collection},
};
use leptos::prelude::*;

use crate::pages::atoms::listbox::describe_selection;

/// Menus without a trigger (react-aria-components `Menu.test.tsx` renders most cases so):
/// - "Inline": Copy, Cut, Paste (disabled); the arrow keys don't wrap (`should_focus_wrap=false`).
/// - "Empty": no items, an empty state "No actions".
/// - "Alignment": single selection that can't become empty (Left selected), shown in
///   `#test-menu-atoms-alignment`.
/// - "Tools menu": a section without a heading, labelled "Tools" (Pen, Brush).
#[component]
pub fn PageAtomMenuStates() -> impl IntoView {
    let items = use_collection(|b| {
        for item in ["Copy", "Cut", "Paste"] {
            b.item(item, item);
        }
    });
    let empty = use_collection(|_| {});
    let alignments = use_collection(|b| {
        for item in ["Left", "Center", "Right"] {
            b.item(item, item);
        }
    });
    let tools = use_collection(|b| {
        b.section("tools", |s| {
            s.item("Pen", "Pen");
            s.item("Brush", "Brush");
        })
        .aria_label("Tools");
    });
    let alignment = RwSignal::new(Selection::keys([Key::from("Left")]));
    view! {
        <div id="test-page-atom-menu-states">
            <h1>"Menu states"</h1>
            <Menu
                aria_label="Inline"
                collection=items
                disabled_keys=Signal::stored(HashSet::from([Key::from("Paste")]))
                should_focus_wrap=false
            >
                <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
            </Menu>
            <Menu aria_label="Empty" collection=empty empty_state=|| "No actions">
                <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
            </Menu>
            <Menu
                aria_label="Alignment"
                collection=alignments
                selection_mode=SelectionMode::Single
                disallow_empty_selection=true
                selection=alignment
                set_selection=alignment
            >
                <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
            </Menu>
            <div>
                "Alignment: "
                <span id="test-menu-atoms-alignment">
                    {move || describe_selection(&alignment.get())}
                </span>
            </div>
            <Menu aria_label="Tools menu" collection=tools>
                <MenuSection key="tools">
                    <MenuItem key="Pen">"Pen"</MenuItem>
                    <MenuItem key="Brush">"Brush"</MenuItem>
                </MenuSection>
            </Menu>
        </div>
    }
}
