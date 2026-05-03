use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::alert_custom::AlertCustomDemo;
use super::demos::alert_variants::AlertVariantsDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageAlert() -> impl IntoView {
    view! {
        <Article>
            <h1 id="alert" class="anchor">
                "Alert"
                <AnchorLink href="#alert" description="Direct link to article header"/>
            </h1>

            <DemoShell source=include_str!("demos/alert_variants.rs")>
                <AlertVariantsDemo />
            </DemoShell>

            <h2 id="customization" class="anchor">
                "Customization"
                <AnchorLink href="#customization" description="Direct link to section: Customization"/>
            </h2>

            <Ul>
                <Li slot>"The "<Code inline=true>"default_icon_slot"</Code> " can be specified to change the default position of the icon. Slot `None` will lead to no icon being rendered."</Li>
                <Li slot>"Both " <Code inline=true>"AlertTitle"</Code> "and" <Code inline=true>"AlertContent"</Code> " can be omitted."</Li>
                <Li slot>"The " <Code inline=true>"AlertPrepend"</Code> "and" <Code inline=true>"AlertAppend"</Code> " slot can be overridden."</Li>
                <Li slot>"Custom ids, classes and styles can be applied to all slots."</Li>
            </Ul>

            <DemoShell source=include_str!("demos/alert_custom.rs")>
                <AlertCustomDemo />
            </DemoShell>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    --alert-margin
                    --alert-padding
                    --alert-border
                    --alert-border-radius
                    --alert-primary-background-color
                    --alert-primary-color
                    --alert-info-background-color
                    --alert-info-color
                    --alert-success-background-color
                    --alert-success-color
                    --alert-warn-background-color
                    --alert-warn-color
                    --alert-danger-background-color
                    --alert-danger-color
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Alert", link: "#alert" },
                Toc::Leaf { title: "Customization", link: "#customization" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
