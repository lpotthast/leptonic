//! Form validation DOM connection hook.
//!
//! Connects the form validation state to native HTML constraint validation.
//! This hook sets `setCustomValidity()` on native input elements, listens
//! for `invalid`/`change`/`reset` events, and coordinates with
//! [`UseFormValidationStateReturn`].

use leptos::prelude::*;
use leptos_element_capture::CapturedElement;
use send_wrapper::SendWrapper;
use wasm_bindgen::{JsCast, closure::Closure};

use super::use_form_validation_state::{
    UseFormValidationStateReturn, ValidationBehavior, ValidationResult, ValidityStateSnapshot,
};
// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/form/src/useFormValidation.ts

// REACT-ARIA DEVIATIONS
//
// OMITTED FEATURES
// - `form.reset()` monkey-patching: React-aria patches `form.reset()` to detect
//   React's automatic resets (via MessageChannel/MessagePort). This is
//   React-specific and not applicable to Leptos.
// - `setInteractionModality('keyboard')`: Not yet implemented in leptonic.
// - Auto-focus of first invalid input on form submit: Deferred to a follow-up.
//
// LEPTOS-SPECIFIC ADAPTATIONS
// - Uses `CapturedElement` instead of React ref for DOM element access.
// - Uses `Effect` with `on_cleanup` for listener lifecycle.
// - Uses `Effect` tracking `realtime_validation` for setCustomValidity sync.

/// Input parameters for [`use_form_validation`].
#[derive(Clone, Copy)]
pub struct UseFormValidationInput {
    /// Reference to the form field element.
    pub element: CapturedElement,

    /// The validation state from
    /// [`use_form_validation_state`](super::use_form_validation_state::use_form_validation_state).
    pub state: UseFormValidationStateReturn,

    /// Validation behavior mode.
    pub validation_behavior: ValidationBehavior,
}

