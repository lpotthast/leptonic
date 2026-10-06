use leptonic::{
    atoms::prelude as atoms,
    hooks::{
        MenuTriggerType,
        collections::{Key, use_collection},
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
        <atoms::MenuTrigger trigger=MenuTriggerType::ContextMenu>
            <atoms::Button classes="demo-context-target">"Right click here"</atoms::Button>
            <atoms::Popover>
                <atoms::Menu
                    collection=actions
                    on_action=move |key: Key| last_action.set(Some(key))
                    classes="demo-menu-list"
                >
                    <atoms::MenuItems classes="demo-menu-atom-item" let:node>
                        {node.text_value.to_string()}
                    </atoms::MenuItems>
                </atoms::Menu>
            </atoms::Popover>
        </atoms::MenuTrigger>
        <p class="demo-status">
            {move || match last_action.get() {
                Some(key) => format!("Last action: {key}."),
                None => "No action yet.".to_owned(),
            }}
        </p>
    }
}
