use leptos::prelude::*;

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

// TODO (new): Is the usage of into_any() ok? Should we do this differently?
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
                        <leptonic-alert-prepend style=slot.style>
                            { (slot.children)() }
                        </leptonic-alert-prepend>
                    }.into_any(),
                    None => match default_icon_slot {
                        AlertIconSlot::Prepend => view! {
                            <leptonic-alert-prepend>
                                <AlertIcon variant />
                            </leptonic-alert-prepend>
                        }.into_any(),
                        AlertIconSlot::Append | AlertIconSlot::None => view! {
                            <leptonic-alert-prepend />
                        }.into_any(),
                    },
                }
            }

            <leptonic-alert-center>
                {
                    match alert_title {
                        Some(slot) => view! {
                            <leptonic-alert-title style=slot.style>
                                {(slot.children)()}
                            </leptonic-alert-title>
                        }.into_any() ,
                        None => ().into_any(),
                    }
                }
                {
                    match alert_content {
                        Some(slot) => view! {
                            <leptonic-alert-content>
                                {(slot.children)()}
                            </leptonic-alert-content>
                        }.into_any() ,
                        None => ().into_any(),
                    }
                }
            </leptonic-alert-center>

            {
                match alert_append {
                    Some(slot) => view! {
                        <leptonic-alert-append style=slot.style>
                            { (slot.children)() }
                        </leptonic-alert-append>
                    }.into_any(),
                    None => match default_icon_slot {
                        AlertIconSlot::Prepend | AlertIconSlot::None => view! {
                            <leptonic-alert-append />
                        }.into_any(),
                        AlertIconSlot::Append => view! {
                            <leptonic-alert-append>
                                <AlertIcon variant />
                            </leptonic-alert-append>
                        }.into_any(),
                    },
                }
            }
        </leptonic-alert>
    }
}

#[component]
pub fn AlertIcon(
    variant: AlertVariant,
) -> impl IntoView {
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
