use leptos::*;
use uuid::Uuid;

use crate::{Disabled, Active};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckboxVariant {
    Primary,
    Secondary,
    Success,
    Info,
    Warn,
    Danger,
}

impl CheckboxVariant {
    pub fn class_name(&self) -> &'static str {
        match self {
            CheckboxVariant::Primary => "type-primary",
            CheckboxVariant::Secondary => "type-secondary",
            CheckboxVariant::Success => "type-success",
            CheckboxVariant::Info => "type-info",
            CheckboxVariant::Warn => "type-warn",
            CheckboxVariant::Danger => "type-danger",
        }
    }
}

impl std::fmt::Display for CheckboxVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.class_name())
    }
}

impl Default for CheckboxVariant {
    fn default() -> Self {
        Self::Primary
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckboxSize {
    Small,
    Normal,
    Big,
}

impl CheckboxSize {
    pub fn class_name(&self) -> &'static str {
        match self {
            CheckboxSize::Small => "small",
            CheckboxSize::Normal => "normal",
            CheckboxSize::Big => "big",
        }
    }
}

impl std::fmt::Display for CheckboxSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.class_name())
    }
}

impl Default for CheckboxSize {
    fn default() -> Self {
        Self::Normal
    }
}

#[component]
pub fn Checkbox(
    cx: Scope,
    checked: (ReadSignal<bool>, WriteSignal<bool>),
    #[prop(optional)] active: Option<Active>,
    #[prop(optional)] disabled: Option<Disabled>,
    #[prop(optional)] id: Option<Uuid>,
    #[prop(optional)] variant: CheckboxVariant,
    #[prop(optional)] size: CheckboxSize,
) -> impl IntoView {
    let id = id.unwrap_or_else(|| Uuid::new_v4());
    view! { cx,
        <leptonic-checkbox>
            <input
                type="checkbox"
                id=id.to_string()
                class=format!("{} {}", variant, size)
                class:active=move || active.map(|it| match it {
                    Active::Static(active) => active,
                    Active::Reactive(active) => active.get(),
                }).unwrap_or(true)
                class:disabled=move || disabled.map(|it| match it {
                    Disabled::Static(disabled) => disabled,
                    Disabled::Reactive(disabled) => disabled.get(),
                }).unwrap_or(false)
                on:click=move |_| checked.1.update(|c| *c = !*c)
                prop:checked=checked.0
            />
        </leptonic-checkbox>
    }
}
