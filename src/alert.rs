use leptos::*;
use leptos_icons::*;

use crate::Bool;

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
    #[prop(optional_no_strip)] dismissible: Bool,
    children: Children,
) -> impl IntoView
where
    T: IntoView + 'static,
{
    view! { cx,
        <div class=format!("alert {variant}")
            class:centered=Signal::derive(cx, move || match centered {
                Bool::Static(val) => val,
                Bool::Reactive(sig) => sig.get(),
            })
            class:dismissible=Signal::derive(cx, move || match centered {
                Bool::Static(val) => val,
                Bool::Reactive(sig) => sig.get(),
            })
            >
            <div class="prepend">
                <LeptosIcon icon=BsIcon::BsCheckCircleFill width="2em" height="2em" />
            </div>
            <div class="content">
                <div class="title">
                    { title.into_view(cx) }
                </div>
                { children(cx) }
            </div>
        </div>
    }
}
