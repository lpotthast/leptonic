//! Hides all elements outside the given targets from assistive technology.
//!
//! When a modal or popover is open, content behind it should be hidden from
//! screen readers. This module sets the `inert` attribute on all sibling
//! elements outside the target elements, making them non-interactive and
//! invisible to assistive technology.
//!
//! Features:
//! - Reference counting supports nested overlays (e.g., a popover inside a modal)
//! - `MutationObserver` automatically hides dynamically added elements
//! - Observer stack manages nesting correctly
//! - Preserves live announcer regions and top-layer elements
//!
//! Based on react-aria's `ariaHideOutside` from
//! `@react-aria/overlays/src/ariaHideOutside.ts`.
//!
//! ## Deviation from react-aria
//!
//! Uses the `inert` attribute exclusively instead of `aria-hidden="true"`.
//! Leptonic targets modern browsers where `inert` is universally supported
//! and strictly superior: it hides from assistive technology AND prevents
//! pointer/keyboard interaction.

/// Options for [`aria_hide_outside`].
#[derive(Default)]
pub struct AriaHideOutsideOptions {
    /// The root element to start hiding from. Defaults to `document.body`.
    pub root: Option<web_sys::Element>,
}

/// Hides all elements in the DOM outside the given targets from assistive
/// technology using the `inert` attribute.
///
/// Returns a cleanup function that restores all hidden elements. The cleanup
/// function must be called exactly once when the overlay closes.
///
/// In addition to the initial walk, a `MutationObserver` watches for new
/// elements and automatically hides them.
///
/// Supports nesting: if called while a previous hide is active, the new call
/// takes over observation. When cleaned up, the previous observer resumes.
#[cfg(feature = "ssr")]
pub fn aria_hide_outside(
    _targets: &[web_sys::Element],
    _options: AriaHideOutsideOptions,
) -> Box<dyn FnOnce()> {
    Box::new(|| {})
}

/// Marks an element as visible within the current hiding context.
///
/// Returns a cleanup function to undo, or `None` if there is no active
/// hiding context or the element is already visible.
#[cfg(feature = "ssr")]
pub fn keep_visible(_element: &web_sys::Element) -> Option<Box<dyn FnOnce()>> {
    None
}

#[cfg(not(feature = "ssr"))]
use std::cell::{Cell, RefCell};
#[cfg(not(feature = "ssr"))]
use std::rc::Rc;

#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::prelude::*;

#[cfg(not(feature = "ssr"))]
use crate::utils::dom_ext::node_contains;

#[cfg(not(feature = "ssr"))]
thread_local! {
    /// Reference count per hidden element. Uses `WeakMap` so removed DOM
    /// elements can be garbage collected.
    static REF_COUNT_MAP: RefCell<js_sys::WeakMap> = RefCell::new(js_sys::WeakMap::new());

    /// Stack of active observers. The topmost observer is the one currently
    /// watching for mutations. When an overlay closes, its observer is removed
    /// and the previous one resumes.
    static OBSERVER_STACK: RefCell<Vec<ObserverWrapper>> = const { RefCell::new(Vec::new()) };

    /// Monotonically increasing ID for observer wrappers.
    static NEXT_WRAPPER_ID: Cell<u64> = const { Cell::new(0) };
}

#[cfg(not(feature = "ssr"))]
struct ObserverWrapper {
    id: u64,
    visible_nodes: Rc<RefCell<Vec<web_sys::Element>>>,
    observer: web_sys::MutationObserver,
    root: web_sys::Element,
    /// Prevent the closure from being dropped while the observer is alive.
    _callback: Closure<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>,
}

#[cfg(not(feature = "ssr"))]
impl ObserverWrapper {
    fn observe(&self) {
        let options = web_sys::MutationObserverInit::new();
        options.set_child_list(true);
        options.set_subtree(true);
        let _ = self.observer.observe_with_options(&self.root, &options);
    }

    fn disconnect(&self) {
        self.observer.disconnect();
    }
}

/// Classification result for a single element during the DOM walk.
#[cfg(not(feature = "ssr"))]
enum WalkAction {
    /// Skip this node and its entire subtree.
    Reject,
    /// Skip this node but process its children.
    Skip,
    /// Hide this node (and implicitly its subtree via `inert`).
    Accept,
}

/// Classify an element: should it be rejected, skipped, or hidden?
#[cfg(not(feature = "ssr"))]
fn classify(
    element: &web_sys::Element,
    visible_nodes: &[web_sys::Element],
    hidden_nodes: &[web_sys::Element],
) -> WalkAction {
    // Already hidden or already visible — skip subtree.
    if hidden_nodes.iter().any(|n| n == element) || visible_nodes.iter().any(|n| n == element) {
        return WalkAction::Reject;
    }

    // Parent already hidden (inert is recursive) — skip subtree.
    // Exception: elements with role="row" — VoiceOver on iOS has issues
    // hiding elements with role="row", so we hide cells individually.
    // https://bugs.webkit.org/show_bug.cgi?id=222623
    if let Some(parent) = element.parent_element() {
        if hidden_nodes.iter().any(|n| n == &parent)
            && parent.get_attribute("role").as_deref() != Some("row")
        {
            return WalkAction::Reject;
        }
    }

    // Node contains a visible target — don't hide it, but recurse into children.
    let contains_visible = visible_nodes
        .iter()
        .any(|v| node_contains(Some(element.as_ref()), Some(v.as_ref())).unwrap_or(false));

    if contains_visible {
        WalkAction::Skip
    } else {
        WalkAction::Accept
    }
}

