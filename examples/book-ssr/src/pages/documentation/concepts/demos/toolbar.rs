use leptonic::{Orientation, atoms};
use leptos::prelude::*;

#[component]
pub fn ToolbarConceptDemo() -> impl IntoView {
    let (last_action, set_last_action) = signal(None::<&'static str>);

    view! {
        // One tab stop: the arrow keys move between the buttons.
        <atoms::toolbar::Toolbar aria_label="Clipboard" classes="demo-toolbar">
            <atoms::button::Button classes="demo-toolbar-atom-button" on_press=move |_| set_last_action.set(Some("Cut"))>"Cut"</atoms::button::Button>
            <atoms::button::Button classes="demo-toolbar-atom-button" on_press=move |_| set_last_action.set(Some("Copy"))>"Copy"</atoms::button::Button>
            <atoms::separator::Separator orientation=Orientation::Vertical classes="demo-toolbar-separator"/>
            <atoms::button::Button classes="demo-toolbar-atom-button" on_press=move |_| set_last_action.set(Some("Paste"))>"Paste"</atoms::button::Button>
        </atoms::toolbar::Toolbar>
        <p class="demo-status">
            {move || match last_action.get() {
                Some(action) => format!("Last action: {action}."),
                None => "No action yet.".to_owned(),
            }}
        </p>
    }
}
