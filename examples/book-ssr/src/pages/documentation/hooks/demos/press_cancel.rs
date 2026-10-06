use leptonic::{hooks::*, utils::data_attributes::flag};
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

#[component]
pub fn PressCancelDemo() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<String>::new(50));
    let log = move |message: String| {
        set_events.update(|events| {
            events.push_overwrite(message);
        });
    };

    let UsePressReturn { props, is_pressed } = use_press(UsePressInput {
        should_cancel_on_pointer_exit: true.into(),
        on_press: Some(Callback::new(move |_| log("Press completed".to_owned()))),
        on_press_start: Some(Callback::new(move |_| log("PressStart".to_owned()))),
        on_press_end: Some(Callback::new(move |e: PressEvent| {
            log(format!("PressEnd: pointer_type={:?}", e.pointer_type));
        })),
        ..Default::default()
    });
    let (attrs, styles) = props.into_parts();

    view! {
        <button {..attrs} style=styles class="demo-press-button" data-pressed=flag(is_pressed)>
            "Drag outside to cancel"
        </button>

        <p class="demo-caption">"Press the button, drag the pointer out and release: the press is cancelled."</p>

        <pre class="demo-event-log demo-interactions-log-short">
            {move || events.with(|events| events.iter().rev().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
