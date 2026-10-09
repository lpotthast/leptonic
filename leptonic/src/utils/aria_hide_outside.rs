// Upstream: react-aria/src/overlays/ariaHideOutside.ts @ 99e6102368
// Upstream: react-aria/test/overlays/ariaHideOutside.test.js @ 99e6102368
//! Hides all elements outside the given targets from assistive technology.
//!
//! When a modal or popover is open, content behind it should be hidden from
//! screen readers. This module hides all elements outside the target elements:
//! with `aria-hidden="true"` (the page stays usable, e.g. behind a combo box's
//! popover or during a drag), or with `inert` (also non-interactive, for modal
//! overlays), see [`HideMode`].
//!
//! Features:
//! - Reference counting supports nested overlays (e.g., a popover inside a modal)
//! - `MutationObserver` automatically hides dynamically added elements
//! - Observer stack manages nesting correctly
//! - Preserves live announcer regions and top-layer elements
//!
//! Based on react-aria's `ariaHideOutside` from
//! `react-aria/src/overlays/ariaHideOutside.ts`.
//!
//! ## Deviations from react-aria
//!
//! - `shouldUseInert: boolean` is the [`HideMode`] enum; `inert` is always supported (modern
//!   browsers only), so there is no `aria-hidden` fallback for it.
//! - Inert HTML elements also get `aria-hidden="true"`: Chromium otherwise drops text
//!   referenced by `aria-labelledby`/`aria-describedby` inside inert content. Both attributes
//!   are restored on cleanup, preserving the author's prior `aria-hidden` value.
//! - Omitted: watching shadow roots around the targets (react-aria's `shadowDOM` flag, off by
//!   default).
//! - Registering an overlay opened from inside (`keep_visible`, a nested `aria_hide_outside`)
//!   shows it again if the current observer hid it already: Leptos effects run after the
//!   observer's callback, react-aria's layout effects before it.

/// How [`aria_hide_outside`] hides elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HideMode {
    /// `aria-hidden="true"`: hidden from assistive technology, still usable with a pointer (a
    /// combo box's popover, a drag).
    #[default]
    AriaHidden,
    /// `inert`: hidden from assistive technology and not interactive (modal overlays).
    Inert,
}

impl HideMode {
    /// The attribute hiding `element` in this mode: `inert` exists on HTML elements only, others
    /// (SVG) get `aria-hidden` (as react-aria's `setHidden`).
    #[cfg(not(feature = "ssr"))]
    fn attribute(self, element: &web_sys::Element) -> &'static str {
        use wasm_bindgen::JsCast;
        match self {
            Self::Inert if element.is_instance_of::<web_sys::HtmlElement>() => "inert",
            Self::AriaHidden | Self::Inert => "aria-hidden",
        }
    }

    #[cfg(not(feature = "ssr"))]
    fn is_hidden(self, element: &web_sys::Element) -> bool {
        match self.attribute(element) {
            "inert" => element.has_attribute("inert"),
            _ => element.get_attribute("aria-hidden").as_deref() == Some("true"),
        }
    }

    #[cfg(not(feature = "ssr"))]
    fn hide(self, element: &web_sys::Element) {
        let attribute = self.attribute(element);
        let value = if attribute == "inert" { "" } else { "true" };
        if attribute == "inert" {
            INERT_ARIA_HIDDEN.with_borrow(|map| {
                let previous = element
                    .get_attribute("aria-hidden")
                    .map_or(JsValue::NULL, JsValue::from);
                map.set(element.unchecked_ref(), &previous);
            });
            let _ = element.set_attribute("aria-hidden", "true");
        }
        let _ = element.set_attribute(attribute, value);
    }

    /// Shows an element again. As react-aria, `aria-hidden` mode removes `inert` as well.
    #[cfg(not(feature = "ssr"))]
    fn show(self, element: &web_sys::Element) {
        let _ = element.remove_attribute(self.attribute(element));
        if self == Self::AriaHidden {
            let _ = element.remove_attribute("inert");
        }
        INERT_ARIA_HIDDEN.with_borrow(|map| {
            let key = element.unchecked_ref();
            if map.has(key) {
                if let Some(previous) = map.get(key).as_string() {
                    let _ = element.set_attribute("aria-hidden", &previous);
                } else {
                    let _ = element.remove_attribute("aria-hidden");
                }
                map.delete(key);
            }
        });
    }
}

