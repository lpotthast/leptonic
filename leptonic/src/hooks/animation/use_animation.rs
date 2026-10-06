// Upstream: react-aria/src/utils/animation.ts @ 99e6102368
//! Internal animation watching utility (react-aria's `useAnimation`).

use std::{cell::Cell, rc::Rc};

use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

/// Calls `on_start` with `element` (it may start Web Animations), then waits for the element's
/// running animations on the document timeline (CSS animations and transitions, Web Animations)
/// and calls `on_end` once they finished; at once without any.
///
/// Returns a cancel function that keeps `on_end` from being called (an effect re-ran, the
/// component unmounted). Interrupted animations reject their `finished` promise; that is ignored.
pub(super) fn watch_animations<T: FnOnce() + 'static>(
    element: &web_sys::Element,
    on_start: Option<Callback<SendWrapper<web_sys::Element>>>,
    on_end: T,
) -> impl FnOnce() + use<T> {
    let cancelled = Rc::new(Cell::new(false));
    let cancel_handle = cancelled.clone();

    if let Some(on_start) = on_start {
        on_start.run(SendWrapper::new(element.clone()));
    }

    // Animations on other timelines (scroll-driven ones) and paused ones never finish by
    // themselves.
    let animations = element.get_animations();
    let promises = js_sys::Array::new();
    for i in 0..animations.length() {
        let animation: web_sys::Animation = animations.get(i).unchecked_into();
        let on_document_timeline = animation
            .timeline()
            .is_none_or(|timeline| timeline.is_instance_of::<web_sys::DocumentTimeline>());
        if on_document_timeline
            && animation.play_state() == web_sys::AnimationPlayState::Running
            && let Ok(finished) = animation.finished()
        {
            promises.push(&finished);
        }
    }

    if promises.length() == 0 {
        on_end();
    } else {
        let all = js_sys::Promise::all(&promises);
        wasm_bindgen_futures::spawn_local(async move {
            if JsFuture::from(all).await.is_ok() && !cancelled.get() {
                on_end();
            }
        });
    }

    move || {
        cancel_handle.set(true);
    }
}
