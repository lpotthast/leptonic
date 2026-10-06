use std::time::Duration;

use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;
use ringbuf::{
    HeapRb,
    traits::{Consumer, Observer, RingBuffer},
};

#[component]
pub fn PressLongDemo() -> impl IntoView {
    let (lp_count, set_lp_count) = signal(0);
    let (lp_events, set_lp_events) = signal(HeapRb::<String>::new(50));
    let (lp_disabled, set_lp_disabled) = signal(false);
    let threshold_ms = RwSignal::new(Some(500_u64));
    let threshold =
        Signal::derive(move || Duration::from_millis(threshold_ms.get().unwrap_or(500)));

    let UsePressReturn { props, .. } = use_press(UsePressInput {
        is_disabled: lp_disabled.into(),
        on_long_press_start: Some(Callback::new(move |e: LongPressEvent| {
            set_lp_events.update(|events| {
                events.push_overwrite(format!(
                    "LongPressStart: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                ));
            });
        })),
        on_long_press: Some(Callback::new(move |e: LongPressEvent| {
            set_lp_count.update(|c| *c += 1);
            set_lp_events.update(|events| {
                events.push_overwrite(format!(
                    "LongPress: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                ));
            });
        })),
        on_long_press_end: Some(Callback::new(move |e: LongPressEvent| {
            set_lp_events.update(|events| {
                events.push_overwrite(format!(
                    "LongPressEnd: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                ));
            });
        })),
        long_press_threshold: Some(threshold),
        long_press_accessibility_description: Some("Long press to increment counter".into()),
        ..Default::default()
    });
    let (attrs, styles) = props.into_parts();

    view! {
        <p>
            "Press and hold the button below for " {move || threshold.get().as_millis()}
            "ms to trigger a long press:"
        </p>

        <button {..attrs} style=styles class="demo-btn-primary">
            "Long press me"
        </button>

        <Checkbox state=(lp_disabled, set_lp_disabled) classes=["demo-form-row", "demo-mt-1"]>"Disabled"</Checkbox>

        <div class="demo-form demo-mt-1">
            <NumberField
                label="Threshold (ms)"
                state=threshold_ms
                min_value=100_u64
                max_value=2000_u64
                step=100_u64
            />
        </div>

        <p>"Long press count: " {move || lp_count.get()}</p>

        <p>"Last " {move || lp_events.with(Observer::occupied_len)} " events:"</p>

        <pre class="demo-event-log">
            {move || lp_events.with(|events| events.iter().rev().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
