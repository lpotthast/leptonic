use leptos::*;
use leptos_icons::*;

use crate::{prelude::*, Bool};

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
    #[prop(optional_no_strip)] centered: Bool,
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
            <div class="content" class:centered=Signal::derive(cx, move || match centered {
                Bool::Static(val) => val,
                Bool::Reactive(sig) => sig.get(),
            })>
                <div class="title">
                    { title.into_view(cx) }
                </div>
                { children(cx) }
            </div>
        </leptonic-alert>
    }
}
