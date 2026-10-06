use std::collections::HashSet;

use leptonic::{
    atoms::toggle_button::{ToggleButton, ToggleButtonGroup},
    hooks::{Orientation, ToggleGroupSelectionMode, collections::Key},
};
use leptos::prelude::*;

/// A toggle button group with buttons "{label} A/B/C"; its selection is shown in
/// `#test-tb-{name}-value`.
#[component]
fn TestGroup(
    name: &'static str,
    label: &'static str,
    #[prop(optional)] selection_mode: ToggleGroupSelectionMode,
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(optional)] is_disabled: bool,
) -> impl IntoView {
    let value = RwSignal::new(String::new());
    view! {
        <ToggleButtonGroup
            aria_label=label
            selection_mode
            orientation
            is_disabled
            on_selection_change={move |keys: HashSet<Key>| {
                let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
                keys.sort();
                value.set(keys.join(","));
            }}
        >
            <ToggleButton value="a">{format!("{label} A")}</ToggleButton>
            <ToggleButton value="b">{format!("{label} B")}</ToggleButton>
            <ToggleButton value="c">{format!("{label} C")}</ToggleButton>
        </ToggleButtonGroup>
        <div>"Selection: " <span id=format!("test-tb-{name}-value")>{value}</span></div>
    }
}

/// Toggle buttons and groups (react-aria-components' `ToggleButton.test.js` and
/// `ToggleButtonGroup.test.js` setups).
#[component]
pub fn PageAtomToggleButton() -> impl IntoView {
    let value = RwSignal::new(false);

    view! {
        <h1>"Toggle Button"</h1>
        <button id="test-tb-before">"Before"</button>
        <ToggleButton on_change=move |selected| value.set(selected)>"Toggle"</ToggleButton>
        <div>"Selected: " <span id="test-tb-value">{move || value.get().to_string()}</span></div>
        <ToggleButton is_disabled=true>"Disabled toggle"</ToggleButton>

        <TestGroup name="single" label="Single" />
        <TestGroup name="multiple" label="Multiple" selection_mode=ToggleGroupSelectionMode::Multiple />
        <TestGroup name="vertical" label="Vertical" orientation=Orientation::Vertical />
        <TestGroup name="disabled" label="Disabled" is_disabled=true />
        <button id="test-tb-after">"After"</button>
    }
}
