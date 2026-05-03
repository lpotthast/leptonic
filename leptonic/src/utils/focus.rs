//! Focus utilities for managing element focus without side effects.
//!
//! This module provides utilities for focusing elements without triggering scroll,
//! and for safely focusing elements during screen reader interactions.

use std::fmt::Debug;

use wasm_bindgen::JsCast;

pub fn focus_event_target(target: &web_sys::EventTarget, prevent_scroll: bool) {
    if let Some(html_element) = try_downcast_to_html_element(target) {
        focus_html_element(html_element, prevent_scroll);
    }
}

pub fn focus_element(element: &web_sys::Element, prevent_scroll: bool) {
    if let Some(html_element) = try_downcast_to_html_element(element) {
        focus_html_element(html_element, prevent_scroll);
    }
}

pub fn focus_html_element(html_element: &web_sys::HtmlElement, prevent_scroll: bool) {
    let options = web_sys::FocusOptions::new();
    options.set_prevent_scroll(prevent_scroll);
    if let Err(err) = html_element.focus_with_options(&options) {
        tracing::warn!(?html_element, "Failed to focus element: {err:?}");
    }
}

/// Focus an element while avoiding undesired side effects such as page scrolling
/// and screen reader issues with CSS transitions.
///
/// When the user is interacting via a virtual cursor (screen reader), focus is
/// deferred until after any CSS transitions complete. This avoids `VoiceOver` on iOS
/// scrolling the page when the focused element is transitioning from off-screen.
///
/// In all other modalities, the element is focused immediately without scrolling.
///
/// Based on react-aria's `focusSafely` from
/// `packages/@react-aria/interactions/src/focusSafely.ts`.
pub fn focus_safely(element: &web_sys::Element) {
    #[cfg(feature = "ssr")]
    {
        let _ = element;
    }

    #[cfg(not(feature = "ssr"))]
    {
        use crate::{
            hooks::{Modality, get_modality},
            utils::shadow_dom::get_active_element,
        };

        if get_modality() == Modality::Virtual {
            // Use ownerDocument to correctly handle iframes and shadow DOM.
            let owner_doc = element.owner_document();
            let active_element = owner_doc.as_ref().and_then(get_active_element);
            let element = element.clone();
            super::run_after_transition::run_after_transition(move || {
                let owner_doc = element.owner_document();
                let current_active = owner_doc.as_ref().and_then(get_active_element);
                let body = owner_doc
                    .as_ref()
                    .and_then(web_sys::Document::body)
                    .map(web_sys::Element::from);

                // Focus only if focus hasn't moved to a different element
                // (still same or lost to body) and the target is still connected.
                let focus_unchanged = current_active == active_element || current_active == body;
                if focus_unchanged && element.is_connected() {
                    focus_element(&element, true);
                }
            });
        } else {
            focus_element(element, true);
        }
    }
}

fn try_downcast_to_html_element<T: JsCast + Debug>(value: &T) -> Option<&web_sys::HtmlElement> {
    if let Some(html_element) = value.dyn_ref::<web_sys::HtmlElement>() {
        Some(html_element)
    } else {
        tracing::warn!(?value, "Not an HTML element.");
        None
    }
}

/// Temporarily prevent an element from receiving focus.
///
/// Attaches `pointerdown` (with `preventDefault()`) and `focusin` (with `blur()`)
/// listeners to the element. These prevent focus from being acquired via pointer
/// interaction. Returns a cleanup function that removes the listeners.
///
/// This is useful for overlay and interaction hooks that need to temporarily
/// suppress focus on certain elements.
///
/// Based on react-aria's `preventFocus` from `utils.ts`.
///
/// During SSR, returns a no-op cleanup function.
#[cfg(feature = "ssr")]
pub fn prevent_focus(_element: &web_sys::Element) -> Box<dyn FnOnce()> {
    Box::new(|| {})
}

/// Temporarily prevent an element from receiving focus.
///
/// Attaches `pointerdown` (with `preventDefault()`) and `focusin` (with `blur()`)
/// listeners to the element. Returns a cleanup function that removes the listeners.
#[cfg(not(feature = "ssr"))]
pub fn prevent_focus(element: &web_sys::Element) -> Box<dyn FnOnce()> {
    use wasm_bindgen::closure::Closure;

    let element_for_focusin = element.clone();

    let pointerdown_handler: Closure<dyn FnMut(web_sys::PointerEvent)> =
        Closure::new(move |e: web_sys::PointerEvent| {
            e.prevent_default();
        });

    let focusin_handler: Closure<dyn FnMut(web_sys::FocusEvent)> =
        Closure::new(move |_e: web_sys::FocusEvent| {
            if let Some(html_el) = element_for_focusin.dyn_ref::<web_sys::HtmlElement>() {
                let _ = html_el.blur();
            }
        });

    let pd_fn: js_sys::Function = pointerdown_handler
        .as_ref()
        .unchecked_ref::<js_sys::Function>()
        .clone();
    let fi_fn: js_sys::Function = focusin_handler
        .as_ref()
        .unchecked_ref::<js_sys::Function>()
        .clone();

    let _ = element.add_event_listener_with_callback("pointerdown", &pd_fn);
    let _ = element.add_event_listener_with_callback("focusin", &fi_fn);

    let el = element.clone();
    Box::new(move || {
        let _ = el.remove_event_listener_with_callback("pointerdown", &pd_fn);
        let _ = el.remove_event_listener_with_callback("focusin", &fi_fn);
        // Drop closures to free them.
        drop(pointerdown_handler);
        drop(focusin_handler);
    })
}
