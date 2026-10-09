use std::collections::VecDeque;

use leptonic::{
    IntoAttrs,
    atoms::checkbox::{CheckboxButton, CheckboxField},
    hooks::focus::{UseFocusInput, UseFocusReturn, use_focus},
};
use leptos::prelude::*;

#[component]
pub fn FocusDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let (is_focused, set_is_focused) = signal(false);
    let (events, set_events) = signal(VecDeque::<String>::new());

    let log = move |entry: String| {
        set_events.update(|events| {
            events.push_front(entry);
            events.truncate(50);
        });
    };

    let UseFocusReturn { props } = use_focus(UseFocusInput {
        is_disabled: disabled.into(),
        on_focus: Some(Callback::new(move |_| log("on_focus".to_string()))),
        on_blur: Some(Callback::new(move |_| log("on_blur".to_string()))),
        on_focus_change: Some(Callback::new(move |focused: bool| {
            set_is_focused.set(focused);
            log(format!("on_focus_change: {focused}"));
        })),
    });

    view! {
        <label class="demo-focus-label">
            "Name "
            <input type="text" class="demo-focus-item" {..props.into_attrs()}/>
        </label>

        <p class="demo-status">
            {move || if is_focused.get() { "The field has focus." } else { "The field doesn\u{2019}t have focus." }}
        </p>
        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>

        <p>"Last " {move || events.with(VecDeque::len)} " events:"</p>
        <pre class="demo-event-log">
            {move || events.with(|events| events.iter().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
