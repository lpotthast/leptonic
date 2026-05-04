use leptonic::hooks::*;
use leptos::prelude::*;
use leptos_classes::Classes;

#[component]
pub fn RadioDemo() -> impl IntoView {
    let UseRadioGroupStateReturn {
        selected_value,
        set_selected,
    } = use_radio_group_state(Some("option1".to_string()));

    let UseRadioGroupReturn {
        group_props,
        label_props,
        error_props: _,
        state,
        is_invalid: _,
        validation_errors: _,
        validation_details: _,
    } = use_radio_group(UseRadioGroupInput {
        label: Some("Select an option".into()),
        description: None,
        is_disabled: Signal::derive(|| false),
        is_read_only: Signal::derive(|| false),
        is_required: false,
        value: selected_value,
        orientation: Orientation::Vertical,
        on_change: Some(Callback::new(move |value: String| {
            set_selected.run(value);
        })),
        ..Default::default()
    });

    let options = vec![
        ("option1", "First Option"),
        ("option2", "Second Option"),
        ("option3", "Third Option"),
    ];

    let label_id = label_props.id.clone();

    view! {
        <fieldset
            {..group_props.into_attrs()}
            style="border: none; padding: 0; margin: 0;"
        >
            <legend id=label_id style="font-weight: bold; margin-bottom: 0.5em;">
                "Select an option"
            </legend>

            {options.into_iter().map(|(value, label)| {
                let value_owned = value.to_string();
                let value_for_check = value_owned.clone();
                let value_for_change = value_owned.clone();
                let state_clone = state.clone();
                view! {
                    <label class=Classes::from("demo-form-row")>
                        <input
                            type="radio"
                            name=state.name
                            value=value
                            checked=move || state_clone.selected_value.get().as_ref() == Some(&value_for_check)
                            on:change=move |_| state.set_selected_value.run(value_for_change.clone())
                        />
                        <span>{ label }</span>
                    </label>
                }
            }).collect::<Vec<_>>()}
        </fieldset>

        <p class=Classes::from("demo-mt-1")>
            "Selected: " <strong>{ move || selected_value.get().unwrap_or_else(|| "None".to_string()) }</strong>
        </p>
    }
}
