use std::collections::HashSet;

use leptonic::{
    atoms::{
        checkbox::{CheckboxButton, CheckboxField},
        toggle_button::{ToggleButton, ToggleButtonGroup},
    },
    hooks::collections::Key,
};
use leptos::prelude::*;

const VIEWS: [&str; 3] = ["List", "Grid", "Gallery"];

#[component]
pub fn ToggleButtonGroupAtomDemo() -> impl IntoView {
    let view_mode = RwSignal::new(HashSet::from([Key::from("List")]));
    let disabled = RwSignal::new(false);

    view! {
        <ToggleButtonGroup
            disallow_empty_selection=true
            value=view_mode
            set_value=view_mode
            is_disabled=disabled
            aria_label="View"
            classes="demo-toggle-group"
        >
            {VIEWS
                .into_iter()
                .map(|value| view! { <ToggleButton value classes="demo-atom-toggle-button">{value}</ToggleButton> })
                .collect_view()}
        </ToggleButtonGroup>

        <p class="demo-status">
            {move || format!("View: {}.", view_mode.get().iter().map(ToString::to_string).collect::<Vec<_>>().join(", "))}
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
