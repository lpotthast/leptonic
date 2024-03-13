use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptos::*;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageAtomAnchorLink() -> impl IntoView {
    view! {
        <Article>
            <H1 id="anchor-link" class="anchor">
                "AnchorLink"
                <AnchorLink href="#anchor-link" description="Direct link to article header"/>
            </H1>

            <P>"..."</P>

            <Code>
                {indoc!(r##"
                    use leptonic::atoms::link::AnchorLink;
                    use leptonic::components::typography::H1;

                    view! {
                        <H1 id="my-section-anchor">
                            <AnchorLink href="#my-section-anchor" description="Direct link to article header"/>
                        </H1>
                    }
                "##)}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "AnchorLink", link: "#anchor-link" },
            ]
        }/>
    }
}
