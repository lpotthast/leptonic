use leptos::*;
use uuid::Uuid;

use crate::{OptionalMaybeSignal, Out};

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
    pub const fn class_name(&self) -> &'static str {
        match self {
            Self::Primary => "type-primary",
            Self::Secondary => "type-secondary",
            Self::Success => "type-success",
            Self::Info => "type-info",
            Self::Warn => "type-warn",
            Self::Danger => "type-danger",
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
    pub const fn class_name(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Normal => "normal",
            Self::Big => "big",
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
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] set_checked: Out<bool>,
    #[prop(into, optional)] active: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(optional)] id: Option<Uuid>,
    #[prop(optional)] variant: CheckboxVariant,
    #[prop(optional)] size: CheckboxSize,
) -> impl IntoView {
    let id = id.unwrap_or_else(Uuid::new_v4);
    view! {
        <leptonic-checkbox>
            <input
                type="checkbox"
                id=id.to_string()
                class=format!("{} {}", variant, size)
                class:active=move || active.0.as_ref().map_or(true, SignalGet::get)
                // TODO: Use aria-disabled instead?
                class:disabled=move || disabled.0.as_ref().map_or(false, SignalGet::get)
                on:click=move |_| set_checked.set(!checked.get_untracked())
                prop:checked=move || checked.get()
            />
        </leptonic-checkbox>
    }
}
