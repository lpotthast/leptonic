use std::collections::HashSet;

use leptonic::hooks::*;
use leptos::prelude::*;
use leptos_classes::Classes;

#[component]
pub fn CheckboxGroupDemo() -> impl IntoView {
    let (group_value, _set_group_value) = signal(HashSet::<String>::new());
    let UseCheckboxGroupReturn {
        group_props,
        label_props: group_label_props,
        state: group_state,
        is_invalid: _,
        validation_errors: _,
        validation_details: _,
    } = use_checkbox_group(UseCheckboxGroupInput {
        value: group_value.into(),
        label: Some("Fruits".into()),
        description: None,
        is_disabled: false.into(),
        is_read_only: false.into(),
        is_required: false,
        orientation: Orientation::Vertical,
        on_change: None,
        ..Default::default()
    });

    view! {
        <fieldset
            role=group_props.role
            aria-labelledby=group_props.aria_labelledby.clone()
            style="border: none; padding: 0; margin: 0;"
        >
            <legend id=group_label_props.id.clone() style="font-weight: bold; margin-bottom: 0.5em;">
                "Select your favorite fruits"
            </legend>

            {["Apple", "Banana", "Cherry", "Date"].into_iter().map(|fruit| {
                let fruit_value = fruit.to_string();
                let fruit_for_check = fruit_value.clone();
                let fruit_for_toggle = fruit_value.clone();
                let state = group_state.clone();
                let state_for_check = group_state.clone();
                view! {
                    <label class=Classes::from("demo-form-row")>
                        <input
                            type="checkbox"
                            checked=move || state_for_check.is_selected.run(fruit_for_check.clone())
                            on:change=move |_| { state.toggle_value.run(fruit_for_toggle.clone()); }
                        />
                        <span>{ fruit }</span>
                    </label>
                }
            }).collect::<Vec<_>>()}
        </fieldset>

        <p class=Classes::from("demo-mt-1")>
            "Selected: " { move || format!("{:?}", group_state.value.get()) }
        </p>
    }
}
