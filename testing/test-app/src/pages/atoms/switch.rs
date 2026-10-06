use leptonic::{atoms::switch::Switch, hooks::ToggleState};
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
        <Switch on_change=move |selected| value.set(selected)>"Basic"</Switch>
        <div>"Selected: " <span id="test-sw-value">{move || value.get().to_string()}</span></div>
        <Switch is_disabled=true>"Disabled"</Switch>
        <Switch is_read_only=true default_selected=true>"Read only"</Switch>
        <Switch state=ToggleState::from(bound)>"Bound"</Switch>
        <button id="test-sw-bound-flip" on:click=move |_| bound.update(|b| *b = !*b)>"Flip"</button>
        <div>"Bound: " <span id="test-sw-bound-value">{move || bound.get().to_string()}</span></div>
        <Switch state=ToggleState::from(bound_read_only) is_read_only=true>"Bound read only"</Switch>
        <div>"Bound read only: " <span id="test-sw-bound-read-only-value">{move || bound_read_only.get().to_string()}</span></div>
    }
}
