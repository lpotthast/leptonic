use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::popover_hover::PopoverHoverDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PagePopover() -> impl IntoView {
    view! {
        <Article>
            <h1 id="popover" class="anchor">
                "Popover"
                <AnchorLink href="#popover" description="Direct link to article header"/>
            </h1>

            <p>
                "A themed popover component with ARIA dialog semantics, positioning, "
                "and dismiss behavior. Bundles overlay, trigger, dialog, and dismiss "
                "button atoms into a single convenience component."
            </p>

            <h2 id="hover-trigger" class="anchor">
                "Hover Trigger"
                <AnchorLink href="#hover-trigger" description="Direct link to section: Hover Trigger"/>
            </h2>

            <p>
                "Use controlled mode with a "<Code inline=true>"Hoverable"</Code>
                " wrapper for hover-triggered popovers."
            </p>

            <DemoShell source=include_str!("demos/popover_hover.rs")>
                <PopoverHoverDemo />
            </DemoShell>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    --popover-padding
                    --popover-border-radius
                    --popover-color
                    --popover-background-color
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Popover", link: "#popover" },
                Toc::Leaf { title: "Hover Trigger", link: "#hover-trigger" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
