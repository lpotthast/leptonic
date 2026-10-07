// Upstream: react-aria/src/overlays/useCloseOnScroll.ts @ 99e6102368

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `on_close` is passed explicitly (react-aria falls back to a close handler registered for the
//   trigger by `useOverlayTrigger`, for backward compatibility).
// - The trigger is a `CapturedElement` (react-aria: a ref).
//
// =============================================================================

use leptos::prelude::*;
use leptos_element_capture::CapturedElement;
#[cfg(not(feature = "ssr"))]
use send_wrapper::SendWrapper;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;

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
        use leptos::ev;

        use crate::utils::{
            event_listeners::{Listener, listen_to},
            shadow_dom::{get_event_target, node_contains, propagation_targets},
        };

        let listeners: StoredValue<Vec<SendWrapper<Listener>>> = StoredValue::new(Vec::new());
        Effect::new(move |_| {
            listeners.update_value(Vec::clear);
            if !is_open.get() {
                return;
            }
            let Some(trigger_el) = trigger_element.get() else {
                return;
            };
            let trigger_node: web_sys::Node = (*trigger_el).clone().into();
            let trigger_node = SendWrapper::new(trigger_node);
            // Scroll events don't bubble: listen in the capture phase on the window and on every
            // shadow root around the trigger (scroll events don't cross shadow boundaries).
            let handles = propagation_targets(&trigger_el)
                .into_iter()
                .map(|target| {
                    let trigger_node = trigger_node.clone();
                    SendWrapper::new(listen_to(
                        &target,
                        ev::scroll,
                        true,
                        move |e: web_sys::Event| {
                            let Some(target) = get_event_target(&e) else {
                                return;
                            };
                            // Ignore scrolling regions outside the trigger's tree. The window isn't a
                            // node, and contains everything.
                            if let Some(target_node) = target.dyn_ref::<web_sys::Node>()
                                && !node_contains(target_node, &trigger_node)
                            {
                                return;
                            }
                            // Ignore scrolling inputs and text areas: their cursor position can scroll
                            // them (e.g. in a combo box).
                            if target.dyn_ref::<web_sys::HtmlInputElement>().is_some()
                                || target.dyn_ref::<web_sys::HtmlTextAreaElement>().is_some()
                            {
                                return;
                            }
                            on_close.run(());
                        },
                    ))
                })
                .collect();
            listeners.set_value(handles);
        });
        on_cleanup(move || listeners.update_value(Vec::clear));
    }

    // SSR: no-op, suppress unused variable warnings.
    #[cfg(feature = "ssr")]
    {
        let _ = (is_open, trigger_element, on_close);
    }
}
