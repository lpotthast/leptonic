use indoc::indoc;
use leptos::prelude::*;

use super::demos::{
    event_propagation::EventPropagationDemo, event_propagation_press::EventPropagationPressDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageEventPropagation() -> impl IntoView {
    view! {
        <DocPage title="Event Propagation">
            <p>
                "On the web, events bubble up the DOM tree unless a handler stops them. Leptonic inverts this default for "
                "the events it handles for you: they stop at the element that handled them. Only a handler that calls "
                <Code inline=true>"continue_propagation()"</Code>" lets its event bubble on."
            </p>

            <Section title="Why Stop by Default">
                <p>"Bubbling causes subtle bugs once interactive elements are nested:"</p>
                <ul>
                    <li>
                        <b>"Double handling."</b>" A button inside a pressable list item: pressing the button also triggers the "
                        "item, unless the button\u{2019}s handler remembers to stop propagation."
                    </li>
                    <li>
                        <b>"Composition across authors."</b>" A menu trigger inside a toolbar, a checkbox inside a draggable row: "
                        "the outer element reacts to events that were meant for the inner one, and neither knows about the other."
                    </li>
                    <li>
                        <b>"The rare case is the default."</b>" Most handlers handle their event completely. Wanting it to "
                        "bubble is the exception."
                    </li>
                </ul>
                <p>
                    "With stopping as the default, every element only sees the events directed at it, and the closest handler "
                    "owns an event unless it explicitly hands it on."
                </p>
            </Section>

            <Section title="Which Events Stop">
                <DocTable headers=&["Events", "Stop"]>
                    <TableRow>
                        <TableCell>
                            "Press events of "<Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>
                            " and everything built on it (buttons, links, menu items, \u{2026})"
                        </TableCell>
                        <TableCell>
                            "Always, whether or not you passed a callback, unless the callback calls "
                            <Code inline=true>"continue_propagation()"</Code>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            "Key events of "<Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>
                            " and the hooks that take "<Code inline=true>"on_key_down"</Code>" / "<Code inline=true>"on_key_up"</Code>
                        </TableCell>
                        <TableCell>
                            "Only when you passed a callback that doesn\u{2019}t call "<Code inline=true>"continue_propagation()"</Code>
                            ". Without a callback, keys bubble; so do keys none of your keyboard shortcuts matches."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            "The pointer and arrow key events "<Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link>
                            " turns into moves, wheel events of "
                            <Link href=routes::doc::interactions::UseScrollWheel.materialize()>"use_scroll_wheel"</Link>" (except while "
                            <Keys keys="Control"/>" is held, for zooming), "<Code inline=true>"contextmenu"</Code>" events of "
                            <Link href=routes::doc::interactions::UseContextMenu.materialize()>"use_context_menu"</Link>", the native "
                            "events of "<Link href=routes::doc::DragAndDrop.materialize()>"drag and drop"</Link>", and the outside "
                            "presses that close an overlay"
                        </TableCell>
                        <TableCell>"Always: their events have no "<Code inline=true>"continue_propagation()"</Code>"."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            "Hover events of "<Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>
                            ", the "<Code inline=true>"FocusEvent"</Code>"s of focus callbacks, and events you handle yourself ("
                            <Code inline=true>"on:click"</Code>" on a plain element)"
                        </TableCell>
                        <TableCell>
                            "Never: they keep the web\u{2019}s default. Stopping focus events would break the "
                            <Link href=routes::doc::focus::UseFocusWithin.materialize()>"focus within"</Link>" tracking of the "
                            "elements around."
                        </TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "Focus events are a case of their own: "<Code inline=true>"focus"</Code>" and "<Code inline=true>"blur"</Code>
                    " don\u{2019}t bubble on the web at all. To react to focus anywhere inside an element, use "
                    <Link href=routes::doc::focus::UseFocusWithin.materialize()>"use_focus_within"</Link>"."
                </p>
            </Section>

            <Section title="Continuing Propagation">
                <p>
                    "Leptonic\u{2019}s event types "<Code inline=true>"PressEvent"</Code>" and "
                    <Code inline=true>"KeyboardEventWrapper"</Code>" implement the "<Code inline=true>"Propagation"</Code>
                    " trait ("<Code inline=true>"leptonic::utils::Propagation"</Code>"). Its "
                    <Code inline=true>"continue_propagation()"</Code>" lets the event bubble on. There is no "
                    <Code inline=true>"stop_propagation()"</Code>": stopping is what happens anyway."
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

                <p>
                    "In the demo, the panel around the field closes on "<Keys keys="Escape"/>", and the field handles "
                    <Keys keys="Enter"/>". Type into the field and press "<Keys keys="Escape"/>": the field\u{2019}s "
                    "handler stops the key, so the panel never sees it, until you let the field\u{2019}s other keys bubble."
                </p>
                <Demo
                    description="Search field handling Enter inside a panel handling Escape, with a checkbox letting the field's other keys bubble"
                    source=include_str!("demos/event_propagation.rs")
                >
                    <EventPropagationDemo/>
                </Demo>

                <Section title="Presses">
                    <p>
                        "A press consists of several DOM events (pointer down, pointer up, click, or key down and key up), "
                        "and each press callback decides only for the events that trigger it: "
                        <Code inline=true>"on_press_start"</Code>" for the pointer or key going down, "
                        <Code inline=true>"on_press_up"</Code>", "<Code inline=true>"on_press_end"</Code>" and "
                        <Code inline=true>"on_press"</Code>" for it going up. A callback you didn\u{2019}t pass stops its "
                        "events. Whether a parent\u{2019}s own press handling sees a complete press therefore depends on "
                        "which of them reach it. To let every press event of an element bubble, set "
                        <Code inline=true>"propagation: PressPropagation::Continue"</Code>" in the input of its "
                        <Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>"."
                    </p>
                    <p>
                        "In the demo, the card counts presses anywhere on it, and the button inside counts its own. "
                        "Pressing the button presses the card too only once all four of the button\u{2019}s press callbacks "
                        "continue propagation."
                    </p>
                    <Demo
                        description="Pressable card with a button inside, with a checkbox letting the button's presses bubble to the card"
                        source=include_str!("demos/event_propagation_press.rs")
                    >
                        <EventPropagationPressDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="Trade-offs">
                <p>
                    "Some code relies on bubbling, such as document-level click tracking, idle detection or analytics. Use "
                    "capture-phase listeners for these: the capture phase runs before the event reaches its target, so stopping "
                    "propagation there does not affect them."
                </p>
            </Section>
        </DocPage>
    }
}
