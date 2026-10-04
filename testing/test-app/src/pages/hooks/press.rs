use leptonic::hooks::{PressEvent, UsePressInput, UsePressReturn, use_press};
use leptos::prelude::*;

/// Every press callback appends to a shared log, so tests can assert the exact event order.
#[derive(Clone, Copy)]
struct EventLog(RwSignal<Vec<String>>);

impl EventLog {
    fn push(self, entry: impl Into<String>) {
        self.0.update(|log| log.push(entry.into()));
    }

    fn render(self) -> impl Fn() -> String {
        move || self.0.with(|log| log.join(","))
    }
}

fn press_input(log: EventLog, disabled: Signal<bool>) -> UsePressInput {
    let entry = |name: &'static str| {
        Callback::new(move |e: PressEvent| log.push(format!("{name}:{}", e.pointer_type)))
    };
    UsePressInput {
        disabled,
        force_prevent_default: false,
        force_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        prevent_focus_on_press: false,
        force_is_pressed: None,
        on_press: entry("press"),
        on_press_up: Some(entry("up")),
        on_press_start: Some(entry("start")),
        on_press_end: Some(entry("end")),
        on_press_change: None,
        on_double_press: None,
        on_long_press_start: None,
        on_long_press: None,
        on_long_press_end: None,
        long_press_threshold: None,
        long_press_accessibility_description: None,
    }
}

#[component]
pub fn PageHookPress() -> impl IntoView {
    view! {
        <div id="test-page-hook-press">
            <h1>"use_press"</h1>
            <BasicPress />
            <DisableOnPressStart />
            <CheckboxInForm />
            <button id="test-press-elsewhere">"Elsewhere"</button>
        </div>
    }
}

/// A plain `<div role="button">` with press handling and a disabled toggle.
#[component]
fn BasicPress() -> impl IntoView {
    let log = EventLog(RwSignal::new(Vec::new()));
    let disabled = RwSignal::new(false);
    let UsePressReturn { props, is_pressed } = use_press(press_input(log, disabled.into()));
    let (attrs, styles) = props.into_parts();

    view! {
        <section>
            <h2>"Basic"</h2>
            <div id="test-press-target" role="button" tabindex="0" {..attrs} style=styles>
                "Press me"
            </div>
            <button id="test-press-toggle-disabled" on:click=move |_| disabled.update(|d| *d = !*d)>
                "Toggle disabled"
            </button>
            <button id="test-press-clear" on:click=move |_| log.0.set(Vec::new())>
                "Clear log"
            </button>
            <div>"Pressed: " <span id="test-press-is-pressed">{move || is_pressed.get().to_string()}</span></div>
            <div>"Log: " <span id="test-press-log">{log.render()}</span></div>
        </section>
    }
}

/// The element disables itself as soon as a press starts (like a spin button reaching its limit
/// on the first step). The press must be cancelled instead of staying active forever.
#[component]
fn DisableOnPressStart() -> impl IntoView {
    let log = EventLog(RwSignal::new(Vec::new()));
    let disabled = RwSignal::new(false);
    let mut input = press_input(log, disabled.into());
    input.on_press_start = Some(Callback::new(move |e: PressEvent| {
        log.push(format!("start:{}", e.pointer_type));
        disabled.set(true);
    }));
    let UsePressReturn { props, is_pressed } = use_press(input);
    let (attrs, styles) = props.into_parts();

    view! {
        <section>
            <h2>"Disable on press start"</h2>
            <div id="test-press-self-disabling" role="button" tabindex="0" {..attrs} style=styles>
                "Press me once"
            </div>
            <div>
                "Pressed: "
                <span id="test-press-self-disabling-is-pressed">
                    {move || is_pressed.get().to_string()}
                </span>
            </div>
            <div>"Log: " <span id="test-press-self-disabling-log">{log.render()}</span></div>
        </section>
    }
}

/// Enter on a pressable checkbox must still submit the surrounding form (implicit submission).
#[component]
fn CheckboxInForm() -> impl IntoView {
    let log = EventLog(RwSignal::new(Vec::new()));
    let submits = RwSignal::new(0u32);
    let UsePressReturn { props, .. } = use_press(press_input(log, Signal::stored(false)));
    let (attrs, styles) = props.into_parts();

    view! {
        <section>
            <h2>"Checkbox in form"</h2>
            <form on:submit=move |e| {
                e.prevent_default();
                submits.update(|s| *s += 1);
            }>
                <input id="test-press-checkbox" type="checkbox" {..attrs} style=styles />
                <button type="submit">"Submit"</button>
            </form>
            <div>"Submits: " <span id="test-press-submits">{submits}</span></div>
            <div>"Log: " <span id="test-press-checkbox-log">{log.render()}</span></div>
        </section>
    }
}
