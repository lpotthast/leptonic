use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, Observer, RingBuffer},
};

#[component]
pub fn HoverDemo() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<String>::new(50));
    let (disabled, set_disabled) = signal(false);

    let UseHoverReturn { props, is_hovered } = use_hover(UseHoverInput {
        is_disabled: disabled.into(),
        on_hover_start: Some(Callback::new(move |e: HoverStartEvent| {
            set_events.update(|events| {
                events.push_overwrite(format!("HoverStart: pointer_type={:?}", e.pointer_type));
            });
        })),
        on_hover_end: Some(Callback::new(move |e: HoverEndEvent| {
            set_events.update(|events| {
                events.push_overwrite(format!("HoverEnd: pointer_type={:?}", e.pointer_type));
            });
        })),
        on_hover_change: None,
    });

    view! {
        <div {..props.into_attrs()} class="demo-hover-target" class:hovered=move || is_hovered.get()>
            "Hover me"
        </div>

        <p>
            "is_hovered: "
            <strong class=move || if is_hovered.get() { "demo-state-active" } else { "demo-state-inactive" }>
                {move || is_hovered.get()}
            </strong>
        </p>

        <Checkbox state=(disabled, set_disabled) classes="demo-form-row">"Disabled"</Checkbox>

        <p>"Last " {move || events.with(Observer::occupied_len)} " events:"</p>
        <pre class="demo-event-log">
            {move || events.with(|events| events.iter().rev().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
