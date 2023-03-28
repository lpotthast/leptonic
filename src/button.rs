use leptos::{ev::MouseEvent, *};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Info,
    Warn,
    Danger,
}

impl ButtonVariant {
    pub fn class_name(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "type-primary",
            ButtonVariant::Secondary => "type-secondary",
            ButtonVariant::Success => "type-success",
            ButtonVariant::Info => "type-info",
            ButtonVariant::Warn => "type-warn",
            ButtonVariant::Danger => "type-danger",
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
    #[prop(optional)] size: BtnSize,
    children: Children,
) -> impl IntoView
where
    F: FnMut(MouseEvent) + 'static,
{
    view! { cx,
        <button class=format!("crud-btn {variant} {size}") on:click=on_click>
            <div class="name">
                { children(cx) }
            </div>
        </button>
    }
}

#[component]
pub fn ButtonGroup(
    cx: Scope,
    children: Children,
) -> impl IntoView{
    view! { cx,
        <div class="crud-btn-group">
            { children(cx) }
        </div>
    }
}

#[component]
pub fn ButtonWrapper(
    cx: Scope,
    children: Children,
) -> impl IntoView{
    view! { cx,
        <div class="crud-btn-wrapper">
            { children(cx) }
        </div>
    }
}
