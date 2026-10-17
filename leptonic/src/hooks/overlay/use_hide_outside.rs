// Upstream: react-aria/src/overlays/usePopover.ts @ 99e6102368
// Upstream: react-aria/src/overlays/useModalOverlay.ts @ 99e6102368
//! Hiding the page outside an open overlay from assistive technology (`use_hide_outside`).

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - A hook of its own, shared by `use_popover` and `use_modal_backdrop` (react-aria: the same
//   effect in `usePopover` and `useModalOverlay`), so that overlays of your own hide the page as
//   leptonic's do.
// - Whether the page is hidden or the overlay is only kept visible is `HideOutsideBehavior`
//   (react-aria: `isNonModal` in `usePopover`, `isEntering` in `useModalOverlay`).
//
// =============================================================================

use leptos::prelude::*;

use crate::CapturedElement;

/// What [`use_hide_outside`] does while its overlay is open.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HideOutsideBehavior {
    /// The rest of the page is inert: hidden from assistive technology and not interactive (a
    /// modal overlay).
    Inert,
    /// The page stays as it is; only the overlay is kept visible should another modal overlay
    /// hide the page (a non-modal popover, or a modal while it animates in).
    KeepVisible,
}

/// Input of [`use_hide_outside`].
#[derive(Debug, Clone, Copy)]
pub struct UseHideOutsideInput {
    /// Whether the overlay is open. The page is restored when it closes (or the hook's owner is
    /// cleaned up).
    pub is_open: Signal<bool>,

    /// The overlay, captured by the hook rendering it (`use_overlay`'s `overlay_element`). Nothing
    /// happens until it is captured, so the overlay itself is never made inert. Give the element
    /// containing a group of overlays (a menu with its submenus) to keep them all usable.
    pub overlay: CapturedElement,

    /// Whether the page is made inert or the overlay only kept visible.
    pub behavior: Signal<HideOutsideBehavior>,
}

/// Makes the page outside an open overlay inert (hidden from assistive technology and not
/// interactive), which is what makes an overlay modal; or, with
/// [`HideOutsideBehavior::KeepVisible`], keeps the overlay usable while another modal overlay
/// hid the page.
///
/// `use_popover` and `use_modal_backdrop` call it; call it yourself for an overlay you build
/// from `use_overlay`. Nested overlays stack: each one restores what it hid, in any order. Built
/// on [`aria_hide_outside`](crate::aria_hide_outside) and [`keep_visible`](crate::keep_visible),
/// which follow elements added to the page while the overlay is open. Does nothing during
/// server-side rendering.
///
/// ```ignore
/// use leptonic::hooks::overlay::{HideOutsideBehavior, UseHideOutsideInput, use_hide_outside};
///
/// let overlay = use_overlay(UseOverlayInput { is_open, .. });
/// use_hide_outside(UseHideOutsideInput {
///     is_open,
///     overlay: overlay.overlay_element,
///     behavior: Signal::stored(HideOutsideBehavior::Inert),
/// });
/// ```
pub fn use_hide_outside(input: UseHideOutsideInput) {
    crate::metrics::subject!();
    let UseHideOutsideInput {
        is_open,
        overlay,
        behavior,
    } = input;

    #[cfg(feature = "ssr")]
    {
        let _ = (is_open, overlay, behavior);
    }

    #[cfg(not(feature = "ssr"))]
    {
        use leptos::prelude::LocalStorage;

        use crate::utils::aria_hide_outside::{
            AriaHideOutsideOptions, HideMode, aria_hide_outside, keep_visible,
        };

        let undo: StoredValue<Option<Box<dyn FnOnce()>>, LocalStorage> =
            StoredValue::new_local(None);
        let restore = move || {
            undo.update_value(|undo| {
                if let Some(undo) = undo.take() {
                    undo();
                }
            });
        };

        Effect::new(move |_| {
            restore();
            if !is_open.get() {
                return;
            }
            let Some(overlay) = overlay.get() else {
                return;
            };
            let overlay: web_sys::Element = (*overlay).clone();
            let new_undo = match behavior.get() {
                HideOutsideBehavior::KeepVisible => keep_visible(&overlay),
                HideOutsideBehavior::Inert => Some(aria_hide_outside(
                    &[overlay],
                    AriaHideOutsideOptions {
                        mode: HideMode::Inert,
                        ..AriaHideOutsideOptions::default()
                    },
                )),
            };
            undo.set_value(new_undo);
        });

        on_cleanup(restore);
    }
}
