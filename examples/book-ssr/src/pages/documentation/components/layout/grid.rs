use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::grid::GridDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageGrid() -> impl IntoView {
    view! {
        <Article>
            <h1 id="grid" class="anchor">
                "Grid"
                <AnchorLink href="#grid" description="Direct link to article header"/>
            </h1>

            <DemoShell source=include_str!("demos/grid.rs")>
                <GridDemo />
            </DemoShell>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Grid", link: "#grid" },
            ]
        }/>
    }
}
