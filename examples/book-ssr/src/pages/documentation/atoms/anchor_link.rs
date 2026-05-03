use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    anchor_link_heading::AnchorLinkHeadingDemo, anchor_link_simple::AnchorLinkSimpleDemo,
};
use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

#[component]
pub fn PageAtomAnchorLink() -> impl IntoView {
    view! {
        <Article>
            <h1 id="anchor-link" class="anchor">
                "AnchorLink"
                <AnchorLink href="#anchor-link" description="Direct link to article header">#</AnchorLink>
            </h1>

            <p>
                "AnchorLinks are links referencing a specific section of the current page by using an identifier which should be present on another, referenced, element. "
            </p>

            <p>
                "When the user interacts with an AnchorLink, the default implementation scrolls to the target element. The `scroll_behavior` property can be used to modify the behavior."
            </p>

            <p>
                "The most common place of anchor links is in article headings. This site uses them as well. "
                "In the following example, we create a heading and give it an id in order to reference it in the AnchorLink. "
                "When an AnchorLink is not created with explicit children, here, by using the shorthand notation for closing the tag, a single `#` character will be rendered instead. "
                "When not giving a descriptive name through children, specifying the description property is advised, so that accessibility technology can clearly describe the target."
            </p>

            <DemoShell source=include_str!("demos/anchor_link_heading.rs")>
                <AnchorLinkHeadingDemo />
            </DemoShell>

            <p>
                "But AnchorLinks can be used everywhere! Here, we simply render it with some text, removing the need for a description property. Interacting with (by pressing on) the link will also jump to the above heading."
            </p>

            <DemoShell source=include_str!("demos/anchor_link_simple.rs")>
                <AnchorLinkSimpleDemo />
            </DemoShell>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "AnchorLink", link: "#anchor-link" },
            ]
        }/>
    }
}
