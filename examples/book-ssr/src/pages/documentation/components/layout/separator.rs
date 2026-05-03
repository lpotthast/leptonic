use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::separator::SeparatorDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageSeparator() -> impl IntoView {
    view! {
        <Article>
            <h1 id="separator" class="anchor">
                "Separator"
                <AnchorLink href="#separator" description="Direct link to article header"/>
            </h1>

            <DemoShell source=include_str!("demos/separator.rs")>
                <SeparatorDemo />
            </DemoShell>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Separator", link: "#separator" },
            ]
        }/>
    }
}
