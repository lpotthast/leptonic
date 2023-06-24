use leptos::*;
use leptos_router::*;

#[component]
pub fn Link<H>(
    cx: Scope,
    /// Used to calculate the link's `href` attribute. Will be resolved relative
    /// to the current route.
    href: H,
    /// If `true`, the link is marked active when the location matches exactly;
    /// if false, link is marked active if the current route starts with it.
    #[prop(optional)]
    exact: bool,
    /// An object of any type that will be pushed to router state
    #[prop(optional)]
    state: Option<State>,
    /// If `true`, the link will not add to the browser's history (so, pressing `Back`
    /// will skip this page.)
    #[prop(optional)]
    replace: bool,
    /// Sets the `id` attribute, making it easier to target.
    #[prop(into, optional)]
    id: Option<String>,
    /// Sets the `class` attribute, making it easier to style.
    #[prop(into, optional)]
    class: Option<AttributeValue>,
    #[prop(into, optional)]
    style: Option<AttributeValue>,
    /// Whether or not this link acts as an anchor.
    #[prop(into, optional, default = false)]
    is_anchor: bool,
    children: Children,
) -> impl IntoView
where
    H: ToHref + 'static,
{
    let a = leptos_router::A::<H>(
        cx,
        AProps::<H> {
            href,
            exact,
            state,
            replace,
            class: None,
            id: None,
            children,
        },
    )
    .into_view(cx);
    view! { cx,
        <leptonic-link id=id class=class style=style>
            { a }
        </leptonic-link>
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinkExtTarget {
    Blank,
    Parent,
    Sel,
    Top,
}

impl std::fmt::Display for LinkExtTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkExtTarget::Blank => f.write_str("_blank"),
            LinkExtTarget::Parent => f.write_str("_parent"),
            LinkExtTarget::Sel => f.write_str("_self"),
            LinkExtTarget::Top => f.write_str("_top"),
        }
    }
}

#[component]
pub fn LinkExt<H>(
    cx: Scope,
    /// Used to calculate the link's `href` attribute.
    href: H,
    target: LinkExtTarget,
    // TODO: Impl this prop
    // /// If `true`, the link will not add to the browser's history (so, pressing `Back`
    // /// will skip this page.)
    // #[prop(optional)]
    // replace: bool,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView
where
    H: ToHref + 'static,
{
    view! { cx,
        <leptonic-link id=id class=class style=style>
            <a href={move || href.to_href()()} target=format!("{target}")>
                { children(cx) }
            </a>
        </leptonic-link>
    }
}
