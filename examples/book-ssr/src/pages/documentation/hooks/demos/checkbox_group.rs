use std::sync::Arc;

use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

const TOPPINGS: [&str; 4] = ["Cheese", "Mushrooms", "Olives", "Peppers"];

#[component]
pub fn CheckboxGroupDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    let state = use_checkbox_group_state(UseCheckboxGroupStateInput {
        default_value: vec![Key::from("Cheese")],
        is_required: Signal::stored(true),
        // Uncheck everything to see the error message.
        validate: Some(Arc::new(|toppings: &Vec<Key>| {
            if toppings.is_empty() {
                Err(vec!["Choose at least one topping.".to_owned()])
            } else {
                Ok(())
            }
        })),
        is_disabled: disabled.into(),
        name: Some("toppings".to_owned()),
        ..UseCheckboxGroupStateInput::default()
    });
    let group = use_checkbox_group(UseCheckboxGroupInput {
        has_label: true.into(),
        ..UseCheckboxGroupInput::new(state)
    });
    let data = group.data;
    let description_props = group.description_props;
    let error_message_props = group.error_message_props;
    let validation_errors = group.validation_errors;

    view! {
        <div {..group.props.into_attrs()} class="demo-choice-group">
            <span {..group.label_props.into_attrs()} class="demo-choice-group-label">"Toppings"</span>
            {TOPPINGS
                .into_iter()
                .map(|topping| view! { <ToppingCheckbox group=data.clone() topping/> })
                .collect_view()}
            <p {..description_props.into_attrs()} class="demo-choice-group-description">"Toppings cost one euro each."</p>
            // Rendered only while invalid, so the checkboxes reference it only then.
            <Show when=move || group.is_invalid.get()>
                <p {..error_message_props.clone().into_attrs()} class="demo-choice-group-error">
                    {move || validation_errors.get().join(" ")}
                </p>
            </Show>
        </div>

        <p class="demo-status">
            {move || {
                let toppings = state.value.get().iter().map(ToString::to_string).collect::<Vec<_>>();
                if toppings.is_empty() { "No toppings.".to_owned() } else { format!("Toppings: {}.", toppings.join(", ")) }
            }}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}

#[component]
fn ToppingCheckbox(group: CheckboxGroupData, topping: &'static str) -> impl IntoView {
    let checkbox =
        use_checkbox_group_item(UseCheckboxGroupItemInput::new(group, Key::from(topping)));
    let (label_attrs, label_styles) = checkbox.label_props.into_parts();
    let (input_attrs, input_styles) = checkbox.input_props.into_parts();
    let is_focus_visible = checkbox.is_focus_visible;
    view! {
        <label
            {..label_attrs}
            style=label_styles
            class="demo-checkbox-label demo-no-margin"
            data-focus-visible=move || is_focus_visible.get().then_some("")
        >
            <input {..input_attrs} style=input_styles/>
            {topping}
        </label>
    }
}
