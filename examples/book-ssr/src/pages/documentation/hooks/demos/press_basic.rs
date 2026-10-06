use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, Observer, RingBuffer},
};

#[component]
pub fn PressBasicDemo() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (dbl_count, set_dbl_count) = signal(0);
    let (events, set_events) = signal(HeapRb::<String>::new(50));
    let (disabled, set_disabled) = signal(false);
    let (press_state, set_press_state) = signal(false);

    let log = move |name: &'static str, e: &PressEvent| {
        set_events.update(|events| {
            events.push_overwrite(format!(
                "{name}: pointer_type={:?}, key={:?}, x={:?}, y={:?}",
                e.pointer_type, e.key, e.x, e.y,
            ));
        });
    };

    let UsePressReturn { props, is_pressed } = use_press(UsePressInput {
        is_disabled: disabled.into(),
        on_press: Callback::new(move |e: PressEvent| {
            set_count.update(|c| *c += 1);
            log("Press", &e);
        }),
        on_press_up: Some(Callback::new(move |e: PressEvent| log("PressUp", &e))),
        on_press_start: Some(Callback::new(move |e: PressEvent| log("PressStart", &e))),
        on_press_end: Some(Callback::new(move |e: PressEvent| log("PressEnd", &e))),
        on_press_change: Some(Callback::new(move |pressed: bool| {
            set_press_state.set(pressed);
        })),
        on_double_press: Some(Callback::new(move |e: PressEvent| {
            set_dbl_count.update(|c| *c += 1);
            log("DoublePress", &e);
        })),
        ..Default::default()
    });
    let (attrs, styles) = props.into_parts();

    let times = |n: i32| if n == 1 { "time" } else { "times" };

    view! {
        <p>"Press the button with mouse, touch or keyboard (Tab to focus, Enter or Space to press)."</p>

        <button {..attrs} style=styles class="demo-press-button" class:pressed=move || is_pressed.get()>
            "Press me"
        </button>

        <Checkbox state=(disabled, set_disabled) classes=["demo-form-row", "demo-mt-1"]>"Disabled"</Checkbox>

        <p>"is_pressed: " {move || is_pressed.get()}</p>
        <p>"on_press_change: " {move || press_state.get()}</p>
        <p>"Pressed " {move || count.get()} " " {move || times(count.get())}</p>
        <p>"Double-pressed " {move || dbl_count.get()} " " {move || times(dbl_count.get())}</p>

        <p>"Last " {move || events.with(Observer::occupied_len)} " events:"</p>
        <pre class="demo-event-log">
            {move || events.with(|events| events.iter().rev().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
