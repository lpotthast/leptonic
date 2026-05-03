use leptos::prelude::*;
use leptos_classes::Classes;
use leptos_styles::Styles;

use super::icon::Icon;

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
    pub style: Option<String>,
}

#[slot]
pub struct AlertAppend {
    pub children: Children,

    #[prop(into, optional)]
    pub style: Option<String>,
}

#[slot]
pub struct AlertTitle {
    pub children: Children,

    #[prop(into, optional)]
    pub style: Option<String>,
}

#[slot]
pub struct AlertContent {
    pub children: Children,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertIconSlot {
    None,
    #[default]
    Prepend,
    Append,
}

// TODO (new): Is the usage of into_any() ok? Should we do this differently?
#[component]
pub fn Alert(
    variant: AlertVariant,
    #[prop(optional)] alert_prepend: Option<AlertPrepend>,
    #[prop(optional)] alert_title: Option<AlertTitle>,
    #[prop(optional)] alert_content: Option<AlertContent>,
    #[prop(optional)] alert_append: Option<AlertAppend>,
    #[prop(optional)] default_icon_slot: AlertIconSlot,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    view! {
        <div class=classes.add("leptonic-alert") style=styles data-variant=variant.to_str()>
            {match alert_prepend {
                Some(slot) => {
                    view! {
                        <div class="leptonic-alert-prepend" style=slot.style>
                            {(slot.children)()}
                        </div>
                    }
                        .into_any()
                }
                None => {
                    match default_icon_slot {
                        AlertIconSlot::Prepend => {
                            view! {
                                <div class="leptonic-alert-prepend">
                                    <AlertIcon variant />
                                </div>
                            }
                                .into_any()
                        }
                        AlertIconSlot::Append | AlertIconSlot::None => {
                            view! { <div class="leptonic-alert-prepend" /> }.into_any()
                        }
                    }
                }
            }}
            <div class="leptonic-alert-center">
                {match alert_title {
                    Some(slot) => {
                        view! {
                            <div class="leptonic-alert-title" style=slot.style>
                                {(slot.children)()}
                            </div>
                        }
                            .into_any()
                    }
                    None => ().into_any(),
                }}
                {match alert_content {
                    Some(slot) => {
                        view! { <div class="leptonic-alert-content">{(slot.children)()}</div> }
                            .into_any()
                    }
                    None => ().into_any(),
                }}
            </div>
            {match alert_append {
                Some(slot) => {
                    view! {
                        <div class="leptonic-alert-append" style=slot.style>
                            {(slot.children)()}
                        </div>
                    }
                        .into_any()
                }
                None => {
                    match default_icon_slot {
                        AlertIconSlot::Prepend | AlertIconSlot::None => {
                            view! { <div class="leptonic-alert-append" /> }.into_any()
                        }
                        AlertIconSlot::Append => {
                            view! {
                                <div class="leptonic-alert-append">
                                    <AlertIcon variant />
                                </div>
                            }
                                .into_any()
                        }
                    }
                }
            }}
        </div>
    }
}

#[component]
pub fn AlertIcon(variant: AlertVariant) -> impl IntoView {
    match variant {
        AlertVariant::Success => view! { <Icon icon=icondata::BsCheckCircleFill /> },
        AlertVariant::Info => view! { <Icon icon=icondata::BsInfoCircleFill /> },
        AlertVariant::Warn => {
            view! { <Icon icon=icondata::BsExclamationCircleFill /> }
        }
        AlertVariant::Danger => {
            view! { <Icon icon=icondata::BsExclamationTriangleFill /> }
        }
    }
}
