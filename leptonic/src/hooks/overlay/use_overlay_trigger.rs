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
    /// Props for the trigger. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseOverlayTriggerProps,
}

/// Props from `use_overlay_trigger` that can be converted to spreadable attributes.
#[derive(Debug, Clone)]
pub struct UseOverlayTriggerProps {
    pub aria_haspopup: AriaHasPopup,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_controls: Signal<Option<String>>,
}

impl UseOverlayTriggerProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseOverlayTriggerAttrs {
        (
            Attr(attr::AriaHaspopup, self.aria_haspopup),
            Attr(attr::AriaExpanded, self.aria_expanded),
            Attr(attr::AriaControls, self.aria_controls),
        )
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseOverlayTriggerAttrs {
        (
            Attr(attr::AriaHaspopup, self.aria_haspopup),
            Attr(attr::AriaExpanded, self.aria_expanded),
            Attr(attr::AriaControls, self.aria_controls),
        )
    }
}

/// These attributes must be spread onto the target element: `<foo {..attrs} />`
pub type UseOverlayTriggerAttrs = (
    Attr<attr::AriaHaspopup, AriaHasPopup>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
    Attr<attr::AriaControls, Signal<Option<String>>>,
);

pub fn use_overlay_trigger(input: UseOverlayTriggerInput) -> UseOverlayTriggerReturn {
    #[cfg(debug_assertions)]
    fn get_overlay_type(input: &UseOverlayTriggerInput) -> AriaHasPopup {
        match input.overlay_type {
            unexpected @ (AriaHasPopup::False | AriaHasPopup::True) => {
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
        props: UseOverlayTriggerProps {
            aria_haspopup: aria_has_popup,
            aria_expanded: Signal::derive(move || Some(AriaExpanded::from(input.show.get()))),
            aria_controls: Signal::derive(move || {
                input
                    .show
                    .get()
                    .then(|| AriaControls(vec![overlay_id.to_string()]).into_attribute_value())
            }),
        },
    }
}
