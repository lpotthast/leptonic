use std::fmt::{Display, Formatter};

use leptos::prelude::*;
use leptos_router::components::ToHref;

use crate::{
    atoms,
    hooks::{ButtonType, HoverEndEvent, HoverStartEvent, LinkTarget, PressEvent},
    utils::{
        aria::{AriaExpanded, AriaHasPopup},
        classes::Classes,
        styles::Styles,
    },
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Flat,
    Outlined,
    #[default]
    Filled,
}

impl ButtonVariant {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Flat => "flat",
            Self::Outlined => "outlined",
            Self::Filled => "filled",
        }
    }
}

impl Display for ButtonVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonColor {
    #[default]
    Primary,
    Secondary,
    Success,
    Info,
    Warn,
    Danger,
}

impl ButtonColor {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Success => "success",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Danger => "danger",
        }
    }
}

impl Display for ButtonColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    #[default]
    Normal,
    Big,
}

impl ButtonSize {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Normal => "normal",
            Self::Big => "big",
        }
    }
}

impl Display for ButtonSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A themed button.
#[component]
pub fn Button(
    /// Called when the button is pressed. Not needed for submit/reset buttons or inside a
    /// `PressResponder`.
    #[prop(into, optional)]
    on_press: Option<Callback<PressEvent>>,
    /// The `type` of the button. Defaults to `button`; `Submit` and `Reset` act on their form.
    #[prop(optional)]
    button_type: ButtonType,
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    #[prop(into, optional)] color: Signal<ButtonColor>,
    #[prop(into, optional)] size: Signal<ButtonSize>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] aria_haspopup: Signal<Option<AriaHasPopup>>,
    #[prop(into, optional)] aria_expanded: Signal<Option<AriaExpanded>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let mut props = atoms::button::ButtonProps::builder()
        .button_type(button_type)
        .is_disabled(disabled)
        .aria_haspopup(aria_haspopup)
        .aria_expanded(aria_expanded)
        .classes(classes.add("leptonic-btn"))
        .styles(styles)
        .children(children)
        .build();
    // An `Option` can't be passed to an optional prop through `view!`.
    props.on_press = on_press;
    atoms::button::Button(props)
        .add_any_attr(leptos::attr::custom::custom_attribute(
            "data-variant",
            move || variant.get().as_str(),
        ))
        .add_any_attr(leptos::attr::custom::custom_attribute(
            "data-color",
            move || color.get().as_str(),
        ))
        .add_any_attr(leptos::attr::custom::custom_attribute(
            "data-size",
            move || size.get().as_str(),
        ))
}

/// Joins adjacent buttons into one seamless row (use the `Filled` variant inside).
#[component]
pub fn ButtonGroup(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <div class=classes.add("leptonic-btn-group") style=styles>{children()}</div> }
}

/// Lays out separate buttons in a row that keeps their spacing and wraps when space runs out.
#[component]
pub fn ButtonWrapper(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <div class=classes.add("leptonic-btn-wrapper") style=styles>{children()}</div> }
}

#[component]
#[allow(clippy::needless_pass_by_value)] // title: Option<AttributeValue>
#[allow(unused_variables)] // `active` is a planned prop not yet used
pub fn LinkButton<H>(
    href: H,
    #[prop(into, optional)] target: Option<LinkTarget>,
    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    #[prop(into, optional)] color: Signal<ButtonColor>,
    #[prop(into, optional)] size: Signal<ButtonSize>,
    #[prop(into, optional)] disabled: Option<Signal<bool>>,
    #[prop(into, optional)] active: Option<Signal<bool>>, // TODO: Use
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
    atoms::button::LinkButton(atoms::button::LinkButtonProps {
        href,
        target,
        is_disabled: disabled.unwrap_or_default(),
        aria_haspopup,
        aria_expanded,
        exact,
        children,
        on_hover_start,
        on_hover_end,
        classes: classes.add("leptonic-btn"),
        styles,
    })
    .into_view()
    .attr("data-variant", move || {
        Oco::Borrowed(variant.get().as_str())
    })
    .attr("data-color", move || Oco::Borrowed(color.get().as_str()))
    .attr("data-size", move || Oco::Borrowed(size.get().as_str()))
}
