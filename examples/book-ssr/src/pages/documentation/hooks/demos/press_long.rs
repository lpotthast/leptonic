use std::collections::VecDeque;

use std::time::Duration;

use leptonic::{
    atoms::{
        checkbox::Checkbox,
        field::Label,
        input::Input,
        number_field::{
            NumberField, NumberFieldDecrementButton, NumberFieldGroup, NumberFieldIncrementButton,
        },
    },
    hooks::*,
    utils::data_attributes::flag,
};
use leptos::prelude::*;

#[component]
pub fn PressLongDemo() -> impl IntoView {
    let (lp_count, set_lp_count) = signal(0);
    let (lp_events, set_lp_events) = signal(VecDeque::<String>::new());
    let (lp_disabled, set_lp_disabled) = signal(false);
    let threshold_ms = RwSignal::new(Some(500_u64));
    let threshold =
        Signal::derive(move || Duration::from_millis(threshold_ms.get().unwrap_or(500)));

    let UsePressReturn { props, is_pressed } = use_press(UsePressInput {
        is_disabled: lp_disabled.into(),
        on_long_press_start: Some(Callback::new(move |e: LongPressEvent| {
            set_lp_events.update(|events| {
                events.push_front(format!(
                    "LongPressStart: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                ));
                events.truncate(50);
            });
        })),
        on_long_press: Some(Callback::new(move |e: LongPressEvent| {
            set_lp_count.update(|c| *c += 1);
            set_lp_events.update(|events| {
                events.push_front(format!(
                    "LongPress: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                ));
                events.truncate(50);
            });
        })),
        on_long_press_end: Some(Callback::new(move |e: LongPressEvent| {
            set_lp_events.update(|events| {
                events.push_front(format!(
                    "LongPressEnd: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                ));
                events.truncate(50);
            });
        })),
        long_press_threshold: Some(threshold),
        long_press_accessibility_description: "Long press to increment counter".into(),
        ..Default::default()
    });
    let (attrs, styles) = props.into_parts();

    view! {
        <p>
            "Press and hold the button for " {move || threshold.get().as_millis()}
            " ms to trigger a long press."
        </p>

        <button {..attrs} style=styles class="demo-press-button" data-pressed=flag(is_pressed) disabled=move || lp_disabled.get()>
            "Long press me"
        </button>

        <p class="demo-status">
            {move || match lp_count.get() {
                1 => "Long-pressed 1 time.".to_owned(),
                n => format!("Long-pressed {n} times."),
            }}
        </p>
        <div class="demo-controls">
            <Checkbox is_selected=lp_disabled set_selected=set_lp_disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
            <NumberField
                value=threshold_ms
                set_value=threshold_ms
                min_value=100_u64
                max_value=2000_u64
                step=100_u64
                classes="demo-field"
            >
                <Label classes="demo-field-label">"Threshold (ms)"</Label>
                <NumberFieldGroup classes="demo-number-field-group">
                    <NumberFieldDecrementButton classes="demo-number-field-stepper">
                        <span aria-hidden="true">"\u{2212}"</span>
                    </NumberFieldDecrementButton>
                    <Input classes="demo-number-field-input"/>
                    <NumberFieldIncrementButton classes="demo-number-field-stepper">
                        <span aria-hidden="true">"+"</span>
                    </NumberFieldIncrementButton>
                </NumberFieldGroup>
            </NumberField>
        </div>

        <p>"Last " {move || lp_events.with(VecDeque::len)} " events:"</p>
        <pre class="demo-event-log">
            {move || lp_events.with(|events| events.iter().cloned().collect::<Vec<_>>().join("\n"))}
        </pre>
    }
}
