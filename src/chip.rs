use std::fmt::{Display, Formatter};

use leptos::*;

use crate::OptionalMaybeSignal;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ChipColor {
    #[default]
    Primary,
    Secondary,
    Success,
    Info,
    Warn,
    Danger,
}

impl ChipColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChipColor::Primary => "primary",
            ChipColor::Secondary => "secondary",
            ChipColor::Success => "success",
            ChipColor::Info => "info",
            ChipColor::Warn => "warn",
            ChipColor::Danger => "danger",
        }
    }
}

impl Display for ChipColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[component]
pub fn Chip(
    cx: Scope,
    #[prop(into, optional)] color: OptionalMaybeSignal<ChipColor>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <leptonic-chip color=move || color.0.as_ref().map(|it| it()).unwrap_or(Default::default()).as_str()>
            { children(cx) }
        </leptonic-chip>
    }
}
