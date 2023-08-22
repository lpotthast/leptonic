use leptos::*;

#[component]
pub fn Field(
    cx: Scope,
    children: Children,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    view! {cx,
        <leptonic-field id=id class=class style=style>
            { children(cx) }
        </leptonic-field>
    }
}

#[component]
pub fn FieldLabel(
    cx: Scope,
    children: Children,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    view! {cx,
        <leptonic-field-label id=id class=class style=style>
            { children(cx) }
        </leptonic-field-label>
    }
}
