use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::app_bar::AppBarDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageAppBar() -> impl IntoView {
    view! {
        <Article>
            <h1 id="overview">
                "App Bar"
                <AnchorLink href="#overview" description="Direct link to main article header"/>
            </h1>

            <p>"The "<Code inline=true>"<AppBar>"</Code>" component sticks to the top of its parent and provides a convenient entrypoint for many app layouts."</p>

            <DemoShell source=include_str!("demos/app_bar.rs")>
                <AppBarDemo />
            </DemoShell>

            <h2 id="styling">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    --app-bar-height
                    --app-bar-background-color
                    --app-bar-border-bottom
                    --app-bar-box-shadow
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "AppBar", link: "#overview" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
