use leptonic::{
    atoms::prelude::{AnchorLink as AnchorLinkAtom, CurrentMatch, Link as LinkAtom},
    hooks::{Href, LinkRel, LinkTarget},
};
use leptos::prelude::*;
use leptos_classes::Classes;
use leptos_router::components::ToHref;

/// A link in a page's text: leptonic's `Link` atom in the book's link style (`.doc-link`).
#[component]
#[allow(clippy::needless_pass_by_value)] // Leptos components own their props.
pub fn Link<H>(
    /// Where the link goes: a route or a URL.
    href: H,
    #[prop(optional)] target: LinkTarget,
    #[prop(optional)] rel: Vec<LinkRel>,
    #[prop(optional)] current_match: CurrentMatch,
    #[prop(into, optional)] classes: Classes,
    children: ChildrenFn,
) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    view! {
        <LinkAtom href target rel current_match classes=classes.add("doc-link")>
            {children()}
        </LinkAtom>
    }
}

/// A link to an element of the same page: leptonic's `AnchorLink` atom in the book's link style (`.doc-link`).
/// Without children, it shows a `#` (then give it an `aria_label`).
#[component]
pub fn AnchorLink(
    /// The element to link to: `"#my-anchor"`.
    #[prop(into)]
    href: Href,
    /// Names the link when its content doesn't (the bare `#`).
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <AnchorLinkAtom href aria_label classes=classes.add("doc-link")>
            {match children {
                Some(children) => children().into_any(),
                None => "#".into_any(),
            }}
        </AnchorLinkAtom>
    }
}
