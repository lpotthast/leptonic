use leptos::*;

use crate::Height;

#[component]
pub fn AppBar(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] height: Option<Height>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <leptonic-app-bar id=id class=class style=style style=("--app-bar-height", move || height.map(|it| format!("{it}")))>
            { children(cx) }
        </leptonic-app-bar>
    }
}
