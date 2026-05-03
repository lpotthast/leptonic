use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::table::TableDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageTable() -> impl IntoView {
    view! {
        <Article>
            <h1 id="table" class="anchor">
                "Table"
                <AnchorLink href="#table" description="Direct link to article header"/>
            </h1>

            <p>"Tables..."</p>

            <DemoShell source=include_str!("demos/table.rs")>
                <TableDemo />
            </DemoShell>

            <h2 id="styling">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    // Table wrapper
                    --table-wrapper-box-shadow-color

                    // Table
                    --table-color
                    --table-background-color
                    --table-background-color-on-hover
                    --table-background-color-of-striped-rows
                    --table-header-background-color
                    --table-border-color
                    --table-cell-box-shadow-on-hover
                    --table-column-background-if-ordered
                    --table-header-cell-padding
                    --table-body-cell-padding
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Table", link: "#table" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
