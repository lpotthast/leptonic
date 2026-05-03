use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::stack::StackDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageStack() -> impl IntoView {
    view! {
        <Article>
            <h1 id="stack" class="anchor">
                "Stack"
                <AnchorLink href="#stack" description="Direct link to article header"/>
            </h1>

            <p>"Use a stack to create a container displaying its list of children one after the other while spacing them out by a predefined distance."</p>

            <DemoShell source=include_str!("demos/stack.rs")>
                <StackDemo />
            </DemoShell>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Stack", link: "#stack" },
            ]
        }/>
    }
}
