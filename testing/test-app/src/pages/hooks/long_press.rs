use std::time::Duration;

use leptonic::hooks::{LongPressEvent, PressEvent, UsePressInput, use_press};
use leptos::prelude::*;

/// Long presses through `use_press` (react-aria's `useLongPress.test.js`). Events are appended to
/// `#test-long-press-log` as `<element>:<event>:<pointer type>`.
#[component]
pub fn PageHookLongPress() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    view! {
        <div id="test-page-hook-long-press">
            <h1>"Long press"</h1>
            <style>".test-long-pressable { display: inline-block; padding: 8px; border: 1px solid }"</style>
            <LongPressable log name="basic" />
            <LongPressable log name="with-press" with_press=true />
            <LongPressable log name="threshold" threshold=Duration::from_millis(1500) />
            <LongPressable log name="description" description="Long press to open a menu" />
            <LongPressable
                log
                name="description-disabled"
                description="Long press to open a menu"
                disabled=true
            />
            <LongPressable
                log
                name="description-no-handler"
                description="Long press to open a menu"
                no_long_press_handler=true
            />
            <button id="test-long-press-reset" on:click=move |_| log.set(Vec::new())>
                "Reset log"
            </button>
            <div>"Log: " <span id="test-long-press-log">{move || log.get().join(",")}</span></div>
        </div>
    }
}

#[component]
fn LongPressable(
    log: RwSignal<Vec<String>>,
    name: &'static str,
    #[prop(optional)] with_press: bool,
    #[prop(optional, into)] threshold: Option<Duration>,
    #[prop(optional)] description: Option<&'static str>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] no_long_press_handler: bool,
) -> impl IntoView {
    let long = move |event: &'static str| {
        Some(Callback::new(move |e: LongPressEvent| {
            log.update(|l| l.push(format!("{name}:{event}:{}", e.pointer_type)));
        }))
    };
    let press = move |event: &'static str| {
        with_press.then(|| {
            Callback::new(move |e: PressEvent| {
                log.update(|l| l.push(format!("{name}:{event}:{}", e.pointer_type)));
            })
        })
    };
    let pressing = use_press(UsePressInput {
        is_disabled: disabled.into(),
        on_press_start: press("pressstart"),
        on_press_end: press("pressend"),
        on_press: press("press"),
        on_long_press_start: long("longpressstart"),
        on_long_press_end: long("longpressend"),
        on_long_press: if no_long_press_handler {
            None
        } else {
            long("longpress")
        },
        long_press_threshold: threshold.map(Signal::stored),
        long_press_accessibility_description: description.map(str::to_owned).into(),
        ..UsePressInput::default()
    });
    let (attrs, styles) = pressing.props.into_parts();
    view! {
        <div
            id=format!("test-long-press-{name}")
            tabindex="0"
            class="test-long-pressable"
            {..attrs}
            style=styles
        >
            {name}
        </div>
    }
}
