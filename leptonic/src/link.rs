use leptos::*;
use leptos_router::*;

#[component]
pub fn Link<H>(
    /// Used to calculate the link's `href` attribute. Will be resolved relative
    /// to the current route.
    href: H,
    #[allow(unused)] // TODO: Remove this when leptos's A component supports the title attribute.
    #[prop(into, optional)]
    title: Option<AttributeValue>, // TODO: This should be limited to string attributes...
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
    id: Option<AttributeValue>,
    /// Sets the `class` attribute, making it easier to style.
    #[prop(into, optional)]
    class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView
where
    H: ToHref + 'static,
{
    view! {
        <leptonic-link id=id class=class style=style>
            <A href=href exact=exact state=state.unwrap_or_default() replace=replace>
                { children() }
            </A>
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
    // NOTE(lukas): rel="noopener" is added for security reasons. See: https://developer.chrome.com/docs/lighthouse/best-practices/external-anchors-use-rel-noopener/
    view! {
        <leptonic-link id=id class=class style=style>
            <a
                href=href.to_href()()
                target=format!("{target}")
                rel={ match target { LinkExtTarget::Blank => Some("noopener"), _ => None } }
            >
                { children() }
            </a>
        </leptonic-link>
    }
}
