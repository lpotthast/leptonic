use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::tab::TabDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageTab() -> impl IntoView {
    view! {
        <Article>
            <h1 id="tab" class="anchor">
                "Tab"
                <AnchorLink href="#tab" description="Direct link to article header"/>
            </h1>

            <p>
                <Code inline=true>"<Tabs>"</Code>" allow you to spread out your UI components into multiple pages, where only one page is shown at any given time. "
                "Every "<Code inline=true>"<Tab>"</Code>" inside represents a page with a label to select it. A user can interact with labels to bring the tab associated to it into view."
            </p>

            <DemoShell source=include_str!("demos/tab.rs")>
                <TabDemo />
            </DemoShell>

            <h2 id="when-are-tabs-rendered" class="anchor">
                "When are tabs rendered?"
                <AnchorLink href="#when-are-tabs-rendered" description="Direct link to section: When are tabs rendered?"/>
            </h2>

            <p>
                "This is where the "<Code inline=true>"mount"</Code>" property comes into play. "
                "There are two variants to choose from:"
            </p>

            <ul>
                <li>
                    <Code inline=true>"Mount::Once"</Code>
                    <p style="margin-top: 0.5em;">
                        "Tab content is rendered once. Tabs are simply hidden when not shown."
                    </p>
                </li>
                <li>
                    <Code inline=true>"Mount::WhenShown"</Code>
                    <p style="margin-top: 0.5em;">
                        "Tab content is rendered every time a tab is shown. The dom of the tab is unmounted when hidden. "
                        "This means that there is only ever one tab in the final dom, not requiring any hiding-mechanism as in the "<Code inline=true>"Mount::Once"</Code>" case."
                    </p>
                </li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Tab", link: "#tab" },
                Toc::Leaf { title: "When are tabs rendered?", link: "#when-are-tabs-rendered" },
            ]
        }/>
    }
}
