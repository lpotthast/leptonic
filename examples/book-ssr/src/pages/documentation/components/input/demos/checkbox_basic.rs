use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn CheckboxBasicDemo() -> impl IntoView {
    let subscribed = RwSignal::new(false);

    view! {
        <Checkbox is_selected=subscribed set_selected=subscribed>"Subscribe to the newsletter"</Checkbox>
        <p class="demo-status">{move || if subscribed.get() { "Subscribed." } else { "Not subscribed." }}</p>
    }
}