/// Options for [`aria_hide_outside`].
#[derive(Debug, Default)]
pub struct AriaHideOutsideOptions {
    /// The root element to start hiding from. Defaults to `document.body`.
    pub root: Option<web_sys::Element>,
    /// How elements are hidden. Default: `aria-hidden`.
    pub mode: HideMode,
}

/// Hides all elements in the DOM outside the given targets from assistive
/// technology, as `options.mode` says.
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
use wasm_bindgen::JsCast;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::prelude::*;

#[cfg(not(feature = "ssr"))]
use crate::utils::dom_ext::node_contains;

/// A set of elements by identity: a JavaScript `Set` (as upstream's), so a membership check is
/// one call into JavaScript instead of one per element compared. Clones share the set.
#[cfg(not(feature = "ssr"))]
#[derive(Clone)]
struct ElementSet(js_sys::Set);

#[cfg(not(feature = "ssr"))]
impl ElementSet {
    fn new(elements: &[web_sys::Element]) -> Self {
        let set = Self(js_sys::Set::new(&JsValue::UNDEFINED));
        for element in elements {
            set.add(element);
        }
        set
    }

    fn has(&self, element: &web_sys::Element) -> bool {
        self.0.has(element.as_ref())
    }

    fn add(&self, element: &web_sys::Element) {
        self.0.add(element.as_ref());
    }

    fn delete(&self, element: &web_sys::Element) {
        self.0.delete(element.as_ref());
    }

    /// The elements, in insertion order.
    fn to_vec(&self) -> Vec<web_sys::Element> {
        js_sys::Array::from(self.0.as_ref())
            .iter()
            .map(JsCast::unchecked_into)
            .collect()
    }

    /// Whether one of the elements contains `node` (or is it).
    fn any_contains(&self, node: &web_sys::Node) -> bool {
        self.to_vec()
            .iter()
            .any(|element| node_contains(Some(element.as_ref()), Some(node)).unwrap_or(false))
    }
}

#[cfg(not(feature = "ssr"))]
thread_local! {
    /// Reference count per hidden element. Uses `WeakMap` so removed DOM
    /// elements can be garbage collected.
    static REF_COUNT_MAP: RefCell<js_sys::WeakMap> = RefCell::new(js_sys::WeakMap::new());

    /// Prior author values of `aria-hidden`, before inert mode adds the naming workaround.
    static INERT_ARIA_HIDDEN: RefCell<js_sys::WeakMap> = RefCell::new(js_sys::WeakMap::new());

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
    visible_nodes: ElementSet,
    hidden_nodes: ElementSet,
    mode: HideMode,
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

    /// Shows the nodes this call hid around `element` (e.g. the portal container of an overlay
    /// opened from inside the hiding one) and hides their other content again.
    ///
    /// Upstream registers such an overlay (`keepVisible`, or a nested `ariaHideOutside` that
    /// disconnects this observer) in a layout effect, before this observer's callback sees the
    /// overlay's insertion. Leptos effects run after it, when the overlay may already be hidden.
    fn reveal(&self, element: &web_sys::Element) {
        let containing: Vec<web_sys::Element> = self
            .hidden_nodes
            .to_vec()
            .into_iter()
            .filter(|hidden| {
                node_contains(Some(hidden.as_ref()), Some(element.as_ref())).unwrap_or(false)
            })
            .collect();
        if containing.is_empty() {
            return;
        }
        for node in &containing {
            self.hidden_nodes.delete(node);
            show_element(node, self.mode);
        }
        // The element stays visible to this walk even when it isn't one of this call's targets
        // (a nested hide's target).
        let mut visible = self.visible_nodes.to_vec();
        visible.push(element.clone());
        let visible = ElementSet::new(&visible);
        for node in &containing {
            walk_children(node, self.mode, &visible, &self.hidden_nodes);
        }
    }
}

