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
// 1. No `overlayProps` / ID generation: react-aria generates an ID for the
//    overlay and returns `overlayProps` with that ID. In leptonic, the ID is
//    generated in `use_overlay` instead, and passed here as `overlay_id`.
//
// 2. No `onPress` in trigger props: react-aria includes an `onPress` handler
//    in the trigger props that toggles the overlay. In leptonic, press handling
//    is the caller's responsibility (e.g., via `use_button` or `use_menu_trigger`).
//
// 3. No `onCloseMap` integration: react-aria integrates with a global close
//    handler map for overlay stacking. Leptonic handles dismiss differently
//    via `use_overlay`.
//
// =============================================================================

/// The type of overlay opened by a trigger.
///
/// This restricts the input to the 5 valid overlay types that react-aria supports,
/// rather than accepting the full `AriaHasPopup` enum (which includes `False` and `True`).
///
/// Note: `aria-haspopup` is only set for `Menu` and `Listbox`. For `Dialog`, `Tree`,
/// and `Grid`, the attribute is omitted because screen readers may misinterpret
/// non-menu values as "menu".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OverlayTriggerType {
    /// The overlay is a dialog (e.g., popover, modal).
    Dialog,
    /// The overlay is a menu.
    Menu,
    /// The overlay is a listbox.
    Listbox,
    /// The overlay is a tree.
    Tree,
    /// The overlay is a grid.
    Grid,
}

#[derive(Debug, Clone)]
pub struct UseOverlayTriggerInput {
    /// Whether the overlay is currently shown.
    pub show: Signal<bool>,

    pub overlay_id: Oco<'static, str>,

    /// The type of overlay opened by this trigger.
    pub overlay_type: OverlayTriggerType,
}

#[derive(Debug)]
pub struct UseOverlayTriggerReturn {
    /// Props for the trigger. Call `.into_attrs()` for view spreading.
    pub props: UseOverlayTriggerProps,
}

/// Props from `use_overlay_trigger` that can be converted to spreadable attributes.
#[derive(Debug)]
pub struct UseOverlayTriggerProps {
    pub aria_haspopup: Option<AriaHasPopup>,
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
    Attr<attr::AriaHaspopup, Option<AriaHasPopup>>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
    Attr<attr::AriaControls, Signal<Option<String>>>,
);

pub fn use_overlay_trigger(input: UseOverlayTriggerInput) -> UseOverlayTriggerReturn {
    let UseOverlayTriggerInput {
        show,
        overlay_id,
        overlay_type,
    } = input;

    // Match react-aria behavior: only set aria-haspopup for Menu and Listbox.
    // Screen readers may misinterpret other values (dialog, tree, grid) as "menu",
    // so we omit the attribute entirely for those types.
    let aria_haspopup = match overlay_type {
        OverlayTriggerType::Menu => Some(AriaHasPopup::True),
        OverlayTriggerType::Listbox => Some(AriaHasPopup::Listbox),
        OverlayTriggerType::Dialog
        | OverlayTriggerType::Tree
        | OverlayTriggerType::Grid => None,
    };

    UseOverlayTriggerReturn {
        props: UseOverlayTriggerProps {
            aria_haspopup,
            aria_expanded: Signal::derive(move || Some(AriaExpanded::from(show.get()))),
            aria_controls: Signal::derive(move || {
                show.get()
                    .then(|| AriaControls(vec![overlay_id.to_string()]).into_attribute_value())
            }),
        },
    }
}
