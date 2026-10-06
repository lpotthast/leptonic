// Upstream: react-aria-components/src/Link.tsx @ 99e6102368
use std::sync::Arc;

use leptos::{
    attr::custom::custom_attribute,
    either::Either,
    prelude::*,
    tachys::html::{class::class, style::style},
};
use leptos_router::components::{A, AProps, ToHref};

pub use crate::hooks::LinkRel;
use crate::{
    ScrollBehavior,
    hooks::{
        HoverEndEvent, HoverStartEvent, Href, LinkElementType, LinkTarget, PressEvent,
        UseAnchorLinkInput, UseLinkInput, use_anchor_link, use_link,
    },
    utils::{aria::AriaCurrent, classes::Classes, data_attributes::flag, styles::Styles},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Rendered through leptos_router's `<A>`: the `href` is resolved relative to the current route,
//   and the link gets `aria-current="page"` while its route is the current one
//   (`current_match`). A link therefore needs a surrounding `<Router>`.
// - `replace` (no history entry) is the router's `replace` property (react-aria:
//   `routerOptions`).
// - Render props become `data-*` attributes plus plain children.
//
// ## OMITTED FEATURES
// - `data-current`: the router sets `aria-current`, which `<A>` doesn't expose; style
//   `[aria-current]`.
//
// ## DIFFERENT BEHAVIOR
// - The `href` is required: a link without one is `use_link` with `LinkElementType::Other`.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - Switching between `<a>` and `<span>` re-renders the element. In builds with
//   `--cfg=erase_components` (cargo-leptos' dev builds), Leptos applies attributes spread onto a
//   component (`attr:id`, ...) only to the element it first rendered, so they are lost once
//   `is_disabled` changes; the link's own props (`classes`, `aria_label`, ...) are not affected.
//
// =============================================================================

/// When a [`Link`] counts as the current page (`aria-current="page"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CurrentMatch {
    /// While the location is the link's route or nested below it.
    #[default]
    Prefix,
    /// Only while the location is exactly the link's route.
    Exact,
}

/// Settings a container gives the [`Link`] inside it (react-aria-components' `LinkContext`), e.g.
/// a [`Breadcrumb`](super::breadcrumbs::Breadcrumb): they add to the link's own.
#[derive(Debug, Clone, Copy)]
pub(crate) struct LinkContext {
    pub(crate) is_disabled: Signal<bool>,
    /// The link's `aria-current` while disabled (enabled, the router decides).
    pub(crate) aria_current: Signal<Option<AriaCurrent>>,
    pub(crate) current_match: Option<CurrentMatch>,
    pub(crate) on_press: Option<Callback<PressEvent>>,
}

/// Lets the enabled and the disabled rendering share the `href`.
pub(crate) struct SharedHref<H>(pub(crate) Arc<H>);

impl<H> Clone for SharedHref<H> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<H: ToHref> ToHref for SharedHref<H> {
    fn to_href(&self) -> Box<dyn Fn() -> String + '_> {
        self.0.to_href()
    }
}

