use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::collapsible::CollapsibleDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageCollapsible() -> impl IntoView {
    view! {
        <Article>
            <h1 id="collapsible" class="anchor">
                "Collapsible"
                <AnchorLink href="#collapsible" description="Direct link to article header"/>
            </h1>

            <DemoShell source=include_str!("demos/collapsible.rs")>
                <CollapsibleDemo />
            </DemoShell>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Collapsible", link: "#collapsible" },
            ]
        }/>
    }
}
