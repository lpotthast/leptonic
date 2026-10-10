use std::time::Duration;

use leptonic::{
    IntoAttrs, Propagation,
    hooks::interactions::{
        PressEvent, PressEventKind, PressPropagation, UseHoverInput, UseKeyboardInput,
        UsePressInput, UsePressReturn, use_hover, use_keyboard, use_press,
    },
};
use leptos::{html, prelude::*, web_sys};
use wasm_bindgen::JsCast;

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
            <DetailPress />
            <TextInputPress />
            <TwoPressLink />
            <ReentrantPress />
            <PlainLinkPress />
            <RoleLinkPress />
            <StyleChangingPress id="test-press-style-background" user_select=false />
            <StyleChangingPress id="test-press-style-user-select" user_select=true />
            <button id="test-press-elsewhere">"Elsewhere"</button>
        </div>
    }
}

/// A plain `<div role="button">` with press handling and a disabled toggle. The clicks that
/// propagate to its parent are counted in `#test-press-parent-clicks`.
#[component]
fn BasicPress() -> impl IntoView {
    let log = EventLog(RwSignal::new(Vec::new()));
    let disabled = RwSignal::new(false);
    let parent_clicks = RwSignal::new(0u32);
    let UsePressReturn { props, is_pressed } = use_press(press_input(log, disabled.into()));
    let (attrs, styles) = props.into_parts();

    view! {
        <section>
            <h2>"Basic"</h2>
            <div on:click=move |_| parent_clicks.update(|c| *c += 1)>
                <div id="test-press-target" role="button" tabindex="0" {..attrs} style=styles>
                    <span id="test-press-label">"Press me"</span>
                </div>
            </div>
            <div>"Parent clicks: " <span id="test-press-parent-clicks">{parent_clicks}</span></div>
            <button id="test-press-toggle-disabled" on:click=move |_| disabled.update(|d| *d = !*d)>
                "Toggle disabled"
            </button>
            <div>
                "Pressed: "
                <span id="test-press-is-pressed">{move || is_pressed.get().to_string()}</span>
            </div>
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
                <div
                    id=format!("{id}-inner")
                    role="button"
                    tabindex="0"
                    {..inner_attrs}
                    style=inner_styles
                >
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
            <button id="test-press-keep" {..attrs} style=styles>
                "Press without focus"
            </button>
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
            <div>
                "Pressed: "
                <span id="test-press-keyup-pressed">{move || is_pressed.get().to_string()}</span>
            </div>
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
            <button id="test-press-focus-move-other" node_ref=other>
                "Other"
            </button>
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
            <a
                id="test-press-link"
                href="#test-press-link-target"
                role="button"
                {..attrs}
                style=styles
            >
                "Link with a button role"
            </a>
            <div id="test-press-link-target">"Link target"</div>
            <div>"Log: " <span id="test-press-link-log">{log.render()}</span></div>
        </section>
    }
}

/// `on_double_press` (a leptonic addition), also with press and hover props spread onto one
/// element (`#test-press-double-hover`).
#[component]
fn DoublePress() -> impl IntoView {
    let hover_log = EventLog(RwSignal::new(Vec::new()));
    let press = use_press(UsePressInput {
        on_double_press: Some(Callback::new(move |_| hover_log.push("double"))),
        ..UsePressInput::default()
    });
    let hover = use_hover(UseHoverInput::default());
    let (double_attrs, double_styles) = press.props.into_parts();
    let hover_attrs = hover.props.into_attrs();
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
            <div
                id="test-press-double-hover"
                role="button"
                tabindex="0"
                {..double_attrs}
                {..hover_attrs}
                style=double_styles
            >
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
                <div
                    id="test-press-continue-inner"
                    role="button"
                    tabindex="0"
                    {..inner_attrs}
                    style=inner_styles
                >
                    "Inner"
                </div>
            </div>
            <div>
                "Outer: " <span id="test-press-continue-outer-log">{outer_log.render()}</span>
            </div>
            <div>
                "Inner: " <span id="test-press-continue-inner-log">{inner_log.render()}</span>
            </div>
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
            <Show when=move || {
                shown.get()
            }>
                {move || {
                    let mut input = press_input(log, Signal::stored(false));
                    input.on_press_start = Some(
                        Callback::new(move |e: PressEvent| {
                            log.push(format!("start:{}", e.pointer_type));
                            set_timeout(move || shown.set(false), Duration::from_millis(100));
                        }),
                    );
                    let UsePressReturn { props, .. } = use_press(input);
                    let (attrs, styles) = props.into_parts();
                    view! {
                        <div
                            id="test-press-removed"
                            role="button"
                            tabindex="0"
                            {..attrs}
                            style=styles
                        >
                            "Removed while pressed"
                        </div>
                    }
                }}
            </Show>
            <div>"Log: " <span id="test-press-removed-log">{log.render()}</span></div>
        </section>
    }
}

