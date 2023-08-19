use leptos::*;
use leptos_icons::BsIcon;

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

// TODO: Use Slots for title and body when available.
// TODO: Icon (or entire prepend) must be overridable.
#[component]
pub fn Alert<V>(
    cx: Scope,
    #[prop(into)] variant: MaybeSignal<AlertVariant>,
    #[prop(into)] title: Callback<Scope, V>,
    #[prop(into, optional)] centered: OptionalMaybeSignal<bool>,
    children: Children,
) -> impl IntoView
where
    V: IntoView + 'static,
{
    view! { cx,
        <leptonic-alert data-variant=move || variant.get().to_str()>
            <div class="prepend">
                {move || match variant.get() {
                    AlertVariant::Success => view!{cx, <Icon icon=BsIcon::BsCheckCircleFill /> },
                    AlertVariant::Info => view!{cx, <Icon icon=BsIcon::BsInfoCircleFill /> },
                    AlertVariant::Warn => view!{cx, <Icon icon=BsIcon::BsExclamationCircleFill /> },
                    AlertVariant::Danger => view!{cx, <Icon icon=BsIcon::BsExclamationTriangleFill /> },
                }}
            </div>
            <div class="content" class:centered=move || centered.0.as_ref().map(|it| it.get()).unwrap_or(false)>
                <div class="title">
                    { move || title.call(cx) }
                </div>
                { children(cx) }
            </div>
        </leptonic-alert>
    }
}
