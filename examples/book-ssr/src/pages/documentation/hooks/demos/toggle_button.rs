use leptonic::{atoms::checkbox::Checkbox, hooks::*};
use leptos::prelude::*;

#[component]
pub fn ToggleButtonDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    let state = use_toggle_state(UseToggleStateInput::default());
    // `use_toggle_button` prepares the input of `use_button`, which renders the button.
    let button = use_button(use_toggle_button(UseToggleButtonInput {
        button: UseButtonInput {
            is_disabled: disabled.into(),
            ..UseButtonInput::default()
        },
        state,
    }));
    let (attrs, styles) = button.props.into_parts();

    // Styled through `aria-pressed` and `data-focus-visible`, which `use_button` sets.
    view! {
        <button {..attrs} style=styles class="demo-toggle-button">"Pin"</button>
        <p class="demo-status">{move || if state.is_selected.get() { "Pinned." } else { "Not pinned." }}</p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}
