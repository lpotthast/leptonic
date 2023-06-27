use indoc::{formatdoc, indoc};
use leptonic::prelude::*;
use leptos::*;
use leptos_icons::BsIcon;
use leptos_router::ToHref;

use crate::pages::documentation::doc_root::DocRoutes;

#[component]
pub fn PageLink(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"Links"</H1>

        <P>"Links bring your users to a different place of your application."</P>

        <H2>"Internal links"</H2>

        <P>"These links, created with the "<Code inline=true>"<Link>"</Code>" component, use the leptos router under the hood and are meant to direct users to a different location inside your app, as the given "<Code inline=true>"href"</Code>" prop is always considered to be relative to your site."</P>

        <Code>
            {formatdoc!(r#"
                <Link href="{}">
                    "This is a link to the current route."
                </Link>
            "#, DocRoutes::Link.to_href()())}
        </Code>

        <Link href=DocRoutes::Link>"This is a link to the current route."</Link>

        <H2>"External links"</H2>

        <P>"These links, created with the "<Code inline=true>"<LinkExt>"</Code>" component, do not use the leptos router and must be used when directing users to external sources."</P>

        <Code>
            {indoc!(r#"
                <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank>
                    <Icon id="github-icon" icon=BsIcon::BsGithub style="font-size: 3em;"/>
                </LinkExt>
            "#)}
        </Code>

        <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank>
            <Icon id="github-icon" icon=BsIcon::BsGithub style="font-size: 3em;"/>
        </LinkExt>

        <H2>"Children"</H2>

        <P>"As you have already seen, links of any form can contain arbitrary children and are not restraint to simple text nodes. Everything can be a link."</P>

        <P>"Wonder how the routing behavior of the \"Read the docs\" button on the welcome page was implemented?"</P>

        <Code>
            {indoc!(r#"
                <Link href=DocRoutes::Overview>
                    <Button on_click=move |_| {}>
                        "Read the docs"
                    </Button>
                </Link>
            "#)}
        </Code>

        <Link href=DocRoutes::Overview>
            <Button on_click=move |_| {}>
                "Read the docs"
            </Button>
        </Link>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --link-color
            "#)}
        </Code>
    }
}
