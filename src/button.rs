use leptos::{ev::MouseEvent, *};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Flat,
    Outlined,
    Filled,
}

impl ButtonVariant {
    pub fn class_name(&self) -> &'static str {
        match self {
            ButtonVariant::Flat => "flat",
            ButtonVariant::Outlined => "outlined",
            ButtonVariant::Filled => "filled",
        }
    }
}

impl std::fmt::Display for ButtonVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.class_name())
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
    pub fn class_name(&self) -> &'static str {
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

impl std::fmt::Display for ButtonColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.class_name())
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
    pub fn class_name(&self) -> &'static str {
        match self {
            BtnSize::Small => "small",
            BtnSize::Normal => "normal",
            BtnSize::Big => "big",
        }
    }
}

impl std::fmt::Display for BtnSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.class_name())
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
        <button class=format!("leptonic-btn {variant} {color} {size}") on:click=on_click>
            <div class="name">
                { children(cx) }
            </div>
        </button>
    }
}

#[component]
pub fn ButtonGroup(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="leptonic-btn-group">
            { children(cx) }
        </div>
    }
}

#[component]
pub fn ButtonWrapper(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="leptonic-btn-wrapper">
            { children(cx) }
        </div>
    }
}
