use leptos::{oco::Oco, prelude::*};

use crate::hooks::{
    interactions::use_prevent_scroll::{use_prevent_scroll, UsePreventScrollInput},
    overlay::use_overlay::{
        use_overlay, UseOverlayAttrs, UseOverlayInput, UseOverlayProps, UseOverlayUnderlayAttrs,
        UseOverlayUnderlayProps,
    },
    IntoAttrs,
};

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/overlays/src/useModalOverlay.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## NAMING
//
// - React-aria calls this `useModalOverlay` and returns `underlayProps`.
//   Leptonic calls this `use_modal_backdrop` and returns `backdrop_props`.
//   "Backdrop" is more descriptive than "underlay" in web development context.
//
// ## OMITTED FEATURES
//
// - `ariaHideOutside()`: Not yet implemented. React-aria's `useModalOverlay`
//   calls `ariaHideOutside()` to hide all content outside the modal from
//   screen readers. We rely on `aria-modal="true"` (set by `use_modal`) instead.
//
// - `useOverlayFocusContain`: React-aria's `useModalOverlay` calls this to
//   signal that focus should be contained. In Leptonic, focus containment is
//   handled by the `FocusScope` atom at the consumer level.
//
// ## COMPOSITION
//
// This hook composes:
// - `use_overlay` for dismiss handling (Escape key with is_composing check,
//   interact-outside detection, blur dismissal, overlay stacking — only the
//   topmost overlay closes).
// - `use_prevent_scroll` for preventing body scroll while the modal is open.
//
// =============================================================================

/// Input parameters for the `use_modal_backdrop` hook.
///
/// This hook provides dismiss behavior and scroll prevention for modal overlays.
/// It delegates to `use_overlay` for Escape key handling, outside-click detection,
/// and overlay stacking (only the topmost overlay is dismissed).
#[derive(Debug, Clone)]
pub struct UseModalBackdropInput {
    /// Whether the modal is open.
    pub is_open: Signal<bool>,

    /// Callback when the modal should close.
    pub on_close: Callback<()>,

    /// Whether to close when the user interacts outside the modal.
    /// Defaults to `false` — modals typically block outside interaction.
    pub is_dismissable: bool,

    /// Whether pressing the Escape key to close should be disabled.
    /// Defaults to `false`.
    pub is_keyboard_dismiss_disabled: bool,

    /// Optional filter for which outside interactions should close the modal.
    /// Only consulted when `is_dismissable` is `true`.
    /// Return `true` from the callback to allow closing, `false` to prevent it.
    pub should_close_on_interact_outside: Option<Callback<web_sys::Element, bool>>,
}

/// The return value of the `use_modal_backdrop` hook.
pub struct UseModalBackdropReturn {
    /// Props for the modal content element (overlay container).
    /// Spread these onto the element that wraps the modal content.
    /// Includes: id, element capture, keydown (Escape), focusin/focusout.
    pub modal_props: UseModalBackdropModalProps,

    /// Props for the backdrop element (the underlay behind the modal).
    /// Spread these onto the backdrop/underlay element.
    /// Includes: pointerdown handler (Firefox text-selection fix).
    pub backdrop_props: UseModalBackdropProps,

    /// Unique overlay ID. Can be passed to `use_overlay_trigger` if needed.
    pub id: Oco<'static, str>,
}

/// Props for the modal content element, delegated from `use_overlay`.
///
/// Call `.into_attrs()` to get spreadable attributes.
#[derive(Debug)]
pub struct UseModalBackdropModalProps(pub UseOverlayProps);

impl IntoAttrs for UseModalBackdropModalProps {
    type Attrs = UseOverlayAttrs;

    fn into_attrs(self) -> Self::Attrs {
        self.0.into_attrs()
    }
}

/// Props for the backdrop element, delegated from `use_overlay`.
///
/// Call `.into_attrs()` to get spreadable attributes.
#[derive(Debug)]
pub struct UseModalBackdropProps(pub UseOverlayUnderlayProps);

impl IntoAttrs for UseModalBackdropProps {
    type Attrs = UseOverlayUnderlayAttrs;

    fn into_attrs(self) -> Self::Attrs {
        self.0.into_attrs()
    }
}

/// Provides dismiss behavior and scroll prevention for modal overlays.
///
/// This hook composes `use_overlay` (for Escape key, interact-outside, blur
/// dismissal, and overlay stacking) with `use_prevent_scroll` (scroll is
/// always prevented while the modal is open).
///
/// Combine with:
/// - `use_modal` for `aria-modal="true"`
/// - `use_dialog` for ARIA role, labeling, and focus-on-mount
/// - `FocusScope` atom for focus trapping and restoration
///
/// # Example
///
/// ```ignore
/// let UseModalBackdropReturn { modal_props, backdrop_props, id: _ } =
///     use_modal_backdrop(UseModalBackdropInput {
///         is_open: state.is_open,
///         on_close: state.close,
///         is_dismissable: true,
///         is_keyboard_dismiss_disabled: false,
///         should_close_on_interact_outside: None,
///     });
///
/// view! {
///     <Show when=move || is_open.get()>
///         <div {..backdrop_props.into_attrs()} class="backdrop">
///             <FocusScope contain=true restore_focus=true auto_focus=true>
///                 <div {..modal_props.into_attrs()} class="modal">
///                     "Modal content"
///                 </div>
///             </FocusScope>
///         </div>
///     </Show>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_modal_backdrop(input: UseModalBackdropInput) -> UseModalBackdropReturn {
    let UseModalBackdropInput {
        is_open,
        on_close,
        is_dismissable,
        is_keyboard_dismiss_disabled,
        should_close_on_interact_outside,
    } = input;

    // 1. Delegate to use_overlay for dismiss behavior (Escape, interact-outside,
    //    blur, overlay stacking). Modals do not close on blur.
    let overlay = use_overlay(UseOverlayInput {
        is_open,
        on_close,
        is_dismissable,
        should_close_on_blur: false,
        is_keyboard_dismiss_disabled,
        should_close_on_interact_outside,
    });

    // 2. Prevent body scroll while the modal is open.
    use_prevent_scroll(UsePreventScrollInput {
        disabled: Signal::derive(move || !is_open.get()),
    });

    // 3. Future: ariaHideOutside

    UseModalBackdropReturn {
        modal_props: UseModalBackdropModalProps(overlay.props),
        backdrop_props: UseModalBackdropProps(overlay.underlay_props),
        id: overlay.id,
    }
}
