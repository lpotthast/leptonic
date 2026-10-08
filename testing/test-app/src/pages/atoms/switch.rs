use leptonic::atoms::switch::{SwitchButton, SwitchField};
use leptos::prelude::*;

/// Switches (react-aria-components' `Switch.test.js` setup).
#[component]
pub fn PageAtomSwitch() -> impl IntoView {
    let value = RwSignal::new(false);
    let bound = RwSignal::new(false);
    let bound_read_only = RwSignal::new(false);

    view! {
        <h1>"Switch"</h1>
        <button id="test-sw-before">"Before"</button>
        <SwitchField on_change=move |selected| value.set(selected)><SwitchButton>"Basic"</SwitchButton></SwitchField>
        <div>"Selected: " <span id="test-sw-value">{move || value.get().to_string()}</span></div>
        <SwitchField is_disabled=true><SwitchButton>"Disabled"</SwitchButton></SwitchField>
        <SwitchField is_read_only=true default_selected=true><SwitchButton>"Read only"</SwitchButton></SwitchField>
        <SwitchField is_selected=bound set_selected=bound><SwitchButton>"Bound"</SwitchButton></SwitchField>
        <button id="test-sw-bound-flip" on:click=move |_| bound.update(|b| *b = !*b)>"Flip"</button>
        <div>"Bound: " <span id="test-sw-bound-value">{move || bound.get().to_string()}</span></div>
        <SwitchField is_selected=bound_read_only set_selected=bound_read_only is_read_only=true><SwitchButton>"Bound read only"</SwitchButton></SwitchField>
        <div>"Bound read only: " <span id="test-sw-bound-read-only-value">{move || bound_read_only.get().to_string()}</span></div>
    }
}
