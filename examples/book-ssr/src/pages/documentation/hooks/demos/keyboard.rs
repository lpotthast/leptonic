use std::collections::VecDeque;

use leptonic::{
    IntoAttrs, Propagation,
    atoms::{
        checkbox::{CheckboxButton, CheckboxField},
        focus_ring::FocusRing,
    },
    hooks::{
        focus::FocusRingTarget,
        interactions::{KeyboardEventWrapper, UseKeyboardInput, UseKeyboardReturn, use_keyboard},
    },
};
use leptos::prelude::*;

#[component]
pub fn KeyboardDemo() -> impl IntoView {
    let (events, set_events) = signal(VecDeque::<String>::new());
    let (disabled, set_disabled) = signal(false);

    let UseKeyboardReturn { props } = use_keyboard(UseKeyboardInput {
        is_disabled: disabled.into(),
        on_key_down: Some(Callback::new(move |e: KeyboardEventWrapper| {
            set_events.update(|events| {
                events.push_front(format!(
                    "KeyDown: key={}, code={}, shift={}, ctrl={}, alt={}, meta={}",
                    e.key_value(),
                    e.code(),
                    e.shift_key(),
                    e.ctrl_key(),
                    e.alt_key(),
                    e.meta_key()
                ));
                events.truncate(50);
            });
            e.continue_propagation();
        })),
        on_key_up: Some(Callback::new(move |e: KeyboardEventWrapper| {
            set_events.update(|events| {
                events.push_front(format!("KeyUp: key={}, code={}", e.key_value(), e.code()));
                events.truncate(50);
            });
            e.continue_propagation();
        })),
        ..Default::default()
    });

    view! {
        <FocusRing is_disabled=disabled target=FocusRingTarget::Within>
            <div {..props.into_attrs()} tabindex="0" class="demo-keyboard-target">
                "Focus me and press keys"
            </div>
        </FocusRing>

        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=set_disabled>
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
