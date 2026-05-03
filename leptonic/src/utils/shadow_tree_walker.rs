//! A tree walker that descends into shadow roots.
//!
//! The native `TreeWalker` API does not cross shadow DOM boundaries.
//! `ShadowTreeWalker` wraps the native walker and maintains a stack of walkers —
//! one per shadow boundary — so that iteration seamlessly enters and exits
//! shadow roots.
//!
//! Based on react-aria's `@react-aria/utils/src/shadowdom/ShadowTreeWalker.ts`.

use wasm_bindgen::JsCast;

/// A tree walker that crosses shadow DOM boundaries.
///
/// Maintains a stack of native `TreeWalker`s. When iteration encounters an element
/// with a `shadowRoot`, a new walker is pushed for that shadow tree. When a walker
/// is exhausted, the stack pops back to the parent.
pub struct ShadowTreeWalker {
    walkers: Vec<web_sys::TreeWalker>,
    what_to_show: u32,
    filter: Option<Box<dyn Fn(&web_sys::Node) -> bool>>,
}

impl ShadowTreeWalker {
    /// Returns the current node of the innermost walker.
    pub fn current_node(&self) -> web_sys::Node {
        self.walkers
            .last()
            .expect("walker stack is never empty")
            .current_node()
    }

    /// Sets the current node on the innermost walker.
    pub fn set_current_node(&self, node: &web_sys::Node) {
        if let Some(walker) = self.walkers.last() {
            walker.set_current_node(node);
        }
    }

    /// Move to the first child of the current node, entering shadow roots.
    #[allow(dead_code)]
    pub fn first_child(&mut self) -> Option<web_sys::Node> {
        let current = self.current_node();

        // If the current node has a shadow root, descend into it.
        if let Some(el) = current.dyn_ref::<web_sys::Element>() {
            if let Some(shadow) = el.shadow_root() {
                let doc = current.owner_document()?;
                let walker = doc
                    .create_tree_walker_with_what_to_show(shadow.as_ref(), self.what_to_show)
                    .ok()?;
                self.walkers.push(walker);
                return self.first_child();
            }
        }

        let walker = self.walkers.last()?;
        let node = walker.first_child().ok()??;
        self.check_shadow_and_filter(node)
    }

    /// Move to the last child of the current node, entering shadow roots.
    #[allow(dead_code)]
    pub fn last_child(&mut self) -> Option<web_sys::Node> {
        let current = self.current_node();

        // If the current node has a shadow root, descend into it.
        if let Some(el) = current.dyn_ref::<web_sys::Element>() {
            if let Some(shadow) = el.shadow_root() {
                let doc = current.owner_document()?;
                let walker = doc
                    .create_tree_walker_with_what_to_show(shadow.as_ref(), self.what_to_show)
                    .ok()?;
                self.walkers.push(walker);
                return self.last_child();
            }
        }

        let walker = self.walkers.last()?;
        let node = walker.last_child().ok()??;
        self.check_shadow_and_filter_backwards(node)
    }

    /// Move to the next node in tree order, entering and exiting shadow roots.
    pub fn next_node(&mut self) -> Option<web_sys::Node> {
        let current = self.current_node();

        // If the current element has a shadow root, push a walker for it.
        if let Some(el) = current.dyn_ref::<web_sys::Element>() {
            if let Some(shadow) = el.shadow_root() {
                if let Some(doc) = current.owner_document() {
                    if let Ok(walker) =
                        doc.create_tree_walker_with_what_to_show(shadow.as_ref(), self.what_to_show)
                    {
                        self.walkers.push(walker);
                        return self.next_node();
                    }
                }
            }
        }

        // Try to advance within the current walker.
        if let Some(node) = self
            .walkers
            .last()
            .and_then(|w| w.next_node().ok().flatten())
        {
            return self.check_shadow_and_filter(node);
        }

        // Current walker exhausted — pop and try the parent walker.
        if self.walkers.len() > 1 {
            self.walkers.pop();
            return self.next_node();
        }

        None
    }

