use indoc::indoc;
use leptonic::{components::prelude::*, hooks::LinkTarget};
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageEventPropagation() -> impl IntoView {
    view! {
        <DocPage title="Event Propagation">
            <p>
                "On the web, events bubble up the DOM tree unless a handler stops them. Leptonic, following "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/" target=LinkTarget::_Blank>"react-aria"</LinkExt>
                ", inverts this default: events handled by leptonic stop propagating, whether or not you passed a handler. "
                "Only a handler that calls "<Code inline=true>"continue_propagation()"</Code>" lets its event bubble on."
            </p>

            <Section title="Why stop by default">
                <p>"Bubbling causes subtle bugs once components are nested:"</p>
                <ul>
                    <li>
                        <b>"Double handling."</b>" A button inside a pressable list item: pressing the button also triggers the "
                        "item, unless the button\u{2019}s handler remembers to stop propagation."
                    </li>
                    <li>
                        <b>"Composition across authors."</b>" A menu trigger inside a toolbar, a checkbox inside a draggable row: "
                        "the outer component reacts to events that were meant for the inner one, and neither knows about the other."
                    </li>
                    <li>
                        <b>"The rare case is the default."</b>" Most handlers handle their event completely. Wanting it to "
                        "bubble is the exception."
                    </li>
                </ul>
                <p>
                    "With stopping as the default, every component only sees the events directed at it, and the closest handler "
                    "owns an event unless it explicitly hands it on."
                </p>
            </Section>

            <Section title="Continuing propagation">
                <p>
                    "Leptonic\u{2019}s own event types, "<Code inline=true>"PressEvent"</Code>" (from "
                    <Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>" and everything built on it) "
                    "and "<Code inline=true>"KeyboardEventWrapper"</Code>" (from "
                    <Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>"), implement the "
                    <Code inline=true>"Propagation"</Code>" trait. Its "<Code inline=true>"continue_propagation()"</Code>
                    " lets the event bubble on. There is no "<Code inline=true>"stop_propagation()"</Code>
                    ": stopping is what happens anyway."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::{hooks::*, utils::{Propagation, key::KeyboardKey}};

                        let keyboard = use_keyboard(UseKeyboardInput {
                            on_key_down: Some(Callback::new(move |e: KeyboardEventWrapper| {
                                if e.key() == KeyboardKey::Enter {
                                    submit();
                                } else {
                                    // Not ours: let a parent handle it.
                                    e.continue_propagation();
                                }
                            })),
                            ..Default::default()
                        });
                    ")}
                </Code>

                <p>
                    "Continuing propagation thereby says \u{201c}I did not handle this, someone else should\u{201d}, which "
                    "gives bubbling a meaning instead of being an accident."
                </p>
            </Section>

            <Section title="Trade-offs">
                <p>
                    "Some code relies on bubbling, such as document-level click tracking, idle detection or analytics. Use "
                    "capture-phase listeners for these: the capture phase runs before the event reaches its target, so stopping "
                    "propagation there does not affect them."
                </p>
                <p>
                    "Native events you handle yourself, like "<Code inline=true>"on:click"</Code>" on a plain element or the "
                    <Code inline=true>"FocusEvent"</Code>"s passed to focus callbacks, keep the web\u{2019}s default and bubble."
                </p>
            </Section>
        </DocPage>
    }
}
