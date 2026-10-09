use std::collections::HashSet;

use leptonic::{
    IntoAttrs,
    atoms::checkbox::{CheckboxButton, CheckboxField},
    hooks::{
        button::{
            ToggleGroupSelectionMode, ToggleGroupState, UseButtonInput, UseToggleButtonGroupInput,
            UseToggleButtonGroupItemInput, UseToggleGroupStateInput, use_button,
            use_toggle_button_group, use_toggle_button_group_item, use_toggle_group_state,
        },
        collections::Key,
        toolbar::UseToolbarInput,
    },
};
use leptos::prelude::*;

const FORMATS: [&str; 3] = ["Bold", "Italic", "Underline"];

#[component]
pub fn ToggleButtonGroupMultipleDemo() -> impl IntoView {
    let formats = RwSignal::new(HashSet::<Key>::new());
    let disabled = RwSignal::new(false);

    // Multiple selection: the group is a toolbar, its buttons have `aria-pressed`. The selection is bound to
    // `formats`, app state the rest of the page can read and set.
    let state = use_toggle_group_state(UseToggleGroupStateInput {
        selection_mode: ToggleGroupSelectionMode::Multiple,
        selected_keys: Some(formats.into()),
        is_disabled: disabled.into(),
        ..UseToggleGroupStateInput::default()
    });
    let group = use_toggle_button_group(UseToggleButtonGroupInput {
        toolbar: UseToolbarInput {
            aria_label: "Text formatting".into(),
            ..UseToolbarInput::default()
        },
        state,
    });

    view! {
        <div {..group.props.into_attrs()} class="demo-toggle-group">
            {FORMATS.into_iter().map(|format| view! { <FormatButton group=state format/> }).collect_view()}
        </div>
        <p class="demo-status">
            {move || {
                let selected: Vec<&str> = FORMATS
                    .into_iter()
                    .filter(|format| formats.with(|keys| keys.contains(&Key::from(*format))))
                    .collect();
                if selected.is_empty() { "Plain text.".to_owned() } else { format!("{}.", selected.join(", ")) }
            }}
        </p>
        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}

#[component]
fn FormatButton(group: ToggleGroupState, format: &'static str) -> impl IntoView {
    let button = use_button(use_toggle_button_group_item(
        UseToggleButtonGroupItemInput {
            group,
            key: format.into(),
            button: UseButtonInput::default(),
        },
    ));
    let (attrs, styles) = button.props.into_parts();
    // Styled through `aria-pressed`.
    view! { <button {..attrs} style=styles class="demo-toggle-button">{format}</button> }
}
