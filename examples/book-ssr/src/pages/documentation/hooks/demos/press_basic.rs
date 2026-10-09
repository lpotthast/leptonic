use std::collections::VecDeque;

use leptonic::{
    atoms::checkbox::{CheckboxButton, CheckboxField},
    flag,
    hooks::interactions::{PressEvent, UsePressInput, UsePressReturn, use_press},
};
use leptos::prelude::*;

#[component]
pub fn PressBasicDemo() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (dbl_count, set_dbl_count) = signal(0);
    let (events, set_events) = signal(VecDeque::<String>::new());
    let (disabled, set_disabled) = signal(false);
    let (press_state, set_press_state) = signal(false);

    let log = move |name: &'static str, e: &PressEvent| {
        set_events.update(|events| {
            events.push_front(format!(
                "{name}: pointer_type={:?}, key={:?}, x={}, y={}",
                e.pointer_type, e.key, e.point.x, e.point.y,
            ));
            events.truncate(50);
        });
    };

    let UsePressReturn { props, is_pressed } = use_press(UsePressInput {
        is_disabled: disabled.into(),
        on_press: Some(Callback::new(move |e: PressEvent| {
            set_count.update(|c| *c += 1);
            log("Press", &e);
        })),
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

    let times = |n: i32| {
        if n == 1 {
            "1 time".to_owned()
        } else {
            format!("{n} times")
        }
    };

    view! {
        <p>"Press the button with mouse, touch or keyboard (Tab to focus, Enter or Space to press)."</p>

        // `data-pressed` shows the hook's `is_pressed` state to CSS.
        <button {..attrs} style=styles class="demo-press-button" data-pressed=flag(is_pressed) disabled=move || disabled.get()>
            "Press me"
        </button>

        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=set_disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>

        <p class="demo-status">
            {move || format!(
                "Pressed {}, double-pressed {}. is_pressed: {}, last on_press_change: {}.",
                times(count.get()),
                times(dbl_count.get()),
                is_pressed.get(),
                press_state.get(),
            )}
        </p>

        <p>"Last " {move || events.with(VecDeque::len)} " events:"</p>
        <pre class="demo-event-log">
            {move || events.with(|events| events.iter().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
