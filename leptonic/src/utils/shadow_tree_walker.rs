// Upstream: react-aria/src/utils/shadowdom/ShadowTreeWalker.ts @ 99e6102368
//! A tree walker that descends into shadow roots.
//!
//! The native `TreeWalker` API does not cross shadow DOM boundaries. `ShadowTreeWalker` keeps a
//! stack of native walkers, one per shadow tree, so that iteration enters and leaves shadow
//! roots seamlessly.
//!
//! As in react-aria, every native walker gets the filter as its `acceptNode` callback: rejected
//! nodes are *skipped* (their descendants are still visited), and the browser keeps
//! `first_child`/`last_child` inside the current node's subtree.

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use wasm_bindgen::{JsCast, closure::Closure};

const FILTER_ACCEPT: u32 = 1;
const FILTER_SKIP: u32 = 3;

/// A tree walker that crosses shadow DOM boundaries.
pub struct ShadowTreeWalker {
    inner: Rc<Inner>,
}

struct Inner {
    doc: web_sys::Document,
    root: web_sys::Node,
    what_to_show: u32,
    /// `true`: visit the node; `false`: skip it (but visit its descendants).
    filter: Option<Box<dyn Fn(&web_sys::Node) -> bool>>,
    /// The native walkers, innermost (current shadow tree) first.
    walkers: RefCell<Vec<web_sys::TreeWalker>>,
    current: RefCell<web_sys::Node>,
    /// Walkers positioned by `set_current_node` (react-aria's `_currentSetFor`).
    current_set_for: RefCell<Vec<web_sys::TreeWalker>>,
    node_filter: RefCell<Option<web_sys::NodeFilter>>,
    _accept_node: RefCell<Option<Closure<dyn Fn(web_sys::Node) -> u32>>>,
}

impl Inner {
    fn accept_node(&self, node: &web_sys::Node) -> u32 {
        if let Some(element) = node.dyn_ref::<web_sys::Element>() {
            if let Some(shadow_root) = element.shadow_root() {
                // Entering a shadow host: continue inside its shadow tree.
                if let Some(walker) = self.create_walker(shadow_root.unchecked_ref()) {
                    self.walkers.borrow_mut().insert(0, walker);
                }
                return FILTER_ACCEPT;
            }
            return match &self.filter {
                Some(filter) if !filter(node) => FILTER_SKIP,
                _ => FILTER_ACCEPT,
            };
        }
        FILTER_SKIP
    }

    fn create_walker(&self, root: &web_sys::Node) -> Option<web_sys::TreeWalker> {
        let filter = self.node_filter.borrow();
        self.doc
            .create_tree_walker_with_what_to_show_and_filter(
                root,
                self.what_to_show,
                filter.as_ref(),
            )
            .ok()
    }

    /// The innermost walker. Cloned, so that no borrow is held while the browser calls the
    /// filter (which may push walkers).
    fn innermost(&self) -> Option<web_sys::TreeWalker> {
        self.walkers.borrow().first().cloned()
    }

    fn pop_innermost(&self) -> bool {
        let mut walkers = self.walkers.borrow_mut();
        if walkers.len() > 1 {
            walkers.remove(0);
            true
        } else {
            false
        }
    }

    fn passes_filter(&self, node: &web_sys::Node) -> bool {
        self.filter.as_ref().is_none_or(|filter| filter(node))
    }
}

impl ShadowTreeWalker {
    /// The node the walker is at.
    pub fn current_node(&self) -> web_sys::Node {
        self.inner.current.borrow().clone()
    }

    /// Positions the walker at `node` (which must be inside the root), rebuilding the walker
    /// stack for the shadow trees `node` is in.
    pub fn set_current_node(&self, node: &web_sys::Node) {
        let inner = &self.inner;
        if !inner.root.contains(Some(node)) && !shadow_contains(&inner.root, node) {
            crate::utils::dev_warn!("ShadowTreeWalker: the node is not inside the root");
            return;
        }
        *inner.current.borrow_mut() = node.clone();

        let mut walkers = Vec::new();
        let mut current_set_for = Vec::new();
        let mut cur: Option<web_sys::Node> = Some(node.clone());
        let mut walker_current = node.clone();
        while let Some(n) = cur.filter(|n| *n != inner.root) {
            if n.node_type() == web_sys::Node::DOCUMENT_FRAGMENT_NODE
                && let Some(shadow_root) = n.dyn_ref::<web_sys::ShadowRoot>()
            {
                if let Some(walker) = inner.create_walker(&n) {
                    walker.set_current_node(&walker_current);
                    current_set_for.push(walker.clone());
                    walkers.push(walker);
                }
                let host: web_sys::Node = shadow_root.host().into();
                walker_current = host.clone();
                cur = Some(host);
            } else {
                cur = n.parent_node();
            }
        }
        if let Some(walker) = inner.create_walker(&inner.root) {
            walker.set_current_node(&walker_current);
            current_set_for.push(walker.clone());
            walkers.push(walker);
        }
        *inner.walkers.borrow_mut() = walkers;
        *inner.current_set_for.borrow_mut() = current_set_for;
    }

    /// Whether `node` passes the filter.
    pub fn matches_filter(&self, node: &web_sys::Node) -> bool {
        self.inner.passes_filter(node)
    }

