use leptos::*;

#[component]
pub fn Popover<S>(
    show: S,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView
where
    S: Fn() -> bool + 'static,
{
    view! {
        <leptonic-popover id=id class=class style=style class:show=show>
            <leptonic-popover-content>
                {children()}
            </leptonic-popover-content>
        </leptonic-popover>
    }
}
