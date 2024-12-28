use std::fmt::{Display, Formatter};

use leptos::prelude::*;
use web_sys::MouseEvent;

use crate::{components::icon::Icon, Out};

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
    #[prop(into, optional)] color: Option<Signal<ChipColor>>,
    #[prop(into, optional)] dismissible: Option<Out<MouseEvent, LocalStorage>>,
    children: Children,
) -> impl IntoView {
    let color = color.unwrap_or_default();

    // TODO: use use_press instead of on:click.
    view! {
        <leptonic-chip data-color=move || color.get().as_str()>
            { children() }
            { match dismissible {
                Some(callback) => view! {
                    <Icon attr:class="dismiss" icon=icondata::BsXCircleFill on:click=move |e| callback.set(e) />
                }.into_any(),
                None => ().into_any(),
            } }
        </leptonic-chip>
    }
}
