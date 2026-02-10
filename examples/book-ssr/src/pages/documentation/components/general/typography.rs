use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageTypography() -> impl IntoView {
    view! {
        <Article>
            <h1 id="typography" class="anchor">
                "Typography"
                <AnchorLink href="#typography" description="Direct link to article header"/>
            </h1>

            <Separator />

            <h1>"Typography - H1"</h1>
            <h2>"Typography - H2"</h2>
            <h3>"Typography - H3"</h3>
            <h4>"Typography - H4"</h4>
            <h5>"Typography - H5"</h5>
            <h6>"Typography - H6"</h6>

            <p>"This is a paragraph"</p>

            <Code inline=false>"Typography - Code"</Code>

            <p>
                "This is a paragraph containing an "
                <Code inline=true>"inlined"</Code>
                " piece of code."
            </p>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code>
                {indoc!(r"
                    --typography-font-family
                    --typography-h1-margin
                    --typography-h1-font-size
                    --typography-h1-font-weight
                    --typography-h2-margin
                    --typography-h2-font-size
                    --typography-h2-font-weight
                    --typography-h3-margin
                    --typography-h3-font-size
                    --typography-h3-font-weight
                    --typography-h4-margin
                    --typography-h4-font-size
                    --typography-h4-font-weight
                    --typography-h5-margin
                    --typography-h5-font-size
                    --typography-h5-font-weight
                    --typography-h6-margin
                    --typography-h6-font-size
                    --typography-h6-font-weight
                    --typography-p-margin
                    --typography-p-font-size
                    --typography-p-font-weight
                    --typography-p-line-height
                    --typography-code-margin
                    --typography-code-padding
                    --typography-code-font-size
                    --typography-code-font-weight
                    --typography-code-line-height
                    --typography-code-border-radius
                    --typography-code-background-color
                    --typography-code-color
                    --typography-inline-code-margin
                    --typography-inline-code-padding
                    --typography-inline-code-line-height
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Typography", link: "#typography" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
