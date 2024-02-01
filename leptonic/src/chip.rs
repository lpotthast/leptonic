use std::fmt::{Display, Formatter};

use leptos::*;
use web_sys::MouseEvent;

use crate::{icon::Icon, OptionalMaybeSignal};

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
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Success => "success",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Danger => "danger",
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
    #[prop(into, optional)] color: OptionalMaybeSignal<ChipColor>,
    #[prop(into, optional)] dismissible: Option<Callback<MouseEvent>>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <leptonic-chip id=id class=class style=style data-color=move || color.0.as_ref().map_or_else(ChipColor::default, SignalGet::get).as_str()>
            { children() }
            { match dismissible {
                Some(callback) => view! {
                    <Icon class="dismiss" icon=icondata::BsXCircleFill on:click=move |e| callback.call(e) />
                }.into_view(),
                None => ().into_view(),
            } }
        </leptonic-chip>
    }
}
