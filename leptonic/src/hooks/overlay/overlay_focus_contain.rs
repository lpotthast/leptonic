// Upstream: react-aria/src/overlays/Overlay.tsx @ 99e6102368
use leptos::prelude::*;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Only the focus containment part of `Overlay`: the portal, focus scope and cleared trigger
//   contexts are the overlay atoms' business. `OverlayContext` is `OverlayFocusContain`.
//
// =============================================================================

/// Provided by an overlay whose focus scope doesn't contain focus by itself (a non-modal popover):
/// content that needs containment (a dialog) switches it on with [`use_overlay_focus_contain`].
#[derive(Debug, Clone, Copy)]
pub struct OverlayFocusContain {
    contain: RwSignal<bool>,
}

impl OverlayFocusContain {
    pub fn new() -> Self {
        Self {
            contain: RwSignal::new(false),
        }
    }

    /// Whether content of the overlay asked for focus containment.
    pub fn contain(&self) -> Signal<bool> {
        self.contain.into()
    }
}

impl Default for OverlayFocusContain {
    fn default() -> Self {
        Self::new()
    }
}

/// Makes the surrounding overlay contain focus, once mounted (react-aria: `useOverlayFocusContain`).
/// Does nothing outside an overlay.
pub fn use_overlay_focus_contain() {
    if let Some(overlay) = use_context::<OverlayFocusContain>() {
        Effect::new(move |_| overlay.contain.set(true));
    }
}
