//! Internal animation watching utility.
//!
//! Waits for all active CSS animations/transitions on an element to finish,
//! then calls a callback. Returns a cancel function.

use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

/// Watches all active animations on `element` and calls `on_end` when they complete.
///
/// If the element has no active animations, `on_end` is called immediately.
/// Returns a cancel function that prevents `on_end` from being called if the
/// watcher is no longer needed (e.g., effect re-ran or component unmounted).
///
/// Errors from cancelled animation promises are silently ignored — this is
/// expected behavior when an animation is interrupted.
pub(super) fn watch_animations<T: FnOnce() + 'static>(
    element: &web_sys::Element,
    on_end: T,
) -> impl FnOnce() + use<T> {
    let cancelled = Rc::new(Cell::new(false));
    let cancel_handle = cancelled.clone();

    let animations = element.get_animations();
    if animations.length() == 0 {
        on_end();
    } else {
        let promises = js_sys::Array::new();
        for i in 0..animations.length() {
            let animation: web_sys::Animation = animations.get(i).unchecked_into();
            if let Ok(finished) = animation.finished() {
                promises.push(&finished);
            }
        }

        let all = js_sys::Promise::all(&promises);

        wasm_bindgen_futures::spawn_local(async move {
            // Ignore errors — cancelled animations reject their `finished` promise.
            let _ = JsFuture::from(all).await;
            if !cancelled.get() {
                on_end();
            }
        });
    }

    move || {
        cancel_handle.set(true);
    }
}
