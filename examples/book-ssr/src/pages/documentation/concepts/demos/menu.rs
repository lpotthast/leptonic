use leptonic::{
    atoms,
    hooks::collections::{Key, use_collection},
};
use leptos::prelude::*;

#[component]
pub fn MenuConceptDemo() -> impl IntoView {
    let last_action = RwSignal::new(None::<Key>);
    let actions = use_collection(|b| {
        b.item("copy", "Copy");
        b.item("cut", "Cut");
        b.item("paste", "Paste");
    });

    view! {
        // Pressing an item performs its action and closes the menu.
        <atoms::menu::MenuTrigger>
            <atoms::button::Button classes="demo-btn">"Edit"</atoms::button::Button>
            <atoms::popover::Popover>
                <atoms::menu::Menu collection=actions on_action=move |key: Key| last_action.set(Some(key)) classes="demo-menu-list">
                    <atoms::menu::MenuItems classes="demo-menu-atom-item" let:node>{node.text_value.to_string()}</atoms::menu::MenuItems>
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
