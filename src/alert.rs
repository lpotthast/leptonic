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
    pub fn class_name(&self) -> &'static str {
        match self {
            AlertVariant::Success => "success",
            AlertVariant::Info => "info",
            AlertVariant::Warn => "warn",
            AlertVariant::Danger => "danger",
        }
    }
}

impl std::fmt::Display for AlertVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.class_name())
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
        <div class=format!("alert {variant}")>
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
        </div>
    }
}
