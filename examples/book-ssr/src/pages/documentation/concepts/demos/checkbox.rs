use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn CheckboxConceptDemo() -> impl IntoView {
    let (subscribed, set_subscribed) = signal(false);

    view! {
        <Checkbox state=(subscribed, set_subscribed) classes="demo-form-row">"Subscribe to the newsletter"</Checkbox>
        <p>{move || if subscribed.get() { "Subscribed." } else { "Not subscribed." }}</p>
    }
}