/// A 100×50 pressable logging every press event as `kind:pointer type:modifiers@x,y` (modifiers
/// `shift`, `ctrl`, `meta`, `alt` joined by `+`, `-` for none; the position relative to the
/// target, rounded) to `#test-press-detail-log`: modifier keys and coordinates.
#[component]
fn DetailPress() -> impl IntoView {
    let log = EventLog(RwSignal::new(Vec::new()));
    let entry = Some(Callback::new(move |e: PressEvent| {
        let kind = match e.kind {
            PressEventKind::PressStart => "start",
            PressEventKind::PressEnd => "end",
            PressEventKind::PressUp => "up",
            PressEventKind::Press => "press",
            PressEventKind::DoublePress => "double",
        };
        let modifiers = [
            (e.modifiers.shift_key, "shift"),
            (e.modifiers.ctrl_key, "ctrl"),
            (e.modifiers.meta_key, "meta"),
            (e.modifiers.alt_key, "alt"),
        ]
        .into_iter()
        .filter_map(|(held, name)| held.then_some(name))
        .collect::<Vec<_>>()
        .join("+");
        let modifiers = if modifiers.is_empty() {
            "-".to_owned()
        } else {
            modifiers
        };
        log.push(format!(
            "{kind}:{}:{modifiers}@{},{}",
            e.pointer_type,
            e.point.x.round(),
            e.point.y.round()
        ));
    }));
    let UsePressReturn { props, .. } = use_press(UsePressInput {
        on_press: entry,
        on_press_up: entry,
        on_press_start: entry,
        on_press_end: entry,
        ..UsePressInput::default()
    });
    let (attrs, styles) = props.into_parts();
    view! {
        <section>
            <style>
                "#test-press-detail { width: 100px; height: 50px; box-sizing: border-box; border: 1px solid }"
            </style>
            <div id="test-press-detail" role="button" tabindex="0" {..attrs} style=styles>
                "Details"
            </div>
            <div>"Log: " <span id="test-press-detail-log">{log.render()}</span></div>
        </section>
    }
}

/// A pressable text input (`<input>` without `type`): typing Space or Enter types, it presses
/// nothing.
#[component]
fn TextInputPress() -> impl IntoView {
    let log = EventLog(RwSignal::new(Vec::new()));
    let UsePressReturn { props, .. } = use_press(press_input(log, Signal::stored(false)));
    let (attrs, styles) = props.into_parts();
    view! {
        <section>
            <input id="test-press-text-input" aria-label="Text input" {..attrs} style=styles />
            <div>"Log: " <span id="test-press-text-input-log">{log.render()}</span></div>
        </section>
    }
}

