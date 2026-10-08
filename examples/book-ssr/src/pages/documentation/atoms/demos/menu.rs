use leptonic::{
    atoms::prelude::{
        Button, CheckboxButton, CheckboxField, Menu, MenuItem, MenuItemLabel, MenuItemShortcut, MenuItems, MenuSection,
        MenuTrigger, Popover,
    },
    hooks::{
        Placement, SelectionMode,
        collections::{Key, Selection, use_collection},
    },
};
use leptos::prelude::*;

#[component]
pub fn MenuDemo() -> impl IntoView {
    let last_action = RwSignal::new(None::<Key>);
    let view = RwSignal::new(Selection::keys([Key::from("sidebar")]));
    let disabled = RwSignal::new(false);

    let actions = use_collection(|b| {
        b.item("copy", "Copy");
        b.item("cut", "Cut");
        b.item("paste", "Paste");
    });
    let view_options = use_collection(|b| {
        b.section("panels", |s| {
            s.header("panels-header", "Panels");
            s.item("sidebar", "Sidebar");
            s.item("toolbar", "Toolbar");
        });
    });

    view! {
        <div class="demo-flex-center-row">
            // An action menu: pressing an item performs the action and closes the menu.
            <MenuTrigger is_disabled=disabled>
                <Button classes="demo-btn">"Edit"</Button>
                <Popover placement=Placement::BottomLeft offset=4.0>
                    <Menu
                        collection=actions
                        on_action=move |key: Key| last_action.set(Some(key))
                        classes="demo-menu-list"
                    >
                        <MenuItems classes="demo-menu-atom-item" let:node>
                            {node.text_value.to_string()}
                        </MenuItems>
                    </Menu>
                </Popover>
            </MenuTrigger>

            // A menu with checkable items in a section; it stays open while you check them.
            <MenuTrigger is_disabled=disabled>
                <Button classes="demo-btn">"View"</Button>
                <Popover placement=Placement::BottomLeft offset=4.0>
                    <Menu
                        collection=view_options
                        selection_mode=SelectionMode::Multiple
                        selection=view
                        set_selection=view
                        classes="demo-menu-list"
                    >
                        <MenuSection key="panels" heading_classes="demo-menu-heading">
                            <MenuItem key="sidebar" classes="demo-menu-atom-item">
                                <span class="demo-menu-atom-check" aria-hidden="true"></span>
                                <MenuItemLabel>"Sidebar"</MenuItemLabel>
                                <MenuItemShortcut classes="demo-menu-atom-shortcut">"Ctrl+B"</MenuItemShortcut>
                            </MenuItem>
                            <MenuItem key="toolbar" classes="demo-menu-atom-item">
                                <span class="demo-menu-atom-check" aria-hidden="true"></span>
                                <MenuItemLabel>"Toolbar"</MenuItemLabel>
                            </MenuItem>
                        </MenuSection>
                    </Menu>
                </Popover>
            </MenuTrigger>
        </div>
        <p class="demo-status">
            {move || match last_action.get() {
                Some(key) => format!("Last action: {key}."),
                None => "No action yet.".to_owned(),
            }}
            " "
            {move || match view.get() {
                Selection::Keys(keys) if keys.is_empty() => "No panels shown.".to_owned(),
                Selection::Keys(keys) => {
                    let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
                    keys.sort();
                    format!("Shown: {}.", keys.join(", "))
                }
                Selection::All => "All panels shown.".to_owned(),
            }}
        </p>
        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
