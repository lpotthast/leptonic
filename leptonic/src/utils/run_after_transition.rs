/// Transition-aware callback utility, matching react-aria's `runAfterTransition.ts`.
///
/// Tracks elements that are currently transitioning via global `transitionrun`/`transitionend`
/// event listeners on `document.body`. Callbacks queued via [`run_after_transition`] will be
/// called once all tracked transitions have completed, ensuring style recalculations don't
/// cause jank in the middle of CSS transitions.
use std::cell::{Cell, RefCell};

// Loosely based on https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/utils/src/runAfterTransition.ts
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, prelude::*};

use super::EventTargetExt;

// We store a global map of elements that are currently transitioning,
// mapped to a set of CSS properties that are transitioning for that element.
// This is necessary rather than a simple count of transitions because of browser
// bugs, e.g. Chrome sometimes fires both transitionend and transitioncancel rather
// than one or the other. So we need to track what's actually transitioning so that
// we can ignore these duplicate events.
//
// We use `js_sys::Map` and `js_sys::Set` here because the keys are `EventTarget`
// (JS objects) and Rust's `HashMap` cannot use them as keys without hashing issues.
thread_local! {
    static TRANSITIONS_BY_ELEMENT: js_sys::Map = js_sys::Map::new();
    static TRANSITION_CALLBACKS: RefCell<Vec<Box<dyn FnOnce()>>> = RefCell::new(Vec::new());
    static INITIALIZED: Cell<bool> = const { Cell::new(false) };
}

fn setup_global_events() {
    let was_initialized = INITIALIZED.with(|init| init.replace(true));
    if was_initialized {
        return;
    }

    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(body) = document.body() else {
        return;
    };

    // --- transitionrun handler ---
    let on_transition_start: Closure<dyn Fn(web_sys::TransitionEvent)> =
        Closure::new(move |e: web_sys::TransitionEvent| {
            let Some(target) = e.target() else {
                return;
            };
            let property_name = e.property_name();

            TRANSITIONS_BY_ELEMENT.with(|map| {
                let existing = map.get(&target);
                let properties: js_sys::Set = if existing.is_undefined() {
                    let set = js_sys::Set::new(&JsValue::UNDEFINED);
                    map.set(&target, &set);

                    // The transitioncancel event must be registered on the element itself,
                    // rather than as a global event. This enables us to handle when the node
                    // is deleted from the document while it is transitioning.
                    // In that case, the cancel event would have nowhere to bubble to so we
                    // need to handle it directly.
                    let target_clone = target.clone();
                    target.listen_once("transitioncancel", move |e: web_sys::TransitionEvent| {
                        let property_name = e.property_name();
                        handle_transition_end(&target_clone, &property_name);
                    });

                    set
                } else {
                    existing.unchecked_into()
                };

                properties.add(&JsValue::from_str(&property_name));
            });
        });

    let _ = body.add_event_listener_with_callback(
        "transitionrun",
        on_transition_start.as_ref().unchecked_ref(),
    );
    on_transition_start.forget();

    // --- transitionend handler ---
    let on_transition_end: Closure<dyn Fn(web_sys::TransitionEvent)> =
        Closure::new(move |e: web_sys::TransitionEvent| {
            let Some(target) = e.target() else {
                return;
            };
            let property_name = e.property_name();
            handle_transition_end(&target, &property_name);
        });

    let _ = body.add_event_listener_with_callback(
        "transitionend",
        on_transition_end.as_ref().unchecked_ref(),
    );
    on_transition_end.forget();
}

fn handle_transition_end(target: &web_sys::EventTarget, property_name: &str) {
    TRANSITIONS_BY_ELEMENT.with(|map| {
        let existing = map.get(target);
        if existing.is_undefined() {
            return;
        }
        let properties: js_sys::Set = existing.unchecked_into();

        properties.delete(&JsValue::from_str(property_name));

        // If empty, remove the element from the map.
        if properties.size() == 0 {
            let _ = target
                .remove_event_listener_with_callback("transitioncancel", &JsValue::NULL.into());
            map.delete(target);
        }

        // If no transitioning elements, call all of the queued callbacks.
        if map.size() == 0 {
            TRANSITION_CALLBACKS.with(|callbacks| {
                let drained = std::mem::take(&mut *callbacks.borrow_mut());
                for cb in drained {
                    cb();
                }
            });
        }
    });
}

/// Cleans up any elements that are no longer in the document.
/// This is necessary because we can't rely on transitionend events to fire
/// for elements that are removed from the document while transitioning.
fn cleanup_detached_elements() {
    TRANSITIONS_BY_ELEMENT.with(|map| {
        let keys_to_remove: Vec<JsValue> = map
            .keys()
            .into_iter()
            .filter_map(Result::ok)
            .filter(|key| {
                // Check if the event target has isConnected and is disconnected
                let is_connected = js_sys::Reflect::get(key, &JsValue::from_str("isConnected"));
                matches!(is_connected, Ok(val) if val == JsValue::FALSE)
            })
            .collect();

        for key in keys_to_remove {
            map.delete(&key);
        }
    });
}

/// Run a callback after all currently running CSS transitions have completed.
///
/// This waits one animation frame to detect if any transitions start (e.g. on mount),
/// then either calls the function immediately if no transitions are running, or queues
/// it until all transitions end.
pub(crate) fn run_after_transition(f: impl FnOnce() + 'static) {
    setup_global_events();

    // Wait one frame to see if an animation starts, e.g. a transition on mount.
    let callback = Closure::once(Box::new(move || {
        cleanup_detached_elements();

        TRANSITIONS_BY_ELEMENT.with(|map| {
            if map.size() == 0 {
                // No transitions running, call immediately.
                f();
            } else {
                // Queue the callback for after transitions complete.
                TRANSITION_CALLBACKS.with(|callbacks| {
                    callbacks.borrow_mut().push(Box::new(f));
                });
            }
        });
    }) as Box<dyn FnOnce()>);

    if let Some(window) = web_sys::window() {
        let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
    }
    callback.forget();
}
