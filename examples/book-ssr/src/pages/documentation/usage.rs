use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

use crate::pages::documentation::doc_root::DocRoutes;

#[component]
pub fn PageUsage() -> impl IntoView {
    view! {
        <H1>"Usage"</H1>

        <P>"Similar to Leptos, this crate comes with a prelude module."</P>

        <P>
            "Just "
            <Code inline=true>"use leptonic::prelude::*;"</Code>
            " and you are almost ready to go."
        </P>

        <P>
            "Leptonic provides the "<Code inline=true>"<Root>"</Code>" component. "
            "It is responsible for enabling the "<Link href=DocRoutes::Themes>"Theming"</Link>", "<Link href=DocRoutes::Modal>"Modal"</Link>" and "
            <Link href=DocRoutes::Toast>"Toast"</Link>" functionality of Leptonic as well as providing global event-listening capabilities."
            "You have to include it in your app once, and render all your content inside it. Many Leptonic components require the "<Code inline=true>"<Root>"</Code>" component to be present."
        </P>

        <P>
            "Using the component is as simple as this:"
        </P>

        <Code>
            {indoc!(r#"
                #[component]
                pub fn App() -> impl IntoView {
                    view! {
                        <Root default_theme=LeptonicTheme::default()>
                            "Content goes here :)"
                        </Root>
                    }
                }
            "#)}
        </Code>

        <H2>"Need help?"</H2>

        <P>
            "If you get stuck at any point integrating or using Leptonic, these are things you may find helpful: "
            <ul>
                <li>"Look for help in the Leptos "<LinkExt href="https://discord.gg/x8NhWWYTV2" target=LinkExtTarget::Blank>"Discord"</LinkExt>" server."</li>
                <li>"If you think you encountered a bug, open a ticket in our " <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank>"repository"</LinkExt></li>
                <li>"Compare the implementation of this book at "<LinkExt href="https://github.com/lpotthast/leptonic/tree/main/examples/book" target=LinkExtTarget::Blank>"GitHub"</LinkExt>" with what you currently have."</li>
            </ul>
        </P>

        <H2>"Contribute"</H2>

        <P>
            "If you have anything to say about Leptonic, be it a component you miss, a feature you feel missing, or a bug you encountered, "
            "feel free to contribute back to the project by reaching the community or us through"
            <ul>
                <li>"the Leptos "<LinkExt href="https://discord.gg/x8NhWWYTV2" target=LinkExtTarget::Blank>"Discord"</LinkExt>" server or "</li>
                <li>"the "<LinkExt href="https://github.com/lpotthast/leptonic/issues" target=LinkExtTarget::Blank>"Issues"</LinkExt>" section of the Leptonic repository."</li>
            </ul>
        </P>

        <P>
            "Writing a component library is a big undertaking. Feel free to contribute by writing new or updating existing code yourself. "
            "As more people are on board, more and better ideas, concepts and implementation details follow. Code contributions are always welcomed!"
        </P>
    }
}
