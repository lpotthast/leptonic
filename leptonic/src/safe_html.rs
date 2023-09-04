use std::borrow::Cow;

use leptos::{html::span, *};

#[component]
pub fn SafeHtml(#[prop(into)] html: Cow<'static, str>) -> impl IntoView {
    // TODO: Sanitize input?
    span().inner_html(html)
}
