use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::toast_creation::ToastCreationDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageToast() -> impl IntoView {
    view! {
        <Article>
            <h1 id="toast" class="anchor">
                "Toast"
                <AnchorLink href="#toast" description="Direct link to article header"/>
            </h1>

            <DemoShell source=include_str!("demos/toast_creation.rs")>
                <ToastCreationDemo />
            </DemoShell>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    --toast-border-radius
                    --toast-header-border-bottom
                    --toast-header-padding
                    --toast-message-padding
                    --toast-info-header-background
                    --toast-info-header-color
                    --toast-info-message-background
                    --toast-info-message-color
                    --toast-success-header-background
                    --toast-success-header-color
                    --toast-success-message-background
                    --toast-success-message-color
                    --toast-warn-header-background
                    --toast-warn-header-color
                    --toast-warn-message-background
                    --toast-warn-message-color
                    --toast-error-header-background
                    --toast-error-header-color
                    --toast-error-message-background
                    --toast-error-message-color
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Toast", link: "#toast" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
