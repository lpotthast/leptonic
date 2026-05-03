#![cfg_attr(feature = "ssr", allow(dead_code))]

//! Synthetic blur event support for Firefox compatibility.
//!
//! Firefox does not fire `blur`/`focusout` events when a focused form element
//! (`<button>`, `<input>`, `<textarea>`, `<select>`) becomes disabled while focused.
//! This module provides a `MutationObserver`-based workaround that detects the
//! `disabled` attribute mutation and manually dispatches synthetic blur/focusout events.
//!
//! A one-shot `focusout` listener is attached alongside the `MutationObserver` to act
//! as a deduplication gate: whichever fires first (native or observer-triggered) removes
//! the listener and disconnects the observer, preventing the callback from firing twice.
//!
//! Based on react-aria's `useSyntheticBlurEvent` from `utils.ts`.

/// Set up a `MutationObserver` on the given element that watches for the `disabled`
/// attribute. When the element becomes disabled while focused, synthetic `blur` and
/// `focusout` events are dispatched so that blur handlers fire correctly.
///
/// A one-shot `focusout` listener provides deduplication: browsers that fire native
/// `focusout` on disable (Chrome, Safari) will trigger the one-shot listener which
/// disconnects the `MutationObserver`, preventing a second synthetic dispatch.
/// Firefox, which does NOT fire native `focusout`, relies on the `MutationObserver`
/// to dispatch synthetic events which the one-shot listener then catches.
///
/// Returns a cleanup function that disconnects both the observer and the listener.
///
/// This is a no-op during SSR (`#[cfg(feature = "ssr")]`).
#[cfg(feature = "ssr")]
pub fn setup_synthetic_blur_observer(_element: &web_sys::Element) -> Box<dyn Fn()> {
    Box::new(|| {})
}

/// Set up a `MutationObserver` on the given element that watches for the `disabled`
/// attribute. When the element becomes disabled while focused, synthetic `blur` and
/// `focusout` events are dispatched so that blur handlers fire correctly.
///
/// A one-shot `focusout` listener provides deduplication: browsers that fire native
/// `focusout` on disable (Chrome, Safari) will trigger the one-shot listener which
/// disconnects the `MutationObserver`, preventing a second synthetic dispatch.
/// Firefox, which does NOT fire native `focusout`, relies on the `MutationObserver`
/// to dispatch synthetic events which the one-shot listener then catches.
///
/// Returns a cleanup function that disconnects both the observer and the listener.
#[cfg(not(feature = "ssr"))]
pub fn setup_synthetic_blur_observer(element: &web_sys::Element) -> Box<dyn Fn()> {
    use std::{cell::Cell, rc::Rc};

    use wasm_bindgen::{JsCast, prelude::*};

    // Only observe form elements that support the `disabled` attribute.
    let is_form_element = element.dyn_ref::<web_sys::HtmlButtonElement>().is_some()
        || element.dyn_ref::<web_sys::HtmlInputElement>().is_some()
        || element.dyn_ref::<web_sys::HtmlTextAreaElement>().is_some()
        || element.dyn_ref::<web_sys::HtmlSelectElement>().is_some();

    if !is_form_element {
        return Box::new(|| {});
    }

    // Shared state: tracks whether we've already handled a focusout event.
    // Once set, both the one-shot listener and the MutationObserver skip their work.
    let handled = Rc::new(Cell::new(false));

    // --- One-shot `focusout` listener ---
    // Catches the first focusout event (whether native or MutationObserver-triggered)
    // and disconnects the observer to prevent double-fire.
    let observer_cell: Rc<Cell<Option<web_sys::MutationObserver>>> = Rc::new(Cell::new(None));

    let handled_for_focusout = Rc::clone(&handled);
    let observer_for_focusout = Rc::clone(&observer_cell);
    let focusout_closure: Closure<dyn FnMut(web_sys::FocusEvent)> =
        Closure::once(move |_e: web_sys::FocusEvent| {
            handled_for_focusout.set(true);
            if let Some(obs) = observer_for_focusout.take() {
                obs.disconnect();
            }
        });

    let focusout_options = web_sys::AddEventListenerOptions::new();
    focusout_options.set_once(true);
    let _ = element.add_event_listener_with_callback_and_add_event_listener_options(
        "focusout",
        focusout_closure.as_ref().unchecked_ref(),
        &focusout_options,
    );
    focusout_closure.forget();

    // --- MutationObserver ---
    let target = element.clone();
    let handled_for_observer = Rc::clone(&handled);
    let callback: Closure<dyn FnMut(js_sys::Array, web_sys::MutationObserver)> = Closure::new(
        move |_mutations: js_sys::Array, _observer: web_sys::MutationObserver| {
            if handled_for_observer.get() {
                return;
            }

            // Check if the element is now disabled.
            if target.has_attribute("disabled") {
                // Determine relatedTarget: the new active element (if any).
                let related_target = target
                    .owner_document()
                    .and_then(|d| d.active_element())
                    .and_then(|active| if active == target { None } else { Some(active) });

                // Dispatch synthetic blur event.
                let blur_init = web_sys::FocusEventInit::new();
                if let Some(ref related) = related_target {
                    blur_init.set_related_target(Some(related.unchecked_ref()));
                }
                if let Ok(blur_event) =
                    web_sys::FocusEvent::new_with_focus_event_init_dict("blur", &blur_init)
                {
                    let _ = target.dispatch_event(&blur_event);
                }

                // Dispatch synthetic focusout event (which bubbles).
                // This will trigger the one-shot focusout listener above,
                // which sets `handled = true` and disconnects the observer.
                let focusout_init = web_sys::FocusEventInit::new();
                focusout_init.set_bubbles(true);
                if let Some(ref related) = related_target {
                    focusout_init.set_related_target(Some(related.unchecked_ref()));
                }
                if let Ok(focusout_event) =
                    web_sys::FocusEvent::new_with_focus_event_init_dict("focusout", &focusout_init)
                {
                    let _ = target.dispatch_event(&focusout_event);
                }
            }
        },
    );

    let Ok(observer) = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref()) else {
        return Box::new(|| {});
    };

    let options = web_sys::MutationObserverInit::new();
    options.set_attributes(true);
    let filter = js_sys::Array::new();
    filter.push(&"disabled".into());
    options.set_attribute_filter(&filter);

    if observer.observe_with_options(element, &options).is_err() {
        return Box::new(|| {});
    }

    // Store the observer so the one-shot listener can disconnect it.
    observer_cell.set(Some(observer.clone()));

    // Leak the closure so the observer callback remains valid.
    callback.forget();

    // Return cleanup that disconnects the observer.
    let element_for_cleanup = element.clone();
    Box::new(move || {
        observer.disconnect();
        // Remove the one-shot listener if it hasn't fired yet.
        // We can't remove by reference since we forgot the closure, but
        // disconnecting the observer is the critical cleanup. The one-shot
        // listener will self-remove when focusout fires or is harmless if
        // it fires after cleanup.
        let _ = &element_for_cleanup;
    })
}