/// A link to another page or resource. Resolved relative to the current route and followed with
/// client-side routing where possible; marked `aria-current="page"` while current. A disabled
/// link renders as a `<span role="link" aria-disabled="true">` without `href`, so it can't be
/// followed.
///
/// Data attributes: `data-pressed`, `data-hovered`, `data-focused`, `data-focus-visible`,
/// `data-disabled`.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn Link<H>(
    /// Where the link goes: a route (resolved relative to the current one) or a URL.
    href: H,
    /// Where to open the linked document. Default: here.
    #[prop(optional)]
    target: LinkTarget,
    /// The relationship of the linked document. `NoOpener` is added for `LinkTarget::Blank`.
    #[prop(optional)]
    rel: Vec<LinkRel>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// When the link is the current page.
    #[prop(optional)]
    current_match: CurrentMatch,
    /// Replace the current history entry instead of adding one (client-side routing only).
    #[prop(optional)]
    replace: bool,
    /// Names the link when its content doesn't.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: ChildrenFn,
) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    let context = use_context::<LinkContext>();
    let is_disabled = match context {
        Some(ctx) => Signal::derive(move || is_disabled.get() || ctx.is_disabled.get()),
        None => is_disabled,
    };
    let current_match = context
        .and_then(|ctx| ctx.current_match)
        .unwrap_or(current_match);
    let on_press = match (on_press, context.and_then(|ctx| ctx.on_press)) {
        (Some(own), Some(ctx)) => Some(Callback::new(move |e: PressEvent| {
            own.run(e.clone());
            ctx.run(e);
        })),
        (own, ctx) => own.or(ctx),
    };
    let aria_current = context.map_or_else(Signal::default, |ctx| ctx.aria_current);
    let href = SharedHref(Arc::new(href));
    let classes = StoredValue::new(classes);
    let styles = StoredValue::new(styles);
    // One link per element: created here, not in the reactive branch below, which would dispose
    // a branch's hook state while its element still reacts to `is_disabled`. Each rendering of a
    // branch spreads clones of its attributes.
    let link = |element_type: LinkElementType| {
        let link = use_link(UseLinkInput {
            target: target.clone(),
            rel: rel.clone(),
            is_disabled,
            element_type,
            aria_label,
            // Enabled, `<A>` sets `aria-current`.
            aria_current: if element_type == LinkElementType::Other {
                aria_current
            } else {
                Signal::default()
            },
            on_press,
            on_hover_start,
            on_hover_end,
            ..UseLinkInput::default()
        });
        let data = (
            custom_attribute("data-pressed", flag(link.is_pressed)),
            custom_attribute("data-hovered", flag(link.is_hovered)),
            custom_attribute("data-focused", flag(link.is_focused)),
            custom_attribute("data-disabled", flag(link.is_disabled)),
        );
        let (attrs, link_styles) = link.props.into_parts();
        StoredValue::new((attrs, data, link_styles.merge(styles.get_value())))
    };
    let anchor = link(LinkElementType::Anchor);
    let span = link(LinkElementType::Other);

    move || {
        if is_disabled.get() {
            let (attrs, data, styles) = span.get_value();
            Either::Left(view! {
                <span {..attrs} {..data} class=classes.get_value() style=styles>
                    {children()}
                </span>
            })
        } else {
            let (attrs, data, styles) = anchor.get_value();
            let children = children.clone();
            Either::Right(
                A(AProps {
                    href: href.clone(),
                    // The hook renders `target` (and `rel`).
                    target: None,
                    exact: current_match == CurrentMatch::Exact,
                    strict_trailing_slash: false,
                    scroll: true,
                    children: Box::new(move || children()),
                })
                .add_any_attr(class(classes.get_value()))
                .add_any_attr(style(styles))
                .add_any_attr(attrs)
                .add_any_attr(data)
                .add_any_attr(leptos::tachys::html::property::prop("replace", replace)),
            )
        }
    }
}

/// A link to an element on the same page: pressing it scrolls the element into view and updates
/// the URL fragment.
#[component]
pub fn AnchorLink(
    /// The element to link to, by id: `"#my-anchor"` (or `"my-anchor"`).
    #[prop(into)]
    href: Href,
    /// How to scroll to the element (`None`: not at all, only the URL fragment changes). Default:
    /// smoothly (unless reduced motion is preferred).
    #[prop(into, default = Some(ScrollBehavior::default()))]
    scroll_behavior: Option<ScrollBehavior>,
    /// Names the link when its content doesn't (e.g. a bare `#`).
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let link = use_anchor_link(UseAnchorLinkInput {
        scroll_behavior,
        link: UseLinkInput {
            aria_label,
            on_press,
            ..UseLinkInput::default()
        },
        ..UseAnchorLinkInput::new(href)
    });
    let (attrs, link_styles) = link.props.into_parts();

    view! {
        <a
            {..attrs}
            class=classes
            style=link_styles.merge(styles)
            data-pressed=flag(link.is_pressed)
            data-hovered=flag(link.is_hovered)
            data-focused=flag(link.is_focused)
        >
            {children()}
        </a>
    }
}
