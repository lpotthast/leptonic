use leptos::*;

#[component]
pub fn Box(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <leptonic-box id=id class=class style=style>
            { children(cx) }
        </leptonic-box>
    }
}
