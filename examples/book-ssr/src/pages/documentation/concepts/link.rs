use leptos::prelude::*;

use super::demos::link::LinkConceptDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageLinkOverview() -> impl IntoView {
    view! {
        <DocPage title="Link">
            <p>
                "Links take users to another page, another site or another place on the current page. Leptonic has "
                <Code inline=true>"Link"</Code>" for pages of your app and other sites (also styled as a button) and "<Code inline=true>"AnchorLink"</Code>" for scrolling to a "
                "section of the current page."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Navigate to another page of your app"</TableCell><TableCell><b>"Link"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Navigate to another site"</TableCell>
                        <TableCell><b>"Link"</b>" with a URL (and "<Code inline=true>"LinkTarget::Blank"</Code>")"</TableCell>
                    </TableRow>
                    <TableRow><TableCell>"Scroll to a section of the current page"</TableCell><TableCell><b>"AnchorLink"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Navigate with an element that looks like a button"</TableCell>
                        <TableCell><b>"Link"</b>" with the styles of your buttons"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Trigger an action (submit, delete)"</TableCell>
                        <TableCell><Link href=routes::doc::Button.materialize()>"Button"</Link></TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "If it navigates, use a link. If it triggers an action, use a button \u{2014} regardless of how the "
                    "element looks."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Links exist as hooks and as atoms. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::link::Hook.materialize()>"Link Hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_link"</Code>": link behavior and ARIA attributes for any element you render, "
                            "including non-anchor elements. "<Code inline=true>"use_anchor_link"</Code>": an in-page link that "
                            "scrolls to its target and updates the URL fragment."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::link::Atom.materialize()>"Link Atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"Link"</Code>" and "<Code inline=true>"AnchorLink"</Code>", styled through "
                            "data attributes. "<Code inline=true>"Link"</Code>" navigates with leptos_router and marks the current page."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The "<Code inline=true>"Link"</Code>" atom to another site, opening in a new tab. Its class is the "
                    "book\u{2019}s own; the "<Link href=format!("{}#styling", routes::doc::link::Atom.materialize())>"styling section"</Link>
                    " of the atoms shows its CSS:"
                </p>

                <Demo description="External link to the leptonic repository" source=include_str!("demos/link.rs") source_open=true>
                    <LinkConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "A link is announced as a link and followed with "<Keys keys="Enter"/>". Native "<Code inline=true>"<a>"</Code>
                    " elements bring this with them; all layers keep it and add:"
                </p>

                <ul>
                    <li>
                        <Code inline=true>"role=\"link\""</Code>" and "<Code inline=true>"tabindex=\"0\""</Code>" on elements other "
                        "than "<Code inline=true>"<a>"</Code>", so they are announced and reached like links."
                    </li>
                    <li>
                        "Disabled links lose their "<Code inline=true>"href"</Code>", leave the tab order and get "
                        <Code inline=true>"aria-disabled=\"true\""</Code>"."
                    </li>
                    <li>
                        <Code inline=true>"aria-current=\"page\""</Code>" on the link to the current page ("
                        <Code inline=true>"Link"</Code>")."
                    </li>
                    <li>
                        <Code inline=true>"rel=\"noopener\""</Code>" on links opening a new tab, so the opened page can\u{2019}t "
                        "reach yours."
                    </li>
                    <li>
                        "A link without text, such as an icon or a bare "<Code inline=true>"#"</Code>", needs an "
                        <Code inline=true>"aria_label"</Code>"."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus to the link."</KeyRow>
                    <KeyRow keys="Enter">"Follows the link. Unlike buttons, links don\u{2019}t react to Space."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::link::Hook.materialize()>"Link Hooks"</Link></li>
                <li><Link href=routes::doc::link::Atom.materialize()>"Link Atoms"</Link></li>
                <li><Link href=routes::doc::Breadcrumbs.materialize()>"Breadcrumbs"</Link></li>
                <li><Link href=routes::doc::Button.materialize()>"Button"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
