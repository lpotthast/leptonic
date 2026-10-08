use leptonic::atoms::checkbox;
use leptos::prelude::*;

#[component]
pub fn CheckboxAtomDemo() -> impl IntoView {
    let subscribed = RwSignal::new(false);
    let indeterminate = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);

    view! {
        // The field holds the state; its button is a `<label>` around a hidden input, whose children draw the box.
        <checkbox::CheckboxField
            is_selected=subscribed
            set_selected=subscribed
            is_indeterminate=indeterminate
            is_disabled=disabled
            is_read_only=read_only
        >
            <checkbox::CheckboxButton classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Subscribe to the newsletter"
            </checkbox::CheckboxButton>
        </checkbox::CheckboxField>

        <p class="demo-status">{move || if subscribed.get() { "Subscribed." } else { "Not subscribed." }}</p>

        <div class="demo-controls">
            <checkbox::CheckboxField is_selected=disabled set_selected=disabled>
                <checkbox::CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </checkbox::CheckboxButton>
            </checkbox::CheckboxField>
            <checkbox::CheckboxField is_selected=read_only set_selected=read_only>
                <checkbox::CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Read-only"
                </checkbox::CheckboxButton>
            </checkbox::CheckboxField>
            <checkbox::CheckboxField is_selected=indeterminate set_selected=indeterminate>
                <checkbox::CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Indeterminate"
                </checkbox::CheckboxButton>
            </checkbox::CheckboxField>
        </div>
    }
}
