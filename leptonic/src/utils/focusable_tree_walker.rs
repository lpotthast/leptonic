// Upstream: react-aria/src/focus/FocusScope.tsx @ 99e6102368
// Upstream: react-aria/test/focus/FocusScope.test.js @ 99e6102368
//! Focusable tree walker utility (react-aria's `getFocusableTreeWalker`).
//!
//! Creates a `ShadowTreeWalker` with a baked-in filter that handles focusability/tabbability
//! checks, radio group deduplication, and custom accept callbacks.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `focusability: Focusability` instead of `tabbable?: boolean`.
// - No `from` option (react-aria: rejects the node and its descendants and starts there; no
//   caller uses it): callers position the walker with `set_current_node`.
// - No `scope` parameter (react-aria: the elements between a `FocusScope`'s sentinels): a scope
//   here is one element, the walker's root.
//
// =============================================================================

use std::sync::Arc;

use wasm_bindgen::JsCast;

use super::{
    focusability::{self, Focusability},
    shadow_tree_walker::{self, NodeFilterResult, ShadowTreeWalker},
};

/// Options for creating a focusable tree walker.
#[derive(Default)]
pub struct FocusableTreeWalkerOptions {
    /// Which elements the walker visits: focusable ones (default), or only tabbable ones.
    pub focusability: Focusability,

    /// Custom filter for acceptable elements.
    pub accept: Option<Arc<dyn Fn(&web_sys::Element) -> bool + Send + Sync>>,
}

/// Create a `ShadowTreeWalker` rooted at `root` that only visits focusable (or tabbable) elements.
///
/// The walker's filter (react-aria's `getFocusableTreeWalker`):
/// 1. for tabbable elements, rejects radios that aren't the tabbable one of their group, and
///    radios of the group of the node the walker is at,
/// 2. accepts focusable (or tabbable) elements the `accept` callback accepts.
///
/// The walker starts at `root`; position it with `set_current_node`.
pub fn get_focusable_tree_walker(
    root: &web_sys::Element,
    opts: FocusableTreeWalkerOptions,
) -> Option<ShadowTreeWalker> {
    let FocusableTreeWalkerOptions {
        focusability,
        accept,
    } = opts;

    let filter = Box::new(move |node: &web_sys::Node, current: &web_sys::Node| {
        let Some(el) = node.dyn_ref::<web_sys::Element>() else {
            return NodeFilterResult::Skip;
        };

        if focusability == Focusability::Tabbable
            && let Some(input) = el.dyn_ref::<web_sys::HtmlInputElement>()
            && input.get_attribute("type").as_deref() == Some("radio")
        {
            // Of a group of radios, only the checked one (or with none checked, every one) is
            // tabbable.
            if !focusability::is_tabbable_radio(input) {
                return NodeFilterResult::Reject;
            }
            // A radio of the group the walker is at: Tab leaves the group.
            if let Some(current) = current.dyn_ref::<web_sys::HtmlInputElement>()
                && current.type_() == "radio"
                && current.name() == input.name()
            {
                return NodeFilterResult::Reject;
            }
        }

        let matches = match focusability {
            Focusability::Focusable => focusability::is_focusable(el),
            Focusability::Tabbable => focusability::is_tabbable(el),
        };
        if matches && accept.as_ref().is_none_or(|accept| accept(el)) {
            NodeFilterResult::Accept
        } else {
            NodeFilterResult::Skip
        }
    });

    // 0x1 = NodeFilter.SHOW_ELEMENT
    shadow_tree_walker::create_shadow_tree_walker(root.as_ref(), 0x1, Some(filter))
}
