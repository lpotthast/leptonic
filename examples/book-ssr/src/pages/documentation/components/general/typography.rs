use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::typography::TypographyDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageTypography() -> impl IntoView {
    view! {
        <Article>
            <h1 id="typography" class="anchor">
                "Typography"
                <AnchorLink href="#typography" description="Direct link to article header"/>
            </h1>

            <DemoShell source=include_str!("demos/typography.rs")>
                <TypographyDemo />
            </DemoShell>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
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
