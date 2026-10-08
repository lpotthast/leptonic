use std::collections::VecDeque;

use leptonic::{atoms::checkbox::{CheckboxButton, CheckboxField}, hooks::*, utils::data_attributes::flag};
use leptos::prelude::*;

#[component]
pub fn HoverDemo() -> impl IntoView {
    let (events, set_events) = signal(VecDeque::<String>::new());
    let disabled = RwSignal::new(false);

    let UseHoverReturn { props, is_hovered } = use_hover(UseHoverInput {
        is_disabled: disabled.into(),
        on_hover_start: Some(Callback::new(move |e: HoverStartEvent| {
            set_events.update(|events| {
                events.push_front(format!("HoverStart: pointer_type={:?}", e.pointer_type));
                events.truncate(50);
            });
        })),
        on_hover_end: Some(Callback::new(move |e: HoverEndEvent| {
            set_events.update(|events| {
                events.push_front(format!("HoverEnd: pointer_type={:?}", e.pointer_type));
                events.truncate(50);
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
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>

        <p class="demo-status">
            {move || if is_hovered.get() { "Hovered." } else { "Not hovered." }}
        </p>

        <p>"Last " {move || events.with(VecDeque::len)} " events:"</p>
        <pre class="demo-event-log">
            {move || events.with(|events| events.iter().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
