use leptonic::{
    atoms::{checkbox::Checkbox, focus_ring::FocusRing}, hooks::*, utils::Propagation,
};
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, Observer, RingBuffer},
};

#[component]
pub fn KeyboardDemo() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<String>::new(50));
    let (disabled, set_disabled) = signal(false);

    let UseKeyboardReturn { props } = use_keyboard(UseKeyboardInput {
        is_disabled: disabled.into(),
        on_key_down: Some(Callback::new(move |e: KeyboardEventWrapper| {
            set_events.update(|events| {
                events.push_overwrite(format!(
                    "KeyDown: key={}, code={}, shift={}, ctrl={}, alt={}, meta={}",
                    e.key_value(),
                    e.code(),
                    e.shift_key(),
                    e.ctrl_key(),
                    e.alt_key(),
                    e.meta_key()
                ));
            });
            e.continue_propagation();
        })),
        on_key_up: Some(Callback::new(move |e: KeyboardEventWrapper| {
            set_events.update(|events| {
                events.push_overwrite(format!("KeyUp: key={}, code={}", e.key_value(), e.code()));
            });
            e.continue_propagation();
        })),
        ..Default::default()
    });

    view! {
        <FocusRing is_disabled=disabled within=true>
            <div {..props.into_attrs()} tabindex="0" class="demo-keyboard-target">
                "Focus me and press keys"
            </div>
        </FocusRing>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=set_disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>

        <p>"Last " {move || events.with(Observer::occupied_len)} " events:"</p>

        <pre class="demo-event-log">
            {move || events.with(|events| events.iter().rev().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
