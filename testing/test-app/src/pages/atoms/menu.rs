use std::collections::HashSet;

use leptonic::{
    atoms::{
        button::Button,
        menu::{
            Menu, MenuItem, MenuItemDescription, MenuItemLabel, MenuItemShortcut, MenuItems,
            MenuSection, MenuTrigger,
        },
        popover::Popover,
        separator::Separator,
    },
    hooks::{
        collections::{Key, Selection, SelectionMode, use_collection},
        focus::use_interaction_modality,
        menu::MenuTriggerType,
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
/// - "Sandwich" (`#test-menu-atoms-sandwich-trigger`): sections with selections of their own,
///   "Veggies" (multiple, Lettuce selected) and "Protein" (single, Ham selected), shown in
///   `#test-menu-atoms-veggies` and `#test-menu-atoms-protein`.
/// - "File" (`#test-menu-atoms-file-trigger`): a section that keeps the menu open
///   (`should_close_on_select=false`: Open, Rename) and one that doesn't (Share).
/// - "Edit" (`#test-menu-atoms-edit-trigger`): a menu that stays open (Undo, Redo).
///
/// `#test-menu-atoms-modality` shows the interaction modality.
#[component]
pub fn PageAtomMenu() -> impl IntoView {
    let actions = RwSignal::new(Vec::<String>::new());
    let view_selection = RwSignal::new(String::new());
    let modality = use_interaction_modality();
    let veggies = RwSignal::new(Selection::keys([Key::from("Lettuce")]));
    let protein = RwSignal::new(Selection::keys([Key::from("Ham")]));
    let sandwich_items = use_collection(|b| {
        b.section("veggies", |s| {
            s.header("veggies-header", "Veggies");
            for item in ["Lettuce", "Tomato", "Onion"] {
                s.item(item, item);
            }
        });
        b.section("protein", |s| {
            s.header("protein-header", "Protein");
            for item in ["Ham", "Tuna", "Tofu"] {
                s.item(item, item);
            }
        });
    });
    let file_items = use_collection(|b| {
        b.section("edit", |s| {
            s.item("Open", "Open");
            s.item("Rename", "Rename");
        });
        b.section("share", |s| {
            s.item("Share", "Share");
        });
    });
    let edit_items = use_collection(|b| {
        b.item("Undo", "Undo");
        b.item("Redo", "Redo");
    });
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
                <Button id="test-menu-atoms-actions-trigger">"Actions"</Button>
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
                <Button id="test-menu-atoms-view-trigger">"View"</Button>
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
                    id="test-menu-atoms-long-trigger"
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

            <MenuTrigger>
                <Button id="test-menu-atoms-sandwich-trigger">"Sandwich"</Button>
                <Popover>
                    <Menu collection=sandwich_items selection_mode=SelectionMode::Multiple>
                        <MenuSection
                            key="veggies"
                            selection_mode=SelectionMode::Multiple
                            selection=veggies
                            set_selection=veggies
                        >
                            <MenuItem key="Lettuce">"Lettuce"</MenuItem>
                            <MenuItem key="Tomato">"Tomato"</MenuItem>
                            <MenuItem key="Onion">"Onion"</MenuItem>
                        </MenuSection>
                        <MenuSection
                            key="protein"
                            selection_mode=SelectionMode::Single
                            selection=protein
                            set_selection=protein
                        >
                            <MenuItem key="Ham">"Ham"</MenuItem>
                            <MenuItem key="Tuna">"Tuna"</MenuItem>
                            <MenuItem key="Tofu">"Tofu"</MenuItem>
                        </MenuSection>
                    </Menu>
                </Popover>
            </MenuTrigger>

            <MenuTrigger>
                <Button id="test-menu-atoms-file-trigger">"File"</Button>
                <Popover>
                    <Menu
                        collection=file_items
                        on_action=move |key: Key| actions.update(|a| a.push(key.to_string()))
                    >
                        <MenuSection key="edit" should_close_on_select=false>
                            <MenuItem key="Open">"Open"</MenuItem>
                            <MenuItem key="Rename">"Rename"</MenuItem>
                        </MenuSection>
                        <MenuSection key="share">
                            <MenuItem key="Share">"Share"</MenuItem>
                        </MenuSection>
                    </Menu>
                </Popover>
            </MenuTrigger>

            <MenuTrigger>
                <Button id="test-menu-atoms-edit-trigger">"Edit"</Button>
                <Popover>
                    <Menu
                        collection=edit_items
                        should_close_on_select=false
                        on_action=move |key: Key| actions.update(|a| a.push(key.to_string()))
                    >
                        <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
                    </Menu>
                </Popover>
            </MenuTrigger>

            <div>"Actions: " <span id="test-menu-atoms-actions">{move || actions.get().join(",")}</span></div>
            <div>"View: " <span id="test-menu-atoms-view-selection">{view_selection}</span></div>
            <div>
                "Veggies: "
                <span id="test-menu-atoms-veggies">{move || describe_selection(&veggies.get())}</span>
            </div>
            <div>
                "Protein: "
                <span id="test-menu-atoms-protein">{move || describe_selection(&protein.get())}</span>
            </div>
            <div>
                "Modality: "
                <span id="test-menu-atoms-modality">
                    {move || modality.get().map(|modality| format!("{modality:?}")).unwrap_or_default()}
                </span>
            </div>
        </div>
    }
}
