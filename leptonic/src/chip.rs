use std::fmt::{Display, Formatter};

use leptos::*;
use leptos_icons::BsIcon;
use web_sys::MouseEvent;

use crate::{
    icon::Icon,
    prelude::{Callable, Callback},
    OptionalMaybeSignal,
};

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
    #[prop(into, optional)] dismissible: Option<Callback<MouseEvent>>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <leptonic-chip id=id class=class style=style data-color=move || color.0.as_ref().map(|it| it.get()).unwrap_or(Default::default()).as_str()>
            { children(cx) }
            { match dismissible {
                Some(callback) => view! {cx,
                    <Icon class="dismiss" icon=BsIcon::BsXCircleFill on:click=move |e| callback.call(e) />
                }.into_view(cx),
                None => ().into_view(cx),
            } }
        </leptonic-chip>
    }
}