/// Discover live announcer and top-layer elements in the subtree and add
/// them to the visible set.
#[cfg(not(feature = "ssr"))]
fn discover_special_elements(root: &web_sys::Element, visible_nodes: &mut Vec<web_sys::Element>) {
    let Ok(special) = root.query_selector_all("[data-live-announcer], [data-leptonic-top-layer]")
    else {
        return;
    };
    for i in 0..special.length() {
        if let Some(el) = special
            .item(i)
            .and_then(|n| n.dyn_ref::<web_sys::Element>().cloned())
        {
            if !visible_nodes.iter().any(|v| v == &el) {
                visible_nodes.push(el);
            }
        }
    }
}

/// Classify a single element then hide it or recurse into its children.
#[cfg(not(feature = "ssr"))]
fn walk(
    element: &web_sys::Element,
    visible_nodes: &Rc<RefCell<Vec<web_sys::Element>>>,
    hidden_nodes: &Rc<RefCell<Vec<web_sys::Element>>>,
) {
    let action = {
        let vis = visible_nodes.borrow();
        let hid = hidden_nodes.borrow();
        classify(element, &vis, &hid)
    };

    match action {
        WalkAction::Reject => {}
        WalkAction::Skip => walk_children(element, visible_nodes, hidden_nodes),
        WalkAction::Accept => hide_element(element, hidden_nodes),
    }
}

/// Walk direct children of an element.
#[cfg(not(feature = "ssr"))]
fn walk_children(
    element: &web_sys::Element,
    visible_nodes: &Rc<RefCell<Vec<web_sys::Element>>>,
    hidden_nodes: &Rc<RefCell<Vec<web_sys::Element>>>,
) {
    let children = element.children();
    for i in 0..children.length() {
        if let Some(child) = children.item(i) {
            walk(&child, visible_nodes, hidden_nodes);
        }
    }
}

/// Hide an element by setting `inert` and updating the reference count.
#[cfg(not(feature = "ssr"))]
fn hide_element(element: &web_sys::Element, hidden_nodes: &Rc<RefCell<Vec<web_sys::Element>>>) {
    REF_COUNT_MAP.with_borrow(|map| {
        let key: &js_sys::Object = element.unchecked_ref();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let count = map.get(key).as_f64().unwrap_or(0.0) as u32;

        // If already inert and ref count is zero, this element was hidden
        // before us (by page author or another mechanism). Don't touch it.
        if count == 0 && element.has_attribute("inert") {
            return;
        }

        if count == 0 {
            let _ = element.set_attribute("inert", "");
        }

        hidden_nodes.borrow_mut().push(element.clone());
        let _ = map.set(key, &JsValue::from_f64(f64::from(count + 1)));
    });
}

/// Restore a hidden element by removing `inert` when its ref count reaches zero.
#[cfg(not(feature = "ssr"))]
fn show_element(element: &web_sys::Element) {
    REF_COUNT_MAP.with_borrow(|map| {
        let key: &js_sys::Object = element.unchecked_ref();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let count = map.get(key).as_f64().map(|n| n as u32);

        match count {
            None | Some(0) => {}
            Some(1) => {
                let _ = element.remove_attribute("inert");
                let _ = map.delete(key);
            }
            Some(n) => {
                let _ = map.set(key, &JsValue::from_f64(f64::from(n - 1)));
            }
        }
    });
}

/// Create a `MutationObserver` that auto-hides newly added DOM elements
/// outside the visible set.
#[cfg(not(feature = "ssr"))]
fn create_mutation_observer(
    root: &web_sys::Element,
    visible_nodes: &Rc<RefCell<Vec<web_sys::Element>>>,
    hidden_nodes: &Rc<RefCell<Vec<web_sys::Element>>>,
) -> Option<(
    web_sys::MutationObserver,
    Closure<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>,
)> {
    let vis_for_cb = Rc::clone(visible_nodes);
    let hid_for_cb = Rc::clone(hidden_nodes);

    let callback: Closure<dyn FnMut(js_sys::Array, web_sys::MutationObserver)> = Closure::new(
        move |mutations: js_sys::Array, _observer: web_sys::MutationObserver| {
            handle_mutations(&mutations, &vis_for_cb, &hid_for_cb);
        },
    );

    let observer = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref()).ok()?;

    let options = web_sys::MutationObserverInit::new();
    options.set_child_list(true);
    options.set_subtree(true);
    let _ = observer.observe_with_options(root, &options);

    Some((observer, callback))
}

