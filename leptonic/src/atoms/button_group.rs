use leptos::*;

#[component]
pub fn ButtonGroup(
    children: Children,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    /// Arbitrary additional attributes.
    #[prop(attrs)]
    attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    // TODO: Manage focus through something like `useFocusContainer`?

    view! {
        <leptonic-btn-group {..attributes} id=id class=class style=style>
            { children() }
        </leptonic-btn-group>
    }
}
