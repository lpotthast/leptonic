use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn LinkButtonComponentDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    view! {
        // Links that look like buttons take the button's variant, color and size.
        <div class="demo-control-row">
            <LinkButton href="/doc/installation" is_disabled=disabled>"Install leptonic"</LinkButton>
            <LinkButton href="/doc/overview" variant=ButtonVariant::Outlined is_disabled=disabled>"Read the docs"</LinkButton>
        </div>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
