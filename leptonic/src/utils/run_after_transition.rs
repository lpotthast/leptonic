// Upstream: react-aria/src/utils/runAfterTransition.ts @ 99e6102368
//! Runs callbacks once all running CSS transitions have finished, so that style recalculations
//! (e.g. moving focus, restoring text selection) don't cause jank in the middle of a
//! transition.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## OMITTED FEATURES
// - The cancel function `runAfterTransition` returns: no caller cancels (react-aria's don't
//   either).
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - The global `transitionrun`/`transitionend` listeners are registered on first use instead of
//   at module load (Rust has no module initializers). Transitions that started before the first
//   call are not tracked. React-aria: registers when the module loads.
//
// =============================================================================

use std::cell::{OnceCell, RefCell};

use leptos::prelude::request_animation_frame;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{AddEventListenerOptions, Event, EventTarget, TransitionEvent};

use super::EventAccessors;

/// The listeners tracking transitions (shared, so the per-element `transitioncancel` listener
/// can be removed again).
struct Listeners {
    on_transition_start: js_sys::Function,
    on_transition_end: js_sys::Function,
}

thread_local! {
    // Elements that are currently transitioning, mapped to the CSS properties that are
    // transitioning. A set rather than a count because of browser bugs: e.g. Chrome sometimes
    // fires both `transitionend` and `transitioncancel` for one transition. `js_sys` collections,
    // as the keys are `EventTarget`s.
    static TRANSITIONS_BY_ELEMENT: js_sys::Map = js_sys::Map::new();
    static TRANSITION_CALLBACKS: RefCell<Vec<Box<dyn FnOnce(bool)>>> =
        const { RefCell::new(Vec::new()) };
    static LISTENERS: OnceCell<Listeners> = const { OnceCell::new() };
}

fn on_transition_start(e: &Event) {
    let Some(e) = e.dyn_ref::<TransitionEvent>() else {
        return;
    };
    let target = e.expect_target();
    TRANSITIONS_BY_ELEMENT.with(|map| {
        let existing = map.get(&target);
        let properties: js_sys::Set = if existing.is_undefined() {
            let set = js_sys::Set::new(&JsValue::UNDEFINED);
            map.set(&target, &set);
            // `transitioncancel` is registered on the element itself: an element removed while
            // transitioning has nowhere to bubble it to.
            with_listeners(|listeners| {
                let options = AddEventListenerOptions::new();
                options.set_once(true);
                let _ = target.add_event_listener_with_callback_and_add_event_listener_options(
                    "transitioncancel",
                    &listeners.on_transition_end,
                    &options,
                );
            });
            set
        } else {
            existing.unchecked_into()
        };
        properties.add(&JsValue::from_str(&e.property_name()));
    });
}

fn on_transition_end(e: &Event) {
    let Some(e) = e.dyn_ref::<TransitionEvent>() else {
        return;
    };
    let target: EventTarget = e.expect_target();
    let all_done = TRANSITIONS_BY_ELEMENT.with(|map| {
        let existing = map.get(&target);
        if existing.is_undefined() {
            return false;
        }
        let properties: js_sys::Set = existing.unchecked_into();
        properties.delete(&JsValue::from_str(&e.property_name()));
        if properties.size() == 0 {
            with_listeners(|listeners| {
                let _ = target.remove_event_listener_with_callback(
                    "transitioncancel",
                    &listeners.on_transition_end,
                );
            });
            map.delete(&target);
        }
        map.size() == 0
    });
    if all_done {
        // Taken out first: callbacks may queue new callbacks.
        let callbacks = TRANSITION_CALLBACKS.with_borrow_mut(std::mem::take);
        for callback in callbacks {
            callback(true);
        }
    }
}

/// Runs `f` with the shared listeners, registering them on the document on first use.
fn with_listeners(f: impl FnOnce(&Listeners)) {
    LISTENERS.with(|listeners| {
        let listeners = listeners.get_or_init(|| {
            let function = |handler: fn(&Event)| -> js_sys::Function {
                Closure::<dyn Fn(Event)>::new(move |e: Event| handler(&e))
                    .into_js_value()
                    .unchecked_into()
            };
            let listeners = Listeners {
                on_transition_start: function(on_transition_start),
                on_transition_end: function(on_transition_end),
            };
            if let Some(document) = leptos_use::use_document().as_ref() {
                let _ = document.add_event_listener_with_callback(
                    "transitionrun",
                    &listeners.on_transition_start,
                );
                let _ = document.add_event_listener_with_callback(
                    "transitionend",
                    &listeners.on_transition_end,
                );
            }
            listeners
        });
        f(listeners);
    });
}

/// Forgets elements that left the document: their `transitionend` never fires.
fn cleanup_detached_elements() {
    TRANSITIONS_BY_ELEMENT.with(|map| {
        let detached: Vec<JsValue> = map
            .keys()
            .into_iter()
            .filter_map(Result::ok)
            .filter(|key| {
                key.dyn_ref::<web_sys::Node>()
                    .is_some_and(|node| !node.is_connected())
            })
            .collect();
        for key in detached {
            map.delete(&key);
        }
    });
}

/// Runs `f` after all currently running CSS transitions have finished: waits one animation frame
/// (a transition may start on mount), then calls `f(false)` right away if nothing transitions,
/// or `f(true)` once the last transition ends.
pub(crate) fn run_after_transition(f: impl FnOnce(bool) + 'static) {
    with_listeners(|_| {});
    request_animation_frame(move || {
        cleanup_detached_elements();
        if TRANSITIONS_BY_ELEMENT.with(js_sys::Map::size) == 0 {
            f(false);
        } else {
            TRANSITION_CALLBACKS.with_borrow_mut(|callbacks| callbacks.push(Box::new(f)));
        }
    });
}
