use leptos::{
    attr,
    attr::{Attr, IntoAttributeValue},
    oco::Oco,
    prelude::*,
};

use crate::{
    hooks::IntoAttrs,
    prelude::{AriaExpanded, AriaHasPopup},
    utils::aria::AriaControls,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
    /// Props for the trigger. Call `.into_attrs()` for view spreading.
    pub props: UseOverlayTriggerProps,
}

/// Props from `use_overlay_trigger` that can be converted to spreadable attributes.
#[derive(Debug)]
pub struct UseOverlayTriggerProps {
    pub aria_haspopup: AriaHasPopup,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_controls: Signal<Option<String>>,
}

impl IntoAttrs for UseOverlayTriggerProps {
    type Attrs = UseOverlayTriggerAttrs;

    fn into_attrs(self) -> Self::Attrs {
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
    fn validate_overlay_type(overlay_type: AriaHasPopup) -> AriaHasPopup {
        match overlay_type {
            unexpected @ (AriaHasPopup::False | AriaHasPopup::True) => {
                tracing::warn!(?unexpected, "use_overlay_trigger received unexpected AriaHasPopup variant. Do not use `False` or `True`.");
                unexpected
            }
            other => other,
        }
    }
    #[cfg(not(debug_assertions))]
    fn validate_overlay_type(overlay_type: AriaHasPopup) -> AriaHasPopup {
        overlay_type
    }

    let UseOverlayTriggerInput {
        show,
        overlay_id,
        overlay_type,
    } = input;

    let aria_has_popup = validate_overlay_type(overlay_type);

    UseOverlayTriggerReturn {
        props: UseOverlayTriggerProps {
            aria_haspopup: aria_has_popup,
            aria_expanded: Signal::derive(move || Some(AriaExpanded::from(show.get()))),
            aria_controls: Signal::derive(move || {
                show.get()
                    .then(|| AriaControls(vec![overlay_id.to_string()]).into_attribute_value())
            }),
        },
    }
}
