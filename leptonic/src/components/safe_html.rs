use leptos::{html, html::InnerHtmlValue, prelude::*};

#[component]
pub fn SafeHtml<H: InnerHtmlValue>(#[prop(into)] html: H) -> impl IntoView {
    // TODO: Sanitize input!
    html::span().inner_html(html)
}
