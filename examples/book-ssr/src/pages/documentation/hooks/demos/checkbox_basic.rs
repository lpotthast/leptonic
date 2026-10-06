use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

#[component]
pub fn CheckboxBasicDemo() -> impl IntoView {
    let (is_indeterminate, set_is_indeterminate) = signal(false);
    let disabled = RwSignal::new(false);

    // Checking or unchecking the checkbox ends the indeterminate state.
    let state = use_toggle_state(UseToggleStateInput {
        on_change: Some(Callback::new(move |_| set_is_indeterminate.set(false))),
        ..UseToggleStateInput::default()
    });
    let checkbox = use_checkbox(UseCheckboxInput {
        is_indeterminate: is_indeterminate.into(),
        options: ToggleOptions {
            is_disabled: disabled.into(),
            name: Some("terms".to_owned()),
            value: Some("accepted".to_owned()),
            ..ToggleOptions::default()
        },
        ..UseCheckboxInput::new(state)
    });
    let (label_attrs, label_styles) = checkbox.label_props.into_parts();
    let (input_attrs, input_styles) = checkbox.input_props.into_parts();
    let is_focus_visible = checkbox.is_focus_visible;

    // The hook tracks keyboard focus; `data-focus-visible` lets the stylesheet draw a focus ring around the input.
    view! {
        <label
            {..label_attrs}
            style=label_styles
            class="demo-checkbox-label"
            data-focus-visible=move || is_focus_visible.get().then_some("")
        >
            <input {..input_attrs} style=input_styles/>
            "Accept the terms and conditions"
        </label>

        <p class="demo-status">
            {move || {
                let checked = if state.is_selected.get() { "Checked" } else { "Not checked" };
                let shown = if is_indeterminate.get() { ", shown as indeterminate." } else { "." };
                format!("{checked}{shown}")
            }}
        </p>

        <div class="demo-controls">
            <Button
                variant=ButtonVariant::Flat
                size=ButtonSize::Small
                on_press=move |_| set_is_indeterminate.update(|value| *value = !*value)
            >
                "Toggle indeterminate"
            </Button>
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
