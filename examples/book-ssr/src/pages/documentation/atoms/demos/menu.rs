use leptonic::{
    atoms::prelude as atoms,
    components::prelude::Checkbox,
    hooks::{
        PlacementX, SelectionMode,
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
            <atoms::MenuTrigger is_disabled=disabled>
                <atoms::Button classes="demo-btn">"Edit"</atoms::Button>
                <atoms::Popover placement_x=PlacementX::Left offset=4.0>
                    <atoms::Menu
                        collection=actions
                        on_action=move |key: Key| last_action.set(Some(key))
                        classes="demo-overlays-menu-list"
                    >
                        <atoms::MenuItems classes="demo-menu-atom-item" let:node>
                            {node.text_value.to_string()}
                        </atoms::MenuItems>
                    </atoms::Menu>
                </atoms::Popover>
            </atoms::MenuTrigger>

            // A menu with checkable items in a section; it stays open while you check them.
            <atoms::MenuTrigger is_disabled=disabled>
                <atoms::Button classes="demo-btn">"View"</atoms::Button>
                <atoms::Popover placement_x=PlacementX::Left offset=4.0>
                    <atoms::Menu
                        collection=view_options
                        selection_mode=SelectionMode::Multiple
                        selection=view
                        classes="demo-overlays-menu-list"
                    >
                        <atoms::MenuSection key="panels" heading_classes="demo-overlays-menu-heading">
                            <atoms::MenuItem key="sidebar" classes="demo-menu-atom-item">
                                <atoms::MenuItemLabel>"Sidebar"</atoms::MenuItemLabel>
                                <atoms::MenuItemShortcut classes="demo-menu-atom-shortcut">"Ctrl+B"</atoms::MenuItemShortcut>
                            </atoms::MenuItem>
                            <atoms::MenuItem key="toolbar" classes="demo-menu-atom-item">
                                <atoms::MenuItemLabel>"Toolbar"</atoms::MenuItemLabel>
                            </atoms::MenuItem>
                        </atoms::MenuSection>
                    </atoms::Menu>
                </atoms::Popover>
            </atoms::MenuTrigger>

            <Checkbox state=disabled>"Disabled"</Checkbox>
        </div>
        <p>
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
    }
}
