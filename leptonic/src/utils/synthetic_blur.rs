// Upstream: react-aria/src/interactions/utils.ts @ 99e6102368
//! Blur events for form elements disabled while focused (react-aria's `useSyntheticBlurEvent`).
//!
//! Firefox fires no `blur`/`focusout` when a focused `<button>`, `<input>`, `<textarea>` or
//! `<select>` becomes disabled. A `MutationObserver` watches the `disabled` attribute and
//! dispatches the events itself. Browsers that do fire `focusout` fire it before the observer
//! runs, and the observer stops then, so handlers never see two blurs.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - A guard instead of a hook: [`SyntheticBlurObserver::observe`] starts watching on focus, and
//   dropping the guard (on blur or when the hook's owner is disposed) disconnects the observer
//   and removes its listener. React-aria keeps the observer in a ref and disconnects it on blur
//   and unmount.
// - The dispatched `blur`/`focusout` reach the hooks' own listeners; there is no separate
//   `onBlur` callback (react-aria calls it from the `focusout` listener for a disabled target).
//
// =============================================================================

use std::{cell::Cell, rc::Rc};

use wasm_bindgen::{JsCast, closure::Closure};

use crate::utils::shadow_dom::get_active_element;

/// Watches a focused form element for becoming disabled; see the module docs. Dropping it stops
/// watching.
pub(crate) struct SyntheticBlurObserver {
    target: web_sys::Element,
    observer: web_sys::MutationObserver,
    on_focusout: Closure<dyn FnMut(web_sys::FocusEvent)>,
    _on_mutation: Closure<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>,
}

impl SyntheticBlurObserver {
    /// Starts watching `target`, which just received focus. `None` for elements that can't be
    /// disabled.
    pub(crate) fn observe(target: &web_sys::Element) -> Option<Self> {
        // Only elements that can be disabled.
        is_disabled(target)?;
        let is_focused = Rc::new(Cell::new(true));
        let observer_slot: Rc<Cell<Option<web_sys::MutationObserver>>> = Rc::new(Cell::new(None));

        let on_focusout = {
            let is_focused = Rc::clone(&is_focused);
            let observer_slot = Rc::clone(&observer_slot);
            // The target was blurred (natively or by the observer): stop watching.
            Closure::<dyn FnMut(web_sys::FocusEvent)>::new(move |_: web_sys::FocusEvent| {
                is_focused.set(false);
                if let Some(observer) = observer_slot.take() {
                    observer.disconnect();
                }
            })
        };
        let options = web_sys::AddEventListenerOptions::new();
        options.set_once(true);
        target
            .add_event_listener_with_callback_and_add_event_listener_options(
                "focusout",
                on_focusout.as_ref().unchecked_ref(),
                &options,
            )
            .ok()?;

        let on_mutation = {
            let target = target.clone();
            let is_focused = Rc::clone(&is_focused);
            let observer_slot = Rc::clone(&observer_slot);
            Closure::<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>::new(
                move |_: js_sys::Array, _: web_sys::MutationObserver| {
                    if !is_focused.get() || is_disabled(&target) != Some(true) {
                        return;
                    }
                    if let Some(observer) = observer_slot.take() {
                        observer.disconnect();
                    }
                    let related_target = target
                        .owner_document()
                        .and_then(|document| get_active_element(&document))
                        .filter(|active| *active != target);
                    dispatch_focus_event(&target, "blur", false, related_target.as_ref());
                    dispatch_focus_event(&target, "focusout", true, related_target.as_ref());
                },
            )
        };
        let observer = web_sys::MutationObserver::new(on_mutation.as_ref().unchecked_ref()).ok()?;
        let init = web_sys::MutationObserverInit::new();
        init.set_attributes(true);
        init.set_attribute_filter(&js_sys::Array::of1(&"disabled".into()));
        observer.observe_with_options(target, &init).ok()?;
        observer_slot.set(Some(observer.clone()));

        Some(Self {
            target: target.clone(),
            observer,
            on_focusout,
            _on_mutation: on_mutation,
        })
    }
}

impl Drop for SyntheticBlurObserver {
    fn drop(&mut self) {
        self.observer.disconnect();
        let _ = self.target.remove_event_listener_with_callback(
            "focusout",
            self.on_focusout.as_ref().unchecked_ref(),
        );
    }
}

/// Whether a form element is disabled; `None` for elements that can't be disabled.
fn is_disabled(element: &web_sys::Element) -> Option<bool> {
    if let Some(button) = element.dyn_ref::<web_sys::HtmlButtonElement>() {
        Some(button.disabled())
    } else if let Some(input) = element.dyn_ref::<web_sys::HtmlInputElement>() {
        Some(input.disabled())
    } else if let Some(text_area) = element.dyn_ref::<web_sys::HtmlTextAreaElement>() {
        Some(text_area.disabled())
    } else {
        element
            .dyn_ref::<web_sys::HtmlSelectElement>()
            .map(web_sys::HtmlSelectElement::disabled)
    }
}

fn dispatch_focus_event(
    target: &web_sys::Element,
    kind: &str,
    bubbles: bool,
    related_target: Option<&web_sys::Element>,
) {
    let init = web_sys::FocusEventInit::new();
    init.set_bubbles(bubbles);
    if let Some(related_target) = related_target {
        init.set_related_target(Some(related_target.unchecked_ref()));
    }
    if let Ok(event) = web_sys::FocusEvent::new_with_focus_event_init_dict(kind, &init) {
        let _ = target.dispatch_event(&event);
    }
}
