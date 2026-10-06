// Upstream: react-aria/src/overlays/useOverlay.ts @ 99e6102368
#![cfg_attr(feature = "ssr", allow(dead_code, unused_imports))]

use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    oco::Oco,
    prelude::*,
};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, KeyboardEvent, PointerEvent};

use super::visible_overlays;
use crate::{
    hooks::{
        IntoAttrs,
        focus::use_focus_within::{UseFocusWithinInput, use_focus_within},
        interactions::{
            use_interact_outside::{UseInteractOutsideInput, use_interact_outside},
            use_keyboard::{UseKeyboardInput, use_keyboard},
        },
    },
    utils::{
        CapturedElement, ElementCaptureAttr, EventAccessors, EventHandler,
        focus_scope_tree::is_element_in_child_of_active_scope,
        id::use_id,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/react-aria/src/overlays/useOverlay.ts

//
// ## DIFFERENT BEHAVIOR
//
// - ID generation: react-aria generates overlay IDs in `useOverlayTrigger`, not `useOverlay`.
//   Leptonic generates the ID here for convenience and returns it in `UseOverlayReturn`.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
//
// - Uses `ElementCaptureAttr` instead of React ref parameter.
//   The overlay element is captured via the attribute spread pattern rather than
//   being passed as a ref argument.
//
// - Uses `EventHandler<E>` / `On<>` attributes instead of React event handler props.
//
// - No `underlay_props`: react-aria's are empty (`{}`) since its Firefox text-selection workaround
//   was removed; an underlay element needs no props.
//

/// Input parameters for the `use_overlay` hook.
#[derive(Debug, Clone)]
pub struct UseOverlayInput {
    /// Whether the overlay is currently open.
    pub is_open: Signal<bool>,

    /// Handler called when the overlay should close.
    pub on_close: Callback<()>,

    /// Whether to close the overlay when the user interacts outside it.
    /// Defaults to `false`.
    pub is_dismissable: bool,

    /// Whether the overlay should close when focus is lost or moves outside it.
    /// Defaults to `false`.
    pub should_close_on_blur: bool,

    /// Whether pressing the Escape key to close the overlay should be disabled.
    /// Defaults to `false`.
    pub is_keyboard_dismiss_disabled: bool,

    /// When the user interacts with an element outside of the overlay,
    /// return `true` if `on_close` should be called. This gives you a chance to
    /// filter out interaction with elements that should not dismiss the overlay.
    /// By default, `on_close` will always be called on interaction outside the overlay.
    pub should_close_on_interact_outside: Option<Callback<web_sys::Element, bool>>,
}

/// The return value of the `use_overlay` hook.
#[derive(Debug)]
pub struct UseOverlayReturn {
    /// Props for the overlay container element. Call `.into_attrs()` for view spreading.
    pub props: UseOverlayProps,

    /// Unique ID for the overlay. Pass to `use_overlay_trigger` as `overlay_id`.
    pub id: Oco<'static, str>,

    /// The captured overlay element. Enables consumers (e.g. `use_popover`,
    /// `use_modal_backdrop`) to pass the element to `aria_hide_outside`.
    pub overlay_element: CapturedElement,
}

/// Props from `use_overlay` that can be converted to spreadable attributes.
#[derive(Debug)]
pub struct UseOverlayProps {
    pub id: String,
    pub element_capture: ElementCaptureAttr,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

impl IntoAttrs for UseOverlayProps {
    type Attrs = UseOverlayAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            self.element_capture,
            self.on_keydown.into_on(ev::keydown),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        )
    }
}

/// These attributes must be spread onto the overlay element: `<div {..overlay_props.into_attrs()} />`
pub type UseOverlayAttrs = (
    Attr<attr::Id, String>,
    ElementCaptureAttr,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
);

/// Provides the behavior for overlays such as dialogs, popovers, and menus.
///
/// Hides the overlay when the user interacts outside it, when the Escape key is pressed,
/// or optionally, on blur. Only the top-most overlay will close at once.
///
/// # Example
///
/// ```ignore
/// let (is_open, set_is_open) = signal(false);
///
/// let UseOverlayReturn { props: overlay_props, .. } = use_overlay(UseOverlayInput {
///     is_open: is_open.into(),
///     on_close: Callback::new(move |_| set_is_open.set(false)),
///     is_dismissable: true,
///     should_close_on_blur: false,
///     is_keyboard_dismiss_disabled: false,
///     should_close_on_interact_outside: None,
/// });
///
/// view! {
///     <Show when=move || is_open.get()>
///         <div {..overlay_props.into_attrs()}>
///             "Overlay content"
///         </div>
///     </Show>
/// }
/// ```
#[allow(clippy::needless_pass_by_value, clippy::too_many_lines)]
pub fn use_overlay(input: UseOverlayInput) -> UseOverlayReturn {
    cfg_if::cfg_if! {
        if #[cfg(feature = "ssr")] {
            let _ = input;
            let id = use_id("overlay");
            let overlay_element = CapturedElement::new();
            UseOverlayReturn {
                props: UseOverlayProps {
                    id: id.clone(),
                    element_capture: overlay_element.attr(),
                    on_keydown: EventHandler::new(|_: KeyboardEvent| {}),
                    on_focusin: EventHandler::new(|_: FocusEvent| {}),
                    on_focusout: EventHandler::new(|_: FocusEvent| {}),
                },
                id: Oco::Owned(id),
                overlay_element,
            }
        } else {
            let UseOverlayInput {
                is_open,
                on_close,
                is_dismissable,
                should_close_on_blur,
                is_keyboard_dismiss_disabled,
                should_close_on_interact_outside,
            } = input;

    let id_string = use_id("overlay");

    // Element capture for the overlay element. This is used by the overlay stack
    // and chained with the element capture from use_interact_outside.
    let overlay_element = CapturedElement::new();

    // Track which overlay was topmost at pointerdown time.
    // This handles the race condition where a click on overlay B's trigger
    // closes overlay A (which was topmost), making B topmost by the time
    // the click handler fires.
    let last_topmost_at_pointerdown: StoredValue<
        Option<SendWrapper<web_sys::Element>>,
        LocalStorage,
    > = StoredValue::new_local(None);

    // Push/remove element from the visible overlays stack when is_open changes.
    Effect::new(move |_| {
        let open = is_open.get();
        if open {
            if let Some(el) = overlay_element.get() {
                visible_overlays::push_overlay(&el);
            }
        } else if let Some(el) = overlay_element.get_untracked() {
            visible_overlays::remove_overlay(&el);
        }
    });

    // Clean up on unmount.
    on_cleanup(move || {
        if let Some(el) = overlay_element.get_untracked() {
            visible_overlays::remove_overlay(&el);
        }
    });

    let on_hide = move || {
        if let Some(el) = overlay_element.get_untracked()
            && visible_overlays::is_topmost(&el) {
                on_close.run(());
            }
    };

    // Escape closes the topmost overlay (a handled shortcut stops propagation). With keyboard
    // dismissal disabled, the key press bubbles on.
    let keyboard = use_keyboard(UseKeyboardInput {
        shortcuts: Some(KeyboardShortcuts::new().on(Shortcut::key("Escape"), move |_| {
            if is_keyboard_dismiss_disabled {
                return false;
            }
            on_hide();
            true
        })),
        ..UseKeyboardInput::default()
    });

    let interact_outside_return = use_interact_outside(UseInteractOutsideInput {
        is_disabled: Signal::derive(move || !(is_dismissable && is_open.get())),

        on_interact_outside_start: Some(Callback::new(move |e: PointerEvent| {
            // Capture topmost at pointerdown time.
            if let Some(el) = overlay_element.get_untracked() {
                if visible_overlays::is_topmost(&el) {
                    last_topmost_at_pointerdown.set_value(Some(SendWrapper::new((*el).clone())));
                } else {
                    last_topmost_at_pointerdown.set_value(None);
                }
            }

            // Check the filter callback.
            let should_close = should_close_on_interact_outside.is_none_or(|filter| {
                e.expect_target()
                    .dyn_into::<web_sys::Element>().is_ok_and(|target_el| filter.run(target_el))
            });

            if should_close
                && let Some(el) = overlay_element.get_untracked()
                    && visible_overlays::is_topmost(&el) {
                        // Only stop propagation (as react-aria): the outside element still gets
                        // its default action (a link navigates, a checkbox toggles).
                        e.stop_propagation();
                    }
        })),

        on_interact_outside: Some(Callback::new(move |e: web_sys::MouseEvent| {
            // Check the filter callback.
            let should_close = should_close_on_interact_outside.is_none_or(|filter| {
                e.expect_target()
                    .dyn_into::<web_sys::Element>().is_ok_and(|target_el| filter.run(target_el))
            });

            if should_close {
                if let Some(el) = overlay_element.get_untracked()
                    && visible_overlays::is_topmost(&el) {
                        // Only stop propagation (as react-aria): the outside element still gets
                        // its default action (a link navigates, a checkbox toggles).
                        e.stop_propagation();
                    }

                // Close if this overlay was topmost at pointerdown time.
                let was_topmost = last_topmost_at_pointerdown
                    .get_value()
                    .is_some_and(|saved| {
                        overlay_element
                            .get_untracked()
                            .is_some_and(|el| *saved == *el)
                    });
                if was_topmost {
                    on_hide();
                }
            }

            last_topmost_at_pointerdown.set_value(None);
        })),
    });

    // Chain element captures: overlay stack management + interact outside.
    let element_capture = overlay_element
        .attr()
        .chain(interact_outside_return.props.element_capture);

    let focus_within_return = use_focus_within(UseFocusWithinInput {
        is_disabled: Signal::derive(move || !should_close_on_blur),

        on_focus_within: None,

        on_blur_within: Some(Callback::new(move |e: crate::hooks::FocusWithinEvent| {
            // Do not close if relatedTarget is null, which means focus is lost to the body.
            // That can happen when switching tabs, or due to a VoiceOver/Chrome bug.
            // Clicking on the body to close the overlay is handled by use_interact_outside.
            let Some(related_target) = e.event.related_target() else {
                return;
            };

            // If focus is moving into a child focus scope (e.g. menu inside a dialog),
            // do not close the outer overlay. At this point, the active scope should
            // still be the outer overlay, since blur events run before focus.
            let Some(related_el) = related_target.dyn_ref::<web_sys::Element>() else {
                return;
            };

            if is_element_in_child_of_active_scope(related_el) {
                return;
            }

            // Check the filter callback.
            let should_close = should_close_on_interact_outside
                .is_none_or(|filter| filter.run(related_el.clone()));

            if should_close {
                // Blur bypasses the topmost check, matching react-aria behavior.
                on_close.run(());
            }
        })),

        on_focus_within_change: None,
    });

            UseOverlayReturn {
                props: UseOverlayProps {
                    id: id_string.clone(),
                    element_capture,
                    on_keydown: keyboard.props.on_keydown,
                    on_focusin: focus_within_return.props.on_focusin,
                    on_focusout: focus_within_return.props.on_focusout,
                },
                id: Oco::Owned(id_string.clone()),
                overlay_element,
            }
        }
    }
}
