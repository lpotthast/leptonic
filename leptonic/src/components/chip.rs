use std::fmt::{Display, Formatter};

use leptos::prelude::*;

use crate::{
    atoms::button::Button,
    components::icon::Icon,
    utils::{classes::Classes, styles::Styles},
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

/// A small label, optionally with a button dismissing it.
#[component]
pub fn Chip(
    #[prop(into, optional)] color: Signal<ChipColor>,
    /// Called when the dismiss button is pressed. With it, the chip renders a dismiss button.
    #[prop(into, optional)]
    on_dismiss: Option<Callback<()>>,
    /// Names the dismiss button. Default: "Dismiss".
    #[prop(into, optional)]
    dismiss_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let dismiss_label = MaybeProp::derive(move || {
        Some(dismiss_label.get().unwrap_or_else(|| "Dismiss".to_owned()))
    });
    view! {
        <div class=classes.add("leptonic-chip") style=styles data-color=move || color.get().as_str()>
            {children()}
            {on_dismiss.map(|on_dismiss| view! {
                <Button
                    classes="leptonic-chip-dismiss"
                    aria_label=dismiss_label
                    on_press=move |_| on_dismiss.run(())
                >
                    <Icon icon=icondata::BsXCircleFill />
                </Button>
            })}
        </div>
    }
}
