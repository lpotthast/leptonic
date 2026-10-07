use leptonic::{
    hooks::{PressEvent, UsePressInput, UsePressReturn, use_press},
    utils::propagation_control::Propagation,
};
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
        is_disabled: disabled,
        on_press: Some(entry("press")),
        on_press_up: Some(entry("up")),
        on_press_start: Some(entry("start")),
        on_press_end: Some(entry("end")),
        ..UsePressInput::default()
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
            <PreventFocusPress />
            <NestedPress id="test-press-nested-stop" continue_inner=false />
            <NestedPress id="test-press-nested-continue" continue_inner=true />
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
                <span id="test-press-label">"Press me"</span>
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

/// An inner pressable inside an outer one (react-aria's "event bubbling" tests). The inner one's
/// callbacks continue propagation when `continue_inner`. Logs to `#{id}-outer-log`/`-inner-log`.
#[component]
fn NestedPress(id: &'static str, continue_inner: bool) -> impl IntoView {
    let outer_log = EventLog(RwSignal::new(Vec::new()));
    let inner_log = EventLog(RwSignal::new(Vec::new()));
    // As upstream's tests: the outer pressable listens to press up only when the inner continues
    // (a pointer up bubbles to it either way, and starts no press there).
    let mut outer_input = press_input(outer_log, Signal::stored(false));
    if !continue_inner {
        outer_input.on_press_up = None;
    }
    let outer = use_press(outer_input);
    let entry = move |name: &'static str| {
        Callback::new(move |e: PressEvent| {
            if continue_inner {
                e.continue_propagation();
            }
            inner_log.push(name);
        })
    };
    let inner = use_press(UsePressInput {
        on_press: Some(entry("press")),
        on_press_up: Some(entry("up")),
        on_press_start: Some(entry("start")),
        on_press_end: Some(entry("end")),
        ..UsePressInput::default()
    });
    let (outer_attrs, outer_styles) = outer.props.into_parts();
    let (inner_attrs, inner_styles) = inner.props.into_parts();
    view! {
        <section id=id>
            <div role="button" tabindex="0" {..outer_attrs} style=outer_styles>
                "Outer "
                <div id=format!("{id}-inner") role="button" tabindex="0" {..inner_attrs} style=inner_styles>
                    "Inner"
                </div>
            </div>
            <div>"Outer: " <span id=format!("{id}-outer-log")>{outer_log.render()}</span></div>
            <div>"Inner: " <span id=format!("{id}-inner-log")>{inner_log.render()}</span></div>
        </section>
    }
}

/// A button pressed without taking focus (`prevent_focus_on_press`): focus stays on the input
/// before it, which logs its blur events to `#test-press-keep-blurs`.
#[component]
fn PreventFocusPress() -> impl IntoView {
    let presses = RwSignal::new(0);
    let blurs = RwSignal::new(0);
    let UsePressReturn { props, .. } = use_press(UsePressInput {
        prevent_focus_on_press: Signal::stored(true),
        on_press: Some(Callback::new(move |_| presses.update(|p| *p += 1))),
        ..UsePressInput::default()
    });
    let (attrs, styles) = props.into_parts();
    view! {
        <section>
            <input id="test-press-keep-input" on:blur=move |_| blurs.update(|b| *b += 1) />
            <button id="test-press-keep" {..attrs} style=styles>"Press without focus"</button>
            <div>"Presses: " <span id="test-press-keep-presses">{presses}</span></div>
            <div>"Blurs: " <span id="test-press-keep-blurs">{blurs}</span></div>
        </section>
    }
}
