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
    pub const fn to_str(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Danger => "danger",
        }
    }
}

#[slot]
pub struct AlertPrepend {
    pub children: Children,
    #[prop(into, optional)]
    pub id: Option<AttributeValue>,
    #[prop(into, optional)]
    pub class: Option<AttributeValue>,
    #[prop(into, optional)]
    pub style: Option<AttributeValue>,
}

#[slot]
pub struct AlertAppend {
    pub children: Children,
    #[prop(into, optional)]
    pub id: Option<AttributeValue>,
    #[prop(into, optional)]
    pub class: Option<AttributeValue>,
    #[prop(into, optional)]
    pub style: Option<AttributeValue>,
}

#[slot]
pub struct AlertTitle {
    pub children: Children,
    #[prop(into, optional)]
    pub id: Option<AttributeValue>,
    #[prop(into, optional)]
    pub class: Option<AttributeValue>,
    #[prop(into, optional)]
    pub style: Option<AttributeValue>,
}

#[slot]
pub struct AlertContent {
    pub children: Children,
    #[prop(into, optional)]
    pub id: Option<AttributeValue>,
    #[prop(into, optional)]
    pub class: Option<AttributeValue>,
    #[prop(into, optional)]
    pub style: Option<AttributeValue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertIconSlot {
    None,
    Prepend,
    Append,
}

impl Default for AlertIconSlot {
    fn default() -> Self {
        Self::Prepend
    }
}

#[component]
pub fn Alert(
    variant: AlertVariant,
    #[prop(optional)] alert_prepend: Option<AlertPrepend>,
    #[prop(optional)] alert_title: Option<AlertTitle>,
    #[prop(optional)] alert_content: Option<AlertContent>,
    #[prop(optional)] alert_append: Option<AlertAppend>,
    #[prop(optional)] default_icon_slot: AlertIconSlot,
) -> impl IntoView {
    view! {
        <leptonic-alert data-variant=variant.to_str()>
            {
                match alert_prepend {
                    Some(slot) => view! {
                        <leptonic-alert-prepend id=slot.id class=slot.class style=slot.style>
                            { (slot.children)() }
                        </leptonic-alert-prepend>
                    }.into_view(),
                    None => match default_icon_slot {
                        AlertIconSlot::Prepend => view! {
                            <leptonic-alert-prepend>
                                <AlertIcon variant />
                            </leptonic-alert-prepend>
                        }.into_view(),
                        AlertIconSlot::Append | AlertIconSlot::None => view! {
                            <leptonic-alert-prepend />
                        }.into_view(),
                    },
                }
            }

            <leptonic-alert-center>
                {
                    match alert_title {
                        Some(slot) => view! {
                            <leptonic-alert-title id=slot.id class=slot.class style=slot.style>
                                {(slot.children)()}
                            </leptonic-alert-title>
                        }.into_view() ,
                        None => ().into_view(),
                    }
                }
                {
                    match alert_content {
                        Some(slot) => view! {
                            <leptonic-alert-content id=slot.id class=slot.class style=slot.style>
                                {(slot.children)()}
                            </leptonic-alert-content>
                        }.into_view() ,
                        None => ().into_view(),
                    }
                }
            </leptonic-alert-center>

            {
                match alert_append {
                    Some(slot) => view! {
                        <leptonic-alert-append id=slot.id class=slot.class style=slot.style>
                            { (slot.children)() }
                        </leptonic-alert-append>
                    }.into_view(),
                    None => match default_icon_slot {
                        AlertIconSlot::Prepend | AlertIconSlot::None => view! {
                            <leptonic-alert-append />
                        }.into_view(),
                        AlertIconSlot::Append => view! {
                            <leptonic-alert-append>
                                <AlertIcon variant />
                            </leptonic-alert-append>
                        }.into_view(),
                    },
                }
            }
        </leptonic-alert>
    }
}

#[component]
pub fn AlertIcon(
    variant: AlertVariant,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    match variant {
        AlertVariant::Success => view! { <Icon id=id class style icon=BsIcon::BsCheckCircleFill /> },
        AlertVariant::Info => view! { <Icon id class style icon=BsIcon::BsInfoCircleFill /> },
        AlertVariant::Warn => {
            view! { <Icon id class style icon=BsIcon::BsExclamationCircleFill /> }
        }
        AlertVariant::Danger => {
            view! { <Icon id class style icon=BsIcon::BsExclamationTriangleFill /> }
        }
    }
}