/// Connects form validation state to the native HTML constraint validation API.
///
/// This hook:
/// - Sets `setCustomValidity()` on the input element (native mode only)
/// - Listens to `invalid` events (form submit constraint violation)
/// - Listens to `change` events (user commits a value change)
/// - Listens to `reset` events on the parent form (resets validation state)
///
/// This hook has no return value — it is purely side-effectual.
pub fn use_form_validation(input: UseFormValidationInput) {
    let UseFormValidationInput {
        element,
        state,
        validation_behavior,
    } = input;

    // ---- Effect 1: Sync setCustomValidity with realtime validation ----
    // Only applies in Native mode (Aria mode uses ARIA attributes only).
    if validation_behavior == ValidationBehavior::Native {
        Effect::new(move |_| {
            let Some(el) = element.get() else { return };
            let Some(input_el) = el.dyn_ref::<web_sys::HtmlInputElement>() else {
                return;
            };
            if input_el.disabled() {
                return;
            }

            let realtime = state.realtime_validation.get();
            let error_message = if realtime.is_invalid {
                let msg = realtime.validation_errors.join(" ");
                if msg.is_empty() {
                    "Invalid value.".to_owned()
                } else {
                    msg
                }
            } else {
                String::new()
            };

            input_el.set_custom_validity(&error_message);

            // Prevent default tooltip for validation message.
            // https://bugzilla.mozilla.org/show_bug.cgi?id=605277
            if !input_el.has_attribute("title") {
                input_el.set_title("");
            }

            // When our custom validation is valid, read native constraint validity.
            if !realtime.is_invalid {
                state.update_validation.run(get_native_validity(input_el));
            }
        });
    }

    // ---- Effect 2: Attach event listeners ----
    Effect::new(move |_| {
        let Some(el) = element.get() else { return };
        let form = get_parent_form(&el);

        // --- invalid event ---
        let invalid_closure = Closure::<dyn Fn(web_sys::Event)>::new(move |e: web_sys::Event| {
            // Only commit validation if not already displaying an error.
            // This avoids clearing server errors the user didn't fix.
            if !state.display_validation.get_untracked().is_invalid {
                state.commit_validation.run(());
            }
            // Prevent default browser validation UI.
            e.prevent_default();
        });

        // --- change event ---
        let change_closure = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {
            state.commit_validation.run(());
        });

        // --- reset event (on parent form) ---
        let reset_closure = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {
            state.reset_validation.run(());
        });

        // Attach listeners to the element.
        let el_target: &web_sys::EventTarget = el.unchecked_ref();
        let _ = el_target
            .add_event_listener_with_callback("invalid", invalid_closure.as_ref().unchecked_ref());
        let _ = el_target
            .add_event_listener_with_callback("change", change_closure.as_ref().unchecked_ref());

        // Attach reset listener to the parent form (if present).
        if let Some(ref form) = form {
            let form_target: &web_sys::EventTarget = form.unchecked_ref();
            let _ = form_target
                .add_event_listener_with_callback("reset", reset_closure.as_ref().unchecked_ref());
        }

        // Wrap for Send + Sync requirement of on_cleanup.
        let el_for_cleanup = el;
        let form_for_cleanup = form.map(SendWrapper::new);
        let invalid_for_cleanup = SendWrapper::new(invalid_closure);
        let change_for_cleanup = SendWrapper::new(change_closure);
        let reset_for_cleanup = SendWrapper::new(reset_closure);

        on_cleanup(move || {
            let el_target: &web_sys::EventTarget = (*el_for_cleanup).unchecked_ref();
            let _ = el_target.remove_event_listener_with_callback(
                "invalid",
                invalid_for_cleanup.as_ref().unchecked_ref(),
            );
            let _ = el_target.remove_event_listener_with_callback(
                "change",
                change_for_cleanup.as_ref().unchecked_ref(),
            );
            if let Some(ref form) = form_for_cleanup {
                let form_target: &web_sys::EventTarget = (**form).unchecked_ref();
                let _ = form_target.remove_event_listener_with_callback(
                    "reset",
                    reset_for_cleanup.as_ref().unchecked_ref(),
                );
            }
        });
    });
}

/// Snapshots the native `ValidityState` of an `HtmlInputElement`.
pub fn get_native_validity(input: &web_sys::HtmlInputElement) -> ValidationResult {
    let validity = input.validity();
    ValidationResult {
        is_invalid: !validity.valid(),
        validation_details: ValidityStateSnapshot {
            bad_input: validity.bad_input(),
            custom_error: validity.custom_error(),
            pattern_mismatch: validity.pattern_mismatch(),
            range_overflow: validity.range_overflow(),
            range_underflow: validity.range_underflow(),
            step_mismatch: validity.step_mismatch(),
            too_long: validity.too_long(),
            too_short: validity.too_short(),
            type_mismatch: validity.type_mismatch(),
            value_missing: validity.value_missing(),
            valid: validity.valid(),
        },
        validation_errors: {
            let msg = input.validation_message().unwrap_or_default();
            if msg.is_empty() { vec![] } else { vec![msg] }
        },
    }
}

/// Finds the parent `<form>` element from a form-associated element.
///
/// Tries casting to `HtmlInputElement`, `HtmlTextAreaElement`, and
/// `HtmlSelectElement` and calling `.form()` on whichever succeeds.
pub(crate) fn get_parent_form(el: &web_sys::Element) -> Option<web_sys::HtmlFormElement> {
    el.dyn_ref::<web_sys::HtmlInputElement>()
        .and_then(web_sys::HtmlInputElement::form)
        .or_else(|| {
            el.dyn_ref::<web_sys::HtmlTextAreaElement>()
                .and_then(web_sys::HtmlTextAreaElement::form)
        })
        .or_else(|| {
            el.dyn_ref::<web_sys::HtmlSelectElement>()
                .and_then(web_sys::HtmlSelectElement::form)
        })
}
