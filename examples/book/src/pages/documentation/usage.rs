use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

use crate::pages::documentation::doc_root::DocRoutes;

#[component]
pub fn PageUsage(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"Usage"</H1>

        <P>"Similar to Leptos, this crate comes with a prelude module."</P>

        <P>
            "Just "
            <Code inline=true>"use leptonic::prelude::*;"</Code>
            " and you are almost ready to go."
        </P>

        <P>
            "Leptonic provides the "<Code inline=true>"<Root>"</Code>" component. "
            "It is responsible for enabling the theming, "<Link href=DocRoutes::Modal>"Modal"</Link>" and "<Link href=DocRoutes::Toast>"Toast"</Link>" functionalities of Leptonic as well as providing some global event-listening capabilities."
            "You have to include it in your app once, and render all your content inside it. Many Leptonic components require the "<Code inline=true>"<Root>"</Code>" component to be present."
        </P>

        <P>
            "Using the component is as simple as this:"
        </P>

        <Code>
            {indoc!(r#"
                #[component]
                pub fn App(cx: Scope) -> impl IntoView {
                    view! {cx,
                        <Root default_theme=LeptonicTheme::Light>
                            "Content goes here :)"
                        </Root>
                    }
                }
            "#)}
        </Code>
    }
}
