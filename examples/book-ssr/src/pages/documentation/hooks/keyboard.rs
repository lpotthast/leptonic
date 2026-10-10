use indoc::indoc;
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
                "See the "<Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>
                " to compare it with the other interaction building blocks."
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
                        "Called when a key is pressed down, before the shortcuts are handled."
                    </ApiRow>
                    <ApiRow name="on_key_up" ty="Option<Callback<KeyboardEventWrapper>>" default="None">"Called when a key is released."</ApiRow>
                    <ApiRow name="shortcuts" ty="Option<KeyboardShortcuts>" default="None">
                        "Shortcuts handled on key down, after "<Code inline=true>"on_key_down"</Code>". See "
                        <AnchorLink href="#keyboard-shortcuts">"Keyboard Shortcuts"</AnchorLink>"."
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
                    "unless you call "<Code inline=true>"continue_propagation()"</Code>" on it (see "
                    <Link href=routes::doc::EventPropagation.materialize()>"Event Propagation"</Link>")."
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
                        use leptonic::{
                            Propagation,
                            hooks::interactions::{KeyboardEventWrapper, UseKeyboardInput, UseKeyboardReturn, use_keyboard},
                        };
                        use leptos::{logging::log, prelude::*};

                        let disabled = RwSignal::new(false);

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

            <Section title="Keyboard Shortcuts">
                <p>
                    "Matching keys by hand in "<Code inline=true>"on_key_down"</Code>" gets tedious quickly, especially once modifiers "
                    "are involved. Instead, describe each shortcut as a "<Code inline=true>"Shortcut"</Code>", bind it to a handler in "
                    <Code inline=true>"KeyboardShortcuts"</Code>" and pass them as "<Code inline=true>"shortcuts"</Code>
                    ". Both live in "<Code inline=true>"leptonic"</Code>"."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            KeyboardKey,
                            KeyboardShortcuts,
                            Shortcut,
                            hooks::interactions::{UseKeyboardInput, UseKeyboardReturn, use_keyboard},
                        };
                        use leptos::prelude::*;

                        let position = RwSignal::new(0);
                        let saves = RwSignal::new(0);

                        let shortcuts = KeyboardShortcuts::new()
                            .on(Shortcut::new(KeyboardKey::ArrowRight), move |_| position.update(|p| *p += 1))
                            .on(Shortcut::new(KeyboardKey::ArrowLeft), move |_| position.update(|p| *p -= 1))
                            .on(Shortcut::new(KeyboardKey::S).primary(), move |_| saves.update(|s| *s += 1))
                            // `false`: not handled, the event keeps its default and bubbles on.
                            .on(Shortcut::new(KeyboardKey::Home), move |_| {
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

                <Demo description="Arrow keys move a counter, the primary modifier with S saves, Home resets" source=include_str!("demos/keyboard_shortcuts.rs")>
                    <KeyboardShortcutsDemo/>
                </Demo>

                <p>
                    "Other hooks accept shortcuts too: "<Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link>
                    " ("<Code inline=true>"shortcuts"</Code>" and "<Code inline=true>"allow_shortcut_repeats"</Code>") and "
                    <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" ("<Code inline=true>"shortcuts"</Code>
                    "). That is how "<Link href=format!("{}#use-menu-trigger", routes::doc::menu::Hook.materialize())>
                    <Code inline=true>"use_menu_trigger"</Code></Link>" opens its menu with the arrow keys."
                </p>

                <p>
                    "Shortcuts of the whole page, which work wherever the focus is (e.g. "<Keys keys="Control + K"/>" ("
                    <Keys keys="Meta + K"/>" on macOS) opening a search), are bound with "
                    <Link href=routes::doc::interactions::UseGlobalShortcuts.materialize()>"use_global_shortcuts"</Link>"."
                </p>

                <Section title="Shortcut">
                    <p>
                        "A "<Code inline=true>"Shortcut"</Code>" is one "<Code inline=true>"KeyboardKey"</Code>" plus the exact set of "
                        "modifiers that must be held: "<Code inline=true>"Shortcut::new(KeyboardKey::Z).primary()"</Code>" matches "
                        <Keys keys="Control + Z"/>" ("<Keys keys="Meta + Z"/>" on macOS), but not "<Keys keys="Control + Shift + Z"/>
                        ". Letters match in either case. The builders are "<Code inline=true>"const"</Code>
                        ", so shortcuts can be constants."
                    </p>

                    <DocTable headers=&["Function", "Description"]>
                        <TableRow>
                            <TableCell><Code inline=true>"Shortcut::new(key)"</Code></TableCell>
                            <TableCell>
                                "The "<Code inline=true>"KeyboardKey"</Code>" without modifiers. A key without a variant of its own is "
                                <Code inline=true>"KeyboardKey::Other(..)"</Code>", with its "<Code inline=true>"KeyboardEvent.key"</Code>" value."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>".primary()"</Code></TableCell>
                            <TableCell>
                                "Also require the platform\u{2019}s primary modifier: Command on Apple devices, Control everywhere else."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>
                                <Code inline=true>".shift()"</Code>", "<Code inline=true>".alt()"</Code>", "
                                <Code inline=true>".ctrl()"</Code>", "<Code inline=true>".meta()"</Code>
                            </TableCell>
                            <TableCell>"Also require this modifier, on every platform."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Shortcut::parse(spec)"</Code></TableCell>
                            <TableCell>
                                "Parses modifiers and one key separated by "<Code inline=true>"+"</Code>", e.g. "
                                <Code inline=true>"\"Mod+Shift+z\""</Code>" ("<Code inline=true>"Mod"</Code>" is the primary "
                                "modifier), for shortcuts from configuration or user input. Names are case-insensitive, and the aliases "
                                <Code inline=true>"Space"</Code>", "<Code inline=true>"Esc"</Code>", "<Code inline=true>"Del"</Code>", "
                                <Code inline=true>"Ins"</Code>", "<Code inline=true>"Left"</Code>", "<Code inline=true>"Right"</Code>", "
                                <Code inline=true>"Up"</Code>" and "<Code inline=true>"Down"</Code>" are accepted. Fails with "
                                <Code inline=true>"InvalidShortcut"</Code>" unless exactly one key is named. "
                                <Code inline=true>"Shortcut"</Code>" also implements "<Code inline=true>"FromStr"</Code>"."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>".matches(&event)"</Code></TableCell>
                            <TableCell>"Whether a "<Code inline=true>"KeyboardEvent"</Code>" triggers the shortcut on the current platform."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>".keys(apple)"</Code></TableCell>
                            <TableCell>
                                "The keys to show for the shortcut, as "<Code inline=true>"KeyboardKey"</Code>"s: the modifiers in the "
                                "order of the platform (Apple\u{2019}s when "<Code inline=true>"apple"</Code>" is true), then the key. The "
                                <Link href=format!("{}#shortcutkeys", routes::doc::Kbd.materialize())><Code inline=true>"ShortcutKeys"</Code></Link>
                                " atom shows them."
                            </TableCell>
                        </TableRow>
                    </DocTable>
                </Section>

                <Section title="KeyboardShortcuts">
                    <p>
                        <Code inline=true>"KeyboardShortcuts::new()"</Code>" starts an empty set. "<Code inline=true>".on(shortcut, handler)"</Code>
                        " binds a handler, which receives the "<Code inline=true>"KeyboardEvent"</Code>"; "
                        <Code inline=true>".with(other)"</Code>" adds another set. When several bindings match an event, the one "
                        "added last wins. Keys that match no shortcut keep bubbling."
                    </p>
                </Section>

                <Section title="ShortcutOutcome">
                    <p>"A handler tells what it did with the event by what it returns:"</p>

                    <DocTable headers=&["Return value", "Effect"]>
                        <TableRow>
                            <TableCell><Code inline=true>"()"</Code>" or "<Code inline=true>"true"</Code></TableCell>
                            <TableCell>"Handled: the browser default is prevented and the event stops propagating."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"false"</Code></TableCell>
                            <TableCell>"Ignored: the event is left alone, as if no shortcut had matched."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"ShortcutOutcome"</Code></TableCell>
                            <TableCell>
                                <Code inline=true>"Handled"</Code>", "<Code inline=true>"Ignored"</Code>", or "
                                <Code inline=true>"Custom { prevent_default, continue_propagation }"</Code>" to decide both separately."
                            </TableCell>
                        </TableRow>
                    </DocTable>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
                <li><Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link></li>
                <li><Link href=routes::doc::interactions::UseGlobalShortcuts.materialize()>"use_global_shortcuts"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
                <li><Link href=routes::doc::Kbd.materialize()>"Kbd Atoms"</Link></li>
                <li><Link href=routes::doc::EventPropagation.materialize()>"Event Propagation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
