use indoc::indoc;
use leptos::prelude::*;

use super::demos::shortcut_keys::ShortcutKeysDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomKbd() -> impl IntoView {
    view! {
        <DocPage title="Kbd Atom">
            <p>
                "The unstyled "<Code inline=true>"ShortcutKeys"</Code>" atom shows a keyboard "<Code inline=true>"Shortcut"</Code>
                " as keys, in the form of the user\u{2019}s platform. See the "
                <Link href=routes::doc::Kbd.materialize()>"Kbd overview"</Link>" for concept guidance."
            </p>

            <Section title="Props">
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

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{atoms::prelude as atoms, utils::keyboard_shortcut::Shortcut};
                        use leptos::prelude::*;

                        const SEARCH: Shortcut = Shortcut::key("k").primary();

                        view! {
                            // "Ctrl + K", or "⌘K" on Apple devices.
                            <p>"Search with "<atoms::ShortcutKeys shortcut=SEARCH classes="my-shortcut"/>"."</p>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="Four shortcuts with modifiers, arrow keys and Escape, in the form of your platform" source=include_str!("demos/shortcut_keys.rs")>
                    <ShortcutKeysDemo/>
                </Demo>
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

            <Section title="Structure">
                <p>
                    "The atom renders one "<Code inline=true>"<kbd>"</Code>" holding a "<Code inline=true>"<kbd>"</Code>
                    " per key, as HTML writes a key combination. A key shown as a glyph or an abbreviation hides it from "
                    "screen readers and adds its name in visually hidden text. "<Keys keys="Control + K"/>" renders as:"
                </p>
                <Code language=Language::Html>
                    {indoc!(r#"
                        <kbd class="my-shortcut">
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
                        "On the "<Code inline=true>"+"</Code>" between two keys, which only the generic form has. Hidden "
                        "from screen readers."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atom adds no classes. Style the outer "<Code inline=true>"<kbd>"</Code>" through your class, the "
                    "keys as "<Code inline=true>"kbd kbd"</Code>" and the separators by their attribute:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-shortcut { display: inline-flex; align-items: center; gap: 0.25em; font: inherit; }
                        .my-shortcut kbd { padding: 0 0.4em; border: 1px solid var(--border); border-radius: 4px; background: var(--surface); }
                        .my-shortcut [data-separator] { color: var(--muted); }
                    ")}
                </Code>
            </Section>

            <Section title="Shortcut::keys">
                <p>
                    <Code inline=true>"shortcut.keys(apple: bool) -> Vec<KeyboardKey>"</Code>" returns the keys the atom "
                    "shows: the modifiers in the order of the platform, then the key. Use it to render the keys yourself, "
                    "e.g. with the themed "<Code inline=true>"KbdKey"</Code>" of the "
                    <Link href=routes::doc::kbd::Component.materialize()>"Kbd Components"</Link>". Pass "
                    <Code inline=true>"false"</Code>" on the server and during hydration, as the atom does, so that the "
                    "markup matches:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{components::prelude::*, utils::keyboard_shortcut::Shortcut};
                        use leptos::prelude::*;

                        // [Control, Shift, Z]; with `true`: [Shift, Command, Z].
                        let keys = Shortcut::key("z").primary().shift().keys(false);

                        view! { <span>{keys.into_iter().map(|key| view! { <KbdKey key/> }).collect_view()}</span> }
                    "#)}
                </Code>
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
                        <atoms::MenuItem key="copy">
                            <atoms::MenuItemLabel>"Copy"</atoms::MenuItemLabel>
                            <atoms::MenuItemShortcut>
                                <atoms::ShortcutKeys shortcut=Shortcut::key("c").primary() classes="my-shortcut"/>
                            </atoms::MenuItemShortcut>
                        </atoms::MenuItem>
                    "#)}
                </Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Kbd.materialize()>"Kbd overview"</Link></li>
                <li><Link href=routes::doc::kbd::Component.materialize()>"Kbd Components"</Link></li>
                <li><Link href=routes::doc::interactions::UseGlobalShortcuts.materialize()>"use_global_shortcuts"</Link></li>
                <li><Link href=format!("{}#shortcut", routes::doc::interactions::UseKeyboard.materialize())>"Shortcut"</Link></li>
                <li><Link href=routes::doc::menu::Atom.materialize()>"Menu Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
