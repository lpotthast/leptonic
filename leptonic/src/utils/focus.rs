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
        use crate::hooks::{get_modality, Modality};

        if get_modality() == Modality::Virtual {
            let active_element = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.active_element());
            let element = element.clone();
            super::run_after_transition::run_after_transition(move || {
                let current_active = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.active_element());
                let body = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.body())
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
