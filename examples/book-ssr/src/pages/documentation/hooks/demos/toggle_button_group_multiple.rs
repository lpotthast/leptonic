use std::collections::HashSet;

use leptonic::hooks::*;
use leptos::prelude::*;

const FORMATS: [&str; 3] = ["Bold", "Italic", "Underline"];

#[component]
pub fn ToggleButtonGroupMultipleDemo() -> impl IntoView {
    let (formats, set_formats) = signal(HashSet::new());

    // Multiple selection: the group is a toolbar, its buttons have `aria-pressed`.
    let state = use_toggle_group_state(UseToggleGroupStateInput {
        selection_mode: ToggleGroupSelectionMode::Multiple,
        on_selection_change: Some(Callback::new(move |keys| set_formats.set(keys))),
        ..UseToggleGroupStateInput::default()
    });
    let group = use_toggle_button_group(UseToggleButtonGroupInput {
        toolbar: UseToolbarInput {
            aria_label: "Text formatting".into(),
            ..UseToolbarInput::default()
        },
        ..UseToggleButtonGroupInput::new(state)
    });

    view! {
        <div {..group.props.into_attrs()} class="demo-toggle-group">
            {FORMATS.into_iter().map(|format| view! { <FormatButton group=state format/> }).collect_view()}
        </div>
        <p class="demo-status">
            {move || {
                let mut selected = formats.get().iter().map(ToString::to_string).collect::<Vec<_>>();
                selected.sort_unstable();
                if selected.is_empty() { "Plain text".to_owned() } else { selected.join(", ") }
            }}
        </p>
    }
}

#[component]
fn FormatButton(group: ToggleGroupState, format: &'static str) -> impl IntoView {
    let button = use_button(use_toggle_button_group_item(
        UseToggleButtonGroupItemInput::new(group, format),
    ));
    let (attrs, styles) = button.props.into_parts();
    view! { <button {..attrs} style=styles class="demo-toggle-button">{format}</button> }
}
