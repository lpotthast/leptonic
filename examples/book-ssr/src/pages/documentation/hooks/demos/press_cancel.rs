use leptonic::hooks::*;
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, RingBuffer},
};

#[component]
pub fn PressCancelDemo() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<String>::new(10));
    let log = move |message: String| {
        set_events.update(|events| {
            events.push_overwrite(message);
        });
    };

    let UsePressReturn { props, is_pressed } = use_press(UsePressInput {
        should_cancel_on_pointer_exit: true,
        on_press: Callback::new(move |_| log("Press completed".to_owned())),
        on_press_start: Some(Callback::new(move |_| log("PressStart".to_owned()))),
        on_press_end: Some(Callback::new(move |e: PressEvent| {
            log(format!("PressEnd: pointer_type={:?}", e.pointer_type));
        })),
        ..Default::default()
    });
    let (attrs, styles) = props.into_parts();

    view! {
        <p>
            <strong>"should_cancel_on_pointer_exit"</strong>
            " \u{2014} Press the button, then drag the pointer outside before releasing. "
            "The press is cancelled and "<code>"on_press"</code>" does not fire."
        </p>

        <button {..attrs} style=styles class="demo-press-button" class:pressed=move || is_pressed.get()>
            "Drag outside to cancel"
        </button>

        <pre class="demo-event-log demo-interactions-log-short">
            {move || events.with(|events| events.iter().rev().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
