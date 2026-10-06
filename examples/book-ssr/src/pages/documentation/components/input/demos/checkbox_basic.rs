use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn CheckboxBasicDemo() -> impl IntoView {
    let (checked, set_checked) = signal(false);

    view! {
        <Checkbox state=(checked, set_checked) classes="demo-control-row">"Subscribe to the newsletter"</Checkbox>
        <p class="demo-status">{move || if checked.get() { "Subscribed" } else { "Not subscribed" }}</p>
    }
}