/// Process `MutationObserver` records — hide newly added elements that are
/// outside the visible/hidden sets.
#[cfg(not(feature = "ssr"))]
fn handle_mutations(
    mutations: &js_sys::Array,
    visible_nodes: &Rc<RefCell<Vec<web_sys::Element>>>,
    hidden_nodes: &Rc<RefCell<Vec<web_sys::Element>>>,
) {
    for i in 0..mutations.length() {
        let record = mutations.get(i);

        let target: Option<web_sys::Node> = js_sys::Reflect::get(&record, &"target".into())
            .ok()
            .and_then(|t| t.dyn_into().ok());
        let Some(target) = target else { continue };

        // Skip disconnected targets.
        let is_connected = js_sys::Reflect::get(&target, &"isConnected".into())
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if !is_connected {
            continue;
        }

        // Skip if target is inside a visible or hidden node.
        {
            let vis = visible_nodes.borrow();
            let hid = hidden_nodes.borrow();
            let inside = vis
                .iter()
                .chain(hid.iter())
                .any(|node| node_contains(Some(node.as_ref()), Some(&target)).unwrap_or(false));
            if inside {
                continue;
            }
        }

        // Process added nodes.
        let added: Option<web_sys::NodeList> = js_sys::Reflect::get(&record, &"addedNodes".into())
            .ok()
            .and_then(|v| v.dyn_into().ok());
        let Some(added) = added else { continue };

        for j in 0..added.length() {
            let Some(node) = added.item(j) else { continue };
            let Some(el) = node.dyn_ref::<web_sys::Element>() else {
                continue;
            };

            if el.has_attribute("data-live-announcer")
                || el.has_attribute("data-leptonic-top-layer")
            {
                visible_nodes.borrow_mut().push(el.clone());
            } else {
                discover_special_elements(el, &mut visible_nodes.borrow_mut());
                walk(el, visible_nodes, hidden_nodes);
            }
        }
    }
}

#[cfg(not(feature = "ssr"))]
pub fn aria_hide_outside(
    targets: &[web_sys::Element],
    options: AriaHideOutsideOptions,
) -> Box<dyn FnOnce()> {
    if targets.is_empty() {
        return Box::new(|| {});
    }

    let root = options.root.or_else(|| {
        web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
            .map(JsCast::unchecked_into::<web_sys::Element>)
    });

    let Some(root) = root else {
        return Box::new(|| {});
    };

    let visible_nodes: Rc<RefCell<Vec<web_sys::Element>>> = Rc::new(RefCell::new(targets.to_vec()));
    let hidden_nodes: Rc<RefCell<Vec<web_sys::Element>>> = Rc::new(RefCell::new(Vec::new()));

    // Discover and preserve live announcer / top-layer elements.
    discover_special_elements(&root, &mut visible_nodes.borrow_mut());

    // Disconnect the previous observer (if nested).
    OBSERVER_STACK.with_borrow(|stack| {
        if let Some(top) = stack.last() {
            top.disconnect();
        }
    });

    // Walk the DOM and hide elements outside targets.
    walk_children(&root, &visible_nodes, &hidden_nodes);

    // Set up MutationObserver for dynamically added elements.
    let Some((observer, callback)) = create_mutation_observer(&root, &visible_nodes, &hidden_nodes)
    else {
        return Box::new(|| {});
    };

    let wrapper_id = NEXT_WRAPPER_ID.with(|id| {
        let current = id.get();
        id.set(current.wrapping_add(1));
        current
    });

    OBSERVER_STACK.with_borrow_mut(|stack| {
        stack.push(ObserverWrapper {
            id: wrapper_id,
            visible_nodes: Rc::clone(&visible_nodes),
            observer,
            root,
            _callback: callback,
        });
    });

    // Cleanup closure.
    let hidden_for_cleanup = Rc::clone(&hidden_nodes);
    Box::new(move || {
        OBSERVER_STACK.with_borrow_mut(|stack| {
            if stack.last().is_some_and(|w| w.id == wrapper_id) {
                if let Some(removed) = stack.pop() {
                    removed.disconnect();
                }
                if let Some(prev) = stack.last() {
                    prev.observe();
                }
            } else if let Some(idx) = stack.iter().position(|w| w.id == wrapper_id) {
                let removed = stack.remove(idx);
                removed.disconnect();
            }
        });

        for node in hidden_for_cleanup.borrow().iter() {
            show_element(node);
        }
    })
}

#[cfg(not(feature = "ssr"))]
pub fn keep_visible(element: &web_sys::Element) -> Option<Box<dyn FnOnce()>> {
    OBSERVER_STACK.with_borrow(|stack| {
        let wrapper = stack.last()?;
        let vis = wrapper.visible_nodes.borrow();
        if vis.iter().any(|n| n == element) {
            return None;
        }
        drop(vis);
        wrapper.visible_nodes.borrow_mut().push(element.clone());
        let nodes = Rc::clone(&wrapper.visible_nodes);
        let el = element.clone();
        Some(Box::new(move || {
            nodes.borrow_mut().retain(|n| n != &el);
        }) as Box<dyn FnOnce()>)
    })
}