/// A link with a button role and two press hooks (react-aria's `LINK_CLICKED` case): Space opens
/// it once. The clicks reaching the link are counted in `#test-press-two-link-clicks` (and
/// prevented, so the page stays).
#[component]
fn TwoPressLink() -> impl IntoView {
    let clicks = RwSignal::new(0);
    let first = use_press(UsePressInput::default());
    let second = use_press(UsePressInput::default());
    let (first_attrs, styles) = first.props.into_parts();
    let (second_attrs, _) = second.props.into_parts();
    view! {
        <section>
            <a
                id="test-press-two-link"
                href="#test-press-two-link-target"
                role="button"
                on:click=move |e: leptos::ev::MouseEvent| {
                    e.prevent_default();
                    clicks.update(|c| *c += 1);
                }
                {..first_attrs}
                {..second_attrs}
                style=styles
            >
                "Link with two press hooks"
            </a>
            <div>"Clicks: " <span id="test-press-two-link-clicks">{clicks}</span></div>
        </section>
    }
}

/// A pressable whose press up clicks it (react-aria's "should ignore synthetic events fired during
/// an onPressUp event"): the click it causes presses nothing.
#[component]
fn ReentrantPress() -> impl IntoView {
    let log = EventLog(RwSignal::new(Vec::new()));
    let mut input = press_input(log, Signal::stored(false));
    input.on_press_up = Some(Callback::new(move |e: PressEvent| {
        log.push(format!("up:{}", e.pointer_type));
        if let Some(target) = e.target.dyn_ref::<web_sys::HtmlElement>() {
            target.click();
        }
    }));
    let UsePressReturn { props, .. } = use_press(input);
    let (attrs, styles) = props.into_parts();
    view! {
        <section>
            <div id="test-press-reentrant" role="button" tabindex="0" {..attrs} style=styles>
                "Clicks itself on press up"
            </div>
            <div>"Log: " <span id="test-press-reentrant-log">{log.render()}</span></div>
        </section>
    }
}

/// A pressable whose press start changes its inline style meanwhile (as a re-render would): a
/// red background, and with `user_select` `user-select: text` too (react-aria's `TestStyleChange`).
#[component]
fn StyleChangingPress(id: &'static str, user_select: bool) -> impl IntoView {
    let UsePressReturn { props, .. } = use_press(UsePressInput {
        on_press_start: Some(Callback::new(move |e: PressEvent| {
            if let Some(target) = e.target.dyn_ref::<web_sys::HtmlElement>() {
                let style = target.style();
                let _ = style.set_property("background-color", "red");
                if user_select {
                    let _ = style.set_property("user-select", "text");
                }
            }
        })),
        ..UsePressInput::default()
    });
    let (attrs, styles) = props.into_parts();
    view! {
        <div id=id role="button" tabindex="0" {..attrs} style=styles>
            "Changes its style on press start"
        </div>
    }
}

/// A pressable `<a href>`: only Enter presses it. Its clicks are counted in
/// `#test-press-plain-link-clicks` (and prevented, so the page stays).
#[component]
fn PlainLinkPress() -> impl IntoView {
    let log = EventLog(RwSignal::new(Vec::new()));
    let clicks = RwSignal::new(0);
    let UsePressReturn { props, .. } = use_press(press_input(log, Signal::stored(false)));
    let (attrs, styles) = props.into_parts();
    view! {
        <section>
            <a
                id="test-press-plain-link"
                href="#test-press-plain-link-target"
                on:click=move |e: leptos::ev::MouseEvent| {
                    e.prevent_default();
                    clicks.update(|c| *c += 1);
                }
                {..attrs}
                style=styles
            >
                "Plain link"
            </a>
            <div>"Clicks: " <span id="test-press-plain-link-clicks">{clicks}</span></div>
            <div>"Log: " <span id="test-press-plain-link-log">{log.render()}</span></div>
        </section>
    }
}

/// A pressable element with the link role (no `href`): only Enter presses it.
#[component]
fn RoleLinkPress() -> impl IntoView {
    let log = EventLog(RwSignal::new(Vec::new()));
    let UsePressReturn { props, .. } = use_press(press_input(log, Signal::stored(false)));
    let (attrs, styles) = props.into_parts();
    view! {
        <section>
            <div id="test-press-role-link" role="link" tabindex="0" {..attrs} style=styles>
                "Role link"
            </div>
            <div>"Log: " <span id="test-press-role-link-log">{log.render()}</span></div>
        </section>
    }
}
