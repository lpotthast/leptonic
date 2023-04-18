use std::fmt::{Display, Formatter};

use leptos::{ev::MouseEvent, *};

use crate::OptionalMaybeSignal;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Flat,
    Outlined,
    #[default]
    Filled,
}

impl ButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonVariant::Flat => "flat",
            ButtonVariant::Outlined => "outlined",
            ButtonVariant::Filled => "filled",
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
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonColor::Primary => "primary",
            ButtonColor::Secondary => "secondary",
            ButtonColor::Success => "success",
            ButtonColor::Info => "info",
            ButtonColor::Warn => "warn",
            ButtonColor::Danger => "danger",
        }
    }
}

impl Display for ButtonColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BtnSize {
    Small,
    #[default]
    Normal,
    Big,
}

impl BtnSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            BtnSize::Small => "small",
            BtnSize::Normal => "normal",
            BtnSize::Big => "big",
        }
    }
}

impl Display for BtnSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[component]
pub fn Button<F>(
    cx: Scope,
    on_click: F,
    #[prop(into, optional)] variant: OptionalMaybeSignal<ButtonVariant>,
    #[prop(into, optional)] color: OptionalMaybeSignal<ButtonColor>,
    #[prop(into, optional)] size: OptionalMaybeSignal<BtnSize>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    children: Children,
) -> impl IntoView
where
    F: FnMut(MouseEvent) + 'static,
{
    view! { cx,
        <button class="leptonic-btn"
            data-variant=move || variant.0.as_ref().map(|it| it()).unwrap_or(Default::default()).as_str()
            data-color=move || color.0.as_ref().map(|it| it()).unwrap_or(Default::default()).as_str()
            data-size=move || size.0.as_ref().map(|it| it()).unwrap_or(Default::default()).as_str()
            aria-disabled=move || disabled.0.as_ref().map(|it| it()).unwrap_or(false)
            on:click=on_click
        >
            <div class="name">
                { children(cx) }
            </div>
        </button>
    }
}

#[component]
pub fn ButtonGroup(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <leptonic-btn-group>
            { children(cx) }
        </leptonic-btn-group>
    }
}

#[component]
pub fn ButtonWrapper(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <leptonic-btn-wrapper>
            { children(cx) }
        </leptonic-btn-wrapper>
    }
}
