use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    anchor_link_heading::AnchorLinkHeadingDemo, anchor_link_simple::AnchorLinkSimpleDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageAtomAnchorLink() -> impl IntoView {
    view! {
        <DocPage title="AnchorLink atom">
            <p>
                "The "<Code inline=true>"AnchorLink"</Code>" component links to an element on the current page by its id "
                "and scrolls to it when pressed. See the "<Link href=routes::doc::Link.materialize()>"Link overview"</Link>
                " for concept guidance."
            </p>

            <p>
                "The unstyled atom lives in "<Code inline=true>"leptonic::atoms::link"</Code>". The themed component in "
                <Code inline=true>"leptonic::components::prelude"</Code>" adds the "<Code inline=true>"leptonic-anchor-link"</Code>
                " class and renders a single "<Code inline=true>"#"</Code>" when you give it no children. The demos use the component."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Link href=routes::doc::link::UseAnchorLink.materialize()><Code inline=true>"use_anchor_link"</Code></Link>
                    ", which scrolls to the target and updates the URL hash without adding a history entry."
                </p>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="atoms::link::AnchorLink">
                    <ApiRow name="href" ty="Href">
                        "The id of the element to link to: "<Code inline=true>"\"#my-section\""</Code>" or "
                        <Code inline=true>"\"my-section\""</Code>" (the "<Code inline=true>"#"</Code>" is optional). "
                        "Converts from "<Code inline=true>"&'static str"</Code>", "<Code inline=true>"String"</Code>" and "
                        <Code inline=true>"Oco"</Code>". Required."
                    </ApiRow>
                    <ApiRow name="scroll_behavior" ty="Option<ScrollBehavior>" default="Smooth">
                        "How to scroll to the target: "<Code inline=true>"Smooth"</Code>" or "<Code inline=true>"Instant"</Code>"."
                    </ApiRow>
                    <ApiRow name="description" ty="Option<Oco<'static, str>>" default="None">
                        "Accessible name of the link. Set it when the link has no descriptive text, such as the default "
                        <Code inline=true>"#"</Code>"."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<a>"</Code>"."</ApiRow>
                    <ApiRow name="children" ty="Children">
                        "The link content. The component makes it optional ("<Code inline=true>"Option<Children>"</Code>
                        ") and falls back to "<Code inline=true>"#"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r##"
                        view! {
                            <AnchorLink href="#installation">"Jump to the installation"</AnchorLink>
                        }
                    "##)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Anchor links are most common in headings; this documentation uses them as well. Give the heading an id "
                    "and point the link at it. Without children the link renders a single "<Code inline=true>"#"</Code>
                    ", so describe it with "<Code inline=true>"description"</Code>":"
                </p>

                <Demo description="Heading with an anchor link" source=include_str!("demos/anchor_link_heading.rs") source_open=true>
                    <AnchorLinkHeadingDemo/>
                </Demo>

                <p>
                    "Anchor links work anywhere. With descriptive text, you don\u{2019}t need a description. This link jumps "
                    "to the heading above:"
                </p>

                <Demo description="Text anchor link to the heading above" source=include_str!("demos/anchor_link_simple.rs") source_open=true>
                    <AnchorLinkSimpleDemo/>
                </Demo>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-focus-visible" ty="true">"Present while the link has keyboard focus."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>"The themed component reads these CSS variables:"</p>

                <CssVariables prefix="--link-" scss=theme_scss!("link")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Link.materialize()>"Link overview"</Link></li>
                <li><Link href=routes::doc::link::UseAnchorLink.materialize()>"use_anchor_link"</Link></li>
                <li><Link href=routes::doc::link::LinkAtom.materialize()>"Link atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
