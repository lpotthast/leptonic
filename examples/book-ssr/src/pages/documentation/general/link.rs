use indoc::{formatdoc, indoc};
use leptonic::prelude::*;
use leptos::*;
use leptos_router::ToHref;

use crate::pages::documentation::doc_root::DocRoutes;

#[component]
pub fn PageLink() -> impl IntoView {
    view! {
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
                    <Icon id="github-icon" icon=icondata::BsGithub style="font-size: 3em;"/>
                </LinkExt>
            "#)}
        </Code>

        <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank>
            <Icon id="github-icon" icon=icondata::BsGithub style="font-size: 3em;"/>
        </LinkExt>

        <H2>"Link Buttons"</H2>

        <P>"It is likely that you want to render a link in the form of a button. Please respect the HTML standard and do not render a <Button> inside a <Link>."</P>

        <P>"Use the "<Code inline=true>"<LinkButton>"</Code>" component, which accepts most props from both the <Link> as well as the <Button> component."</P>

        <P>"The \"Read the docs\" button on the welcome page was implemented this way!"</P>

        <Code>
            {indoc!(r#"
                <LinkButton href=DocRoutes::Overview>
                    "Read the docs"
                </LinkButton>
            "#)}
        </Code>

        <LinkButton href=DocRoutes::Overview>
            "Read the docs"
        </LinkButton>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r"
                --link-color
            ")}
        </Code>
    }
}
