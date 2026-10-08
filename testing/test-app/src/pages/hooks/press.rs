use std::time::Duration;

use leptonic::{
    hooks::{
        IntoAttrs, PressEvent, PressPropagation, UseHoverInput, UseKeyboardInput, UsePressInput,
        UsePressReturn, use_hover, use_keyboard, use_press,
    },
    utils::{MergeWith, propagation_control::Propagation},
};
use leptos::{html, prelude::*};

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
            <KeyUpStoppingPress />
            <DragInPress />
            <ClickStoppingChild />
            <FocusMovingPress />
            <ContentsPress />
            <CancelOnExitPress />
            <LinkPress />
            <DoublePress />
            <ContinuingPress />
            <RemovedWhilePressed />
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

/// A button whose own keyup handler stops the event (`use_keyboard` with `on_key_up`, as
/// `FocusablePress` merges them): the keyboard press still ends. `#test-press-keyup-presses` counts
/// the presses, `#test-press-keyup-pressed` shows `is_pressed`.
#[component]
fn KeyUpStoppingPress() -> impl IntoView {
    let presses = RwSignal::new(0);
    let key_ups = RwSignal::new(0);
    let UsePressReturn {
        props, is_pressed, ..
    } = use_press(UsePressInput {
        on_press: Some(Callback::new(move |_| presses.update(|p| *p += 1))),
        ..UsePressInput::default()
    });
    let keyboard = use_keyboard(UseKeyboardInput {
        on_key_up: Some(Callback::new(move |_| key_ups.update(|k| *k += 1))),
        ..UseKeyboardInput::default()
    });
    let (attrs, styles) = props.into_parts();
    view! {
        <section>
            <div
                id="test-press-keyup"
                role="button"
                tabindex="0"
                {..attrs}
                {..keyboard.props.into_attrs()}
                style=styles
            >
                "Key up stops"
            </div>
            <div>"Presses: " <span id="test-press-keyup-presses">{presses}</span></div>
            <div>"Key ups: " <span id="test-press-keyup-key-ups">{key_ups}</span></div>
            <div>"Pressed: " <span id="test-press-keyup-pressed">{move || is_pressed.get().to_string()}</span></div>
        </section>
    }
}

/// A pressable with `on_press_end` holding a draggable image: a drag that starts inside cancels the
/// press (Safari fires no pointercancel then). `#test-press-drag-log` logs the press events.
#[component]
fn DragInPress() -> impl IntoView {
    let log = EventLog(RwSignal::new(Vec::new()));
    let UsePressReturn { props, .. } = use_press(UsePressInput {
        on_press: Some(Callback::new(move |_| log.push("press"))),
        on_press_start: Some(Callback::new(move |_| log.push("start"))),
        on_press_end: Some(Callback::new(move |_| log.push("end"))),
        ..UsePressInput::default()
    });
    let (attrs, styles) = props.into_parts();
    view! {
        <section>
            <div id="test-press-drag" role="button" tabindex="0" {..attrs} style=styles>
                <img
                    id="test-press-drag-image"
                    draggable="true"
                    alt="Drag me"
                    width="40"
                    height="40"
                    src="data:image/gif;base64,R0lGODlhAQABAAAAACw="
                />
            </div>
            <div>"Log: " <span id="test-press-drag-log">{log.render()}</span></div>
        </section>
    }
}

/// A pressable whose child stops its `click` (react-aria's "should cancel press if onClick
/// propagation is stopped"): the press is cancelled, not completed by the click fallback.
#[component]
fn ClickStoppingChild() -> impl IntoView {
    let log = EventLog(RwSignal::new(Vec::new()));
    let UsePressReturn { props, .. } = use_press(press_input(log, Signal::stored(false)));
    let (attrs, styles) = props.into_parts();
    view! {
        <section>
            <div id="test-press-click-stop" role="button" tabindex="0" {..attrs} style=styles>
                <span
                    id="test-press-click-stop-child"
                    on:click=|e: leptos::ev::MouseEvent| e.stop_propagation()
                >
                    "Child stopping clicks"
                </span>
            </div>
            <div>"Log: " <span id="test-press-click-stop-log">{log.render()}</span></div>
        </section>
    }
}

/// A pressable moving focus to another button when its press starts ("should handle when focus
/// moves between keydown and keyup"): the key up happens there, so there is no press up and no
/// press.
#[component]
fn FocusMovingPress() -> impl IntoView {
    let log = EventLog(RwSignal::new(Vec::new()));
    let other = NodeRef::<html::Button>::new();
    let mut input = press_input(log, Signal::stored(false));
    input.on_press_start = Some(Callback::new(move |e: PressEvent| {
        log.push(format!("start:{}", e.pointer_type));
        if let Some(other) = other.get_untracked() {
            let _ = other.focus();
        }
    }));
    let UsePressReturn { props, .. } = use_press(input);
    let (attrs, styles) = props.into_parts();
    view! {
        <section>
            <div id="test-press-focus-move" role="button" tabindex="0" {..attrs} style=styles>
                "Moves focus on press start"
            </div>
            <button id="test-press-focus-move-other" node_ref=other>"Other"</button>
            <div>"Log: " <span id="test-press-focus-move-log">{log.render()}</span></div>
        </section>
    }
}

/// A pressable `display: contents` element (no box of its own): dragging out of and back into its
/// child ends and restarts the press.
#[component]
fn ContentsPress() -> impl IntoView {
    let log = EventLog(RwSignal::new(Vec::new()));
    let UsePressReturn { props, .. } = use_press(press_input(log, Signal::stored(false)));
    let (attrs, styles) = props.into_parts();
    view! {
        <section>
            <style>"#test-press-contents { display: contents }"</style>
            <div id="test-press-contents" {..attrs} style=styles>
                <span id="test-press-contents-child" style="display: inline-block; padding: 8px">
                    "Inside display: contents"
                </span>
            </div>
            <div>"Log: " <span id="test-press-contents-log">{log.render()}</span></div>
        </section>
    }
}

