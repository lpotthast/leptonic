use leptos::prelude::*;

use super::demos::separator::SeparatorConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageSeparatorOverview() -> impl IntoView {
    view! {
        <DocPage title="Separator">
            <p>
                "A separator is a line dividing sections of content or groups of controls, such as the items of a menu "
                "or the button groups of a toolbar. Unlike a border or spacing, it carries meaning: screen readers "
                "announce it as a boundary."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Mark a semantic boundary between content sections"</TableCell><TableCell><b>"Separator"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Add visual spacing without semantic meaning"</TableCell><TableCell>"CSS margin / padding"</TableCell></TableRow>
                    <TableRow><TableCell>"Add a decorative line"</TableCell><TableCell>"CSS border"</TableCell></TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Separators exist at all three layers. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::separator::Hook.materialize()>"use_separator"</Link></TableCell>
                        <TableCell>
                            "Separator semantics for any element, e.g. a vertical "<Code inline=true>"<div>"</Code>
                            " between toolbar groups."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::separator::Atom.materialize()>"Separator Atom"</Link></TableCell>
                        <TableCell>"An unstyled separator in the right element for its orientation (and inside menus)."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>"The themed component draws the line for you:"</p>

                <Demo description="Horizontal separator between two paragraphs" source=include_str!("demos/separator.rs") source_open=true>
                    <SeparatorConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Separators follow the WAI-ARIA "
                    <Link href="https://www.w3.org/TR/wai-aria-1.2/#separator" target=LinkTarget::Blank>"separator role"</Link>
                    ". They are not focusable and have no keyboard interaction."
                </p>

                <ul>
                    <li>
                        "A horizontal separator is an "<Code inline=true>"<hr>"</Code>", which has the role implicitly. "
                        "Other elements get "<Code inline=true>"role=\"separator\""</Code>"."
                    </li>
                    <li>
                        "Vertical separators get "<Code inline=true>"aria-orientation=\"vertical\""</Code>". Horizontal is "
                        "the role\u{2019}s default orientation, so horizontal separators carry no "
                        <Code inline=true>"aria-orientation"</Code>"."
                    </li>
                    <li>
                        "Name a separator ("<Code inline=true>"aria_label"</Code>") only where the boundary itself needs "
                        "explaining; most separators need no name."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::separator::Hook.materialize()>"use_separator"</Link></li>
                <li><Link href=routes::doc::separator::Atom.materialize()>"Separator Atom"</Link></li>
                <li><Link href=routes::doc::Toolbar.materialize()>"Toolbar"</Link></li>
                <li><Link href=routes::doc::Menu.materialize()>"Menu"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
