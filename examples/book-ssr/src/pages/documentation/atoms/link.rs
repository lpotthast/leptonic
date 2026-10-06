use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    link_button::LinkButtonDemo, link_external::LinkExternalDemo, link_internal::LinkInternalDemo,
    link_rel::LinkRelDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageAtomLink() -> impl IntoView {
    view! {
        <DocPage title="Link atom">
            <p>
                "The "<Code inline=true>"Link"</Code>" and "<Code inline=true>"LinkExt"</Code>" components take users to "
                "another page of your app or to another site. See the "
                <Link href=routes::doc::Link.materialize()>"Link overview"</Link>" for concept guidance."
            </p>

            <p>
                "Both exist as unstyled atoms in "<Code inline=true>"leptonic::atoms::link"</Code>" and as themed components "
                "in "<Code inline=true>"leptonic::components::prelude"</Code>". The components add the "
                <Code inline=true>"leptonic-link"</Code>" class and take the same props. The demos use the components."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Link href=routes::doc::link::UseLink.materialize()><Code inline=true>"use_link"</Code></Link>
                    ", which composes "<Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>", "
                    <Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link>" and "
                    <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>"."
                </p>
            </Section>

            <Section title="Link">
                <p>
                    "A link inside your app. It renders leptos_router\u{2019}s "<Code inline=true>"<A>"</Code>
                    ", so navigation happens on the client and "<Code inline=true>"href"</Code>
                    " is resolved relative to the current route."
                </p>

                <ApiTable kind=ApiKind::Props of="atoms::link::Link">
                    <ApiRow name="href" ty="impl ToHref">"The link target."</ApiRow>
                    <ApiRow name="exact" ty="bool" default="false">
                        "Whether the link is only marked as active ("<Code inline=true>"aria-current=\"page\""</Code>
                        ") when the location matches exactly, instead of when it starts with the target."
                    </ApiRow>
                    <ApiRow name="on_press" ty="Option<Callback<PressEvent>>" default="None">"Called when the link is pressed."</ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<a>"</Code>"."</ApiRow>
                    <ApiRow name="children" ty="Children">"The link content."</ApiRow>
                </ApiTable>

                <Demo description="Link to the current page" source=include_str!("demos/link_internal.rs")>
                    <LinkInternalDemo/>
                </Demo>
            </Section>

            <Section title="LinkExt">
                <p>"A link to another site. It renders a plain "<Code inline=true>"<a>"</Code>" without the router."</p>

                <ApiTable kind=ApiKind::Props of="atoms::link::LinkExt">
                    <ApiRow name="href" ty="impl ToHref">"The link target."</ApiRow>
                    <ApiRow name="target" ty="LinkTarget">
                        "Where to open the link, e.g. "<Code inline=true>"LinkTarget::_Blank"</Code>" for a new tab. Required."
                    </ApiRow>
                    <ApiRow name="rel" ty="Vec<LinkRel>" default="vec![]">
                        "Values of the "<Code inline=true>"rel"</Code>" attribute. With "<Code inline=true>"LinkTarget::_Blank"</Code>
                        ", "<Code inline=true>"LinkRel::NoOpener"</Code>" is added automatically."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the link is disabled."</ApiRow>
                    <ApiRow name="on_press" ty="Option<Callback<PressEvent>>" default="None">"Called when the link is pressed."</ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<a>"</Code>"."</ApiRow>
                    <ApiRow name="children" ty="Children">"The link content."</ApiRow>
                </ApiTable>

                <Demo description="Icon link to GitHub opening in a new tab" source=include_str!("demos/link_external.rs")>
                    <LinkExternalDemo/>
                </Demo>

                <Section title="Rel Attribute">
                    <p>
                        <Code inline=true>"rel"</Code>" describes the relationship to the linked page, for example "
                        <Code inline=true>"LinkRel::NoFollow"</Code>" for links search engines should not follow:"
                    </p>

                    <Demo description="External link with nofollow and noreferrer" source=include_str!("demos/link_rel.rs")>
                        <LinkRelDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::link::{Link, LinkExt};

                        view! {
                            <Link href="/settings" classes="my-link">"Settings"</Link>
                            <LinkExt href="https://leptos.dev" target=LinkTarget::_Blank classes="my-link">
                                "Leptos"
                            </LinkExt>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-focus-visible" ty="true">"Present while the link has keyboard focus."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "Style the atoms with "<Code inline=true>"classes"</Code>". Target "<Code inline=true>"[data-focus-visible]"</Code>
                    " for a focus ring, "<Code inline=true>"[aria-current=\"page\"]"</Code>" for the active "
                    <Code inline=true>"Link"</Code>" and "<Code inline=true>"[aria-disabled=\"true\"]"</Code>" for a disabled "
                    <Code inline=true>"LinkExt"</Code>". The themed components read these CSS variables:"
                </p>

                <CssVariables prefix="--link-" scss=theme_scss!("link")/>
            </Section>

            <Section title="LinkButton">
                <p>
                    "A link that looks like a button. Don\u{2019}t put a "<Code inline=true>"Button"</Code>" inside a "
                    <Code inline=true>"Link"</Code>"; HTML doesn\u{2019}t allow interactive content inside links. Use "
                    <Code inline=true>"LinkButton"</Code>" instead: it takes "<Code inline=true>"href"</Code>", "
                    <Code inline=true>"target"</Code>" and "<Code inline=true>"exact"</Code>" like a link and the variant, "
                    "color, size and disabled props of the "<Link href=routes::doc::button::Component.materialize()>"Button component"</Link>
                    ". The \u{201c}Read the docs\u{201d} button on the welcome page is one."
                </p>

                <Demo description="Link styled as a button" source=include_str!("demos/link_button.rs")>
                    <LinkButtonDemo/>
                </Demo>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Link.materialize()>"Link overview"</Link></li>
                <li><Link href=routes::doc::link::UseLink.materialize()>"use_link"</Link></li>
                <li><Link href=routes::doc::link::AnchorLinkAtom.materialize()>"AnchorLink atom"</Link></li>
                <li><Link href=routes::doc::button::Atom.materialize()>"Button atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
