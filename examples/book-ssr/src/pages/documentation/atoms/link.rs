use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    link_button::LinkButtonDemo, link_external::LinkExternalDemo, link_internal::LinkInternalDemo,
    link_rel::LinkRelDemo,
};
use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

#[component]
pub fn PageAtomLink() -> impl IntoView {
    view! {
        <Article>
            <h1 id="link" class="anchor">
                "Link"
                <AnchorLink href="#link" description="Direct link to article header"/>
            </h1>

            <p>"Links bring your users to a different place of your application."</p>

            <h2 id="internal-links" class="anchor">
                "Internal links"
                <AnchorLink href="#internal-links" description="Direct link to section: Internal links"/>
            </h2>

            <p>"These links, created with the "<Code inline=true>"<Link>"</Code>" component, use the leptos router under the hood and are meant to direct users to a different location inside your app, as the given "<Code inline=true>"href"</Code>" prop is always considered to be relative to your site."</p>

            <DemoShell source=include_str!("demos/link_internal.rs")>
                <LinkInternalDemo />
            </DemoShell>

            <h2 id="external-links" class="anchor">
                "External links"
                <AnchorLink href="#external-links" description="Direct link to section: External links"/>
            </h2>

            <p>"These links, created with the "<Code inline=true>"<LinkExt>"</Code>" component, do not use the leptos router and must be used when directing users to external sources."</p>

            <DemoShell source=include_str!("demos/link_external.rs")>
                <LinkExternalDemo />
            </DemoShell>

            <h2 id="rel-attribute" class="anchor">
                "Rel Attribute"
                <AnchorLink href="#rel-attribute" description="Direct link to section: Rel Attribute"/>
            </h2>

            <p>
                "The "<Code inline=true>"rel"</Code>" prop accepts a "<Code inline=true>"Vec<LinkRel>"</Code>
                " to set semantic relationship values on the link. "
                "When "<Code inline=true>"target=LinkTarget::_Blank"</Code>", "
                <Code inline=true>"LinkRel::NoOpener"</Code>" is automatically added for security."
            </p>

            <DemoShell source=include_str!("demos/link_rel.rs")>
                <LinkRelDemo />
            </DemoShell>

            <h2 id="link-buttons" class="anchor">
                "Link Buttons"
                <AnchorLink href="#link-buttons" description="Direct link to section: Link Buttons"/>
            </h2>

            <p>"It is likely that you want to render a link in the form of a button. Please respect the HTML standard and do not render a <Button> inside a <Link>."</p>

            <p>"Use the "<Code inline=true>"<LinkButton>"</Code>" component, which accepts most props from both the <Link> as well as the <Button> component."</p>

            <p>"The \"Read the docs\" button on the welcome page was implemented this way!"</p>

            <DemoShell source=include_str!("demos/link_button.rs")>
                <LinkButtonDemo />
            </DemoShell>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    --link-color
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Link", link: "#link" },
                Toc::Leaf { title: "Internal links", link: "#internal-links" },
                Toc::Leaf { title: "External links", link: "#external-links" },
                Toc::Leaf { title: "Rel Attribute", link: "#rel-attribute" },
                Toc::Leaf { title: "Link buttons", link: "#link-buttons" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
