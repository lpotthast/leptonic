// Upstream: react-aria/src/utils/shadowdom/DOMFunctions.ts @ 99e6102368
// Upstream: react-aria/src/utils/domHelpers.ts @ 99e6102368
// Upstream: react-aria/test/utils/DOMFunctions.test.tsx @ 99e6102368
// Upstream: react-aria/test/utils/domHelpers.test.js @ 99e6102368
//! Shadow DOM utilities for cross-shadow-boundary DOM operations.
//!
//! Provides functions for working with shadow DOM boundaries, including
//! piercing active element resolution and cross-shadow event target access.
//!
//! Based on react-aria's `react-aria/src/utils/shadowdom/DOMFunctions.ts`
//! and `react-aria/src/utils/domHelpers.ts`.

use wasm_bindgen::JsCast;

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

/// Whether `node` contains `other` (or is it), across shadow roots and slots: a slotted element
/// counts as inside its slot's parent, an element in a shadow root as inside its host.
pub fn node_contains(node: &web_sys::Node, other: &web_sys::Node) -> bool {
    let mut current = Some(other.clone());
    while let Some(candidate) = current {
        if candidate == *node {
            return true;
        }
        let slot_parent = (!candidate.has_type::<web_sys::HtmlSlotElement>())
            .then(|| {
                candidate
                    .dyn_ref::<web_sys::Element>()?
                    .assigned_slot()?
                    .parent_node()
            })
            .flatten();
        current = match slot_parent {
            Some(parent) => Some(parent),
            None => match candidate.dyn_ref::<web_sys::ShadowRoot>() {
                Some(shadow) => Some(shadow.host().into()),
                None => candidate.parent_node(),
            },
        };
    }
    false
}

/// Get the true event target, accounting for shadow DOM retargeting.
///
/// When an event originates inside an open shadow root, `event.target` is retargeted to the shadow
/// host. For a target that hosts a shadow root, this returns `event.composedPath()[0]`, the original
/// target inside the shadow tree (react-aria's `getEventTarget`); otherwise `event.target`.
pub fn get_event_target<E: AsRef<web_sys::Event>>(event: &E) -> Option<web_sys::EventTarget> {
    let event = event.as_ref();
    let target = event.target();
    let hosts_shadow_root = target
        .as_ref()
        .and_then(|target| target.dyn_ref::<web_sys::Element>())
        .is_some_and(|element| element.shadow_root().is_some());
    if hosts_shadow_root {
        let first = event.composed_path().get(0);
        return (!first.is_undefined() && !first.is_null()).then(|| first.unchecked_into());
    }
    target
}

/// The targets a listener for a non-composed event (`scroll`, `change`, ...) from inside `from`
/// must be added to (react-aria's `getPropagationTargets`): `from`'s window, and every shadow root
/// between `from` and the document, since such events don't cross shadow boundaries.
#[cfg(not(feature = "ssr"))]
pub fn propagation_targets(from: &web_sys::Element) -> Vec<web_sys::EventTarget> {
    let mut targets: Vec<web_sys::EventTarget> = from
        .owner_document()
        .and_then(|document| document.default_view())
        .map(Into::into)
        .into_iter()
        .collect();
    let mut current = from.get_root_node();
    while let Some(shadow_root) = current.dyn_ref::<web_sys::ShadowRoot>() {
        targets.push(shadow_root.clone().into());
        current = shadow_root.host().get_root_node();
    }
    targets
}
