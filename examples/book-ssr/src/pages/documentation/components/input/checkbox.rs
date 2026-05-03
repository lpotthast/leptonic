use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::checkbox_basic::CheckboxBasicDemo;
use super::demos::checkbox_disabled::CheckboxDisabledDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageCheckbox() -> impl IntoView {
    view! {
        <Article>
            <h1 id="checkbox" class="anchor">
                "Checkbox"
                <AnchorLink href="#checkbox" description="Direct link to article header"/>
            </h1>

            <p>"Checkbox..."</p>

            <DemoShell source=include_str!("demos/checkbox_basic.rs")>
                <CheckboxBasicDemo />
            </DemoShell>

            <h2 id="disabled" class="anchor">
                "Disabled"
                <AnchorLink href="#disabled" description="Direct link to section: Disabled"/>
            </h2>

            <p>"Checkboxes support the " <Code inline=true>"disabled"</Code> " property, making them unmodifiable if set true."</p>

            <DemoShell source=include_str!("demos/checkbox_disabled.rs")>
                <CheckboxDisabledDemo />
            </DemoShell>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    --checkbox-size
                    --checkbox-padding
                    --checkbox-border-radius
                    --checkbox-border
                    --checkbox-border-color
                    --checkbox-color
                    --checkbox-background-color
                    --checkbox-hover-border-color
                    --checkbox-hover-color
                    --checkbox-hover-background-color
                    --checkbox-checked-border-color
                    --checkbox-checked-color
                    --checkbox-checked-background-color
                    --checkbox-checked-hover-border-color
                    --checkbox-checked-hover-color
                    --checkbox-checked-hover-background-color
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Checkbox", link: "#checkbox" },
                Toc::Leaf { title: "Disabled", link: "#disabled" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
