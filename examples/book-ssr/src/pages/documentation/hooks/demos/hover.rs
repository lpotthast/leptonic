use leptonic::{atoms::checkbox::Checkbox, hooks::*, utils::data_attributes::flag};
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, Observer, RingBuffer},
};

#[component]
pub fn HoverDemo() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<String>::new(50));
    let disabled = RwSignal::new(false);

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
        ..Default::default()
    });

    view! {
        // `data-hovered` shows the hook's `is_hovered` state to CSS.
        <div {..props.into_attrs()} class="demo-hover-target" data-hovered=flag(is_hovered)>
            "Hover me"
        </div>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>

        <p class="demo-status">
            {move || if is_hovered.get() { "Hovered." } else { "Not hovered." }}
        </p>

        <p>"Last " {move || events.with(Observer::occupied_len)} " events:"</p>
        <pre class="demo-event-log">
            {move || events.with(|events| events.iter().rev().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
