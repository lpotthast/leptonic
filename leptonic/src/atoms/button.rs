use leptos::{
    prelude::*,
    tachys::html::{class::class, style::style},
};
use leptos_router::components::{A, AProps, ToHref};

use crate::{
    hooks::{LinkTarget, *},
    utils::{
        aria::{AriaExpanded, AriaHasPopup, AriaPressed},
        classes::Classes,
        styles::Styles,
    },
};

#[component]
pub fn Button(
    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    /// The `type` of the button. Defaults to `button`, so that buttons in forms don't submit them
    /// unless asked to.
    #[prop(optional)]
    button_type: ButtonType,
    #[prop(into, optional)] exclude_from_tab_order: Signal<bool>,
    #[prop(into, optional)] aria_haspopup: Signal<Option<AriaHasPopup>>,
    #[prop(into, optional)] aria_expanded: Signal<Option<AriaExpanded>>,
    #[prop(into, optional)] aria_pressed: Signal<Option<AriaPressed>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let UseButtonReturn { props, .. } = use_button(UseButtonInput {
        button_type,
        disabled,
        exclude_from_tab_order,
        aria_haspopup,
        aria_expanded,
        aria_pressed,
        on_press,
        on_hover_start,
        on_hover_end,
        ..UseButtonInput::default()
    });

    let (button_attrs, button_styles) = props.into_parts();
    let styles = button_styles.merge(styles);

    view! {
        <button {..button_attrs} class=classes style=styles>
            {children()}
        </button>
    }
}

#[component]
pub fn LinkButton<H>(
    href: H,

    /// Where to display the linked URL, as the name for a browsing context (a tab, window, or `<iframe>`).
    #[prop(into, optional)]
    target: Option<LinkTarget>,

    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,

    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,

    #[prop(into, optional)] disabled: Option<Signal<bool>>,

    #[prop(into, optional)] aria_haspopup: Option<Signal<Option<AriaHasPopup>>>,

    #[prop(into, optional)] aria_expanded: Option<Signal<Option<AriaExpanded>>>,

    #[prop(into, optional)] classes: Classes,

    #[prop(into, optional)] styles: Styles,

    /// If `true`, the link is marked active when the location matches exactly;
    /// if false, link is marked active if the current route starts with it.
    #[prop(optional)]
    exact: bool,

    children: Children,
) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    let disabled = disabled.unwrap_or(Signal::from(false));

    // Navigation is handled by the router's `<A>`. The button hook only adds press, hover and
    // focus behavior; propagation must continue so the router sees the click.
    let UseButtonReturn { props, .. } = use_button(UseButtonInput {
        element_type: ButtonElementType::Anchor,
        disabled,
        aria_haspopup: aria_haspopup.unwrap_or_default(),
        aria_expanded: aria_expanded.unwrap_or_default(),
        on_hover_start,
        on_hover_end,
        ..UseButtonInput::default()
    });

    let target: Option<Oco<'static, str>> = Some(target.unwrap_or_default())
        .filter(|it| it != &LinkTarget::_Self)
        .map(|it| it.to_oco());

    // TODO: Propagate scroll and strict_trailing_slash?
    // TODO (new): Does a class in props.attrs override this? Do we need the old "prepend" logic?

    let (button_attrs, button_styles) = props.into_parts();
    let styles = button_styles.merge(styles);

    A(AProps {
        href,
        target,
        exact,
        strict_trailing_slash: false,
        scroll: true,
        children,
    })
    .add_any_attr(class(classes))
    .add_any_attr(style(styles))
    .add_any_attr(button_attrs)
}

#[component]
pub fn ButtonWrapper(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <div class=classes.add("leptonic-btn-wrapper") style=styles>{children()}</div> }
}
