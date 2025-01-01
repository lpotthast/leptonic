use crate::app::SegmentRenderer;
use crate::pages::documentation::article::Article;
use crate::pages::documentation::doc_root::doc_routes;
use crate::pages::documentation::toc::Toc;
use indoc::{formatdoc, indoc};
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::prelude::*;
use leptos::prelude::*;

#[component]
pub fn PageLink() -> impl IntoView {
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

            <Code>
                {formatdoc!(r#"
                    <Link href="{}">
                        "This is a link to the current route."
                    </Link>
                "#, doc_routes::LINK.to_href())}
            </Code>

            <Link href=doc_routes::LINK.to_href()>"This is a link to the current route."</Link>

            <h2 id="external-links" class="anchor">
                "External links"
                <AnchorLink href="#external-links" description="Direct link to section: External links"/>
            </h2>

            <p>"These links, created with the "<Code inline=true>"<LinkExt>"</Code>" component, do not use the leptos router and must be used when directing users to external sources."</p>

            <Code>
                {indoc!(r#"
                    <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank>
                        <Icon id="github-icon" icon=icondata::BsGithub style="font-size: 3em;"/>
                    </LinkExt>
                "#)}
            </Code>

            <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank>
                <Icon attr:id="github-icon" icon=icondata::BsGithub attr:style="font-size: 3em;"/>
            </LinkExt>

            <h2 id="link-buttons" class="anchor">
                "Link Buttons"
                <AnchorLink href="#link-buttons" description="Direct link to section: Link Buttons"/>
            </h2>

            <p>"It is likely that you want to render a link in the form of a button. Please respect the HTML standard and do not render a <Button> inside a <Link>."</p>

            <p>"Use the "<Code inline=true>"<LinkButton>"</Code>" component, which accepts most props from both the <Link> as well as the <Button> component."</p>

            <p>"The \"Read the docs\" button on the welcome page was implemented this way!"</p>

            <Code>
                {indoc!(r#"
                    <LinkButton href=DocRoutes::Overview>
                        "Read the docs"
                    </LinkButton>
                "#)}
            </Code>

            <LinkButton href=doc_routes::OVERVIEW.to_href()>
                "Read the docs"
            </LinkButton>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code>
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
                Toc::Leaf { title: "Link buttons", link: "#link-buttons" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
