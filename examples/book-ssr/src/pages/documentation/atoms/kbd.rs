use indoc::indoc;
use leptos::prelude::*;

use super::demos::{keys::KeysDemo, shortcut_keys::ShortcutKeysDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomKbd() -> impl IntoView {
    view! {
        <DocPage title="Kbd Atoms">
            <p>
                "Key caps show keys and keyboard shortcuts in text: in help texts, in menus or next to the actions they "
                "trigger, so that users learn them. They only display keys. Handling the key presses is up to "
                <Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>" (keys pressed on "
                "one element) and "<Link href=routes::doc::interactions::UseGlobalShortcuts.materialize()>"use_global_shortcuts"</Link>
                " (shortcuts of the whole page)."
            </p>
            <p>
                "Two unstyled atoms render them: "<AnchorLink href="#shortcutkeys"><Code inline=true>"ShortcutKeys"</Code></AnchorLink>
                " shows a "<Code inline=true>"Shortcut"</Code>" your app handles, in the form of the user\u{2019}s "
                "platform (\u{201c}Ctrl + K\u{201d}, or \u{201c}\u{2318}K\u{201d} on Apple devices); "
                <AnchorLink href="#keys"><Code inline=true>"Keys"</Code></AnchorLink>" shows the keys you name, the same "
                "on every platform."
            </p>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{Shortcut, atoms};
                        use leptos::prelude::*;

                        const SEARCH: Shortcut = Shortcut::new(KeyboardKey::K).primary();

                        view! {
                            // "Ctrl + K", or "⌘K" on Apple devices.
                            <p>"Search with "<atoms::kbd::ShortcutKeys shortcut=SEARCH classes="my-keys"/>"."</p>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="Four shortcuts with modifiers, arrow keys and Escape, in the form of your platform" source=include_str!("demos/shortcut_keys.rs")>
                    <ShortcutKeysDemo/>
                </Demo>
            </Section>

            <Section title="ShortcutKeys">
                <p>
                    "Shows a keyboard "<Code inline=true>"Shortcut"</Code>" as keys, in the form of the user\u{2019}s "
                    "platform."
                </p>
                <Section title="Props" id="shortcutkeys-props">
                    <ApiTable kind=ApiKind::Props of="ShortcutKeys">
                        <ApiRow name="shortcut" ty="Shortcut">
                            "The shortcut to show, usually the one your app handles with "
                            <Link href=routes::doc::interactions::UseGlobalShortcuts.materialize()>"use_global_shortcuts"</Link>
                            " or "<Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>". Required."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the outer "<Code inline=true>"<kbd>"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Platforms">
                    <p>
                        "Apple platforms write shortcuts as glyphs without separators, in the order Control, Option, Shift, "
                        "Command; other platforms name the keys and join them with "<Code inline=true>"+"</Code>
                        ", in the order Control, Alt, Shift, Meta. The platform\u{2019}s primary modifier ("
                        <Code inline=true>".primary()"</Code>") is Command on Apple platforms and Control elsewhere:"
                    </p>

                    <DocTable headers=&["Shortcut", "Windows, Linux, Android", "macOS, iOS"]>
                        <TableRow>
                            <TableCell><Code inline=true>"Shortcut::key(\"k\").primary()"</Code></TableCell>
                            <TableCell>"Ctrl + K"</TableCell>
                            <TableCell>"\u{2318}K"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Shortcut::key(\"z\").primary().shift()"</Code></TableCell>
                            <TableCell>"Ctrl + \u{21e7} + Z"</TableCell>
                            <TableCell>"\u{21e7}\u{2318}Z"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Shortcut::key(\"ArrowDown\").alt()"</Code></TableCell>
                            <TableCell>"Alt + \u{2193}"</TableCell>
                            <TableCell>"\u{2325}\u{2193}"</TableCell>
                        </TableRow>
                    </DocTable>

                    <p>
                        "The server can\u{2019}t know the user\u{2019}s platform. So that hydration finds the markup it "
                        "rendered, the server and the first render in the browser show the generic form; on Apple platforms, "
                        "the keys switch to the Apple form right after hydration. Without server-side rendering, the switch "
                        "happens right after the first render."
                    </p>
                </Section>

                <Section title="Shortcut::keys">
                    <p>
                        <Code inline=true>"shortcut.keys(apple: bool) -> Vec<KeyboardKey>"</Code>" returns the keys the atom "
                        "shows: the modifiers in the order of the platform, then the key. Use it to render the keys "
                        "differently, e.g. with "<AnchorLink href="#keys"><Code inline=true>"Keys"</Code></AnchorLink>
                        " and your own separators. Pass "<Code inline=true>"false"</Code>" on the server and during "
                        "hydration, as the atom does, so that the markup matches:"
                    </p>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{Shortcut, atoms::kbd::Keys};
                            use leptos::prelude::*;

                            // [Control, Shift, Z]; with `true`: [Shift, Command, Z].
                            let keys = Shortcut::new(KeyboardKey::Z).primary().shift().keys(false);

                            view! { <Keys keys separators=false classes="my-keys"/> }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="Keys">
                <p>
                    "Shows the keys you name, the same on every platform, e.g. in documentation ("
                    "\u{201c}Command + X\u{201d} also on Windows). With "<Code inline=true>"separators=false"</Code>
                    ", the keys are written together, as Apple platforms do (\u{201c}\u{2318}X\u{201d})."
                </p>
                <Demo description="A sentence with the key Escape and the keys Control + Enter" source=include_str!("demos/keys.rs") source_open=true>
                    <KeysDemo/>
                </Demo>
                <Section title="Props" id="keys-props">
                    <ApiTable kind=ApiKind::Props of="Keys">
                        <ApiRow name="keys" ty="Signal<Vec<KeyboardKey>>">
                            "The keys to show, in this order: a value or any signal. Required."
                        </ApiRow>
                        <ApiRow name="separators" ty="bool" default="true">
                            "Whether a "<Code inline=true>"+"</Code>" separates the keys."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the outer "<Code inline=true>"<kbd>"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Structure">
                <p>
                    "Both atoms render one "<Code inline=true>"<kbd>"</Code>" holding a "<Code inline=true>"<kbd>"</Code>
                    " per key, as HTML writes a key combination. A key shown as a glyph or an abbreviation hides it from "
                    "screen readers and adds its name in visually hidden text. The outer "<Code inline=true>"<kbd>"</Code>
                    " is always left-to-right ("<Code inline=true>"dir=\"ltr\""</Code>"), so that a shortcut keeps its order "
                    "in right-to-left pages. "<Keys keys="Control + K"/>" renders as:"
                </p>
                <Code language=Language::Html>
                    {indoc!(r#"
                        <kbd dir="ltr" class="leptonic-ShortcutKeys my-keys">
                            <kbd><span aria-hidden="true">Ctrl</span><span style="/* visually hidden */">Control</span></kbd>
                            <span data-separator="" aria-hidden="true">+</span>
                            <kbd>K</kbd>
                        </kbd>
                    "#)}
                </Code>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-separator" ty="\"\"">
                        "On the "<Code inline=true>"+"</Code>" between two keys. Hidden from screen readers."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. "<Code inline=true>"ShortcutKeys"</Code>" renders the class "
                    <Code inline=true>"leptonic-ShortcutKeys"</Code>", "<Code inline=true>"Keys"</Code>" the class "
                    <Code inline=true>"leptonic-Keys"</Code>", each followed by the "<Code inline=true>"classes"</Code>
                    " you pass. Style the outer "<Code inline=true>"<kbd>"</Code>" through the class, the key caps as "
                    <Code inline=true>"kbd kbd"</Code>" and the separators by their attribute. The book\u{2019}s demos use "
                    "these rules:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-keys { display: inline-flex; align-items: center; gap: 0.25em; font-family: monospace; font-size: 0.85em; }
                        .my-keys kbd { min-width: 1.5em; padding: 0 0.25em; border: 1px solid var(--border); border-radius: 4px; background: var(--surface); text-align: center; }
                        .my-keys [data-separator] { color: var(--muted); }
                    ")}
                </Code>
                <p>
                    "Leptonic also ships an optional atom theme that styles the default classes, for apps that don\u{2019}t "
                    "want to start from scratch: "<Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>"."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    "In a menu, put the keys into the item\u{2019}s "
                    <Link href=format!("{}#menuitemshortcut", routes::doc::menu::Atom.materialize())>
                        <Code inline=true>"MenuItemShortcut"</Code>
                    </Link>", which announces them with the item:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        <atoms::menu::MenuItem key="copy">
                            <atoms::menu::MenuItemLabel>"Copy"</atoms::menu::MenuItemLabel>
                            <atoms::menu::MenuItemShortcut>
                                <atoms::kbd::ShortcutKeys shortcut=Shortcut::new(KeyboardKey::C).primary() classes="my-keys"/>
                            </atoms::menu::MenuItemShortcut>
                        </atoms::menu::MenuItem>
                    "#)}
                </Code>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "Keys are "<Code inline=true>"<kbd>"</Code>" elements, HTML\u{2019}s element for keyboard input. A "
                        "combination is a "<Code inline=true>"<kbd>"</Code>" holding one "<Code inline=true>"<kbd>"</Code>
                        " per key."
                    </li>
                    <li>
                        "Screen readers read a "<Code inline=true>"<kbd>"</Code>" as its text. Where a key\u{2019}s label is "
                        "a glyph or an abbreviation they would read wrongly (\u{2318}, \u{21e7}, PgUp), the atoms hide the "
                        "label from them and add the key\u{2019}s spoken name in visually hidden text (\u{201c}Command\u{201d}, "
                        "\u{201c}Shift\u{201d}, \u{201c}Page Up\u{201d}). The "<Code inline=true>"+"</Code>" between keys is "
                        "hidden from them."
                    </li>
                    <li>
                        "Key caps are text: they are not focusable and have no keyboard interaction. To tell assistive "
                        "technology about a shortcut, set "<Code inline=true>"aria-keyshortcuts"</Code>" on the element it "
                        "triggers (see "
                        <Link href=format!("{}#accessibility", routes::doc::interactions::UseGlobalShortcuts.materialize())>
                            "use_global_shortcuts"
                        </Link>")."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::interactions::UseGlobalShortcuts.materialize()>"use_global_shortcuts"</Link></li>
                <li><Link href=format!("{}#shortcut", routes::doc::interactions::UseKeyboard.materialize())>"Shortcut"</Link></li>
                <li><Link href=routes::doc::menu::Atom.materialize()>"Menu Atoms"</Link></li>
                <li><Link href=format!("{}#typography", routes::doc::Layout.materialize())>"Typography"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
