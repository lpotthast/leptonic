use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;
use leptonic::atoms::focus_ring::FocusRing;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;
use ringbuf::traits::{Consumer, Observer, RingBuffer};
use ringbuf::HeapRb;

#[component]
pub fn PageUseKeyboard() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<String>::new(20));
    let (disabled, set_disabled) = signal(false);

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

    let UseKeyboardReturn { props } = use_keyboard(UseKeyboardInput {
        disabled: disabled.into(),
        on_key_down: Some(Callback::new(move |e: KeyboardEventWrapper| {
            set_events.update(|events| {
                events.push_overwrite(format!(
                    "KeyDown: key={}, code={}, shift={}, ctrl={}, alt={}, meta={}",
                    e.key(),
                    e.code(),
                    e.shift_key(),
                    e.ctrl_key(),
                    e.alt_key(),
                    e.meta_key()
                ));
            });
            // Allow propagation so other handlers can also receive the event
            e.continue_propagation();
        })),
        on_key_up: Some(Callback::new(move |e: KeyboardEventWrapper| {
            set_events.update(|events| {
                events.push_overwrite(format!("KeyUp: key={}, code={}", e.key(), e.code()));
            });
            e.continue_propagation();
        })),
    });

    view! {
        <Article>
            <h1 id="use_keyboard" class="anchor">
                "use_keyboard"
                <AnchorLink href="#use_keyboard" description="Direct link to article header"/>
            </h1>

            <p>"Handle keyboard events with support for disabling and controlling event propagation."</p>

            <Code>
                {indoc!(r#"
                    let UseKeyboardReturn { attrs } = use_keyboard(UseKeyboardInput {
                        disabled: disabled.into(),
                        on_key_down: Some(Callback::new(|e: KeyboardEventWrapper| {
                            // Handle key press
                            e.continue_propagation(); // Allow parent handlers
                        })),
                        on_key_up: Some(Callback::new(|e: KeyboardEventWrapper| {
                            // Handle key release
                        })),
                    });

                    view! {
                        <div tabindex="0" {..attrs}>
                            "Focus me and press keys"
                        </div>
                    }
                "#)}
            </Code>

            <p>"Focus the box below and press any key to see keyboard events:"</p>

            <FocusRing disabled within=true>
                <div
                    {..props.into_attrs()}
                    tabindex="0"
                    style="
                        display: inline-flex;
                        border: 2px solid var(--brand-color);
                        padding: 1em 2em;
                        cursor: pointer;
                        border-radius: 4px;
                        outline: none;
                    "
                >
                    "Focus me and press keys"
                </div>
            </FocusRing>

            <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em; margin-top: 1em;">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <p>"Last " { move || events.with(|events| events.occupied_len()) } " events:"</p>

            <pre style="
                width: 100%;
                height: 12em;
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
                <li>"Handles keydown and keyup events"</li>
                <li>"Supports disabling via signal"</li>
                <li>"Controls event propagation (stops by default, call continue_propagation() to allow)"</li>
                <li>"Wraps KeyboardEvent with convenient accessors"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_keyboard", link: "#use_keyboard" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
