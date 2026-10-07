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

    // Single selection (the default): the group is a `radiogroup`, its buttons are radios.
    let state = use_toggle_group_state(UseToggleGroupStateInput {
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
        state,
    });

    view! {
        <div {..group.props.into_attrs()} class="demo-toggle-group">
            {ALIGNMENTS.into_iter().map(|alignment| view! { <AlignmentButton group=state alignment/> }).collect_view()}
        </div>
        <p class="demo-status">
            {move || {
                let selected = state.selected_keys.get().iter().map(ToString::to_string).collect::<Vec<_>>();
                format!("Alignment: {}.", selected.join(", "))
            }}
        </p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}

#[component]
fn AlignmentButton(group: ToggleGroupState, alignment: &'static str) -> impl IntoView {
    let button = use_button(use_toggle_button_group_item(
        UseToggleButtonGroupItemInput {
            group,
            key: alignment.into(),
            button: UseButtonInput::default(),
        },
    ));
    let (attrs, styles) = button.props.into_parts();
    // Styled through `aria-checked` (`aria-pressed` in a multiple-selection group) and `data-focus-visible`.
    view! { <button {..attrs} style=styles class="demo-toggle-button">{alignment}</button> }
}