    /// Moves to the first (filtered) descendant of the current node.
    pub fn first_child(&mut self) -> Option<web_sys::Node> {
        let current = self.current_node();
        let next = self.next_node();
        match next {
            Some(next) if current.contains(Some(&next)) || shadow_contains(&current, &next) => {
                self.set_current_node(&next);
                Some(next)
            }
            _ => {
                self.set_current_node(&current);
                None
            }
        }
    }

    /// Moves to the last (filtered) child of the current node.
    pub fn last_child(&mut self) -> Option<web_sys::Node> {
        let walker = self.inner.innermost()?;
        let node = walker.last_child().ok().flatten()?;
        self.set_current_node(&node);
        Some(node)
    }

    /// Moves to the next (filtered) node in tree order, entering and leaving shadow roots.
    pub fn next_node(&mut self) -> Option<web_sys::Node> {
        let inner = Rc::clone(&self.inner);
        let walker = inner.innermost()?;
        if let Some(next) = walker.next_node().ok().flatten() {
            let is_host = next
                .dyn_ref::<web_sys::Element>()
                .is_some_and(|el| el.shadow_root().is_some());
            if is_host && !inner.passes_filter(&next) {
                // The filter pushed the host's shadow walker: continue inside it.
                let node = self.next_node();
                if let Some(node) = &node {
                    *inner.current.borrow_mut() = node.clone();
                }
                return node;
            }
            *inner.current.borrow_mut() = next.clone();
            return Some(next);
        }
        if inner.pop_innermost() {
            let node = self.next_node();
            if let Some(node) = &node {
                *inner.current.borrow_mut() = node.clone();
            }
            return node;
        }
        None
    }

    /// Moves to the previous (filtered) node in tree order, entering and leaving shadow roots.
    pub fn previous_node(&mut self) -> Option<web_sys::Node> {
        let inner = Rc::clone(&self.inner);
        let walker = inner.innermost()?;

        if walker.current_node() == walker.root() {
            let was_set = {
                let mut set_for = inner.current_set_for.borrow_mut();
                let index = set_for.iter().position(|w| *w == walker);
                index.map(|i| set_for.remove(i)).is_some()
            };
            if was_set && inner.pop_innermost() {
                let node = self.previous_node();
                if let Some(node) = &node {
                    *inner.current.borrow_mut() = node.clone();
                }
                return node;
            }
            return None;
        }

        if let Some(previous) = walker.previous_node().ok().flatten() {
            let is_host = previous
                .dyn_ref::<web_sys::Element>()
                .is_some_and(|el| el.shadow_root().is_some());
            if is_host && !inner.passes_filter(&previous) {
                let node = self.last_child();
                if let Some(node) = &node {
                    *inner.current.borrow_mut() = node.clone();
                }
                return node;
            }
            *inner.current.borrow_mut() = previous.clone();
            return Some(previous);
        }
        if inner.pop_innermost() {
            let node = self.previous_node();
            if let Some(node) = &node {
                *inner.current.borrow_mut() = node.clone();
            }
            return node;
        }
        None
    }
}

/// Whether `node` is inside `ancestor`, also across shadow boundaries.
fn shadow_contains(ancestor: &web_sys::Node, node: &web_sys::Node) -> bool {
    let mut current = Some(node.clone());
    while let Some(n) = current {
        if n == *ancestor {
            return true;
        }
        current = match n.dyn_ref::<web_sys::ShadowRoot>() {
            Some(shadow_root) => Some(shadow_root.host().into()),
            None => n.parent_node(),
        };
    }
    false
}

/// Create a `ShadowTreeWalker` rooted at the given node.
///
/// `what_to_show` is the `NodeFilter.SHOW_*` bitmask (e.g., `0x1` for elements). `filter` decides
/// which nodes are visited; nodes it rejects are skipped, but their descendants are not.
pub fn create_shadow_tree_walker(
    root: &web_sys::Node,
    what_to_show: u32,
    filter: Option<Box<dyn Fn(&web_sys::Node) -> bool>>,
) -> Option<ShadowTreeWalker> {
    let doc = root.owner_document()?;
    let inner = Rc::new(Inner {
        doc,
        root: root.clone(),
        what_to_show,
        filter,
        walkers: RefCell::new(Vec::new()),
        current: RefCell::new(root.clone()),
        current_set_for: RefCell::new(Vec::new()),
        node_filter: RefCell::new(None),
        _accept_node: RefCell::new(None),
    });

    let weak: Weak<Inner> = Rc::downgrade(&inner);
    let accept_node = Closure::<dyn Fn(web_sys::Node) -> u32>::new(move |node: web_sys::Node| {
        weak.upgrade()
            .map_or(FILTER_SKIP, |inner| inner.accept_node(&node))
    });
    let node_filter = web_sys::NodeFilter::new();
    node_filter.set_accept_node(accept_node.as_ref().unchecked_ref());
    *inner.node_filter.borrow_mut() = Some(node_filter);
    *inner._accept_node.borrow_mut() = Some(accept_node);

    let mut walkers = Vec::new();
    if let Some(shadow_root) = root
        .dyn_ref::<web_sys::Element>()
        .and_then(web_sys::Element::shadow_root)
        && let Some(walker) = inner.create_walker(shadow_root.unchecked_ref())
    {
        walkers.push(walker);
    }
    walkers.push(inner.create_walker(root)?);
    *inner.walkers.borrow_mut() = walkers;

    Some(ShadowTreeWalker { inner })
}
