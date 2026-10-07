use leptonic::atoms::checkbox::Checkbox;
use leptos::prelude::*;

#[component]
pub fn CheckboxConceptDemo() -> impl IntoView {
    let subscribed = RwSignal::new(false);
    let disabled = RwSignal::new(false);

    view! {
        // The atom renders a `<label>` around a visually hidden input; the children draw the box.
        <Checkbox is_selected=subscribed set_selected=subscribed is_disabled=disabled classes="demo-check">
            <span class="demo-check-box" aria-hidden="true"></span>
            "Subscribe to the newsletter"
        </Checkbox>
        <p class="demo-status">{move || if subscribed.get() { "Subscribed." } else { "Not subscribed." }}</p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}
