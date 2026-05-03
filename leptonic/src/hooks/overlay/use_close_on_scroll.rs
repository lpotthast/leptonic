//
// This hook is based on React Aria's `useCloseOnScroll`:
// https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/overlays/src/useCloseOnScroll.ts
//
// ## DIFFERENT BEHAVIOR
//
// - React Aria's `useCloseOnScroll` is called from `useOverlayPosition` and can
//   also retrieve the close handler from a `WeakMap` keyed by the trigger element
//   (for backward compat with `useOverlayTrigger`). Leptonic passes `on_close`
//   explicitly.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
//
// - Uses `CapturedElement` instead of a React ref for the trigger element.
// - Uses `Effect::new` + `on_cleanup` instead of `useEffect`.
//

use leptos::prelude::*;
use leptos_element_capture::CapturedElement;
#[cfg(not(feature = "ssr"))]
use send_wrapper::SendWrapper;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::{JsCast, prelude::Closure};

/// Input parameters for the `use_close_on_scroll` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseCloseOnScrollInput {
    /// Whether the overlay is currently open.
    pub is_open: Signal<bool>,

    /// The trigger element whose scroll parents are monitored.
    pub trigger_element: CapturedElement,

    /// Called when a scroll event is detected on a parent of the trigger.
    pub on_close: Callback<()>,
}

/// Closes the overlay when a scrollable ancestor of the trigger element scrolls.
///
/// This prevents stale positioning when the trigger scrolls out of view.
/// Ignores scroll events on input/textarea elements (e.g. combobox text scrolling).
pub fn use_close_on_scroll(input: UseCloseOnScrollInput) {
    let UseCloseOnScrollInput {
        is_open,
        trigger_element,
        on_close,
    } = input;

    #[cfg(not(feature = "ssr"))]
    {
        let cleanup_listener: StoredValue<Option<SendWrapper<Box<dyn FnOnce()>>>, LocalStorage> =
            StoredValue::new_local(None);

        let do_cleanup = move || {
            cleanup_listener.update_value(|opt| {
                if let Some(f) = opt.take() {
                    f.take()();
                }
            });
        };

        Effect::new(move |_| {
            // Always clean up previous listener first.
            do_cleanup();

            if !is_open.get() {
                return;
            }

            let Some(trigger_el) = trigger_element.get() else {
                return;
            };

            let Some(window) = web_sys::window() else {
                return;
            };

            // Single handler on the window in the capture phase.
            // Capture phase is required because scroll events do not bubble —
            // they only fire on the element that scrolled. Capturing on window
            // lets us intercept scroll events from any element in the page.
            let handler = Closure::<dyn Fn(web_sys::Event)>::new(move |e: web_sys::Event| {
                let Some(target) = e.target() else {
                    return;
                };

                // Ignore scroll events on elements that don't contain the trigger.
                // Window-level scroll events have `document` as target, and
                // `Document::contains()` returns true for all elements in the
                // document, so document-level scroll correctly passes this check.
                if let Some(target_node) = target.dyn_ref::<web_sys::Node>() {
                    let trigger_node: &web_sys::Node = trigger_el.unchecked_ref();
                    if !target_node.contains(Some(trigger_node)) {
                        return;
                    }
                }

                // Ignore scroll events on input/textarea elements — their cursor
                // position can cause internal scrolling (e.g. combobox input).
                if target.dyn_ref::<web_sys::HtmlInputElement>().is_some()
                    || target.dyn_ref::<web_sys::HtmlTextAreaElement>().is_some()
                {
                    return;
                }

                on_close.run(());
            });

            let handler_fn = handler.as_ref().unchecked_ref::<js_sys::Function>().clone();

            let _ = window.add_event_listener_with_callback_and_bool(
                "scroll",
                &handler_fn,
                true, // capture phase
            );

            // Store cleanup that removes the listener and drops the closure.
            let cleanup_fn: Box<dyn FnOnce()> = Box::new(move || {
                let _ = window.remove_event_listener_with_callback_and_bool(
                    "scroll",
                    handler.as_ref().unchecked_ref(),
                    true,
                );
                // `handler` (Closure) is dropped here, releasing the JS reference.
            });

            cleanup_listener.set_value(Some(SendWrapper::new(cleanup_fn)));
        });

        on_cleanup(do_cleanup);
    }

    // SSR: no-op, suppress unused variable warnings.
    #[cfg(feature = "ssr")]
    {
        let _ = (is_open, trigger_element, on_close);
    }
}
