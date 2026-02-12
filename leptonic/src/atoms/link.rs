use leptos::prelude::*;
use leptos_router::components::{ToHref, A};

pub use crate::hooks::LinkRel;
use crate::{
    hooks::{use_anchor_link, Href, UseAnchorLinkInput, UseAnchorLinkReturn, *},
    utils::{classes::Classes, styles::Styles},
    ScrollBehavior,
};

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

    children: Children,
) -> impl IntoView {
    // We make links "use_press", so that optional PressResponder's higher up the component tree can react on link interactions
    // and so that a custom `on_press` handler can immediately work with the underlying link element.
    let UseAnchorLinkReturn {
        props,
        is_pressed: _,
        is_focus_visible: _,
        focus_handle: _,
    } = use_anchor_link(UseAnchorLinkInput {
        href: Href::from_str(href).expect("valid href"),
        scroll_behavior: scroll_behavior.or(Some(ScrollBehavior::default())),
        disabled: false.into(),
        element_type: LinkElementType::default(),
        description,
        on_press: None,
        on_press_start: None,
        on_press_end: None,
    });

    view! {
        <a {..props.into_attrs()} class=classes style=styles>
            {children()}
        </a>
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
    let UseLinkReturn {
        props,
        is_disabled: _,
        is_pressed: _,
        is_focus_visible: _,
        focus_handle: _,
    } = use_link(UseLinkInput {
        // href/target/rel are handled by the <A> component, not the hook.
        href: None,
        target: None,
        rel: vec![],
        is_disabled: Signal::default(),
        element_type: LinkElementType::default(),
        aria_current: None,
        on_press,
        on_press_start: None,
        on_press_end: None,
    });

    // Spread only the interaction/ARIA/focus attrs from use_link.
    // href, target, rel, and exact are handled by <A> itself.
    // TODO: This uses an inefficient `attr:class=move || classes.to_class_string()`, because
    //  leptos currently does not allow `attr:class=classes` (requires IntoAttributeValue, but we
    //  only implement IntoClass).
    // TODO: Uses inefficient `move || styles.to_style_string()`.
    view! {
        <A
            {..props.into_attrs()}
            href=href
            exact=exact
            attr:class=move || classes.to_class_string()
            attr:style=move || styles.to_style_string()
        >
            {children()}
        </A>
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
    // Automatically add NoOpener for _blank targets as a security default.
    // See: <https://developer.chrome.com/docs/lighthouse/best-practices/external-anchors-use-rel-noopener/>
    let mut effective_rel = rel;
    if target == LinkTarget::_Blank && !effective_rel.contains(&LinkRel::NoOpener) {
        effective_rel.push(LinkRel::NoOpener);
    }

    let UseLinkReturn {
        props,
        is_disabled: _,
        is_pressed: _,
        is_focus_visible: _,
        focus_handle: _,
    } = use_link(UseLinkInput {
        // href is set via the <a> element below (from the reactive ToHref prop).
        href: None,
        target: Some(target),
        rel: effective_rel,
        is_disabled: disabled,
        element_type: LinkElementType::default(),
        aria_current: None,
        on_press,
        on_press_start: None,
        on_press_end: None,
    });

    view! {
        <a {..props.into_attrs()} class=classes style=styles href=move || href.to_href()()>
            {children()}
        </a>
    }
}
