use leptos::*;

use crate::Height;

#[component]
pub fn AppBar(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] height: Option<Height>,
    children: Children,
) -> impl IntoView {
    view! {
        <leptonic-app-bar id=id class=class style=style style=("--app-bar-height", move || height.map(|it| format!("{it}")))>
            { children() }
        </leptonic-app-bar>
    }
}
