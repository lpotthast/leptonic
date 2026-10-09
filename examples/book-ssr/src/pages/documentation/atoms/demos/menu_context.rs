use leptonic::{
    atoms,
    hooks::{
        collections::{Key, use_collection},
        menu::MenuTriggerType,
    },
};
use leptos::prelude::*;

#[component]
pub fn MenuContextDemo() -> impl IntoView {
    let last_action = RwSignal::new(None::<Key>);
    let actions = use_collection(|b| {
        b.item("cut", "Cut");
        b.item("copy", "Copy");
        b.item("paste", "Paste");
    });

    view! {
        // Right click the button, press Shift+F10 (or the context menu key) on it, or long press it on iOS:
        // the menu opens where it was requested.
        <atoms::menu::MenuTrigger trigger=MenuTriggerType::ContextMenu>
            <atoms::button::Button classes="demo-context-target">"Right click here"</atoms::button::Button>
            <atoms::popover::Popover>
                <atoms::menu::Menu
                    collection=actions
                    on_action=move |key: Key| last_action.set(Some(key))
                    classes="demo-menu-list"
                >
                    <atoms::menu::MenuItems classes="demo-menu-atom-item" let:node>
                        {node.text_value.to_string()}
                    </atoms::menu::MenuItems>
                </atoms::menu::Menu>
            </atoms::popover::Popover>
        </atoms::menu::MenuTrigger>
        <p class="demo-status">
            {move || match last_action.get() {
                Some(key) => format!("Last action: {key}."),
                None => "No action yet.".to_owned(),
            }}
        </p>
    }
}
