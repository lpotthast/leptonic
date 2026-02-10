use crate::atoms::link::{AnchorLink as AnchorLinkAtom, Link as LinkAtom, LinkExt as LinkExtAtom};
pub use crate::hooks::LinkRel;
use crate::hooks::{LinkTarget, PressEvent};
use crate::utils::classes::Classes;
use crate::utils::styles::Styles;
use crate::ScrollBehavior;
use leptos::prelude::*;
use leptos_router::components::ToHref;

#[component]
pub fn AnchorLink(
    /// The anchor link. For example: "#my-anchor".
    #[prop(into)]
    href: Oco<'static, str>,

    #[prop(into, optional)] scroll_behavior: Option<ScrollBehavior>,

    /// Description of this anchor for accessibility.
    /// If text is provided in children, this could be omitted.
    /// If no children are provided, this component renders a single `#`,
    /// which should be described using this field.
    #[prop(into, optional)]
    description: Option<Oco<'static, str>>,

    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,

    /// If no children are provided, this component renders a single `#` character.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    view! {
        <AnchorLinkAtom
            href
            nostrip:scroll_behavior
            nostrip:description
            classes=classes.add("leptonic-anchor-link")
            styles
        >
            {match children {
                Some(children) => children().into_any(),
                None => "#".into_any(),
            }}
        </AnchorLinkAtom>
    }
}

/// A link to a location internal to this application. Potentially resolvable via
/// client-side-routing.
// TODO: Use router state again (leptos_router::location::State) (accepting a prop)
#[component]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::type_complexity)]
pub fn Link<H>(
    /// Used to calculate the link's `href` attribute. Will be resolved relative
    /// to the current route.
    href: H,

    /// If `true`, the link is marked active when the location matches exactly;
    /// if false, link is marked active if the current route starts with it.
    #[prop(optional)]
    exact: bool,

    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    view! {
        <LinkAtom
            href
            exact
            nostrip:on_press
            classes=classes.add("leptonic-link")
            styles=styles
        >
            {children()}
        </LinkAtom>
    }
}

/// A link to a location external to this application.
#[component]
#[allow(clippy::needless_pass_by_value)] // `H` could be `&H`.
pub fn LinkExt<H>(
    /// Used to calculate the link's `href` attribute.
    href: H,
    target: LinkTarget,
    /// The `rel` attribute values for the link. `NoOpener` is automatically
    /// added when `target` is `Blank` for security reasons.
    #[prop(optional)]
    rel: Vec<LinkRel>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    // TODO: Impl this prop
    // /// If `true`, the link will not add to the browser's history (so, pressing `Back`
    // /// will skip this page.)
    // #[prop(optional)]
    // replace: bool,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    view! {
        <LinkExtAtom
            href
            target
            rel
            disabled
            nostrip:on_press
            classes=classes.add("leptonic-link")
            styles
        >
            {children()}
        </LinkExtAtom>
    }
}
