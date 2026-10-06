use std::collections::HashSet;

use leptonic::{
    atoms::toggle_button::{ToggleButton, ToggleButtonGroup},
    components::prelude::Checkbox,
    hooks::{Key, ToggleGroupSelectionMode},
};
use leptos::prelude::*;

const VIEWS: [&str; 3] = ["List", "Grid", "Gallery"];

#[component]
pub fn ToggleButtonGroupAtomDemo() -> impl IntoView {
    let (layout, set_layout) = signal(HashSet::from([Key::from("List")]));
    let disabled = RwSignal::new(false);

    view! {
        <ToggleButtonGroup
            selection_mode=ToggleGroupSelectionMode::Single
            disallow_empty_selection=true
            default_selected_keys=HashSet::from([Key::from("List")])
            on_selection_change=move |keys| set_layout.set(keys)
            is_disabled=disabled
            aria_label="View"
            classes="demo-toggle-group"
        >
            {VIEWS
                .into_iter()
                .map(|value| view! { <ToggleButton value classes="demo-toggle-button">{value}</ToggleButton> })
                .collect_view()}
        </ToggleButtonGroup>

        <p class="demo-status">
            {move || format!("View: {}", layout.get().iter().map(ToString::to_string).collect::<Vec<_>>().join(", "))}
        </p>

        <div class="demo-toggle-settings">
            <Checkbox state=disabled>"Disable the group"</Checkbox>
        </div>
    }
}
