// Upstream: react-aria/src/overlays/useOverlayTrigger.ts @ 99e6102368
// Upstream: react-aria/test/overlays/useOverlayTrigger.test.js @ 99e6102368
use leptos::{attr, attr::Attr, prelude::*};

use crate::{
    IntoAttrs,
    utils::aria::{AriaExpanded, AriaHasPopup},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Takes the open state as `is_open` and the overlay's id as `overlay_id` (react-aria: the state,
//   and an id it generates and returns in `overlayProps`): the id comes from `use_overlay`.
// - No `onPress` in the trigger props (react-aria: `state.toggle`): pressing is the caller's
//   (`use_button`, `use_menu_trigger`).
// - The overlay type is the `OverlayTriggerType` enum.
//
// ## OMITTED FEATURES
// - The `onCloseMap` registration (react-aria's backward compatibility for closing on scroll):
//   `use_overlay_position` takes `on_close`.
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
    /// Whether the overlay is open.
    pub is_open: Signal<bool>,

    /// The overlay's id (from `use_overlay`, `use_popover`, ...): the trigger's `aria-controls`
    /// while it is open.
    pub overlay_id: String,

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

/// The trigger's ARIA attributes for the overlay it opens: `aria-expanded`, `aria-controls` (while
/// open) and, for menus and list boxes, `aria-haspopup`.
pub fn use_overlay_trigger(input: UseOverlayTriggerInput) -> UseOverlayTriggerReturn {
    let UseOverlayTriggerInput {
        is_open,
        overlay_id,
        overlay_type,
    } = input;

    // Match react-aria behavior: only set aria-haspopup for Menu and Listbox.
    // Screen readers may misinterpret other values (dialog, tree, grid) as "menu",
    // so we omit the attribute entirely for those types.
    let aria_haspopup = match overlay_type {
        OverlayTriggerType::Menu => Some(AriaHasPopup::True),
        OverlayTriggerType::Listbox => Some(AriaHasPopup::Listbox),
        OverlayTriggerType::Dialog | OverlayTriggerType::Tree | OverlayTriggerType::Grid => None,
    };

    UseOverlayTriggerReturn {
        props: UseOverlayTriggerProps {
            aria_haspopup,
            aria_expanded: Signal::derive(move || Some(AriaExpanded::from(is_open.get()))),
            aria_controls: Signal::derive(move || is_open.get().then(|| overlay_id.clone())),
        },
    }
}
