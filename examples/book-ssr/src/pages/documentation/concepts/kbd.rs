use leptos::prelude::*;

use super::demos::kbd::KbdConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageKbdOverview() -> impl IntoView {
    view! {
        <DocPage title="Kbd">
            <p>
                "Key caps show keys and keyboard shortcuts in text: in help texts, in menus or next to the actions they "
                "trigger, so that users learn them. They only display keys. Handling the key presses is up to "
                <Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>" (keys pressed on "
                "one element) and "<Link href=routes::doc::interactions::UseGlobalShortcuts.materialize()>"use_global_shortcuts"</Link>
                " (shortcuts of the whole page)."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Show a shortcut your app handles, written as the user\u{2019}s platform writes it"</TableCell>
                        <TableCell><b><Code inline=true>"ShortcutKeys"</Code></b>" (atom)"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a key or a combination of keys with the theme\u{2019}s key caps"</TableCell>
                        <TableCell><b><Code inline=true>"KbdKey"</Code>", "<Code inline=true>"KbdShortcut"</Code></b>" (components)"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show the shortcut of a menu item"</TableCell>
                        <TableCell>
                            "Keys inside the item\u{2019}s "
                            <Link href=format!("{}#menuitemshortcut", routes::doc::menu::Atom.materialize())>
                                <Code inline=true>"MenuItemShortcut"</Code>
                            </Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"React to key presses"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>", "
                            <Link href=routes::doc::interactions::UseGlobalShortcuts.materialize()>"use_global_shortcuts"</Link>
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Key caps exist as an atom and as components. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::kbd::Atom.materialize()>"Kbd Atom"</Link></TableCell>
                        <TableCell>
                            "Unstyled keys of a "<Code inline=true>"Shortcut"</Code>", in the form of the user\u{2019}s "
                            "platform: \u{201c}Ctrl + K\u{201d}, or \u{201c}\u{2318}K\u{201d} on Apple devices."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::kbd::Component.materialize()>"Kbd Components"</Link></TableCell>
                        <TableCell>"Themed key caps for the keys you name, the same on every platform."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>"The themed components show keys in a sentence:"</p>

                <Demo description="Sentence with the key Escape and the shortcut Control + Enter" source=include_str!("demos/kbd.rs") source_open=true>
                    <KbdConceptDemo/>
                </Demo>
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
                        "a glyph or an abbreviation they would read wrongly (\u{2318}, \u{21e7}, PgUp), both layers hide the "
                        "label from them and add the key\u{2019}s spoken name in visually hidden text (\u{201c}Command\u{201d}, "
                        "\u{201c}Shift\u{201d}, \u{201c}Page Up\u{201d})."
                    </li>
                    <li>
                        "The atom hides the "<Code inline=true>"+"</Code>" between keys from screen readers; the "
                        "components\u{2019} separators are read."
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
                <li><Link href=routes::doc::kbd::Atom.materialize()>"Kbd Atom"</Link></li>
                <li><Link href=routes::doc::kbd::Component.materialize()>"Kbd Components"</Link></li>
                <li><Link href=routes::doc::interactions::UseGlobalShortcuts.materialize()>"use_global_shortcuts"</Link></li>
                <li><Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link></li>
                <li><Link href=routes::doc::Typography.materialize()>"Typography"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
