use leptos::prelude::*;

use super::{Code, Disclosure, Language, demo_styles::styles_for};

/// Frame for an interactive demo.
///
/// `description` (required) summarizes the demo for the Markdown export, which replaces the demo with
/// `*[Interactive Demo: <description>]*`. `source` (usually `include_str!("demos/<name>.rs")`) is shown in a
/// "View source" [`Disclosure`], together with the demo styles it uses (see [`styles_for`]). With `source_open`, the
/// source starts expanded; use it instead of repeating the demo code in a separate snippet.
#[component]
pub fn Demo(
    description: &'static str,
    #[prop(optional)] source: Option<&'static str>,
    #[prop(optional)] source_open: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="doc-demo">
            <div class="demo" data-demo-description=description>
                {children()}
            </div>
            {source.map(|source| view! {
                <Disclosure label="View source" default_expanded=source_open>
                    <Code language=Language::Rust>{source}</Code>
                </Disclosure>
                {Some(styles_for(source)).filter(|styles| !styles.is_empty()).map(|styles| view! {
                    <Disclosure label="View styles">
                        <Code language=Language::Css>{styles}</Code>
                    </Disclosure>
                })}
            })}
        </div>
    }
}
