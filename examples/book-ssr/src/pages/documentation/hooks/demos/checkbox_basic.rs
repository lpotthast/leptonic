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

    // `data-focus-visible` draws a focus ring around the label for keyboard users.
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
            {move || format!("Checked: {}, indeterminate: {}", state.is_selected.get(), is_indeterminate.get())}
        </p>

        <div class="demo-toggle-settings">
            <Button
                variant=ButtonVariant::Flat
                size=ButtonSize::Small
                on_press=move |_| set_is_indeterminate.update(|value| *value = !*value)
            >
                "Toggle indeterminate"
            </Button>
            <Checkbox state=disabled>"Disabled"</Checkbox>
        </div>
    }
}
