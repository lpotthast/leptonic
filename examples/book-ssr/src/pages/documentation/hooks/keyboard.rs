use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::{keyboard::KeyboardDemo, keyboard_shortcuts::KeyboardShortcutsDemo};

#[component]
pub fn PageUseKeyboard() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use-keyboard" class="anchor">
                "use_keyboard"
                <AnchorLink href="#use-keyboard" description="Direct link to section: use_keyboard"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_keyboard"</Code>" hook handles keyboard events on an element: raw key down and key up callbacks, "
                "typed keyboard shortcuts, disabling, and control over event propagation. "
                "See the "<Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link>" for domain guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useKeyboard.html" target=LinkTarget::_Blank>
                    "useKeyboard"
                </LinkExt>
                "."
            </p>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to section: Input"/>
            </h2>

            <p>
                <Code inline=true>"UseKeyboardInput"</Code>" implements "<Code inline=true>"Default"</Code>
                ", so you only set the fields you need and finish with "<Code inline=true>"..Default::default()"</Code>
                ". It is "<Code inline=true>"Clone"</Code>" but not "<Code inline=true>"Copy"</Code>", because it can own a set of shortcuts."
            </p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Default"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"disabled"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell><Code inline=true>"false"</Code></TableCell>
                            <TableCell>"Disables all keyboard event handling when true."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_key_down"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<KeyboardEventWrapper>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Called when a key is pressed down, before any shortcut is handled."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_key_up"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<KeyboardEventWrapper>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Called when a key is released."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"shortcuts"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<KeyboardShortcuts>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Shortcuts handled on key down. See "<a href="#shortcuts">"Shortcuts"</a>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"allow_repeats"</Code></TableCell>
                            <TableCell><Code inline=true>"bool"</Code></TableCell>
                            <TableCell><Code inline=true>"false"</Code></TableCell>
                            <TableCell>"Whether shortcuts also fire for auto-repeated key presses while a key is held down. Turn this on for navigation keys, so that holding an arrow key keeps moving."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"allow_composing"</Code></TableCell>
                            <TableCell><Code inline=true>"bool"</Code></TableCell>
                            <TableCell><Code inline=true>"false"</Code></TableCell>
                            <TableCell>"Whether shortcuts also fire while an input method editor (IME) is composing text, e.g. while typing Japanese or Chinese."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return" class="anchor">
                "Return"
                <AnchorLink href="#return" description="Direct link to section: Return"/>
            </h2>

            <p><Code inline=true>"UseKeyboardReturn"</Code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"props"</Code></TableCell>
                            <TableCell><Code inline=true>"UseKeyboardProps"</Code></TableCell>
                            <TableCell>"Spread onto the target element via "<Code inline=true>"props.into_attrs()"</Code>" to wire up keydown/keyup listeners."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="demo" class="anchor">
                "Demo"
                <AnchorLink href="#demo" description="Direct link to section: Demo"/>
            </h2>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let UseKeyboardReturn { props } = use_keyboard(UseKeyboardInput {
                        disabled: disabled.into(),
                        on_key_down: Some(Callback::new(|e: KeyboardEventWrapper| {
                            // e.key(), e.code(), e.shift_key(), e.ctrl_key(), ...
                            e.continue_propagation(); // stops propagation by default
                        })),
                        on_key_up: Some(Callback::new(|e: KeyboardEventWrapper| {
                            e.continue_propagation();
                        })),
                        ..Default::default()
                    });

                    view! {
                        <div tabindex="0" {..props.into_attrs()}>
                            "Focus me and press keys"
                        </div>
                    }
                "#)}
            </Code>

            <DemoShell source=include_str!("demos/keyboard.rs")>
                <KeyboardDemo />
            </DemoShell>

            <h2 id="shortcuts" class="anchor">
                "Shortcuts"
                <AnchorLink href="#shortcuts" description="Direct link to section: Shortcuts"/>
            </h2>

            <p>
                "Matching keys by hand in "<Code inline=true>"on_key_down"</Code>" gets tedious quickly, especially once modifiers "
                "are involved. Instead, describe each shortcut as a "<Code inline=true>"Shortcut"</Code>" and collect them in "
                <Code inline=true>"KeyboardShortcuts"</Code>", both from "<Code inline=true>"leptonic::utils::keyboard_shortcut"</Code>"."
            </p>

            <p>
                "A shortcut is one key plus the exact set of modifiers that must be held. "
                <Code inline=true>"Shortcut::key(\"z\").primary()"</Code>" matches Ctrl+Z, but not Ctrl+Shift+Z. "
                <Code inline=true>"primary()"</Code>" is the platform\u{2019}s main modifier: Command on Apple devices, Control everywhere else "
                "(react-aria calls it "<Code inline=true>"Mod"</Code>"). Keys are "
                <Code inline=true>"KeyboardEvent.key"</Code>" values and compare case-insensitively. If your shortcuts come from configuration "
                "or user input, "<Code inline=true>"Shortcut::parse(\"Mod+Shift+z\")"</Code>" understands react-aria\u{2019}s string syntax."
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

            <DemoShell
                source=include_str!("demos/keyboard_shortcuts.rs")
                description="Arrow keys move a counter, Mod+S saves, Home resets"
            >
                <KeyboardShortcutsDemo />
            </DemoShell>

            <p>
                "Other hooks accept shortcuts too: "<Link href=crate::routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link>
                " ("<Code inline=true>"shortcuts"</Code>" and "<Code inline=true>"allow_shortcut_repeats"</Code>") and "
                <Link href=crate::routes::doc::button::Hook.materialize()>"use_button"</Link>" ("<Code inline=true>"shortcuts"</Code>
                "). That is how "<Code inline=true>"use_menu_trigger"</Code>" opens its menu with the arrow keys."
            </p>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to section: Features"/>
            </h2>

            <ul>
                <li>"Handles keydown and keyup events"</li>
                <li>"Typed keyboard shortcuts with exact modifier matching and a platform-aware primary modifier"</li>
                <li>"Shortcuts skip auto-repeated key presses and IME composition unless you opt in"</li>
                <li>"Supports disabling via signal"</li>
                <li>"Controls event propagation (stops by default, call "<Code inline=true>"continue_propagation()"</Code>" to allow)"</li>
                <li>"Wraps "<Code inline=true>"KeyboardEvent"</Code>" with convenient accessors via "<Code inline=true>"KeyboardEventWrapper"</Code></li>
            </ul>

            <h2 id="deviations" class="anchor">
                "Deviations from react-aria"
                <AnchorLink href="#deviations" description="Direct link to section: Deviations"/>
            </h2>

            <ul>
                <li>
                    <b>"Typed shortcuts."</b>" React-aria describes shortcuts as strings like "<Code inline=true>"\"Mod+Shift+z\""</Code>
                    " that are parsed at runtime. Leptonic uses "<Code inline=true>"Shortcut"</Code>" values, which can be "
                    <Code inline=true>"const"</Code>". "<Code inline=true>"Shortcut::parse"</Code>" still accepts the string syntax."
                </li>
                <li>
                    <b>"No portal filtering."</b>" React re-dispatches events through the component tree, so react-aria skips events "
                    "from portals that aren\u{2019}t DOM descendants. Leptos uses native DOM events, which only bubble through DOM ancestors, "
                    "so there is nothing to filter."
                </li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UseHover.materialize()>"use_hover"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_keyboard", link: "#use-keyboard" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return", link: "#return" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Shortcuts", link: "#shortcuts" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "Deviations", link: "#deviations" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
