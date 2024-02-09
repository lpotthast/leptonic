use std::fmt::{Display, Formatter};

use leptos::*;
use leptos_router::{State, ToHref, A};

use crate::{
    hooks::press::PressEvent, prelude::Consumer, utils::aria::{AriaExpanded, AriaHasPopup}, OptMaybeSignal
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

#[component]
pub fn Button(
    #[prop(into)] on_press: Consumer<PressEvent>,
    #[prop(into, optional)] variant: OptMaybeSignal<ButtonVariant>,
    #[prop(into, optional)] color: OptMaybeSignal<ButtonColor>,
    #[prop(into, optional)] size: OptMaybeSignal<ButtonSize>,
    #[prop(into, optional)] disabled: OptMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] aria_haspopup: OptMaybeSignal<AriaHasPopup>,
    #[prop(into, optional)] aria_expanded: OptMaybeSignal<AriaExpanded>,
    children: Children,
) -> impl IntoView {
    view! {
        <crate::atoms::button::Button
            on_press=on_press
            disabled=disabled
            aria_haspopup=aria_haspopup
            aria_expanded=aria_expanded
            id=id
            class=class
            style=style
            attr:data-variant=move || variant.get().as_str()
            attr:data-color=move || color.get().as_str()
            attr:data-size=move || size.get().as_str()
        >
            { children() }
        </crate::atoms::button::Button>
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
    #[prop(into, optional)] variant: OptMaybeSignal<ButtonVariant>,
    #[prop(into, optional)] color: OptMaybeSignal<ButtonColor>,
    #[prop(into, optional)] size: OptMaybeSignal<ButtonSize>,
    #[prop(into, optional)] disabled: OptMaybeSignal<bool>,
    #[prop(into, optional)] active: OptMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
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
    children: Children,
) -> impl IntoView
where
    H: ToHref + 'static,
{
    view! {
        <leptonic-link
            id=id
            class=move || {
                let user = class.get();
                let active = active.get().then_some("active").unwrap_or_default();
                format!("leptonic-btn {user} {active}")
            }
            data-variant=move || variant.get().as_str()
            data-color=move || color.get().as_str()
            data-size=move || size.get().as_str()
            aria-disabled=move || match disabled.get() {
                true => "true",
                false => "false",
            }
            style=style
        >
            <A href=href exact=exact state=state.unwrap_or_default() replace=replace>
                <div class="name">
                    { children() }
                </div>
            </A>
        </leptonic-link>
    }
}