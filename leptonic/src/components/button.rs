use std::fmt::{Display, Formatter};

use leptos::prelude::*;
use leptos_router::components::ToHref;

use crate::atoms::button::LinkTarget;
use crate::{atoms, hooks::{
    interactions::use_hover::{HoverEndEvent, HoverStartEvent},
    interactions::use_press::PressEvent,
}, utils::aria::{AriaExpanded, AriaHasPopup}};

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

#[component]
pub fn Button(
    #[prop(into)] on_press: Callback<(PressEvent,)>,
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    #[prop(into, optional)] color: Signal<ButtonColor>,
    #[prop(into, optional)] size: Signal<ButtonSize>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] aria_haspopup: Signal<AriaHasPopup>,
    #[prop(into, optional)] aria_expanded: Signal<AriaExpanded>,
    children: Children,
) -> impl IntoView {
    view! {
        <atoms::button::Button
            on_press=on_press
            disabled=disabled
            aria_haspopup=aria_haspopup
            aria_expanded=aria_expanded
            attr:data-variant=move || variant.get().as_str()
            attr:data-color=move || color.get().as_str()
            attr:data-size=move || size.get().as_str()
        >
            { children() }
        </atoms::button::Button>
    }
}

#[component]
pub fn ButtonGroup(children: Children) -> impl IntoView {
    view! {
        <leptonic-btn-group>
            { children() }
        </leptonic-btn-group>
    }
}

#[component]
pub fn ButtonWrapper(children: Children) -> impl IntoView {
    view! {
        <leptonic-btn-wrapper>
            { children() }
        </leptonic-btn-wrapper>
    }
}

#[component]
#[allow(clippy::needless_pass_by_value)] // title: Option<AttributeValue>
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
    #[prop(into, optional)] aria_haspopup: Option<Signal<AriaHasPopup>>,
    #[prop(into, optional)] aria_expanded: Option<Signal<AriaExpanded>>,
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
        disabled,
        aria_haspopup,
        aria_expanded,
        exact,
        children,
        on_hover_start,
        on_hover_end,
    })
    .into_view()
        .attr(
            "data-variant",
           move || {
                   Oco::Borrowed(variant.get().as_str())
               }
        )
        .attr(
            "data-color",
           move || {
                   Oco::Borrowed(color.get().as_str())
               }
        )
        .attr(
            "data-size",
           move || {
                   Oco::Borrowed(size.get().as_str())
               }
        )
}
