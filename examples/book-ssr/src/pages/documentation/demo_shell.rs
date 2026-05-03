use leptonic::components::prelude::{Code, Language};
use leptos::prelude::*;
use leptos_classes::Classes;

const DEMO_CSS: &str = include_str!("../../../style/demo-classes.scss");

#[component]
pub fn DemoShell(
    children: Children,
    #[prop(optional)] title: Option<&'static str>,
    #[prop(optional)] description: Option<&'static str>,
    #[prop(optional)] source: Option<&'static str>,
) -> impl IntoView {
    let has_source = source.is_some();
    view! {
        <div class="demo-shell" class:has-source=has_source>
            <div class="demo" data-demo-description=description>
                {title.map(|t| view! {
                    <div class=Classes::from("demo-container-title")>{t}</div>
                })}
                {children()}
            </div>
            {source.map(|src| view! {
                <details class="source">
                    <summary>"View source"</summary>
                    <Code language=Language::Rust>
                        {src}
                    </Code>
                </details>
                <details class="source">
                    <summary>"View styles"</summary>
                    <Code language=Language::Css>
                        {DEMO_CSS}
                    </Code>
                </details>
            })}
        </div>
    }
}
