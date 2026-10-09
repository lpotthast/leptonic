// Upstream: react-aria/src/utils/useFormReset.ts @ 99e6102368
// Upstream: react-aria/test/utils/useFormReset.test.tsx @ 99e6102368
//! Restores a field to its initial value when its `<form>` is reset.

use leptos::prelude::*;
use leptos_element_capture::CapturedElement;
use send_wrapper::SendWrapper;

use super::use_form_validation::get_parent_form;
use crate::utils::event_listeners::listen;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The initial value is fixed when the hook is called (react-aria: the latest render's). Callers
//   pass the value the field started with, which doesn't change.
//
// =============================================================================

/// Input parameters for [`use_form_reset`].
pub struct UseFormResetInput<T: Clone + Send + Sync + 'static> {
    /// The field's `<input>`, `<textarea>` or `<select>`, whose form is listened to.
    pub element: CapturedElement,

    /// The value to restore when the form is reset.
    pub initial_value: T,

    /// Called with `initial_value` when the form is reset (unless the `reset` event's default was
    /// prevented).
    pub on_reset: Callback<T>,
}

/// Calls `on_reset` with the initial value when the field's `<form>` is reset (and the `reset`
/// event wasn't canceled).
///
/// # Example
///
/// ```ignore
/// let element = CapturedElement::new();
///
/// use_form_reset(UseFormResetInput {
///     element,
///     initial_value: String::new(),
///     on_reset: Callback::new(move |val: String| {
///         set_value.set(val);
///     }),
/// });
/// ```
pub fn use_form_reset<T: Clone + Send + Sync + 'static>(input: UseFormResetInput<T>) {
    let UseFormResetInput {
        element,
        initial_value,
        on_reset,
    } = input;

    Effect::new(move |_| {
        let Some(el) = element.get() else { return };
        let Some(form) = get_parent_form(&el) else {
            return;
        };
        let initial = initial_value.clone();
        // `reset` doesn't cross shadow DOM boundaries; this listener is on the field's own form.
        let listener = listen(&form, "reset", false, move |e: web_sys::Event| {
            if !e.default_prevented() {
                on_reset.run(initial.clone());
            }
        });
        let listener = SendWrapper::new(listener);
        on_cleanup(move || drop(listener));
    });
}
