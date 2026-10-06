use std::collections::HashSet;

use leptonic::{
    atoms::prelude::{
        Button, Menu, MenuItem, MenuItemDescription, MenuItemLabel, MenuItemShortcut, MenuItems,
        MenuSection, MenuTrigger, Popover, Separator,
    },
    hooks::{
        MenuTriggerType, SelectionMode,
        collections::{Key, Selection, use_collection},
    },
};
use leptos::prelude::*;

use crate::pages::atoms::listbox::describe_selection;

/// Menus built from the menu atoms (react-aria-components `Menu.test.tsx` setups):
/// - "Actions" (`#test-menu-atoms-actions-trigger`): Copy, Cut, Paste (disabled), Delete, with
///   `MenuItems`. Every action is appended to `#test-menu-atoms-actions`.
/// - "More" (`#test-menu-atoms-long-trigger`): the actions menu opened by a long press; a press
///   appends "More pressed".
/// - "View" (`#test-menu-atoms-view-trigger`): multiple selection, sections "Panels" (Sidebar
///   with a description and a shortcut, Toolbar) and "Zoom" (Fit), a `Separator` between them. The selection is shown in
///   `#test-menu-atoms-view-selection`.
#[component]
pub fn PageAtomMenu() -> impl IntoView {
    let actions = RwSignal::new(Vec::<String>::new());
    let view_selection = RwSignal::new(String::new());
    let action_items = use_collection(|b| {
        for action in ["Copy", "Cut", "Paste", "Delete"] {
            b.item(action, action);
        }
    });
    let view_items = use_collection(|b| {
        b.section("panels", |s| {
            s.header("panels-header", "Panels");
            s.item("Sidebar", "Sidebar");
            s.item("Toolbar", "Toolbar");
        });
        b.section("zoom", |s| {
            s.header("zoom-header", "Zoom");
            s.item("Fit", "Fit");
        });
    });

    view! {
        <div id="test-page-atom-menu">
            <h1>"Menu atoms"</h1>
            <button id="test-menu-atoms-before">"Before"</button>

            <MenuTrigger>
                <Button attr:id="test-menu-atoms-actions-trigger">"Actions"</Button>
                <Popover>
                    <Menu
                        collection=action_items
                        disabled_keys=Signal::stored(HashSet::from([Key::from("Paste")]))
                        on_action=move |key: Key| actions.update(|a| a.push(key.to_string()))
                    >
                        <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
                    </Menu>
                </Popover>
            </MenuTrigger>

            <MenuTrigger>
                <Button attr:id="test-menu-atoms-view-trigger">"View"</Button>
                <Popover>
                    <Menu
                        collection=view_items
                        selection_mode=SelectionMode::Multiple
                        on_selection_change=move |selection: Selection| {
                            view_selection.set(describe_selection(&selection));
                        }
                    >
                        <MenuSection key="panels">
                            <MenuItem key="Sidebar">
                                <MenuItemLabel>"Sidebar"</MenuItemLabel>
                                <MenuItemDescription>"Show the file tree"</MenuItemDescription>
                                <MenuItemShortcut>"Ctrl+B"</MenuItemShortcut>
                            </MenuItem>
                            <MenuItem key="Toolbar">"Toolbar"</MenuItem>
                        </MenuSection>
                        <Separator />
                        <MenuSection key="zoom">
                            <MenuItem key="Fit">"Fit"</MenuItem>
                        </MenuSection>
                    </Menu>
                </Popover>
            </MenuTrigger>

            // Opens on a long press (or Alt+ArrowDown); a press performs the button's own action.
            <MenuTrigger trigger=MenuTriggerType::LongPress>
                <Button
                    attr:id="test-menu-atoms-long-trigger"
                    on_press=move |_| actions.update(|a| a.push("More pressed".to_owned()))
                >
                    "More"
                </Button>
                <Popover>
                    <Menu
                        collection=action_items
                        on_action=move |key: Key| actions.update(|a| a.push(key.to_string()))
                    >
                        <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
                    </Menu>
                </Popover>
            </MenuTrigger>

            <button id="test-menu-atoms-after">"After"</button>
            <div>"Actions: " <span id="test-menu-atoms-actions">{move || actions.get().join(",")}</span></div>
            <div>"View: " <span id="test-menu-atoms-view-selection">{view_selection}</span></div>
        </div>
    }
}
