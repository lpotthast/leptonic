use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::radio_basic::RadioBasicDemo;
use super::demos::radio_disabled::RadioDisabledDemo;
use super::demos::radio_group::RadioGroupDemo;
use super::demos::radio_labeled::RadioLabeledDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageRadio() -> impl IntoView {
    view! {
        <Article>
            <h1 id="radio" class="anchor">
                "Radio"
                <AnchorLink href="#radio" description="Direct link to article header"/>
            </h1>

            <p>"Radio..."</p>

            <DemoShell source=include_str!("demos/radio_basic.rs")>
                <RadioBasicDemo />
            </DemoShell>

            <h2 id="radio-groups" class="anchor">
                "Radio groups"
                 <AnchorLink href="#radio-groups" description="Direct link to section: Radio groups"/>
            </h2>

            <DemoShell source=include_str!("demos/radio_group.rs")>
                <RadioGroupDemo />
            </DemoShell>

            <h2 id="labeled" class="anchor">
                "Labeled"
                <AnchorLink href="#labeled" description="Direct link to section: Labeled"/>
            </h2>

            <p>"Wrap an input and a label to link them together."</p>

            <DemoShell source=include_str!("demos/radio_labeled.rs")>
                <RadioLabeledDemo />
            </DemoShell>

            <h2 id="disabled" class="anchor">
                "Disabled"
                <AnchorLink href="#disabled" description="Direct link to section: Disabled"/>
            </h2>

            <p>"Radio buttons support the " <Code inline=true>"disabled"</Code> " property, making them unmodifiable if set true."</p>

            <DemoShell source=include_str!("demos/radio_disabled.rs")>
                <RadioDisabledDemo />
            </DemoShell>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    --radio-size
                    --radio-fill-size
                    --radio-border
                    --radio-border-radius
                    --radio-border-color
                    --radio-hover-border-color
                    --radio-checked-border-color
                    --radio-background-color
                    --radio-checked-fill-background-color
                    --radio-disabled-filter
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Radio", link: "#radio" },
                Toc::Leaf { title: "Groups", link: "#radio-groups" },
                Toc::Leaf { title: "Labeled", link: "#labeled" },
                Toc::Leaf { title: "Disabled", link: "#disabled" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
