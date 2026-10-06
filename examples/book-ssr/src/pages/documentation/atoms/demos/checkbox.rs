use leptonic::{atoms::checkbox, components::prelude::Checkbox};
use leptos::prelude::*;

#[component]
pub fn CheckboxAtomDemo() -> impl IntoView {
    let subscribed = RwSignal::new(false);
    let indeterminate = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);

    view! {
        // The atom renders a `<label>` around a visually hidden input; the children draw the box.
        <checkbox::Checkbox
            state=subscribed
            is_indeterminate=indeterminate
            is_disabled=disabled
            is_read_only=read_only
            classes="demo-check"
        >
            <span class="demo-check-box" aria-hidden="true"></span>
            "Subscribe to the newsletter"
        </checkbox::Checkbox>

        <p class="demo-status">{move || if subscribed.get() { "Subscribed" } else { "Not subscribed" }}</p>

        <div class="demo-toggle-settings">
            <Checkbox state=indeterminate>"Indeterminate"</Checkbox>
            <Checkbox state=disabled>"Disabled"</Checkbox>
            <Checkbox state=read_only>"Read-only"</Checkbox>
        </div>
    }
}
