//! Form reset detection hook.
//!
//! Detects `<form>` reset events and restores a field to its initial value.

use leptos::prelude::*;
use leptos_element_capture::CapturedElement;
use send_wrapper::SendWrapper;
use wasm_bindgen::{JsCast, closure::Closure};

use super::use_form_validation::get_parent_form;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/utils/src/useFormReset.ts

// No intentional deviations from the react-aria implementation.

/// Input parameters for [`use_form_reset`].
pub struct UseFormResetInput<T: Clone + Send + Sync + 'static> {
    /// Reference to the form field element, used to find the parent `<form>`.
    pub element: CapturedElement,

    /// The initial/default value to restore on form reset.
    pub initial_value: T,

    /// Callback invoked with the initial value when the parent form is reset.
    pub on_reset: Callback<T>,
}

/// Detects `<form>` reset events and calls `on_reset` with the initial value.
///
/// This hook finds the parent `<form>` element from the captured element
/// reference and listens for the native `reset` event. When fired, it calls
/// `on_reset` with the provided `initial_value`, allowing the field to
/// restore its default state.
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
        let closure = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {
            on_reset.run(initial.clone());
        });

        let form_target: &web_sys::EventTarget = form.unchecked_ref();
        let _ =
            form_target.add_event_listener_with_callback("reset", closure.as_ref().unchecked_ref());

        // Wrap for Send + Sync requirement of on_cleanup.
        let form = SendWrapper::new(form);
        let closure = SendWrapper::new(closure);
        on_cleanup(move || {
            let form_target: &web_sys::EventTarget = (*form).unchecked_ref();
            let _ = form_target
                .remove_event_listener_with_callback("reset", closure.as_ref().unchecked_ref());
        });
    });
}
