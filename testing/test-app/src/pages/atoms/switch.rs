use leptonic::atoms::switch::{SwitchButton, SwitchField};
use leptos::prelude::*;

/// Switches (react-aria-components' `Switch.test.js` setup). "Remounted":
/// `#test-sw-remount-toggle` unmounts and mounts its `SwitchButton`; its selection is in
/// `#test-sw-remounted-value`.
#[component]
pub fn PageAtomSwitch() -> impl IntoView {
    let value = RwSignal::new(false);
    let bound = RwSignal::new(false);
    let bound_read_only = RwSignal::new(false);
    let remounted = RwSignal::new(false);
    let remounted_shown = RwSignal::new(true);

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
        <SwitchField is_selected=remounted set_selected=remounted>
            <Show when=move || remounted_shown.get()>
                <SwitchButton>"Remounted"</SwitchButton>
            </Show>
        </SwitchField>
        <button id="test-sw-remount-toggle" on:click=move |_| remounted_shown.update(|shown| *shown = !*shown)>
            "Hide or show the remounted switch"
        </button>
        <div>"Remounted: " <span id="test-sw-remounted-value">{move || remounted.get().to_string()}</span></div>
    }
}
