use std::borrow::Cow;

use leptos::*;

#[component]
pub fn Box(
    cx: Scope,
    #[prop(into, optional)] id: Option<Cow<'static, str>>,
    #[prop(into, optional)] class: Option<Cow<'static, str>>,
    #[prop(into, optional)] style: Option<Cow<'static, str>>,
    children: Children,
) -> impl IntoView {
    let id = id.map(|it| it.to_owned().to_string());
    let class = class.map(|it| it.to_owned().to_string());
    let style = style.map(|it| it.to_owned().to_string());
    view! { cx,
        <leptonic-box id=id class=class style=style>
            { children(cx) }
        </leptonic-box>
    }
}
