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

            <P>
                "AnchorLinks are links referencing a specific section of the current page by using an identifier which should be present on another, referenced, element. "
            </P>

            <P>
                "When the user interacts with an AnchorLink, the default implementation scrolls to the target element. The `scroll_behavior` property can be used to modify the behavior."
            </P>

            <P>
                "The most common place of anchor links is in article headings. This site uses them as well. "
                "In the following example, we create a heading and give it an id in order to reference it in the AnchorLink. "
                "When an AnchorLink is not created with explicit children, here, by using the shorthand notation for closing the tag, a single `#` character will be rendered instead. "
                "When not giving a descriptive name through children, specifying the description property is advised, so that accessibility technology can clearly describe the target."
            </P>

            <Code>
                {indoc!(r##"
                    use leptonic::atoms::link::AnchorLink;
                    use leptonic::components::typography::H1;

                    view! {
                        <H2 id="my-section-anchor">
                            "My Section"
                            <AnchorLink href="#my-section-anchor" description="Direct link to section: My Section"/>
                        </H2>
                    }
                "##)}
            </Code>

            <div style="
                padding: 1em;
                border-radius: 0.25em;
                border: 0.1em solid var(--typography-code-background-color);
            ">
                <H2 id="my-section-anchor">
                    "My Section"
                    <AnchorLink href="#my-section-anchor" description="Direct link to section: My Section"/>
                </H2>
            </div>

            <P>
                "But AnchorLinks can be used everywhere! Here, we simply render it with some text, removing the need for a description property. Interacting with (by pressing on) the link will also jump to the above heading."
            </P>

            <Code>
                {indoc!(r##"
                    <AnchorLink href="#my-section-anchor">
                        "My Section"
                    </AnchorLink>
                "##)}
            </Code>

            <AnchorLink href="#my-section-anchor">
                "My Section"
            </AnchorLink>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "AnchorLink", link: "#anchor-link" },
            ]
        }/>
    }
}
