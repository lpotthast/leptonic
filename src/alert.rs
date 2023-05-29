use leptos::*;
use leptos_icons::*;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertVariant {
    Success,
    Info,
    Warn,
    Danger,
}

impl AlertVariant {
    pub fn to_str(&self) -> &'static str {
        match self {
            AlertVariant::Success => "success",
            AlertVariant::Info => "info",
            AlertVariant::Warn => "warn",
            AlertVariant::Danger => "danger",
        }
    }
}

#[component]
pub fn Alert<T>(
    cx: Scope,
    variant: AlertVariant,
    title: T,
    #[prop(into, optional)] centered: OptionalMaybeSignal<bool>,
    children: Children,
) -> impl IntoView
where
    T: IntoView + 'static,
{
    view! { cx,
        <leptonic-alert variant=variant.to_str()>
            <div class="prepend">
                <Icon icon=BsIcon::BsCheckCircleFill />
            </div>
            <div class="content" class:centered=centered.0.as_ref().map(|it| it.get()).unwrap_or(false)>
                <div class="title">
                    { title.into_view(cx) }
                </div>
                { children(cx) }
            </div>
        </leptonic-alert>
    }
}
