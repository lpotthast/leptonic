use leptonic::{
    atoms::focus_ring::FocusRing, components::prelude::*, hooks::*, utils::Propagation,
};
use leptos::prelude::*;
use leptos_classes::Classes;
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

#[component]
pub fn KeyboardDemo() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<String>::new(20));
    let (disabled, set_disabled) = signal(false);

    let string = Memo::new(move |_| {
        events.with(|events| {
            let mut result = String::new();
            for e in events.iter().rev() {
                result.push_str(e.as_str());
                result.push('\n');
            }
            result
        })
    });

    let UseKeyboardReturn { props } = use_keyboard(UseKeyboardInput {
        disabled: disabled.into(),
        on_key_down: Some(Callback::new(move |e: KeyboardEventWrapper| {
            set_events.update(|events| {
                events.push_overwrite(format!(
                    "KeyDown: key={}, code={}, shift={}, ctrl={}, alt={}, meta={}",
                    e.key(),
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
                events.push_overwrite(format!("KeyUp: key={}, code={}", e.key(), e.code()));
            });
            e.continue_propagation();
        })),
    });

    view! {
        <FocusRing disabled within=true>
            <div
                {..props.into_attrs()}
                tabindex="0"
                class=Classes::from("demo-btn")
            >
                "Focus me and press keys"
            </div>
        </FocusRing>

        <FormControl classes="demo-form-row">
            <Checkbox checked=disabled set_checked=set_disabled />
            <Label>"Disabled"</Label>
        </FormControl>

        <p>"Last " { move || events.with(ringbuf::traits::Observer::occupied_len) } " events:"</p>

        <pre class=Classes::from("demo-event-log")>
            { move || string.get() }
        </pre>
    }
}
