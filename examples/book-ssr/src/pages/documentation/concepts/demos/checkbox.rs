use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn CheckboxConceptDemo() -> impl IntoView {
    let subscribed = RwSignal::new(false);
    let disabled = RwSignal::new(false);

    view! {
        <Checkbox is_selected=subscribed set_selected=subscribed is_disabled=disabled>"Subscribe to the newsletter"</Checkbox>
        <p class="demo-status">{move || if subscribed.get() { "Subscribed." } else { "Not subscribed." }}</p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
