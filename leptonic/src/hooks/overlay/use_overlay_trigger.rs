use educe::Educe;
use leptos::attr;
use leptos::attr::{Attr, IntoAttributeValue};
use leptos::oco::Oco;
use leptos::prelude::*;

use crate::{
    prelude::{AriaExpanded, AriaHasPopup},
    utils::aria::AriaControls,
};

#[derive(Debug, Clone)]
pub struct UseOverlayTriggerInput {
    /// Whether the overlay is currently shown.
    pub show: Signal<bool>,

    pub overlay_id: Oco<'static, str>,

    /// The type of overlay opened by this trigger.
    /// Using the variants `False` or `True` will result in a runtime warning on debug builds!
    /// Prefer `AriaHasPopup::Menu` if you are unsure what to use otherwise.
    pub overlay_type: AriaHasPopup,
}

#[derive(Debug)]
pub struct UseOverlayTriggerReturn {
    /// Props for the trigger.
    pub attrs: UseOverlayTriggerAttrs,
}

/// These attributes must be spread onto the target element: `<foo {..attrs} />`
pub type UseOverlayTriggerAttrs = (
    Attr<attr::AriaHaspopup, &'static str>,
    Attr<attr::AriaExpanded, Signal<&'static str>>,
    Attr<attr::AriaControls, Signal<Option<String>>>,
);

#[derive(Educe)]
#[educe(Debug)]
pub struct UseOverlayTriggerOverlayProps {
    /// These attributes must be spread onto the target element: `<foo {..attrs} />`
    pub attrs: (),
}

pub fn use_overlay_trigger(input: UseOverlayTriggerInput) -> UseOverlayTriggerReturn {
    #[cfg(debug_assertions)]
    fn get_overlay_type(input: &UseOverlayTriggerInput) -> AriaHasPopup {
        match input.overlay_type {
            unexpected @ AriaHasPopup::False | unexpected @ AriaHasPopup::True => {
                tracing::warn!(?unexpected, "use_overlay_trigger received unexpected AriaHasPopup variant. Do not use `False` or `True`.");
                unexpected
            }
            other => other,
        }
    }
    #[cfg(not(debug_assertions))]
    fn get_overlay_type(input: &UseOverlayTriggerInput) -> AriaHasPopup {
        input.overlay_type
    }
    let aria_has_popup = get_overlay_type(&input);

    let overlay_id = input.overlay_id;

    UseOverlayTriggerReturn {
        attrs: (
            Attr(attr::AriaHaspopup, aria_has_popup.into_attribute_value()),
            Attr(
                attr::AriaExpanded,
                Signal::derive(move || {
                    AriaExpanded::from(input.show.get()).into_attribute_value()
                })
            ),
            Attr(
                attr::AriaControls,
                Signal::derive(move ||
                    match input.show.get() {
                        true => AriaControls::Id(vec![overlay_id.to_string()]),
                        false => AriaControls::Undefined,
                    }.into_attribute_value()
                )
            ),
        ),
    }
}
