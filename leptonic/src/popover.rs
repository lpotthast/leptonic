use leptos::*;

#[component]
pub fn Popover<S>(
    cx: Scope,
    show: S,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView
where
    S: Fn() -> bool + 'static,
{
    view! {cx,
        <leptonic-popover id=id class=class style=style class:show=move || show()>
            <leptonic-popover-content>
                {children(cx)}
            </leptonic-popover-content>
        </leptonic-popover>
    }
}
