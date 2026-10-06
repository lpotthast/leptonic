use leptos::prelude::*;
use leptos_router::components::ToHref;

use crate::{
    ScrollBehavior,
    atoms::link::{AnchorLink as AnchorLinkAtom, Link as LinkAtom},
    hooks::{Href, LinkTarget, PressEvent},
    utils::{classes::Classes, styles::Styles},
};
pub use crate::{atoms::link::CurrentMatch, hooks::LinkRel};

/// A themed link to an element on the same page. Without children, it shows a `#`.
#[component]
pub fn AnchorLink(
    /// The element to link to, by id: `"#my-anchor"` (or `"my-anchor"`).
    #[prop(into)]
    href: Href,
    /// `None`: no scrolling. Default: smoothly.
    #[prop(into, default = Some(ScrollBehavior::default()))]
    scroll_behavior: Option<ScrollBehavior>,
    /// Names the link when its content doesn't (e.g. the bare `#`).
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <AnchorLinkAtom
            href
            scroll_behavior
            aria_label
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

/// A themed link to another page or resource; see the [`Link`](crate::atoms::link::Link) atom.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn Link<H>(
    /// Where the link goes: a route (resolved relative to the current one) or a URL.
    href: H,
    #[prop(optional)] target: LinkTarget,
    /// The relationship of the linked document. `NoOpener` is added for `LinkTarget::Blank`.
    #[prop(optional)]
    rel: Vec<LinkRel>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(optional)] current_match: CurrentMatch,
    /// Replace the current history entry instead of adding one.
    #[prop(optional)]
    replace: bool,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: ChildrenFn,
) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    view! {
        <LinkAtom
            href
            target
            rel
            is_disabled
            current_match
            replace
            aria_label
            nostrip:on_press
            classes=classes.add("leptonic-link")
            styles
        >
            {children()}
        </LinkAtom>
    }
}
