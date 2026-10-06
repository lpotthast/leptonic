use leptonic::atoms::toggle_button::ToggleButton;
use leptos::prelude::*;

#[component]
pub fn ToggleButtonConceptDemo() -> impl IntoView {
    let starred = RwSignal::new(false);

    view! {
        <ToggleButton state=starred classes="demo-toggle-button">"Star"</ToggleButton>
        <p class="demo-status">{move || if starred.get() { "Starred" } else { "Not starred" }}</p>
    }
}
