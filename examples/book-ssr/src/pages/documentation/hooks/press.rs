use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    press_basic::PressBasicDemo, press_cancel::PressCancelDemo, press_long::PressLongDemo,
    press_no_focus::PressNoFocusDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageUsePress() -> impl IntoView {
    view! {
        <DocPage title="use_press">
            <p>
                "The "<Code inline=true>"use_press"</Code>" hook handles press interactions consistently across mouse, touch, "
                "keyboard and screen readers. It reports when a press starts, ends and completes, which pointer type caused it, "
                "and whether the element is currently pressed. See the "
                <Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>" for domain guidance."
            </p>

            <ReactAria hook="usePress"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UsePressInput"</Code>" implements "<Code inline=true>"Default"</Code>": everything is off and "
                    <Code inline=true>"on_press"</Code>" does nothing, so you only name the options and callbacks you need."
                </p>

                <ApiTable kind=ApiKind::Input of="UsePressInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Whether the element is disabled. A disabled element ignores all press interactions."
                    </ApiRow>
                    <ApiRow name="on_press" ty="Callback<PressEvent>" default="no-op">
                        "Called when a press completes: the pointer or key is released over the element."
                    </ApiRow>
                    <ApiRow name="on_press_start" ty="Option<Callback<PressEvent>>" default="None">
                        "Called when a press starts."
                    </ApiRow>
                    <ApiRow name="on_press_end" ty="Option<Callback<PressEvent>>" default="None">
                        "Called when a press ends, whether it completed or was cancelled."
                    </ApiRow>
                    <ApiRow name="on_press_up" ty="Option<Callback<PressEvent>>" default="None">
                        "Called when the pointer or key is released over the element, even if the press didn\u{2019}t start "
                        "on it."
                    </ApiRow>
                    <ApiRow name="on_press_change" ty="Option<Callback<bool>>" default="None">
                        "Called with "<Code inline=true>"true"</Code>" when a press starts and "<Code inline=true>"false"</Code>
                        " when it ends."
                    </ApiRow>
                    <ApiRow name="on_double_press" ty="Option<Callback<PressEvent>>" default="None">
                        "Called when the element receives a native "<Code inline=true>"dblclick"</Code>" event."
                    </ApiRow>
                    <ApiRow name="on_long_press_start, on_long_press, on_long_press_end" ty="Option<Callback<LongPressEvent>>" default="None">
                        "Long press callbacks. Setting any of them enables long press detection, see "
                        <a href="#long-press">"Long press"</a>"."
                    </ApiRow>
                    <ApiRow name="long_press_threshold" ty="Option<Signal<Duration>>" default="None (500 ms)">
                        "How long the element has to be held before "<Code inline=true>"on_long_press"</Code>" fires."
                    </ApiRow>
                    <ApiRow name="long_press_accessibility_description" ty="Option<Oco<'static, str>>" default="None">
                        "Describes the long press action to assistive technology, e.g. \u{201c}Long press to open menu\u{201d}. "
                        "Only applied when "<Code inline=true>"on_long_press"</Code>" is set."
                    </ApiRow>
                    <ApiRow name="long_press_disabled" ty="Signal<bool>" default="false">
                        "Turns long press detection off while "<Code inline=true>"true"</Code>
                        ", so presses aren\u{2019}t cancelled after the threshold."
                    </ApiRow>
                    <ApiRow name="prevent_focus_on_press" ty="bool" default="false">
                        "Don\u{2019}t move focus to the element when it is pressed, e.g. for toolbar buttons next to a text editor."
                    </ApiRow>
                    <ApiRow name="should_cancel_on_pointer_exit" ty="bool" default="false">
                        "Cancel the press when the pointer leaves the element. By default, you can drag out and back in and "
                        "still complete the press."
                    </ApiRow>
                    <ApiRow name="allow_text_selection_on_press" ty="bool" default="false">
                        "Allow selecting text inside the element during a press. By default, text selection is disabled "
                        "while pressing."
                    </ApiRow>
                    <ApiRow name="force_prevent_default" ty="bool" default="false">
                        "Call "<Code inline=true>"prevent_default()"</Code>" on all pointer and keyboard events, so no "
                        "browser-specific behavior happens on interaction."
                    </ApiRow>
                    <ApiRow name="force_propagation" ty="bool" default="false">
                        "Always let events propagate. By default, propagation is stopped unless a callback calls "
                        <Code inline=true>"continue_propagation()"</Code>" on its event."
                    </ApiRow>
                    <ApiRow name="force_is_pressed" ty="Option<Signal<bool>>" default="None">
                        "Forces the pressed state: "<Code inline=true>"is_pressed"</Code>" is "<Code inline=true>"true"</Code>
                        " while this signal is. Parent components use it to keep a trigger looking pressed."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UsePressReturn">
                    <ApiRow name="props" ty="PropsWithStyles<UsePressProps>">
                        "Event handlers and styles for the pressable element. Call "<Code inline=true>"props.into_parts()"</Code>
                        " to get "<Code inline=true>"(attrs, styles)"</Code>", then spread "<Code inline=true>"{..attrs}"</Code>
                        " and set "<Code inline=true>"style=styles"</Code>"."
                    </ApiRow>
                    <ApiRow name="is_pressed" ty="Signal<bool>">"Whether the element is currently pressed."</ApiRow>
                </ApiTable>

                <Section title="PressEvent">
                    <p>"Every press callback receives a "<Code inline=true>"PressEvent"</Code>":"</p>

                    <ApiTable kind=ApiKind::Fields of="PressEvent">
                        <ApiRow name="pointer_type" ty="PointerType">
                            <Code inline=true>"Mouse"</Code>", "<Code inline=true>"Pen"</Code>", "<Code inline=true>"Touch"</Code>", "
                            <Code inline=true>"Keyboard"</Code>", "<Code inline=true>"Virtual"</Code>" (screen readers and "
                            "programmatic clicks) or "<Code inline=true>"Other"</Code>"."
                        </ApiRow>
                        <ApiRow name="target" ty="SendWrapper<EventTarget>">"The pressed element."</ApiRow>
                        <ApiRow name="modifiers" ty="Modifiers">"The modifier keys held during the event."</ApiRow>
                        <ApiRow name="x, y" ty="Option<f64>">
                            "Pointer position relative to the element. "<Code inline=true>"None"</Code>" for keyboard presses."
                        </ApiRow>
                        <ApiRow name="key" ty="Option<KeyboardKey>">
                            "The key that triggered a keyboard press, so you can tell Enter and Space apart. "
                            <Code inline=true>"None"</Code>" for pointer presses."
                        </ApiRow>
                    </ApiTable>

                    <p>
                        "Call "<Code inline=true>"continue_propagation()"</Code>" on the event to let it bubble to parent handlers."
                    </p>
                </Section>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let UsePressReturn { props, is_pressed } = use_press(UsePressInput {
                            is_disabled: disabled.into(),
                            on_press: Callback::new(move |e: PressEvent| log!("pressed with {:?}", e.pointer_type)),
                            ..Default::default()
                        });
                        let (attrs, styles) = props.into_parts();

                        view! {
                            <button {..attrs} style=styles>"Press me"</button>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "The event log shows every callback with the pointer type that caused it. Tab to the button and press "
                    "Enter or Space to see keyboard presses."
                </p>

                <Demo description="Press, press start/end/up and double press events with an event log" source=include_str!("demos/press_basic.rs")>
                    <PressBasicDemo/>
                </Demo>

                <p>
                    "Besides "<Code inline=true>"is_pressed"</Code>", the hook normalizes a lot of browser behavior for you: "
                    "presses that leave the element and come back still complete, screen reader clicks are detected as "
                    <Code inline=true>"Virtual"</Code>" presses, text selection is suppressed while pressing, and browser "
                    "quirks (Safari not cancelling presses on drag, iOS pointer capture, stuck keys after Meta shortcuts on "
                    "macOS) are handled."
                </p>
            </Section>

            <Section title="Options">
                <p>
                    "Pressing the button below and dragging the pointer out cancels the press, because "
                    <Code inline=true>"should_cancel_on_pointer_exit"</Code>" is set."
                </p>

                <Demo description="A press that is cancelled when the pointer leaves the button" source=include_str!("demos/press_cancel.rs")>
                    <PressCancelDemo/>
                </Demo>

                <p>"With "<Code inline=true>"prevent_focus_on_press"</Code>", the button keeps focus where it was."</p>

                <Demo description="A button that does not take focus when pressed" source=include_str!("demos/press_no_focus.rs")>
                    <PressNoFocusDemo/>
                </Demo>
            </Section>

            <Section title="Long press">
                <p>
                    "Long press detection is part of "<Code inline=true>"use_press"</Code>". Set any of "
                    <Code inline=true>"on_long_press_start"</Code>", "<Code inline=true>"on_long_press"</Code>" or "
                    <Code inline=true>"on_long_press_end"</Code>", and the hook starts a timer when a mouse or touch press "
                    "starts. Keyboard presses never become long presses."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        let (threshold, set_threshold) = signal(Duration::from_millis(500));

                        let UsePressReturn { props, .. } = use_press(UsePressInput {
                            on_long_press: Some(Callback::new(|e: LongPressEvent| { /* ... */ })),
                            long_press_threshold: Some(threshold.into()),
                            long_press_accessibility_description: Some("Long press to open menu".into()),
                            ..Default::default()
                        });
                    "#)}
                </Code>

                <Demo description="Long press with an adjustable threshold and an event log" source=include_str!("demos/press_long.rs")>
                    <PressLongDemo/>
                </Demo>

                <p>"When the press is held for the threshold (500 ms unless you set one), the following happens:"</p>

                <ol>
                    <li>
                        "A synthetic "<Code inline=true>"pointercancel"</Code>" event is dispatched on the element, which "
                        "cancels the ongoing press."
                    </li>
                    <li><Code inline=true>"on_long_press_end"</Code>" fires because of the cancellation."</li>
                    <li>"The element is focused without scrolling."</li>
                    <li><Code inline=true>"on_long_press"</Code>" fires."</li>
                </ol>

                <p>
                    "If you release before the threshold, the timer is cancelled and only "<Code inline=true>"on_long_press_start"</Code>
                    " and "<Code inline=true>"on_long_press_end"</Code>" fire. Because a long press cancels the press, "
                    <Code inline=true>"on_press"</Code>" doesn\u{2019}t fire for it. On touch devices, the native context menu "
                    "is suppressed during the interaction. The "<Code inline=true>"long_press_accessibility_description"</Code>
                    " is linked to the element through "<Code inline=true>"aria-describedby"</Code>"."
                </p>

                <p>
                    <Code inline=true>"LongPressEvent"</Code>" has the fields "<Code inline=true>"event_type"</Code>" ("
                    <Code inline=true>"LongPressStart"</Code>", "<Code inline=true>"LongPress"</Code>" or "
                    <Code inline=true>"LongPressEnd"</Code>"), "<Code inline=true>"pointer_type"</Code>", "
                    <Code inline=true>"target"</Code>", "<Code inline=true>"modifiers"</Code>", "<Code inline=true>"x"</Code>
                    " and "<Code inline=true>"y"</Code>"."
                </p>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Tab">"Focus the pressable element (if it is focusable)."</KeyRow>
                    <KeyRow keys="Enter / Space">
                        "Press the element. The "<Code inline=true>"PressEvent"</Code>" has the pointer type "
                        <Code inline=true>"Keyboard"</Code>" and the key in "<Code inline=true>"key"</Code>"."
                    </KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=routes::doc::interactions::PressResponder.materialize()>"PressResponder"</Link></li>
                <li><Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link></li>
                <li><Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link></li>
                <li><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" (composes use_press)"</li>
            </SeeAlso>
        </DocPage>
    }
}
