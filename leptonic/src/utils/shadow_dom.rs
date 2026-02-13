//! Shadow DOM utilities for cross-shadow-boundary DOM operations.
//!
//! Provides functions for working with shadow DOM boundaries, including
//! piercing active element resolution and cross-shadow event target access.
//!
//! Based on react-aria's `@react-aria/utils/src/shadowdom/DOMFunctions.ts`
//! and `@react-aria/utils/src/domHelpers.ts`.

use wasm_bindgen::JsCast;

/// Returns `true` if the given node is a `ShadowRoot`.
#[allow(dead_code)]
pub fn is_shadow_root(node: &web_sys::Node) -> bool {
    node.dyn_ref::<web_sys::ShadowRoot>().is_some()
}

/// Get the active element, piercing through shadow DOM boundaries.
///
/// Starts with `document.activeElement` and follows the chain of
/// `element.shadowRoot?.activeElement` to find the deepest active element.
pub fn get_active_element(doc: &web_sys::Document) -> Option<web_sys::Element> {
    let mut active = doc.active_element()?;

    while let Some(shadow) = active.shadow_root() {
        if let Some(inner) = shadow.active_element() {
            active = inner;
        } else {
            break;
        }
    }

    Some(active)
}

/// Get the true event target, accounting for shadow DOM retargeting.
///
/// When an event originates inside a shadow root, `event.target` is retargeted
/// to the shadow host. This function uses `event.composedPath()[0]` to get the
/// original target element from within the shadow tree.
pub fn get_event_target<E: AsRef<web_sys::Event>>(event: &E) -> Option<web_sys::EventTarget> {
    let event = event.as_ref();
    let path = event.composed_path();
    if path.length() > 0 {
        let first = path.get(0);
        if !first.is_undefined() && !first.is_null() {
            return Some(first.unchecked_into());
        }
    }
    event.target()
}
