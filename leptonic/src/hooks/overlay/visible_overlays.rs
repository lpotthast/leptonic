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

/// Push an overlay element onto the stack. Skips if already present.
pub(super) fn push_overlay(element: &web_sys::Element) {
    VISIBLE_OVERLAYS.with_borrow_mut(|stack| {
        if !stack.iter().any(|el| **el == *element) {
            stack.push(SendWrapper::new(element.clone()));
        }
    });
}

/// Remove an overlay element from the stack.
pub(super) fn remove_overlay(element: &web_sys::Element) {
    VISIBLE_OVERLAYS.with_borrow_mut(|stack| {
        stack.retain(|el| **el != *element);
    });
}

/// Check if the given element is the topmost (last) in the overlay stack.
pub(super) fn is_topmost(element: &web_sys::Element) -> bool {
    VISIBLE_OVERLAYS.with_borrow(|stack| stack.last().is_some_and(|el| **el == *element))
}
