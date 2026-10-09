// Upstream: react-aria/src/interactions/focusSafely.ts @ 99e6102368
// Upstream: react-aria/src/utils/focusWithoutScrolling.ts @ 99e6102368
// Upstream: react-aria/test/interactions/focusSafely.test.js @ 99e6102368
//! Focus utilities for managing element focus without side effects.
//!
//! This module provides utilities for focusing elements without triggering scroll,
//! and for safely focusing elements during screen reader interactions.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## OMITTED FEATURES
// - `focusWithoutScrolling`'s fallback for browsers without `focus({preventScroll})` (saving and
//   restoring the scroll positions of every scroll parent): every supported browser has it.
//
// =============================================================================

use wasm_bindgen::JsCast;

/// Focuses `target` if it is an HTML or SVG element (see [`focus_element`]).
pub fn focus_event_target(target: &web_sys::EventTarget, prevent_scroll: bool) {
    if let Some(element) = target.dyn_ref::<web_sys::Element>() {
        focus_element(element, prevent_scroll);
    } else {
        tracing::warn!(?target, "Not an element, can't focus it.");
    }
}

/// Focuses an HTML or SVG element (both can take focus), optionally without scrolling it into
/// view (react-aria's `focusWithoutScrolling`).
pub fn focus_element(element: &web_sys::Element, prevent_scroll: bool) {
    let options = web_sys::FocusOptions::new();
    options.set_prevent_scroll(prevent_scroll);
    let result = if let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
        html_element.focus_with_options(&options)
    } else if let Some(svg_element) = element.dyn_ref::<web_sys::SvgElement>() {
        svg_element.focus_with_options(&options)
    } else {
        tracing::warn!(
            ?element,
            "Neither an HTML nor an SVG element, can't focus it."
        );
        return;
    };
    if let Err(err) = result {
        tracing::warn!(?element, "Failed to focus element: {err:?}");
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
pub fn focus_safely(element: &web_sys::Element) {
    #[cfg(feature = "ssr")]
    {
        let _ = element;
    }

    #[cfg(not(feature = "ssr"))]
    {
        use crate::{
            hooks::focus::{Modality, get_modality},
            utils::shadow_dom::get_active_element,
        };

        if !element.is_connected() {
            return;
        }

        if get_modality() == Some(Modality::Virtual) {
            // Use ownerDocument to correctly handle iframes and shadow DOM.
            let owner_doc = element.owner_document();
            let active_element = owner_doc.as_ref().and_then(get_active_element);
            let element = element.clone();
            super::run_after_transition::run_after_transition(move |_| {
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
