use leptos::*;

#[component]
pub fn Field(
    
    children: Children,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    view! {
        <leptonic-field id=id class=class style=style>
            { children() }
        </leptonic-field>
    }
}

#[component]
pub fn FieldLabel(
    
    children: Children,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    view! {
        <leptonic-field-label id=id class=class style=style>
            { children() }
        </leptonic-field-label>
    }
}
