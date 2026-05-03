use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::toggle_basic::ToggleBasicDemo;
use super::demos::toggle_icons::ToggleIconsDemo;
use super::demos::toggle_stationary::ToggleStationaryDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageToggle() -> impl IntoView {
    view! {
        <Article>
            <h1 id="toggle" class="anchor">
                "Toggle"
                <AnchorLink href="#toggle" description="Direct link to article header"/>
            </h1>

            <p>"A toggle is a representation of a boolean value."</p>

            <DemoShell source=include_str!("demos/toggle_basic.rs")>
                <ToggleBasicDemo />
            </DemoShell>

            <h2 id="icons" class="anchor">
                "Icons"
                <AnchorLink href="#icons" description="Direct link to section: Icons"/>
            </h2>

            <p>"A toggle can be configured with a pair of icons. One icon being rendered in the off position, the other being rendered in the on position."</p>

            <DemoShell source=include_str!("demos/toggle_icons.rs")>
                <ToggleIconsDemo />
            </DemoShell>

            <h2 id="variations" class="anchor">
                "Variations"
                <AnchorLink href="#variations" description="Direct link to section: Variations"/>
            </h2>

            <p>"The toggle comes in two variants: Sliding and Stationary. Sliding toggles are the default and the ones we have used so far."</p>
            <p>"Stationary toggles are not animated and only consist of a single circle."</p>

            <DemoShell source=include_str!("demos/toggle_stationary.rs")>
                <ToggleStationaryDemo />
            </DemoShell>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Toggle", link: "#toggle" },
                Toc::Leaf { title: "Icons", link: "#icons" },
                Toc::Leaf { title: "Variations", link: "#variations" },
            ]
        }/>
    }
}