/// Classification result for a single element during the DOM walk.
#[cfg(not(feature = "ssr"))]
enum WalkAction {
    /// Skip this node and its entire subtree.
    Reject,
    /// Skip this node but process its children.
    Skip,
    /// Hide this node, then visit its children: they are rejected below a node this call hid
    /// (hiding is recursive), except below a `role="row"` and below a node hidden by its author.
    Accept,
}

/// Classify an element: should it be rejected, skipped, or hidden?
#[cfg(not(feature = "ssr"))]
fn classify(
    element: &web_sys::Element,
    visible_nodes: &ElementSet,
    hidden_nodes: &ElementSet,
) -> WalkAction {
    // Already hidden or already visible — skip subtree.
    if hidden_nodes.has(element) || visible_nodes.has(element) {
        return WalkAction::Reject;
    }

    // Parent already hidden (hiding is recursive) — skip subtree.
    // Exception: elements with role="row" — VoiceOver on iOS has issues
    // hiding elements with role="row", so we hide cells individually.
    // https://bugs.webkit.org/show_bug.cgi?id=222623
    if let Some(parent) = element.parent_element()
        && hidden_nodes.has(&parent)
        && parent.get_attribute("role").as_deref() != Some("row")
    {
        return WalkAction::Reject;
    }

    // Node contains a visible target — don't hide it, but recurse into children.
    let contains_visible = visible_nodes
        .to_vec()
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
fn discover_special_elements(root: &web_sys::Element, visible_nodes: &ElementSet) {
    let Ok(special) = root.query_selector_all("[data-live-announcer], [data-leptonic-top-layer]")
    else {
        return;
    };
    for i in 0..special.length() {
        if let Some(el) = special
            .item(i)
            .and_then(|n| n.dyn_ref::<web_sys::Element>().cloned())
        {
            visible_nodes.add(&el);
        }
    }
}

/// Classify a single element, hide it and/or recurse into its children.
#[cfg(not(feature = "ssr"))]
fn walk(
    element: &web_sys::Element,
    mode: HideMode,
    visible_nodes: &ElementSet,
    hidden_nodes: &ElementSet,
) {
    match classify(element, visible_nodes, hidden_nodes) {
        WalkAction::Reject => {}
        WalkAction::Skip => walk_children(element, mode, visible_nodes, hidden_nodes),
        // As upstream's TreeWalker, which descends into accepted nodes.
        WalkAction::Accept => {
            hide_element(element, mode, hidden_nodes);
            walk_children(element, mode, visible_nodes, hidden_nodes);
        }
    }
}

/// Walk direct children of an element.
#[cfg(not(feature = "ssr"))]
fn walk_children(
    element: &web_sys::Element,
    mode: HideMode,
    visible_nodes: &ElementSet,
    hidden_nodes: &ElementSet,
) {
    let children = element.children();
    for i in 0..children.length() {
        if let Some(child) = children.item(i) {
            walk(&child, mode, visible_nodes, hidden_nodes);
        }
    }
}

/// Hide an element and update the reference count.
#[cfg(not(feature = "ssr"))]
fn hide_element(element: &web_sys::Element, mode: HideMode, hidden_nodes: &ElementSet) {
    REF_COUNT_MAP.with_borrow(|map| {
        let key: &js_sys::Object = element.unchecked_ref();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let count = map.get(key).as_f64().unwrap_or(0.0) as u32;

        // If already hidden and ref count is zero, this element was hidden
        // before us (by page author or another mechanism). Don't touch it.
        if count == 0 && mode.is_hidden(element) {
            return;
        }

        if count == 0 {
            mode.hide(element);
        }

        hidden_nodes.add(element);
        let _ = map.set(key, &JsValue::from_f64(f64::from(count + 1)));
    });
}

/// Show a hidden element again when its ref count reaches zero.
#[cfg(not(feature = "ssr"))]
fn show_element(element: &web_sys::Element, mode: HideMode) {
    REF_COUNT_MAP.with_borrow(|map| {
        let key: &js_sys::Object = element.unchecked_ref();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let count = map.get(key).as_f64().map(|n| n as u32);

        match count {
            None | Some(0) => {}
            Some(1) => {
                mode.show(element);
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
    mode: HideMode,
    visible_nodes: &ElementSet,
    hidden_nodes: &ElementSet,
) -> Option<(
    web_sys::MutationObserver,
    Closure<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>,
)> {
    let (visible_nodes, hidden_nodes) = (visible_nodes.clone(), hidden_nodes.clone());
    let callback: Closure<dyn FnMut(js_sys::Array, web_sys::MutationObserver)> = Closure::new(
        move |mutations: js_sys::Array, _observer: web_sys::MutationObserver| {
            handle_mutations(&mutations, mode, &visible_nodes, &hidden_nodes);
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
    mode: HideMode,
    visible_nodes: &ElementSet,
    hidden_nodes: &ElementSet,
) {
    for i in 0..mutations.length() {
        let Ok(record) = mutations.get(i).dyn_into::<web_sys::MutationRecord>() else {
            continue;
        };
        let Some(target) = record.target() else {
            continue;
        };
        // Skip disconnected targets, and targets inside a visible or hidden node.
        if !target.is_connected()
            || visible_nodes.any_contains(&target)
            || hidden_nodes.any_contains(&target)
        {
            continue;
        }

        let added = record.added_nodes();
        for j in 0..added.length() {
            let Some(node) = added.item(j) else { continue };
            let Some(el) = node.dyn_ref::<web_sys::Element>() else {
                continue;
            };

            if el.has_attribute("data-live-announcer")
                || el.has_attribute("data-leptonic-top-layer")
            {
                visible_nodes.add(el);
            } else {
                discover_special_elements(el, visible_nodes);
                walk(el, mode, visible_nodes, hidden_nodes);
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

    let AriaHideOutsideOptions { root, mode } = options;
    let root = root.or_else(|| {
        leptos_use::use_document()
            .as_ref()
            .and_then(web_sys::Document::body)
            .map(JsCast::unchecked_into::<web_sys::Element>)
    });

    let Some(root) = root else {
        return Box::new(|| {});
    };

    let visible_nodes = ElementSet::new(targets);
    let hidden_nodes = ElementSet::new(&[]);

    // Discover and preserve live announcer / top-layer elements.
    discover_special_elements(&root, &visible_nodes);

    // Disconnect the previous observer (if nested), keeping the targets visible to it.
    OBSERVER_STACK.with_borrow(|stack| {
        if let Some(top) = stack.last() {
            top.disconnect();
            for target in targets {
                top.reveal(target);
            }
        }
    });

    // Walk the DOM and hide elements outside targets.
    // The root itself is classified too (hidden when no target is inside it).
    walk(&root, mode, &visible_nodes, &hidden_nodes);

    // Set up MutationObserver for dynamically added elements.
    let Some((observer, callback)) =
        create_mutation_observer(&root, mode, &visible_nodes, &hidden_nodes)
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
            visible_nodes,
            hidden_nodes: hidden_nodes.clone(),
            mode,
            observer,
            root,
            _callback: callback,
        });
    });

    // Cleanup closure.
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

        for node in hidden_nodes.to_vec() {
            show_element(&node, mode);
        }
    })
}

#[cfg(not(feature = "ssr"))]
pub fn keep_visible(element: &web_sys::Element) -> Option<Box<dyn FnOnce()>> {
    OBSERVER_STACK.with_borrow(|stack| {
        let wrapper = stack.last()?;
        if wrapper.visible_nodes.has(element) {
            return None;
        }
        wrapper.visible_nodes.add(element);
        wrapper.reveal(element);
        let nodes = wrapper.visible_nodes.clone();
        let el = element.clone();
        Some(Box::new(move || nodes.delete(&el)) as Box<dyn FnOnce()>)
    })
}
