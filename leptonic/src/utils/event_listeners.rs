// Upstream: react-aria/src/utils/useGlobalListeners.ts @ 99e6102368
//! Listeners added for the duration of an interaction (react-aria's `useGlobalListeners`).

use std::borrow::Cow;

use leptos::ev::EventDescriptor;
use wasm_bindgen::{JsCast, closure::Closure};

/// An event listener, removed when dropped.
///
/// For listeners that come and go with an interaction (a press, a drag), e.g. on the document
/// between pointer down and pointer up. Unlike `leptos_use::use_event_listener`, it doesn't
/// register anything with the reactive owner, so creating one per interaction doesn't
/// accumulate effects and cleanups until the component unmounts. Its handler is an `Fn`
/// closure, so it can be re-entered (e.g. a focus listener that moves focus).
pub(crate) struct Listener {
    target: web_sys::EventTarget,
    event: Cow<'static, str>,
    capture: bool,
    closure: Closure<dyn Fn(web_sys::Event)>,
}

impl std::fmt::Debug for Listener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Listener")
            .field("event", &self.event)
            .field("capture", &self.capture)
            .finish_non_exhaustive()
    }
}

impl Drop for Listener {
    fn drop(&mut self) {
        let _ = self.target.remove_event_listener_with_callback_and_bool(
            &self.event,
            self.closure.as_ref().unchecked_ref(),
            self.capture,
        );
    }
}

/// Listen for the event named `event` on `target` until the returned [`Listener`] is dropped.
pub(crate) fn listen(
    target: &web_sys::EventTarget,
    event: impl Into<Cow<'static, str>>,
    capture: bool,
    handler: impl Fn(web_sys::Event) + 'static,
) -> Listener {
    listen_with_options(target, event, capture, None, handler)
}

/// The same listener, with the event descriptor's optional native listener options.
pub(crate) fn listen_with_options(
    target: &web_sys::EventTarget,
    event: impl Into<Cow<'static, str>>,
    capture: bool,
    options: Option<&web_sys::AddEventListenerOptions>,
    handler: impl Fn(web_sys::Event) + 'static,
) -> Listener {
    let event = event.into();
    let closure = Closure::<dyn Fn(web_sys::Event)>::new(handler);
    if let Some(options) = options {
        let _ = target.add_event_listener_with_callback_and_add_event_listener_options(
            &event,
            closure.as_ref().unchecked_ref(),
            options,
        );
    } else {
        let _ = target.add_event_listener_with_callback_and_bool(
            &event,
            closure.as_ref().unchecked_ref(),
            capture,
        );
    }
    Listener {
        target: target.clone(),
        event,
        capture: options.map_or(capture, |options| options.get_capture().unwrap_or(false)),
        closure,
    }
}

/// Listen for `event` (e.g. `ev::pointermove`) on `target` until the returned [`Listener`] is
/// dropped, with the typed event.
pub(crate) fn listen_to<D>(
    target: &web_sys::EventTarget,
    event: D,
    capture: bool,
    handler: impl Fn(D::EventType) + 'static,
) -> Listener
where
    D: EventDescriptor,
    D::EventType: JsCast,
{
    listen(target, event.name(), capture, move |e| {
        handler(e.unchecked_into());
    })
}
