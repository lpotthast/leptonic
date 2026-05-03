//! Focusable tree walker utility.
//!
//! Creates a `ShadowTreeWalker` with a baked-in filter that handles focusability/tabbability
//! checks, radio group deduplication, and custom accept callbacks.
//!
//! Based on react-aria's `getFocusableTreeWalker` from `@react-aria/focus/src/FocusScope.tsx`.

use std::sync::Arc;

use wasm_bindgen::JsCast;

use super::{
    focusability,
    shadow_tree_walker::{self, ShadowTreeWalker},
};

/// Options for creating a focusable tree walker.
#[derive(Default)]
pub struct FocusableTreeWalkerOptions {
    /// Whether to filter for tabbable elements only (vs all focusable elements).
    pub tabbable: bool,

    /// Starting element. The walker will be positioned at this element.
    pub from: Option<web_sys::Element>,

    /// Radio group name of the `from` element. Used to skip other radios in the same group.
    pub from_radio_group: Option<String>,

    /// Custom filter for acceptable elements.
    pub accept: Option<Arc<dyn Fn(&web_sys::Element) -> bool + Send + Sync>>,
}

/// Create a `ShadowTreeWalker` rooted at `root` that only visits focusable (or tabbable) elements.
///
/// The walker's filter handles:
/// 1. `is_focusable()` or `is_tabbable()` check (based on `opts.tabbable`)
/// 2. Radio group dedup via `is_tabbable_radio()` + same-group skip
/// 3. Custom `accept` callback
///
/// If `opts.from` is provided, the walker is positioned at that element.
pub fn get_focusable_tree_walker(
    root: &web_sys::Element,
    opts: FocusableTreeWalkerOptions,
) -> Option<ShadowTreeWalker> {
    let tabbable = opts.tabbable;
    let from_radio_group = opts.from_radio_group;
    let accept = opts.accept;

    let filter: Box<dyn Fn(&web_sys::Node) -> bool> = Box::new(move |node: &web_sys::Node| {
        let Some(el) = node.dyn_ref::<web_sys::Element>() else {
            return false;
        };

        if tabbable {
            // Radio group handling: skip non-tabbable radios and radios in the
            // same group as the starting element.
            if let Some(input) = el.dyn_ref::<web_sys::HtmlInputElement>() {
                if input.type_() == "radio" {
                    if !focusability::is_tabbable_radio(input) {
                        return false;
                    }
                    if let Some(ref group) = from_radio_group {
                        if input.name() == *group {
                            return false;
                        }
                    }
                }
            }
            if !focusability::is_tabbable(el) {
                return false;
            }
        } else if !focusability::is_focusable(el) {
            return false;
        }

        if let Some(ref accept_fn) = accept {
            if !accept_fn(el) {
                return false;
            }
        }

        true
    });

    // 0x1 = NodeFilter.SHOW_ELEMENT
    let walker = shadow_tree_walker::create_shadow_tree_walker(root.as_ref(), 0x1, Some(filter))?;

    if let Some(ref from) = opts.from {
        walker.set_current_node(from);
    }

    Some(walker)
}