    /// Move to the previous node in tree order, entering and exiting shadow roots.
    pub fn previous_node(&mut self) -> Option<web_sys::Node> {
        let current = self.current_node();

        // If the current element has a shadow root, push a walker for it
        // and seek to the last descendant.
        if let Some(el) = current.dyn_ref::<web_sys::Element>() {
            if let Some(shadow) = el.shadow_root() {
                if let Some(doc) = current.owner_document() {
                    if let Ok(walker) =
                        doc.create_tree_walker_with_what_to_show(shadow.as_ref(), self.what_to_show)
                    {
                        self.walkers.push(walker);
                        // Seek to the deepest last node.
                        let mut node: web_sys::Node = shadow.into();
                        while let Some(last) = node.last_child() {
                            node = last;
                        }
                        // Set current node on the walker now in the stack.
                        if let Some(w) = self.walkers.last() {
                            w.set_current_node(&node);
                        }
                        return self.check_shadow_and_filter_backwards(node);
                    }
                }
            }
        }

        // Try to go backwards within the current walker.
        if let Some(node) = self
            .walkers
            .last()
            .and_then(|w| w.previous_node().ok().flatten())
        {
            return self.check_shadow_and_filter_backwards(node);
        }

        // Current walker exhausted — pop and try the parent walker.
        if self.walkers.len() > 1 {
            self.walkers.pop();
            return self.previous_node();
        }

        None
    }

    /// Check whether the given node passes the filter (without advancing the walker).
    pub fn matches_filter(&self, node: &web_sys::Node) -> bool {
        match &self.filter {
            Some(f) => f(node),
            None => true,
        }
    }

    /// If the node has a shadow root, descend into it and get the first descendant.
    /// Also applies the filter. If the filter rejects, advance to next.
    fn check_shadow_and_filter(&mut self, node: web_sys::Node) -> Option<web_sys::Node> {
        // Check if this element has a shadow root we should descend into.
        if let Some(el) = node.dyn_ref::<web_sys::Element>() {
            if el.shadow_root().is_some() {
                // Set current to this element, then next_node will descend.
                if let Some(walker) = self.walkers.last() {
                    walker.set_current_node(&node);
                }
                return self.next_node();
            }
        }

        // Apply filter.
        if let Some(ref filter) = self.filter {
            if !filter(&node) {
                // Advance past rejected node.
                if let Some(walker) = self.walkers.last() {
                    walker.set_current_node(&node);
                }
                return self.next_node();
            }
        }

        Some(node)
    }

    /// Backwards variant of `check_shadow_and_filter`.
    fn check_shadow_and_filter_backwards(&mut self, node: web_sys::Node) -> Option<web_sys::Node> {
        if let Some(el) = node.dyn_ref::<web_sys::Element>() {
            if el.shadow_root().is_some() {
                if let Some(walker) = self.walkers.last() {
                    walker.set_current_node(&node);
                }
                return self.previous_node();
            }
        }

        if let Some(ref filter) = self.filter {
            if !filter(&node) {
                if let Some(walker) = self.walkers.last() {
                    walker.set_current_node(&node);
                }
                return self.previous_node();
            }
        }

        Some(node)
    }
}

/// Create a `ShadowTreeWalker` rooted at the given node.
///
/// `what_to_show` is the `NodeFilter.SHOW_*` bitmask (e.g., `0x1` for elements).
/// `filter` is an optional predicate applied to each visited node.
pub fn create_shadow_tree_walker(
    root: &web_sys::Node,
    what_to_show: u32,
    filter: Option<Box<dyn Fn(&web_sys::Node) -> bool>>,
) -> Option<ShadowTreeWalker> {
    let doc = root.owner_document()?;
    let walker = doc
        .create_tree_walker_with_what_to_show(root, what_to_show)
        .ok()?;

    Some(ShadowTreeWalker {
        walkers: vec![walker],
        what_to_show,
        filter,
    })
}
