use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{keyboard::KeyboardDemo, keyboard_shortcuts::KeyboardShortcutsDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageUseKeyboard() -> impl IntoView {
    view! {
        <DocPage title="use_keyboard">
            <p>
                "The "<Code inline=true>"use_keyboard"</Code>" hook handles keyboard events on an element: raw key down and key up callbacks, "
                "typed keyboard shortcuts, disabling, and control over event propagation. "
                "See the "<Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>" for domain guidance."
            </p>

            <ReactAria hook="useKeyboard"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseKeyboardInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    ", so you only set the fields you need and finish with "<Code inline=true>"..Default::default()"</Code>
                    ". It is "<Code inline=true>"Clone"</Code>" but not "<Code inline=true>"Copy"</Code>", because it can own a set of shortcuts."
                </p>

                <ApiTable kind=ApiKind::Input of="UseKeyboardInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Ignore all keyboard events while "<Code inline=true>"true"</Code>"."</ApiRow>
                    <ApiRow name="on_key_down" ty="Option<Callback<KeyboardEventWrapper>>" default="None">
                        "Called when a key is pressed down, before any shortcut is handled."
                    </ApiRow>
                    <ApiRow name="on_key_up" ty="Option<Callback<KeyboardEventWrapper>>" default="None">"Called when a key is released."</ApiRow>
                    <ApiRow name="shortcuts" ty="Option<KeyboardShortcuts>" default="None">
                        "Shortcuts handled on key down. See "<a href="#shortcuts">"Shortcuts"</a>"."
                    </ApiRow>
                    <ApiRow name="allow_repeats" ty="bool" default="false">
                        "Whether shortcuts also fire for auto-repeated key presses while a key is held down. Turn this on for "
                        "navigation keys, so that holding an arrow key keeps moving."
                    </ApiRow>
                    <ApiRow name="allow_composing" ty="bool" default="false">
                        "Whether shortcuts also fire while an input method editor (IME) is composing text, e.g. while typing "
                        "Japanese or Chinese."
                    </ApiRow>
                </ApiTable>

                <p>
                    <Code inline=true>"KeyboardEventWrapper"</Code>" wraps the "<Code inline=true>"KeyboardEvent"</Code>
                    " and gives access to its key, code and modifiers. The event stops propagating after your callback "
                    "unless you call "<Code inline=true>"continue_propagation()"</Code>" on it."
                </p>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseKeyboardReturn">
                    <ApiRow name="props" ty="UseKeyboardProps">
                        "The "<Code inline=true>"keydown"</Code>" and "<Code inline=true>"keyup"</Code>" handlers. Spread them onto "
                        "a focusable element with "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let UseKeyboardReturn { props } = use_keyboard(UseKeyboardInput {
                            is_disabled: disabled.into(),
                            on_key_down: Some(Callback::new(|e: KeyboardEventWrapper| {
                                log!("{} pressed", e.key_value());
                                e.continue_propagation(); // events stop propagating by default
                            })),
                            ..Default::default()
                        });

                        view! {
                            <div tabindex="0" {..props.into_attrs()}>"Focus me and press keys"</div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="Key down and key up events with modifiers in an event log" source=include_str!("demos/keyboard.rs")>
                    <KeyboardDemo/>
                </Demo>
            </Section>

            <Section title="Shortcuts">
                <p>
                    "Matching keys by hand in "<Code inline=true>"on_key_down"</Code>" gets tedious quickly, especially once modifiers "
                    "are involved. Instead, describe each shortcut as a "<Code inline=true>"Shortcut"</Code>" and collect them in "
                    <Code inline=true>"KeyboardShortcuts"</Code>", both from "<Code inline=true>"leptonic::utils::keyboard_shortcut"</Code>"."
                </p>

                <p>
                    "A shortcut is one key plus the exact set of modifiers that must be held. "
                    <Code inline=true>"Shortcut::key(\"z\").primary()"</Code>" matches Ctrl+Z, but not Ctrl+Shift+Z. "
                    <Code inline=true>"primary()"</Code>" is the platform\u{2019}s main modifier: Command on Apple devices, Control everywhere else "
                    "("<Code inline=true>"Mod"</Code>" in parsed shortcuts). Keys are "
                    <Code inline=true>"KeyboardEvent.key"</Code>" values and compare case-insensitively. If your shortcuts come from configuration "
                    "or user input, "<Code inline=true>"Shortcut::parse(\"Mod+Shift+z\")"</Code>" parses modifiers and a key separated by "<Code inline=true>"+"</Code>"."
                </p>

                <p>
                    "Each handler decides what happened by what it returns. Returning nothing means \u{201c}handled\u{201d}: the browser default "
                    "is prevented and the event stops propagating. Returning a "<Code inline=true>"bool"</Code>" lets you decline: "
                    <Code inline=true>"false"</Code>" leaves the event alone, as if no shortcut had matched. For full control, return a "
                    <Code inline=true>"ShortcutOutcome"</Code>". Keys that match no shortcut keep bubbling."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::utils::keyboard_shortcut::{KeyboardShortcuts, Shortcut};

                        let shortcuts = KeyboardShortcuts::new()
                            .on(Shortcut::key("ArrowRight"), move |_| position.update(|p| *p += 1))
                            .on(Shortcut::key("ArrowLeft"), move |_| position.update(|p| *p -= 1))
                            .on(Shortcut::key("s").primary(), move |_| save())
                            // `false`: not handled, the event keeps its default and bubbles on.
                            .on(Shortcut::key("Home"), move |_| {
                                if position.get_untracked() == 0 {
                                    return false;
                                }
                                position.set(0);
                                true
                            });

                        let UseKeyboardReturn { props } = use_keyboard(UseKeyboardInput {
                            shortcuts: Some(shortcuts),
                            allow_repeats: true, // holding an arrow key keeps moving
                            ..Default::default()
                        });

                        view! { <div tabindex="0" {..props.into_attrs()}>"Focus me"</div> }
                    "#)}
                </Code>

                <Demo description="Arrow keys move a counter, Mod+S saves, Home resets" source=include_str!("demos/keyboard_shortcuts.rs")>
                    <KeyboardShortcutsDemo/>
                </Demo>

                <p>
                    "Other hooks accept shortcuts too: "<Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link>
                    " ("<Code inline=true>"shortcuts"</Code>" and "<Code inline=true>"allow_shortcut_repeats"</Code>") and "
                    <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" ("<Code inline=true>"shortcuts"</Code>
                    "). That is how "<Code inline=true>"use_menu_trigger"</Code>" opens its menu with the arrow keys."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
                <li><Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
