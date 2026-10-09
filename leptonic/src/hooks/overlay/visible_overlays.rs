// Upstream: react-aria/src/overlays/useOverlay.ts @ 99e6102368
#![cfg_attr(feature = "ssr", allow(dead_code))]

//! Thread-local stack of visible overlays.
//!
//! This mirrors react-aria's module-level `visibleOverlays: RefObject<Element | null>[]`
//! in `useOverlay.ts`. The stack tracks which overlays are currently open in order,
//! so that dismiss actions (Escape, outside click) only close the topmost overlay.

use std::cell::RefCell;

use send_wrapper::SendWrapper;

thread_local! {
    static VISIBLE_OVERLAYS: RefCell<Vec<SendWrapper<web_sys::Element>>> = const { RefCell::new(Vec::new()) };
}

/// Pushes an overlay element onto the stack, also when it is on it already:
///
/// One entry per open overlay: the overlays of a group (a root popover and its submenus' popovers)
/// share their group's element, which stays on the stack while any of them is open.
pub(super) fn push_overlay(element: &web_sys::Element) {
    VISIBLE_OVERLAYS.with_borrow_mut(|stack| {
        stack.push(SendWrapper::new(element.clone()));
    });
}

/// Remove one entry of an overlay element from the stack.
pub(super) fn remove_overlay(element: &web_sys::Element) {
    VISIBLE_OVERLAYS.with_borrow_mut(|stack| {
        if let Some(index) = stack.iter().rposition(|el| **el == *element) {
            stack.remove(index);
        }
    });
}

/// Check if the given element is the topmost (last) in the overlay stack.
pub(super) fn is_topmost(element: &web_sys::Element) -> bool {
    VISIBLE_OVERLAYS.with_borrow(|stack| stack.last().is_some_and(|el| **el == *element))
}
