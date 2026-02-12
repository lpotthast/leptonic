use indoc::indoc;
use std::time::Duration;

use leptonic::atoms::slider::{
    Slider as SliderAtom, SliderOutput, SliderThumb, SliderTrack, SliderTrackFill,
};
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::utils::key::Key;
use leptonic::utils::styles::Style::*;
use leptonic::utils::styles::Styles;
use leptos::prelude::*;
use ringbuf::traits::{Consumer, Observer, RingBuffer};
use ringbuf::HeapRb;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

fn track_style() -> Styles {
    Styles::from([
        (Display, "flex"),
        (Width, "200px"),
        (Height, "8px"),
        (Background, "#ddd"),
        (BorderRadius, "4px"),
        (Position, "relative"),
        (Cursor, "pointer"),
    ])
}

fn thumb_style() -> Styles {
    Styles::builder()
        .with((Width, "20px"))
        .with((Height, "20px"))
        .with((BackgroundColor, "var(--brand-color)"))
        .with((BorderRadius, "50%"))
        .with((Border, "2px solid white"))
        .with((BoxShadow, "0 2px 4px rgba(0,0,0,0.2)"))
        .with((Cursor, "grab"))
        .build()
}

#[component]
pub fn PageUsePress() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (dbl_count, set_dbl_count) = signal(0);
    let (events, set_events) = signal(HeapRb::<Oco<'static, str>>::new(50));
    let (disabled, set_disabled) = signal(false);
    let (press_state, set_press_state) = signal(false);

    let string = Memo::new(move |_| {
        events.with(|events| {
            let mut result = String::new();
            for e in events.iter().rev() {
                result.push_str(e.as_str());
                result.push('\n');
            }
            result
        })
    });

    let UsePressReturn { props, is_pressed } = use_press(UsePressInput {
        disabled: disabled.into(),
        force_prevent_default: false,
        force_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        prevent_focus_on_press: false,
        force_is_pressed: None,
        on_press: Callback::new(move |e: PressEvent| {
            set_count.update(|c| *c += 1);
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "Press: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                )));
            });
        }),
        on_press_up: Some(Callback::new(move |e: PressEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "PressUp: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                )));
            });
        })),
        on_press_start: Some(Callback::new(move |e: PressEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "PressStart: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                )));
            });
        })),
        on_press_end: Some(Callback::new(move |e: PressEvent| {
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "PressEnd: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                )));
            });
        })),
        on_press_change: Some(Callback::new(move |pressed: bool| {
            set_press_state.set(pressed);
        })),
        on_double_press: Some(Callback::new(move |e: PressEvent| {
            set_dbl_count.update(|c| *c += 1);
            set_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "DoublePress: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                )));
            });
        })),
        on_long_press_start: None,
        on_long_press: None,
        on_long_press_end: None,
        long_press_threshold: None,
        long_press_accessibility_description: None,
    });

    // Long press demo state
    let (lp_count, set_lp_count) = signal(0);
    let (lp_events, set_lp_events) = signal(HeapRb::<Oco<'static, str>>::new(20));
    let (lp_disabled, set_lp_disabled) = signal(false);
    let (threshold, set_threshold) = signal(Duration::from_millis(500));

    let lp_string = Memo::new(move |_| {
        lp_events.with(|events| {
            let mut result = String::new();
            for e in events.iter().rev() {
                result.push_str(e.as_str());
                result.push('\n');
            }
            result
        })
    });

    let UsePressReturn {
        props: lp_props,
        is_pressed: _,
    } = use_press(UsePressInput {
        disabled: lp_disabled.into(),
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
        on_long_press_start: Some(Callback::new(move |e: LongPressEvent| {
            set_lp_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "LongPressStart: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                )));
            });
        })),
        on_long_press: Some(Callback::new(move |e: LongPressEvent| {
            set_lp_count.update(|c| *c += 1);
            set_lp_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "LongPress: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                )));
            });
        })),
        on_long_press_end: Some(Callback::new(move |e: LongPressEvent| {
            set_lp_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "LongPressEnd: pointer_type={:?}, x={:?}, y={:?}",
                    e.pointer_type, e.x, e.y,
                )));
            });
        })),
        long_press_threshold: Some(threshold.into()),
        long_press_accessibility_description: Some("Long press to increment counter".into()),
    });

    // Configuration options demo: cancel_on_pointer_exit button
    let (cancel_events, set_cancel_events) = signal(HeapRb::<Oco<'static, str>>::new(10));
    let cancel_string = Memo::new(move |_| {
        cancel_events.with(|events| {
            let mut result = String::new();
            for e in events.iter().rev() {
                result.push_str(e.as_str());
                result.push('\n');
            }
            result
        })
    });
    let UsePressReturn {
        props: cancel_props,
        is_pressed: cancel_is_pressed,
    } = use_press(UsePressInput {
        disabled: Signal::derive(|| false),
        force_prevent_default: false,
        force_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: true,
        prevent_focus_on_press: false,
        force_is_pressed: None,
        on_press: Callback::new(move |_: PressEvent| {
            set_cancel_events.update(|events| {
                events.push_overwrite(Oco::Borrowed("Press completed"));
            });
        }),
        on_press_up: None,
        on_press_start: Some(Callback::new(move |_: PressEvent| {
            set_cancel_events.update(|events| {
                events.push_overwrite(Oco::Borrowed("PressStart"));
            });
        })),
        on_press_end: Some(Callback::new(move |e: PressEvent| {
            set_cancel_events.update(|events| {
                events.push_overwrite(Oco::Owned(format!(
                    "PressEnd: pointer_type={:?}",
                    e.pointer_type,
                )));
            });
        })),
        on_press_change: None,
        on_double_press: None,
        on_long_press_start: None,
        on_long_press: None,
        on_long_press_end: None,
        long_press_threshold: None,
        long_press_accessibility_description: None,
    });

    // Configuration options demo: prevent_focus_on_press button
    let (focus_demo_focused, set_focus_demo_focused) = signal(false);
    let UsePressReturn {
        props: no_focus_props,
        is_pressed: no_focus_is_pressed,
    } = use_press(UsePressInput {
        disabled: Signal::derive(|| false),
        force_prevent_default: false,
        force_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        prevent_focus_on_press: true,
        force_is_pressed: None,
        on_press: Callback::new(|_| {}),
        on_press_up: None,
        on_press_start: None,
        on_press_end: None,
        on_press_change: None,
        on_double_press: None,
        on_long_press_start: None,
        on_long_press: None,
        on_long_press_end: None,
        long_press_threshold: None,
        long_press_accessibility_description: None,
    });

    view! {
        <Article>
            <h1 id="use_press" class="anchor">
                "use_press"
                <AnchorLink href="#use_press" description="Direct link to article header"/>
            </h1>

            <p>
                "Normalizes press interactions across mouse, touch, keyboard, and screen readers. "
                "Provides consistent press event handling with support for press start, end, up, "
                "and completion events, along with pointer type detection."
            </p>

            <h2 id="example" class="anchor">
                "Example"
                <AnchorLink href="#example" description="Direct link to example"/>
            </h2>

            <Code>
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

            <p>"Try interacting with the button below using mouse, touch, or keyboard (Tab to focus, Enter/Space to press)."</p>

            <button
                {..props.into_attrs()}
                style:background=move || if is_pressed.get() { "var(--brand-color)" } else { "" }
                style:color=move || if is_pressed.get() { "white" } else { "" }
                style:transform=move || if is_pressed.get() { "scale(0.97)" } else { "" }
                style:transition="background 0.1s, color 0.1s, transform 0.1s"
            >
                "Press me"
            </button>

            <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em;">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <p>"Is pressed: " { move || is_pressed.get() }</p>
            <p>"on_press_change: " { move || press_state.get() }</p>
            <p>"Was pressed: " { move || count.get() } { move || match count.get() {
                1 => " time",
                _ => " times",
            } }</p>
            <p>"Was double-pressed: " { move || dbl_count.get() } { move || match dbl_count.get() {
                1 => " time",
                _ => " times",
            } }</p>

            <p>"Last " { move || events.with(|events| events.occupied_len()) } " events: "</p>

            <pre style="
                width: 100%;
                height: 15em;
                overflow: auto;
                padding: var(--typography-code-padding);
                border: none;
                border-radius: var(--typography-code-border-radius);
                background-color: var(--typography-code-background-color);
                color: var(--typography-code-color);
            ">
                { move || string.get() }
            </pre>

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

            <p><strong>"should_cancel_on_pointer_exit"</strong>
               " — Press the button below, then drag the pointer outside before releasing. "
               "The press is cancelled and " <code>"on_press"</code> " does not fire."</p>

            <button
                {..cancel_props.into_attrs()}
                style:background=move || if cancel_is_pressed.get() { "var(--brand-color)" } else { "" }
                style:color=move || if cancel_is_pressed.get() { "white" } else { "" }
                style:transform=move || if cancel_is_pressed.get() { "scale(0.97)" } else { "" }
                style:transition="background 0.1s, color 0.1s, transform 0.1s"
            >
                "Drag outside to cancel"
            </button>

            <pre style="
                width: 100%;
                height: 5em;
                overflow: auto;
                padding: var(--typography-code-padding);
                border: none;
                border-radius: var(--typography-code-border-radius);
                background-color: var(--typography-code-background-color);
                color: var(--typography-code-color);
            ">
                { move || cancel_string.get() }
            </pre>

            <p><strong>"prevent_focus_on_press"</strong>
               " — Click the button below: it will not receive focus (no outline appears). "
               "Compare with the demo button above which receives focus on click."</p>

            <button
                {..no_focus_props.into_attrs()}
                on:focus=move |_| set_focus_demo_focused.set(true)
                on:blur=move |_| set_focus_demo_focused.set(false)
                style:background=move || if no_focus_is_pressed.get() { "var(--brand-color)" } else { "" }
                style:color=move || if no_focus_is_pressed.get() { "white" } else { "" }
                style:transform=move || if no_focus_is_pressed.get() { "scale(0.97)" } else { "" }
                style:transition="background 0.1s, color 0.1s, transform 0.1s"
                style:outline=move || if focus_demo_focused.get() { "2px solid var(--brand-color)" } else { "none" }
                style:outline-offset="2px"
            >
                "Click me (no focus)"
            </button>

            <p>"Focused: " { move || focus_demo_focused.get() }</p>

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

            <Code>
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

            <p>"Press and hold the button below for " { move || threshold.get().as_millis() } "ms to trigger a long press:"</p>

            <button
                {..lp_props.into_attrs()}
                style="
                    display: inline-flex;
                    border: 2px solid var(--brand-color);
                    padding: 1em 2em;
                    cursor: pointer;
                    border-radius: 4px;
                    background: var(--brand-color);
                    color: white;
                    font-size: 1em;
                "
            >
                "Long press me"
            </button>

            <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em; margin-top: 1em;">
                <Checkbox checked=lp_disabled set_checked=set_lp_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <div style="display: flex; align-items: center; gap: 0.5em; margin-top: 0.5em;">
                <Label>"Threshold:"</Label>
                <SliderAtom
                    values=SliderValues::Uncontrolled(vec![500.0])
                    min=100.0
                    max=2000.0
                    step=100.0
                    on_change=Callback::new(move |vals: Vec<f64>| {
                        set_threshold.set(Duration::from_millis(vals[0] as u64));
                    })
                    styles=[(Display, "flex"), (AlignItems, "center"), (Gap, "0.5em"), (Flex, "0 0 200px")]
                >
                    <SliderTrack styles=track_style()>
                        <SliderTrackFill styles=[(BackgroundColor, "var(--brand-color)"), (BorderRadius, "4px")]/>
                        <SliderThumb styles=thumb_style()/>
                    </SliderTrack>
                    <SliderOutput let:attrs let:values>
                        <output {..attrs} style=Styles::from([(MinWidth, "60px"), (TextAlign, "right")])>
                            { move || format!("{}ms", values.get()[0] as u64) }
                        </output>
                    </SliderOutput>
                </SliderAtom>
            </div>

            <p>"Long press count: " { move || lp_count.get() }</p>

            <p>"Last " { move || lp_events.with(|events| events.occupied_len()) } " events:"</p>

            <pre style="
                width: 100%;
                height: 10em;
                overflow: auto;
                padding: var(--typography-code-padding);
                border: none;
                border-radius: var(--typography-code-border-radius);
                background-color: var(--typography-code-background-color);
                color: var(--typography-code-color);
            ">
                { move || lp_string.get() }
            </pre>

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
            ]
        }/>
    }
}
