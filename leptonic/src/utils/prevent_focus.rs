// Upstream: react-aria/src/interactions/utils.ts @ 99e6102368
//! Keeping focus where it is when an element is pressed (react-aria's `preventFocus`).

#[cfg(not(feature = "ssr"))]
use std::cell::Cell;

#[cfg(not(feature = "ssr"))]
thread_local! {
    /// Whether focus events are being redirected by [`prevent_focus`] (focus-visible tracking
    /// ignores them meanwhile, as react-aria's `ignoreFocusEvent`).
    static IGNORE_FOCUS_EVENT: Cell<bool> = const { Cell::new(false) };
}

/// Whether focus events are being redirected by [`prevent_focus`].
#[cfg(not(feature = "ssr"))]
pub(crate) fn is_ignoring_focus_events() -> bool {
    IGNORE_FOCUS_EVENT.with(Cell::get)
}

/// A running [`prevent_focus`]: it ends by itself after the next frame, or earlier with
/// [`dispose`](Self::dispose) (when the press ends or its element unmounts).
#[cfg(not(feature = "ssr"))]
pub(crate) struct FocusPrevention {
    cleanup: Box<dyn Fn()>,
}

#[cfg(not(feature = "ssr"))]
impl FocusPrevention {
    /// Ends the prevention now: removes its listeners (react-aria's returned cleanup).
    pub(crate) fn dispose(self) {
        (self.cleanup)();
    }
}

/// Keeps the focus on the active element while `target` (or its nearest focusable ancestor, which
/// the browser would focus) is pressed: the focus and blur events of the move are stopped and the
/// focus moves back. Call it on mouse down; it cleans up after the next frame, or when disposed.
/// `None` when there is nothing to prevent (the focus is on the target already).
#[cfg(not(feature = "ssr"))]
pub(crate) fn prevent_focus(target: Option<web_sys::Element>) -> Option<FocusPrevention> {
    use std::{cell::RefCell, rc::Rc};

    use wasm_bindgen::JsCast;

    use crate::utils::{
        event_listeners::{Listener, listen},
        focus::focus_element,
        focusability::is_focusable_ignoring_visibility,
        shadow_dom::{get_active_element, node_contains},
    };

    // The browser focuses the nearest focusable ancestor of the target.
    let mut target = target;
    while let Some(element) = &target
        && !is_focusable_ignoring_visibility(element)
    {
        target = element.parent_element();
    }
    let document = target
        .as_ref()
        .and_then(|target| target.owner_document())
        .or_else(|| leptos_use::use_document().as_ref().cloned())?;
    let active = get_active_element(&document)?;
    if target.as_ref() == Some(&active) {
        return None;
    }
    // Focus events inside a shadow root don't reach the window.
    let root: web_sys::EventTarget = match target.as_ref().map(|target| target.get_root_node()) {
        Some(root) if root.dyn_ref::<web_sys::ShadowRoot>().is_some() => root.unchecked_into(),
        _ => document.default_view()?.unchecked_into(),
    };

    let is_moving_to_target = {
        let target = target.clone();
        move |element: Option<web_sys::Element>| {
            target.as_ref().is_some_and(|target| {
                element.is_some_and(|element| {
                    &element == target || node_contains(target.as_ref(), element.as_ref())
                })
            })
        }
    };
    let is_blur_from_active = {
        let active = active.clone();
        move |element: Option<web_sys::Element>| {
            element.is_some_and(|element| {
                element == active || node_contains(active.as_ref(), element.as_ref())
            })
        }
    };
    let element_of = |e: &web_sys::Event| e.target().and_then(|t| t.dyn_into().ok());

    IGNORE_FOCUS_EVENT.with(|ignore| ignore.set(true));
    let is_refocusing = Rc::new(Cell::new(false));
    let listeners: Rc<RefCell<Option<Vec<Listener>>>> = Rc::new(RefCell::new(None));
    let frame: Rc<Cell<Option<leptos::prelude::AnimationFrameRequestHandle>>> =
        Rc::new(Cell::new(None));
    let cleanup = {
        let listeners = listeners.clone();
        let is_refocusing = is_refocusing.clone();
        let frame = frame.clone();
        move || {
            if let Some(frame) = frame.take() {
                frame.cancel();
            }
            listeners.borrow_mut().take();
            IGNORE_FOCUS_EVENT.with(|ignore| ignore.set(false));
            is_refocusing.set(false);
        }
    };
    // Listeners can't be dropped while one of them runs: clean up after the event.
    let cleanup_after_event = {
        let cleanup = cleanup.clone();
        move || {
            let cleanup = cleanup.clone();
            leptos::prelude::queue_microtask(cleanup);
        }
    };
    // Nothing had focus (the active element is `<body>`, which `focus()` ignores): blur the
    // element that got focus instead, so focus stays nowhere. Upstream refocuses `<body>`, which
    // leaves the focus on the target (e.g. a tree's expand button on a fresh page).
    let nothing_focused = document.body().is_some_and(|body| active == *body);
    let refocus = {
        let is_refocusing = is_refocusing.clone();
        let active = active.clone();
        let document = document.clone();
        move || {
            if !is_refocusing.get() {
                is_refocusing.set(true);
                if nothing_focused {
                    if let Some(focused) = get_active_element(&document)
                        .and_then(|focused| focused.dyn_into::<web_sys::HtmlElement>().ok())
                    {
                        let _ = focused.blur();
                    }
                } else {
                    focus_element(&active, true);
                }
                cleanup_after_event();
            }
        }
    };

    let on_blur = {
        let is_refocusing = is_refocusing.clone();
        let is_blur_from_active = is_blur_from_active.clone();
        move |e: web_sys::Event| {
            if is_blur_from_active(element_of(&e)) || is_refocusing.get() {
                e.stop_immediate_propagation();
            }
        }
    };
    let on_focus_out = {
        let is_refocusing = is_refocusing.clone();
        let has_target = target.is_some();
        let refocus = refocus.clone();
        move |e: web_sys::Event| {
            if is_blur_from_active(element_of(&e)) || is_refocusing.get() {
                e.stop_immediate_propagation();
                // Without a focusable ancestor, no focus event follows: refocus now.
                if !has_target {
                    refocus();
                }
            }
        }
    };
    let on_focus = {
        let is_refocusing = is_refocusing.clone();
        let is_moving_to_target = is_moving_to_target.clone();
        move |e: web_sys::Event| {
            if is_moving_to_target(element_of(&e)) || is_refocusing.get() {
                e.stop_immediate_propagation();
            }
        }
    };
    let on_focus_in = {
        let is_refocusing = is_refocusing.clone();
        move |e: web_sys::Event| {
            if is_moving_to_target(element_of(&e)) || is_refocusing.get() {
                e.stop_immediate_propagation();
                refocus();
            }
        }
    };
    *listeners.borrow_mut() = Some(vec![
        listen(&root, "blur", true, on_blur),
        listen(&root, "focusout", true, on_focus_out),
        listen(&root, "focusin", true, on_focus_in),
        listen(&root, "focus", true, on_focus),
    ]);
    let after_frame = cleanup.clone();
    frame.set(leptos::prelude::request_animation_frame_with_handle(after_frame).ok());
    Some(FocusPrevention {
        cleanup: Box::new(cleanup),
    })
}
