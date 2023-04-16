use std::fmt::{Display, Formatter};

use leptos::{ev::MouseEvent, *};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Flat,
    Outlined,
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

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Filled
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonColor {
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

impl Default for ButtonColor {
    fn default() -> Self {
        Self::Primary
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BtnSize {
    Small,
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

impl Default for BtnSize {
    fn default() -> Self {
        Self::Normal
    }
}

#[component]
pub fn Button<F>(
    cx: Scope,
    on_click: F,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] color: ButtonColor,
    #[prop(optional)] size: BtnSize,
    children: Children,
) -> impl IntoView
where
    F: FnMut(MouseEvent) + 'static,
{
    view! { cx,
        <button class="leptonic-btn"
            data-variant=variant.as_str()
            data-color=color.as_str()
            data-size=size.as_str()
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
