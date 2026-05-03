use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::popover::PopoverDemo;
use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

#[component]
pub fn PageAtomPopover() -> impl IntoView {
    view! {
        <Article>
            <h1 id="popover" class="anchor">
                "Popover"
                <AnchorLink href="#popover" description="Direct link to article header"/>
            </h1>

            <p>"..."</p>

            <DemoShell source=include_str!("demos/popover.rs")>
                <PopoverDemo />
            </DemoShell>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Popover", link: "#popover" },
            ]
        }/>
    }
}
