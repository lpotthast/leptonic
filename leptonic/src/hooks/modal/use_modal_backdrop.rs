// Upstream: react-aria/src/overlays/useModalOverlay.ts @ 99e6102368
// Upstream: react-aria/test/overlays/useModalOverlay.test.js @ 99e6102368
use leptos::prelude::*;

use crate::{
    IntoAttrs,
    hooks::overlay::{
        OverlayState, OverlayTriggerState,
        use_overlay::{UseOverlayAttrs, UseOverlayInput, UseOverlayProps, use_overlay},
        use_prevent_scroll::{UsePreventScrollInput, use_prevent_scroll},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Named `use_modal_backdrop` (react-aria: `useModalOverlay`). No `underlayProps`: react-aria's
//   are empty; the backdrop element needs no props.
// - The modal element is captured from `modal_props` instead of passing a ref.
// - `is_entering` is a signal.
// - Generic over the open state (`S: OverlayState`), as `use_popover` (react-aria: structural
//   typing of `OverlayTriggerState`).
//
// ## DIFFERENT BEHAVIOR
// - `useOverlayFocusContain`: focus containment is the `FocusScope` (atom `ModalContent`) the
//   consumer renders around the modal.
//
// ## OMITTED FEATURES
// - `UNSTABLE_overrideFocus`.
//
// =============================================================================

/// Input of [`use_modal_backdrop`].
#[derive(Debug, Clone)]
pub struct UseModalBackdropInput<S: OverlayState = OverlayTriggerState> {
    /// Whether the modal is open; dismissing (Escape, outside interaction) closes it. An
    /// `OverlayTriggerState`, or a component state with its own closing logic.
    pub state: S,

    /// Whether interacting outside the modal closes it. Default: `false` (modals block outside
    /// interaction).
    pub is_dismissable: Signal<bool>,

    /// Whether Escape no longer closes the modal.
    pub is_keyboard_dismiss_disabled: Signal<bool>,

    /// Which outside interactions close the modal (when `is_dismissable`): `true` closes.
    pub should_close_on_interact_outside: Option<crate::hooks::overlay::InteractOutsideFilter>,

    /// Whether the modal is still animating in. While it is, the content outside isn't hidden
    /// from assistive technology yet (and a parent modal doesn't hide this one).
    pub is_entering: Signal<bool>,
}

/// The return value of the `use_modal_backdrop` hook.
#[derive(Debug)]
pub struct UseModalBackdropReturn {
    /// Props for the modal content element (overlay container).
    /// Spread these onto the element that wraps the modal content.
    /// Includes: id, element capture, keydown (Escape), focusin/focusout.
    pub modal_props: UseModalBackdropModalProps,

    /// Unique overlay ID. Can be passed to `use_overlay_trigger` if needed.
    pub id: String,
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

/// Provides dismiss behavior and scroll prevention for modal overlays.
///
/// This hook composes `use_overlay` (for Escape key, interact-outside, blur
/// dismissal, and overlay stacking) with `use_prevent_scroll` (scroll is
/// always prevented while the modal is open).
///
/// Everything outside the modal element is inert while it is open, which makes the modal modal
/// (no `aria-modal` needed). Combine with:
/// - `use_dialog` for ARIA role, labeling, and focus-on-mount
/// - `FocusScope` atom for focus trapping and restoration
///
/// # Example
///
/// ```ignore
/// let state = use_overlay_trigger_state(UseOverlayTriggerStateInput::default());
/// let UseModalBackdropReturn { modal_props, .. } =
///     use_modal_backdrop(UseModalBackdropInput {
///         state,
///         is_dismissable: Signal::stored(true),
///         is_keyboard_dismiss_disabled: Signal::stored(false),
///         should_close_on_interact_outside: None,
///         is_entering: Signal::stored(false),
///     });
///
/// view! {
///     <Show when=move || state.is_open.get()>
///         <div class="backdrop">
///             <FocusScope contain=true restore_focus=true>
///                 <div {..modal_props.into_attrs()} class="modal">"Modal content"</div>
///             </FocusScope>
///         </div>
///     </Show>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_modal_backdrop<S: OverlayState>(
    input: UseModalBackdropInput<S>,
) -> UseModalBackdropReturn {
    let UseModalBackdropInput {
        state,
        is_dismissable,
        is_keyboard_dismiss_disabled,
        should_close_on_interact_outside,
        is_entering,
    } = input;
    let is_open = Signal::derive(move || state.is_open());
    #[cfg(feature = "ssr")]
    let _ = is_entering;

    // 1. Delegate to use_overlay for dismiss behavior (Escape, interact-outside,
    //    blur, overlay stacking). Modals do not close on blur.
    let overlay = use_overlay(UseOverlayInput {
        is_open,
        on_close: Callback::new(move |()| state.close()),
        is_dismissable,
        should_close_on_blur: Signal::stored(false),
        is_keyboard_dismiss_disabled,
        should_close_on_interact_outside,
        group: None,
    });

    // 2. Prevent body scroll while the modal is open.
    use_prevent_scroll(UsePreventScrollInput {
        is_disabled: Signal::derive(move || !is_open.get()),
    });

    // 3. Hide outside elements from assistive technology.
    #[cfg(not(feature = "ssr"))]
    {
        use crate::utils::aria_hide_outside::{
            AriaHideOutsideOptions, HideMode, aria_hide_outside, keep_visible,
        };

        let overlay_element = overlay.overlay_element;

        let hide_cleanup: StoredValue<Option<Box<dyn FnOnce()>>, LocalStorage> =
            StoredValue::new_local(None);

        Effect::new(move |_| {
            // Clean up previous hide (if any).
            hide_cleanup.update_value(|opt| {
                if let Some(f) = opt.take() {
                    f();
                }
            });

            if is_open.get()
                && let Some(el) = overlay_element.get()
            {
                let undo = if is_entering.get() {
                    // Keep a parent modal from hiding this one while it enters.
                    keep_visible(&el)
                } else {
                    Some(aria_hide_outside(
                        &[(*el).clone()],
                        AriaHideOutsideOptions {
                            mode: HideMode::Inert,
                            ..AriaHideOutsideOptions::default()
                        },
                    ))
                };
                hide_cleanup.set_value(undo);
            }
        });

        on_cleanup(move || {
            hide_cleanup.update_value(|opt| {
                if let Some(f) = opt.take() {
                    f();
                }
            });
        });
    }

    UseModalBackdropReturn {
        modal_props: UseModalBackdropModalProps(overlay.props),
        id: overlay.id,
    }
}
