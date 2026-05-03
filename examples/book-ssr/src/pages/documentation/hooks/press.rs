use indoc::indoc;
use leptonic::{components::prelude::*, utils::key::Key};
use leptos::prelude::*;

use super::demos::{
    press_basic::PressBasicDemo, press_cancel::PressCancelDemo, press_long::PressLongDemo,
    press_no_focus::PressNoFocusDemo,
};
use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

#[component]
pub fn PageUsePress() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_press" class="anchor">
                "use_press"
                <AnchorLink href="#use_press" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_press"</Code>" hook normalizes press interactions across mouse, touch, keyboard, and screen readers. "
                "Provides consistent press event handling with support for press start, end, up, "
                "and completion events, along with pointer type detection. "
                "See the "<Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link>" for domain guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/usePress.html" target=LinkTarget::_Blank>
                    "usePress"
                </LinkExt>
                "."
            </p>

            <h2 id="example" class="anchor">
                "Example"
                <AnchorLink href="#example" description="Direct link to example"/>
            </h2>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let UsePressReturn { props, is_pressed } = use_press(UsePressInput {
                        disabled: disabled.into(),
                        force_prevent_default: false,
                        force_propagation: false,
                        allow_text_selection_on_press: false,
                        should_cancel_on_pointer_exit: false,
                        prevent_focus_on_press: false,
                        force_is_pressed: None,
                        on_press: Callback::new(move |e| { /* ... */ }),
                        on_press_up: None,
                        on_press_start: None,
                        on_press_end: None,
                        on_press_change: None,
                    });

                    view! {
                        <button {..props.into_attrs()}>
                            "Press me"
                        </button>
                    }
                "#)}
            </Code>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <DemoShell source=include_str!("demos/press_basic.rs")>
                <PressBasicDemo />
            </DemoShell>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Normalizes press events across mouse, touch, keyboard, and screen readers"</li>
                <li>"Handles press start/end/up lifecycle with pointer type detection"</li>
                <li>"Supports drag-out and drag-back-in behavior during press"</li>
                <li>"Virtual click detection for screen reader accessibility"</li>
                <li>"Text selection prevention during press interactions"</li>
                <li>"Safari drag cancellation workaround"</li>
                <li>"iOS pointer capture release for correct touch events"</li>
                <li>"Double-press detection via native dblclick"</li>
                <li>"macOS Meta key workaround for stuck key states"</li>
            </ul>

            <h2 id="keyboard" class="anchor">
                "Keyboard Interaction"
                <AnchorLink href="#keyboard" description="Direct link to keyboard interaction"/>
            </h2>

            <p>
                "Press events from keyboard activation include a "
                <code>"key"</code> " field on the " <code>"PressEvent"</code> " so consumers "
                "can distinguish which key was used."
            </p>

            <ul>
                <li><KbdKey key=Key::Tab/> " — Focus the pressable element"</li>
                <li><KbdKey key=Key::Enter/> " — Activate the press"</li>
                <li><KbdKey key=Key::Space/> " — Activate the press"</li>
            </ul>

            <p>
                "Tab to the demo button above, then try pressing Enter vs Space "
                "— the event log shows the pointer type as " <code>"Keyboard"</code>
                " and double-press also works with keyboard activation."
            </p>

            <h2 id="options" class="anchor">
                "Configuration Options"
                <AnchorLink href="#options" description="Direct link to configuration options"/>
            </h2>

            <ul>
                <li><code>"force_prevent_default"</code> " — When true, calls " <code>"prevent_default()"</code> " on all pointer and keyboard events. Useful for elements that should not exhibit default browser behavior (e.g., preventing form submission on Enter)."</li>
                <li><code>"force_propagation"</code> " — When true, unconditionally allows event propagation, bypassing per-event " <code>"continue_propagation()"</code> " control. By default (false), propagation is stopped unless a callback explicitly calls " <code>"continue_propagation()"</code> "."</li>
                <li><code>"prevent_focus_on_press"</code> " — When true, prevents the pressed element from receiving focus. Useful for toolbar buttons adjacent to text editors."</li>
                <li><code>"force_is_pressed"</code> " — An optional external signal that, when true, forces the pressed visual state regardless of actual press interactions."</li>
                <li><code>"allow_text_selection_on_press"</code> " — When true, allows text within the pressable element to be selected during press."</li>
                <li><code>"should_cancel_on_pointer_exit"</code> " — When true, cancels the press when the pointer leaves the element boundary."</li>
            </ul>

            <h3 id="options-demo" class="anchor">
                "Configuration Options Demo"
                <AnchorLink href="#options-demo" description="Direct link to configuration options demo"/>
            </h3>

            <DemoShell source=include_str!("demos/press_cancel.rs")>
                <PressCancelDemo />
            </DemoShell>

            <DemoShell source=include_str!("demos/press_no_focus.rs")>
                <PressNoFocusDemo />
            </DemoShell>

            <h2 id="long-press" class="anchor">
                "Long Press"
                <AnchorLink href="#long-press" description="Direct link to long press"/>
            </h2>

            <p>
                "Long press detection is built into " <code>"use_press"</code> " via the optional "
                <code>"on_long_press"</code> ", " <code>"on_long_press_start"</code> ", and "
                <code>"on_long_press_end"</code> " fields. When any of these callbacks are set, "
                <code>"use_press"</code> " will start a timer on mouse/touch press start and "
                "fire the long press callback when the threshold is met."
            </p>

            <h3 id="long-press-example" class="anchor">
                "Long Press Example"
                <AnchorLink href="#long-press-example" description="Direct link to long press example"/>
            </h3>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let (threshold, set_threshold) = signal(Duration::from_millis(500));

                    let UsePressReturn { props, .. } = use_press(UsePressInput {
                        disabled: Signal::derive(|| false),
                        force_prevent_default: false,
                        force_propagation: false,
                        allow_text_selection_on_press: false,
                        should_cancel_on_pointer_exit: false,
                        prevent_focus_on_press: false,
                        force_is_pressed: None,
                        on_press: Callback::new(|_| {}),
                        on_press_up: None,
                        on_press_start: None,
                        on_press_end: None,
                        on_press_change: None,
                        on_double_press: None,
                        on_long_press_start: Some(Callback::new(|e| { /* ... */ })),
                        on_long_press: Some(Callback::new(|e| { /* ... */ })),
                        on_long_press_end: Some(Callback::new(|e| { /* ... */ })),
                        long_press_threshold: Some(threshold.into()), // reactive Signal<Duration>
                        long_press_accessibility_description: Some("Long press to open menu"),
                    });

                    view! {
                        <button {..props.into_attrs()}>
                            "Long press me"
                        </button>
                    }
                "#)}
            </Code>

            <h3 id="long-press-demo" class="anchor">
                "Long Press Demo"
                <AnchorLink href="#long-press-demo" description="Direct link to long press demo"/>
            </h3>

            <DemoShell source=include_str!("demos/press_long.rs")>
                <PressLongDemo />
            </DemoShell>

            <h3 id="long-press-features" class="anchor">
                "Long Press Features"
                <AnchorLink href="#long-press-features" description="Direct link to long press features"/>
            </h3>

            <ul>
                <li>"Customizable time threshold (default 500ms), reactive via " <code>"Signal<u64>"</code></li>
                <li>"Handles both mouse and touch events"</li>
                <li>"Prevents context menu on touch devices during long press"</li>
                <li>"Three event callbacks: start, threshold met, and end"</li>
                <li>"Accessibility via " <code>"aria-describedby"</code></li>
                <li>"Automatic focus management when threshold is met"</li>
                <li>"Cancels the press via synthetic " <code>"pointercancel"</code> " — " <code>"on_press"</code> " will not fire for long presses"</li>
            </ul>

            <h3 id="long-press-how-it-works" class="anchor">
                "How Long Press Works"
                <AnchorLink href="#long-press-how-it-works" description="Direct link to how long press works"/>
            </h3>

            <p>
                "When the user presses down on the target element, a timer starts. If the user "
                "holds the press for the configured threshold duration (default 500ms), the following "
                "sequence occurs:"
            </p>

            <ol>
                <li>"A synthetic " <code>"pointercancel"</code> " event is dispatched on the target, "
                    "which cancels the ongoing press interaction."</li>
                <li>"The " <code>"on_long_press_end"</code> " callback fires (triggered by the cancellation)."</li>
                <li>"The target element is focused without scrolling."</li>
                <li>"The " <code>"on_long_press"</code> " callback fires."</li>
            </ol>

            <p>
                "If the user releases before the threshold, the timer is cancelled and only "
                <code>"on_long_press_start"</code> " and " <code>"on_long_press_end"</code> " fire. "
                "On touch devices, the native context menu is automatically prevented during the interaction. "
                "Because the long press dispatches " <code>"pointercancel"</code> ", the "
                <code>"on_press"</code> " callback is suppressed for interactions that became long presses."
            </p>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UseHover.materialize()>"use_hover"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link></li>
                <li><Link href=crate::routes::doc::button::Hook.materialize()>"use_button"</Link>" (composes use_press)"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_press", link: "#use_press" },
                Toc::Leaf { title: "Example", link: "#example" },
                Toc::Leaf { title: "Interactive Demo", link: "#demo" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "Keyboard Interaction", link: "#keyboard" },
                Toc::Leaf { title: "Configuration Options", link: "#options" },
                Toc::Leaf { title: "Configuration Options Demo", link: "#options-demo" },
                Toc::Leaf { title: "Long Press", link: "#long-press" },
                Toc::Leaf { title: "Long Press Example", link: "#long-press-example" },
                Toc::Leaf { title: "Long Press Demo", link: "#long-press-demo" },
                Toc::Leaf { title: "Long Press Features", link: "#long-press-features" },
                Toc::Leaf { title: "How Long Press Works", link: "#long-press-how-it-works" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
