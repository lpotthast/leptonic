use std::time::Duration;

use leptonic::hooks::interactions::{
    DEFAULT_LONG_PRESS_THRESHOLD, LongPress, LongPressEvent, PressEvent, UsePressInput, use_press,
};
use leptos::prelude::*;

/// Long presses through `use_press` (react-aria's `useLongPress.test.js`). Events are appended to
/// `#test-long-press-log` as `<element>:<event>:<pointer type>`.
#[component]
pub fn PageHookLongPress() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    view! {
        <div id="test-page-hook-long-press">
            <h1>"Long press"</h1>
            <style>
                ".test-long-pressable { display: inline-block; padding: 8px; border: 1px solid }"
            </style>
            <LongPressable log name="basic" />
            <LongPressable log name="with-press" with_press=true />
            <LongPressable log name="threshold" threshold=Duration::from_millis(1000) />
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
            <LongPressSubmit log />
            <button id="test-long-press-elsewhere">"Elsewhere"</button>
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
        long_press: Some(LongPress {
            on_long_press_start: long("longpressstart"),
            on_long_press_end: long("longpressend"),
            on_long_press: if no_long_press_handler {
                None
            } else {
                long("longpress")
            },
            threshold: Signal::stored(threshold.unwrap_or(DEFAULT_LONG_PRESS_THRESHOLD)),
            accessibility_description: description.map(str::to_owned).into(),
            is_disabled: Signal::stored(false),
        }),
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

/// A long-pressable submit button in a form counting its submissions in
/// `#test-long-press-submits`: the click after a long press must not submit.
#[component]
fn LongPressSubmit(log: RwSignal<Vec<String>>) -> impl IntoView {
    let submits = RwSignal::new(0);
    let pressing = use_press(UsePressInput {
        long_press: Some(LongPress {
            on_long_press: Some(Callback::new(move |e: LongPressEvent| {
                log.update(|l| l.push(format!("submit:longpress:{}", e.pointer_type)));
            })),
            ..LongPress::default()
        }),
        ..UsePressInput::default()
    });
    let (attrs, styles) = pressing.props.into_parts();
    view! {
        <form on:submit=move |e| {
            e.prevent_default();
            submits.update(|s| *s += 1);
        }>
            <button id="test-long-press-submit" type="submit" {..attrs} style=styles>
                "Submit"
            </button>
        </form>
        <div>"Submits: " <span id="test-long-press-submits">{submits}</span></div>
    }
}
