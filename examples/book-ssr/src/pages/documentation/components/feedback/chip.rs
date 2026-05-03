use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::chip_colors::ChipColorsDemo;
use super::demos::chip_dismissible::ChipDismissibleDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageChip() -> impl IntoView {
    view! {
        <Article>
            <h1 id="chip" class="anchor">
                "Chip"
                <AnchorLink href="#chip" description="Direct link to article header"/>
            </h1>

            <DemoShell source=include_str!("demos/chip_colors.rs")>
                <ChipColorsDemo />
            </DemoShell>

            <h2 id="dismissible-chips" class="anchor">
                "Dismissible chips"
                <AnchorLink href="#dismissible-chips" description="Direct link to section: Dismissible chips"/>
            </h2>

            <p>
                "As chips are often used to convey mutable state, we allow chips to be dismissible. "
                "Dismissible chips display an "<Code inline=true>"X"</Code>" icon which lets the user dismiss the chip. "
                "The component embedding the chip is responsible of actually removing it, e.g. not rendering it again."
            </p>

            <DemoShell source=include_str!("demos/chip_dismissible.rs")>
                <ChipDismissibleDemo />
            </DemoShell>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    --chip-font-size
                    --chip-margin
                    --chip-padding
                    --chip-border
                    --chip-border-radius
                    --chip-primary-text-color
                    --chip-primary-text-color-hover
                    --chip-primary-background-color
                    --chip-primary-background-color-hover
                    --chip-secondary-text-color
                    --chip-secondary-text-color-hover
                    --chip-secondary-background-color
                    --chip-secondary-background-color-hover
                    --chip-success-text-color
                    --chip-success-text-color-hover
                    --chip-success-background-color
                    --chip-success-background-color-hover
                    --chip-info-text-color
                    --chip-info-text-color-hover
                    --chip-info-background-color
                    --chip-info-background-color-hover
                    --chip-warn-text-color
                    --chip-warn-text-color-hover
                    --chip-warn-background-color
                    --chip-warn-background-color-hover
                    --chip-danger-text-color
                    --chip-danger-text-color-hover
                    --chip-danger-background-color
                    --chip-danger-background-color-hover
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Chip", link: "#chip" },
                Toc::Leaf { title: "Dismissible chips", link: "#dismissible-chips" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
