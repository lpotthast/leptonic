use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::separator::SeparatorConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageSeparatorOverview() -> impl IntoView {
    view! {
        <DocPage title="Separator">
            <p>
                "Separators are visual dividers that create clear boundaries between sections of content. "
                "They carry semantic meaning \u{2014} assistive technology announces them as thematic breaks, "
                "distinguishing them from purely decorative borders or spacing."
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
                    "Separators exist as a hook and as a component. See "
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
                        <TableCell><Link href=routes::doc::separator::Component.materialize()>"Separator component"</Link></TableCell>
                        <TableCell>"A themed horizontal "<Code inline=true>"<hr>"</Code>"."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>"The component is the quickest way to a separator:"</p>

                <Demo description="Horizontal separator between two paragraphs" source=include_str!("demos/separator.rs") source_open=true>
                    <SeparatorConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Separators follow the WAI-ARIA "
                    <LinkExt href="https://www.w3.org/TR/wai-aria-1.2/#separator" target=LinkTarget::_Blank>"separator role"</LinkExt>
                    ". They are not focusable and have no keyboard interaction."
                </p>

                <ul>
                    <li>
                        <Code inline=true>"role=\"separator\""</Code>" is set on elements other than "<Code inline=true>"<hr>"</Code>
                        "; a native "<Code inline=true>"<hr>"</Code>" has the role implicitly."
                    </li>
                    <li>
                        <Code inline=true>"aria-orientation"</Code>" (\u{201c}horizontal\u{201d} or \u{201c}vertical\u{201d}) "
                        "is set alongside the role. Horizontal is the default."
                    </li>
                </ul>
            </Section>
        </DocPage>
    }
}
