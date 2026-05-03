use leptonic::{
    components::prelude::*,
    hooks::*,
    utils::{classes::Classes, css::em},
};
use leptos::prelude::*;

#[component]
pub fn CheckboxBasicDemo() -> impl IntoView {
    let (is_checked, set_is_checked) = signal(false);
    let (is_indeterminate, set_is_indeterminate) = signal(false);
    let (is_disabled, set_is_disabled) = signal(false);

    let UseCheckboxReturn { input_props, .. } = use_checkbox(UseCheckboxInput {
        is_selected: is_checked.into(),
        is_indeterminate: is_indeterminate.into(),
        is_disabled: is_disabled.into(),
        is_read_only: false.into(),
        is_required: false,
        name: Some("example-checkbox"),
        value: Some("example"),
        aria_label: None,
        on_change: Some(Callback::new(move |checked| {
            set_is_checked.set(checked);
            if checked {
                set_is_indeterminate.set(false);
            }
        })),
        ..Default::default()
    });

    view! {
        <label class=Classes::from("demo-form-row")>
            <input type="checkbox" {..input_props.into_attrs()} />
            <span>"Accept terms and conditions"</span>
        </label>

        <p class=Classes::from("demo-mt-1")>
            "Checked: " <strong>{ move || is_checked.get().to_string() }</strong>
            " | Indeterminate: " <strong>{ move || is_indeterminate.get().to_string() }</strong>
        </p>

        <Stack orientation=StackOrientation::Horizontal spacing=em(0.5)>
            <button
                on:click=move |_| set_is_indeterminate.update(|v| *v = !*v)
                class=Classes::from("demo-btn")
            >
                "Toggle Indeterminate"
            </button>
            <button
                on:click=move |_| set_is_disabled.update(|v| *v = !*v)
                class=Classes::from("demo-btn")
            >
                { move || if is_disabled.get() { "Enable" } else { "Disable" } }
            </button>
        </Stack>
    }
}