/// `should_cancel_on_pointer_exit`: leaving the element cancels the press for good.
#[component]
fn CancelOnExitPress() -> impl IntoView {
    let log = EventLog(RwSignal::new(Vec::new()));
    let UsePressReturn { props, .. } = use_press(UsePressInput {
        should_cancel_on_pointer_exit: Signal::stored(true),
        ..press_input(log, Signal::stored(false))
    });
    let (attrs, styles) = props.into_parts();
    view! {
        <section>
            <div id="test-press-cancel-exit" role="button" tabindex="0" {..attrs} style=styles>
                "Cancels on exit"
            </div>
            <div>"Log: " <span id="test-press-cancel-exit-log">{log.render()}</span></div>
        </section>
    }
}

/// A link with a button role ("should explicitly call click method when Space key is triggered on
/// a link with href and role=button"): Space presses it once and follows it.
#[component]
fn LinkPress() -> impl IntoView {
    let log = EventLog(RwSignal::new(Vec::new()));
    let UsePressReturn { props, .. } = use_press(press_input(log, Signal::stored(false)));
    let (attrs, styles) = props.into_parts();
    view! {
        <section>
            <a id="test-press-link" href="#test-press-link-target" role="button" {..attrs} style=styles>
                "Link with a button role"
            </a>
            <div id="test-press-link-target">"Link target"</div>
            <div>"Log: " <span id="test-press-link-log">{log.render()}</span></div>
        </section>
    }
}

/// `on_double_press` (a leptonic addition), also with press and hover props merged
/// (`#test-press-double-hover`).
#[component]
fn DoublePress() -> impl IntoView {
    let hover_log = EventLog(RwSignal::new(Vec::new()));
    let press = use_press(UsePressInput {
        on_double_press: Some(Callback::new(move |_| hover_log.push("double"))),
        ..UsePressInput::default()
    });
    let hover = use_hover(UseHoverInput::default());
    let (hover_attrs, hover_styles) = press.props.merge_with(hover.props).into_parts();
    let log = EventLog(RwSignal::new(Vec::new()));
    let UsePressReturn { props, .. } = use_press(UsePressInput {
        on_press: Some(Callback::new(move |_| log.push("press"))),
        on_double_press: Some(Callback::new(move |e: PressEvent| {
            log.push(format!("double:{}", e.pointer_type));
        })),
        ..UsePressInput::default()
    });
    let (attrs, styles) = props.into_parts();
    view! {
        <section>
            <div id="test-press-double" role="button" tabindex="0" {..attrs} style=styles>
                "Double press me"
            </div>
            <div>"Log: " <span id="test-press-double-log">{log.render()}</span></div>
            <div id="test-press-double-hover" role="button" tabindex="0" {..hover_attrs} style=hover_styles>
                "Double press me (with hover)"
            </div>
            <div>"Log: " <span id="test-press-double-hover-log">{hover_log.render()}</span></div>
        </section>
    }
}

/// An inner pressable with `PressPropagation::Continue` inside an outer one: both are pressed.
#[component]
fn ContinuingPress() -> impl IntoView {
    let outer_log = EventLog(RwSignal::new(Vec::new()));
    let inner_log = EventLog(RwSignal::new(Vec::new()));
    let outer = use_press(UsePressInput {
        on_press: Some(Callback::new(move |_| outer_log.push("press"))),
        ..UsePressInput::default()
    });
    let inner = use_press(UsePressInput {
        propagation: PressPropagation::Continue,
        on_press: Some(Callback::new(move |_| inner_log.push("press"))),
        ..UsePressInput::default()
    });
    let (outer_attrs, outer_styles) = outer.props.into_parts();
    let (inner_attrs, inner_styles) = inner.props.into_parts();
    view! {
        <section>
            <div role="button" tabindex="0" {..outer_attrs} style=outer_styles>
                "Outer "
                <div id="test-press-continue-inner" role="button" tabindex="0" {..inner_attrs} style=inner_styles>
                    "Inner"
                </div>
            </div>
            <div>"Outer: " <span id="test-press-continue-outer-log">{outer_log.render()}</span></div>
            <div>"Inner: " <span id="test-press-continue-inner-log">{inner_log.render()}</span></div>
        </section>
    }
}

/// A pressable removed 100 ms into its press: its listeners and the disabled text selection go
/// with it, nothing panics.
#[component]
fn RemovedWhilePressed() -> impl IntoView {
    let shown = RwSignal::new(true);
    let log = EventLog(RwSignal::new(Vec::new()));
    view! {
        <section>
            <Show when=move || shown.get()>
                {move || {
                    let mut input = press_input(log, Signal::stored(false));
                    input.on_press_start = Some(Callback::new(move |e: PressEvent| {
                        log.push(format!("start:{}", e.pointer_type));
                        set_timeout(move || shown.set(false), Duration::from_millis(100));
                    }));
                    let UsePressReturn { props, .. } = use_press(input);
                    let (attrs, styles) = props.into_parts();
                    view! {
                        <div id="test-press-removed" role="button" tabindex="0" {..attrs} style=styles>
                            "Removed while pressed"
                        </div>
                    }
                }}
            </Show>
            <div>"Log: " <span id="test-press-removed-log">{log.render()}</span></div>
        </section>
    }
}
