use leptonic::{atoms::prelude::LinkButton, components::prelude::Checkbox};
use leptos::prelude::*;

#[component]
pub fn LinkButtonAtomDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    view! {
        // It navigates, so it is a link, even though it looks like a button.
        <LinkButton href="/doc/installation" is_disabled=disabled classes=["demo-btn", "demo-link-button"]>
            "Install leptonic"
        </LinkButton>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
