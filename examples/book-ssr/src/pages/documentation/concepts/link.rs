use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::link::LinkConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageLinkOverview() -> impl IntoView {
    view! {
        <DocPage title="Link">
            <p>
                "Links take users to another page, another site or another place on the current page. Leptonic has "
                <Code inline=true>"Link"</Code>" for pages of your app, "<Code inline=true>"LinkExt"</Code>
                " for other sites and "<Code inline=true>"AnchorLink"</Code>" for scrolling to a section of the current page."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Navigate to another page of your app"</TableCell><TableCell><b>"Link"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Navigate to another site"</TableCell><TableCell><b>"LinkExt"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Scroll to a section of the current page"</TableCell><TableCell><b>"AnchorLink"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Trigger an action (submit, delete)"</TableCell><TableCell><Link href=routes::doc::Button.materialize()>"Button"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Navigate with an element that looks like a button"</TableCell><TableCell><Code inline=true>"LinkButton"</Code></TableCell></TableRow>
                </DocTable>

                <p>
                    "If it navigates, use a link. If it triggers an action, use a button \u{2014} regardless of how the "
                    "element looks."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Links exist as hooks and as atoms; the atoms also come as themed components. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::link::UseLink.materialize()>"use_link"</Link></TableCell>
                        <TableCell>"Link behavior and ARIA attributes for any element you render, including non-anchor elements."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::link::UseAnchorLink.materialize()>"use_anchor_link"</Link></TableCell>
                        <TableCell>"An in-page link that scrolls to its target and updates the URL hash."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::link::LinkAtom.materialize()>"Link atom"</Link></TableCell>
                        <TableCell><Code inline=true>"Link"</Code>", "<Code inline=true>"LinkExt"</Code>" and "<Code inline=true>"LinkButton"</Code>", unstyled or themed."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::link::AnchorLinkAtom.materialize()>"AnchorLink atom"</Link></TableCell>
                        <TableCell>"An anchor link, unstyled or themed."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>"A link to another site, opening in a new tab:"</p>

                <Demo description="External link to the leptonic repository" source=include_str!("demos/link.rs") source_open=true>
                    <LinkConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Native "<Code inline=true>"<a>"</Code>" elements bring link semantics with them. When you render a "
                    "link as another element, "<Code inline=true>"use_link"</Code>" adds what is missing."
                </p>

                <ul>
                    <li><Code inline=true>"role=\"link\""</Code>" on elements other than "<Code inline=true>"<a>"</Code>"."</li>
                    <li><Code inline=true>"aria-disabled=\"true\""</Code>" while the link is disabled."</li>
                    <li><Code inline=true>"aria-current"</Code>" marks the link to the current page in a navigation."</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus to the link."</KeyRow>
                    <KeyRow keys="Enter">"Activates the link."</KeyRow>
                </KeyboardTable>
            </Section>
        </DocPage>
    }
}
