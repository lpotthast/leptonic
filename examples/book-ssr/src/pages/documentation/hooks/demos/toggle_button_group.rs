use std::collections::HashSet;

use leptonic::{
    components::prelude::*,
    hooks::{collections::Key, *},
};
use leptos::prelude::*;

const ALIGNMENTS: [&str; 3] = ["Left", "Center", "Right"];

#[component]
pub fn ToggleButtonGroupDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    // Single selection: the group is a `radiogroup`, its buttons are radios.
    let state = use_toggle_group_state(UseToggleGroupStateInput {
        selection_mode: ToggleGroupSelectionMode::Single,
        disallow_empty_selection: true,
        default_selected_keys: HashSet::from([Key::from("Left")]),
        is_disabled: disabled.into(),
        ..UseToggleGroupStateInput::default()
    });
    let group = use_toggle_button_group(UseToggleButtonGroupInput {
        toolbar: UseToolbarInput {
            aria_label: "Text alignment".into(),
            ..UseToolbarInput::default()
        },
        ..UseToggleButtonGroupInput::new(state)
    });

    view! {
        <div {..group.props.into_attrs()} class="demo-toggle-group">
            {ALIGNMENTS.into_iter().map(|alignment| view! { <AlignmentButton group=state alignment/> }).collect_view()}
        </div>
        <p class="demo-status">
            {move || {
                let selected = state.selected_keys.get().iter().map(ToString::to_string).collect::<Vec<_>>();
                format!("Alignment: {}", selected.join(", "))
            }}
        </p>
        <div class="demo-toggle-settings">
            <Checkbox state=disabled>"Disable the group"</Checkbox>
        </div>
    }
}

#[component]
fn AlignmentButton(group: ToggleGroupState, alignment: &'static str) -> impl IntoView {
    let button = use_button(use_toggle_button_group_item(
        UseToggleButtonGroupItemInput::new(group, alignment),
    ));
    let (attrs, styles) = button.props.into_parts();
    // Styled through `aria-checked` (`aria-pressed` in a multiple-selection group).
    view! { <button {..attrs} style=styles class="demo-toggle-button">{alignment}</button> }
}
