use leptonic::{
    atoms::prelude as atoms,
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
        <atoms::MenuTrigger>
            <atoms::Button classes="demo-btn">"Edit"</atoms::Button>
            <atoms::Popover>
                <atoms::Menu collection=actions on_action=move |key: Key| last_action.set(Some(key)) classes="demo-menu-list">
                    <atoms::MenuItems classes="demo-menu-atom-item" let:node>{node.text_value.to_string()}</atoms::MenuItems>
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
