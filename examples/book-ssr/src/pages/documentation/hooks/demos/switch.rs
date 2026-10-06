use leptonic::{components::prelude::*, hooks::*, utils::visually_hidden::visually_hidden_styles};
use leptos::prelude::*;

#[component]
pub fn SwitchDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);

    let state = use_toggle_state(UseToggleStateInput {
        is_read_only: read_only.into(),
        ..UseToggleStateInput::default()
    });
    let switch = use_switch(UseSwitchInput {
        options: ToggleOptions {
            is_disabled: disabled.into(),
            is_read_only: read_only.into(),
            name: Some("notifications".to_owned()),
            ..ToggleOptions::default()
        },
        ..UseSwitchInput::new(state)
    });
    let (label_attrs, label_styles) = switch.label_props.into_parts();
    let (input_attrs, input_styles) = switch.input_props.into_parts();

    // The hook renders no state: the demo exposes it as data attributes for its stylesheet.
    let flag = |signal: Signal<bool>| move || signal.get().then_some("");

    view! {
        <label
            {..label_attrs}
            style=label_styles
            class="demo-switch"
            data-selected=flag(switch.is_selected)
            data-focus-visible=flag(switch.is_focus_visible)
            data-disabled=flag(switch.is_disabled)
            data-readonly=flag(switch.is_read_only)
        >
            // The input stays in the page for assistive technology and forms, but invisible.
            <input {..input_attrs} style=input_styles.merge(visually_hidden_styles())/>
            <span class="demo-switch-track" aria-hidden="true">
                <span class="demo-switch-thumb"></span>
            </span>
            "Notifications"
        </label>

        <p class="demo-status">
            {move || if state.is_selected.get() { "Notifications are on." } else { "Notifications are off." }}
        </p>

        <div class="demo-toggle-settings">
            <Checkbox state=disabled>"Disabled"</Checkbox>
            <Checkbox state=read_only>"Read-only"</Checkbox>
        </div>
    }
}
