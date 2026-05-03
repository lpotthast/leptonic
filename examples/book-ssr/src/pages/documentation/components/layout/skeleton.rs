use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::skeleton::SkeletonDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageSkeleton() -> impl IntoView {
    view! {
        <Article>
            <h1 id="skeleton" class="anchor">
                "Skeleton"
                <AnchorLink href="#skeleton" description="Direct link to article header"/>
            </h1>

            <p>
                "A skeleton is a placeholder element of a specific shape and size which can be displayed in place of some actual content, "
                "whenever, for example, this content cannot be displayed because required data is still being fetched from a network resource. "
                "This reduces layout shifts and prepares the user for where content will be visible when available."
            </p>

            <DemoShell source=include_str!("demos/skeleton.rs")>
                <SkeletonDemo />
            </DemoShell>

            <p>
                "Albeit used quite often these days, I would like to remind you that this concept is only tries to mitigate the problem of slowly loading resources. "
                "All that might just not be required, if resources are preloaded, if services providing data do that in a few milliseconds, and so on and so forth... Try avoiding overly aggressive use of the skeleton component."
                "But, even if your services respond quickly, keep in mind that the (uncontrollable) user-network-speeds may still result in slow resources."
            </p>

            <h2 id="styling">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    --skeleton-background-color
                    --skeleton-animation-highlight-color
                    --skeleton-border-radius
                    --skeleton-padding
                    --skeleton-cursor
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Skeleton", link: "#skeleton" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
