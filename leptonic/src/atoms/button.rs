use std::sync::Arc;

use leptos::{
    attr::custom::custom_attribute,
    either::Either,
    prelude::*,
    tachys::html::{class::class, style::style},
};
use leptos_router::components::{A, AProps, ToHref};

use super::link::SharedHref;
use crate::{
    hooks::{LinkTarget, *},
    utils::{
        aria::{AriaCurrent, AriaExpanded, AriaHasPopup, AriaPressed},
        classes::Classes,
        data_attributes::flag,
        styles::Styles,
    },
};

/// A button: presses from mouse, touch, keyboard and screen readers through `use_button`.
///
/// Data attributes: `data-pressed`, `data-hovered`, `data-focused`, `data-focus-visible`,
/// `data-disabled`.
#[component]
pub fn Button(
    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// The `type` of the button. Defaults to `button`, so that buttons in forms don't submit them
    /// unless asked to.
    #[prop(optional)]
    button_type: ButtonType,
    #[prop(into, optional)] exclude_from_tab_order: Signal<bool>,
    #[prop(into, optional)] aria_haspopup: Signal<Option<AriaHasPopup>>,
    #[prop(into, optional)] aria_expanded: Signal<Option<AriaExpanded>>,
    #[prop(into, optional)] aria_pressed: Signal<Option<AriaPressed>>,
    /// Labels the button when its content doesn't (icon-only buttons).
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Signal<Option<String>>,
    #[prop(into, optional)] aria_controls: Signal<Option<String>>,
    /// Marks the button as the current item of a set (e.g. the current page of a pagination).
    #[prop(into, optional)]
    aria_current: Signal<Option<AriaCurrent>>,
    /// The form attributes of a submit or reset button (`form`, `formaction`, `name`, ...).
    #[prop(optional)]
    form: ButtonFormAttributes,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let UseButtonReturn {
        props,
        is_disabled,
        is_pressed,
        is_hovered,
        is_focused,
        ..
    } = use_button(UseButtonInput {
        button_type,
        is_disabled,
        exclude_from_tab_order,
        aria_haspopup,
        aria_expanded,
        aria_pressed,
        aria_label,
        aria_labelledby: Signal::stored(aria_labelledby),
        aria_describedby,
        aria_controls,
        aria_current,
        form,
        on_press,
        on_hover_start,
        on_hover_end,
        ..UseButtonInput::default()
    });

    let (button_attrs, button_styles) = props.into_parts();
    let styles = button_styles.merge(styles);

    view! {
        <button
            {..button_attrs}
            class=classes
            style=styles
            data-pressed=flag(is_pressed)
            data-hovered=flag(is_hovered)
            data-focused=flag(is_focused)
            data-disabled=flag(is_disabled)
        >
            {children()}
        </button>
    }
}

#[component]
pub fn LinkButton<H>(
    href: H,

    /// Where to display the linked URL, as the name for a browsing context (a tab, window, or `<iframe>`).
    #[prop(optional)]
    target: LinkTarget,

    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,

    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,

    #[prop(into, optional)] is_disabled: Signal<bool>,

    #[prop(into, optional)] aria_haspopup: Option<Signal<Option<AriaHasPopup>>>,

    #[prop(into, optional)] aria_expanded: Option<Signal<Option<AriaExpanded>>>,

    #[prop(into, optional)] classes: Classes,

    #[prop(into, optional)] styles: Styles,

    /// When the link is the current page (`aria-current="page"`).
    #[prop(optional)]
    current_match: crate::atoms::link::CurrentMatch,

    /// Rendered again when the link is disabled or enabled.
    children: ChildrenFn,
) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    // Navigation is handled by the router's `<A>`. The button hook only adds press, hover and
    // focus behavior; propagation must continue so the router sees the click.
    let UseButtonReturn {
        props,
        is_disabled,
        is_pressed,
        is_hovered,
        is_focused,
        ..
    } = use_button(UseButtonInput {
        element_type: ButtonElementType::Anchor,
        is_disabled,
        aria_haspopup: aria_haspopup.unwrap_or_default(),
        aria_expanded: aria_expanded.unwrap_or_default(),
        on_hover_start,
        on_hover_end,
        ..UseButtonInput::default()
    });

    let target = (target != LinkTarget::Same).then(|| target.to_oco());
    let (button_attrs, button_styles) = props.into_parts();
    // Cloned for each rendering (the element is rendered again when it is disabled or enabled).
    let attrs = StoredValue::new((
        button_attrs,
        custom_attribute("data-pressed", flag(is_pressed)),
        custom_attribute("data-hovered", flag(is_hovered)),
        custom_attribute("data-focused", flag(is_focused)),
        custom_attribute("data-disabled", flag(is_disabled)),
    ));
    let styles = StoredValue::new(button_styles.merge(styles));
    let classes = StoredValue::new(classes);
    let href = SharedHref(Arc::new(href));

    // Disabled, the link has no `href`: it can't be followed in any way (react-aria's `useButton`
    // drops it, as the `Link` atom).
    move || {
        if is_disabled.get() {
            Either::Left(view! {
                <a {..attrs.get_value()} class=classes.get_value() style=styles.get_value()>
                    {children()}
                </a>
            })
        } else {
            let children = children.clone();
            Either::Right(
                A(AProps {
                    href: href.clone(),
                    target: target.clone(),
                    exact: current_match == crate::atoms::link::CurrentMatch::Exact,
                    strict_trailing_slash: false,
                    scroll: true,
                    children: Box::new(move || children()),
                })
                .add_any_attr(class(classes.get_value()))
                .add_any_attr(style(styles.get_value()))
                .add_any_attr(attrs.get_value()),
            )
        }
    }
}
