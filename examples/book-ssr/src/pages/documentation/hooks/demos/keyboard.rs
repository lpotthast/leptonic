use leptonic::{
    atoms::focus_ring::FocusRing, components::prelude::*, hooks::*, utils::Propagation,
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

        <Checkbox state=(disabled, set_disabled) classes="demo-form-row">"Disabled"</Checkbox>

        <p>"Last " {move || events.with(Observer::occupied_len)} " events:"</p>

        <pre class="demo-event-log">
            {move || events.with(|events| events.iter().rev().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
