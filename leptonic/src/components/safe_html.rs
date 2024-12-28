use leptos::html;
use leptos::html::InnerHtmlValue;
use leptos::prelude::*;

#[component]
pub fn SafeHtml<H: InnerHtmlValue>(#[prop(into)] html: H) -> impl IntoView {
    // TODO: Sanitize input!
    html::span().inner_html(html)
}
