//! Thread-local tree tracking parent-child relationships between `FocusScope`s.
//!
//! Each `FocusScope` registers itself in this tree on mount and unregisters on
//! cleanup. The tree enables nested-scope queries: "is this element inside this
//! scope or any of its descendant scopes?" — which is critical for letting an
//! inner containing scope (e.g., a dropdown inside a modal) hold focus without
//! the outer scope yanking it back.

use std::{cell::RefCell, collections::HashMap};

use wasm_bindgen::JsCast;

/// Unique identifier for a `FocusScope` instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(u64);

/// Leptos context provided by each `FocusScope` so that nested scopes can
/// discover their parent.
#[derive(Clone, Copy)]
pub struct FocusScopeParentContext {
    pub scope_id: ScopeId,
}

type GetElementFn = Box<dyn Fn() -> Option<web_sys::Element>>;

struct ScopeNode {
    parent: Option<ScopeId>,
    children: Vec<ScopeId>,
    get_element: GetElementFn,
    contain: bool,
    /// The element that had focus before this scope was mounted.
    /// Used for focus restoration when the scope unmounts.
    node_to_restore: Option<web_sys::Element>,
}

struct FocusScopeTree {
    nodes: HashMap<ScopeId, ScopeNode>,
    /// The scope that most recently received focus.
    active_scope: Option<ScopeId>,
    next_id: u64,
}

impl FocusScopeTree {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            active_scope: None,
            next_id: 1,
        }
    }
}

thread_local! {
    static TREE: RefCell<FocusScopeTree> = RefCell::new(FocusScopeTree::new());
}

/// Allocate a new unique `ScopeId`.
pub fn allocate_id() -> ScopeId {
    TREE.with_borrow_mut(|tree| {
        let id = ScopeId(tree.next_id);
        tree.next_id += 1;
        id
    })
}

/// Register a scope in the tree.
///
/// `parent` is discovered via `use_context::<FocusScopeParentContext>()`.
/// `get_element` is a closure returning the scope's current DOM element.
pub fn register_scope(
    id: ScopeId,
    parent: Option<ScopeId>,
    get_element: impl Fn() -> Option<web_sys::Element> + 'static,
    contain: bool,
) {
    TREE.with_borrow_mut(|tree| {
        // Add as child of parent.
        if let Some(parent_id) = parent {
            if let Some(parent_node) = tree.nodes.get_mut(&parent_id) {
                parent_node.children.push(id);
            }
        }

        tree.nodes.insert(
            id,
            ScopeNode {
                parent,
                children: Vec::new(),
                get_element: Box::new(get_element),
                contain,
                node_to_restore: None,
            },
        );
    });
}

/// Unregister a scope from the tree.
///
/// Before removing, propagates `node_to_restore`: if any sibling scope's
/// `node_to_restore` points into this scope's DOM subtree, it is updated
/// to this scope's own `node_to_restore` (matching react-aria's
/// `removeTreeNode` behavior).
pub fn unregister_scope(id: ScopeId) {
    TREE.with_borrow_mut(|tree| {
        // Propagate node_to_restore before removal.
        // If this scope's element contains another scope's node_to_restore,
        // replace that with this scope's node_to_restore.
        let my_element = tree.nodes.get(&id).and_then(|node| (node.get_element)());
        let my_node_to_restore = tree
            .nodes
            .get(&id)
            .and_then(|node| node.node_to_restore.clone());

        if let Some(my_el) = my_element {
            if let Some(my_node) = my_el.dyn_ref::<web_sys::Node>() {
                for (other_id, other_node) in &mut tree.nodes {
                    if *other_id == id {
                        continue;
                    }
                    if let Some(ref restore_el) = other_node.node_to_restore {
                        if let Some(restore_node) = restore_el.dyn_ref::<web_sys::Node>() {
                            if my_node.contains(Some(restore_node)) {
                                other_node.node_to_restore.clone_from(&my_node_to_restore);
                            }
                        }
                    }
                }
            }
        }

        // Remove from parent's children list.
        if let Some(node) = tree.nodes.get(&id) {
            if let Some(parent_id) = node.parent {
                if let Some(parent_node) = tree.nodes.get_mut(&parent_id) {
                    parent_node.children.retain(|child| *child != id);
                }
            }
        }

        tree.nodes.remove(&id);

        // Clear active scope if it was this one.
        if tree.active_scope == Some(id) {
            tree.active_scope = None;
        }
    });
}

/// Set the `node_to_restore` for a scope (the element that had focus before the scope mounted).
pub fn set_node_to_restore(id: ScopeId, element: Option<web_sys::Element>) {
    TREE.with_borrow_mut(|tree| {
        if let Some(node) = tree.nodes.get_mut(&id) {
            node.node_to_restore = element;
        }
    });
}

/// Get the `node_to_restore` for a scope.
pub fn get_node_to_restore(id: ScopeId) -> Option<web_sys::Element> {
    TREE.with_borrow(|tree| {
        tree.nodes
            .get(&id)
            .and_then(|node| node.node_to_restore.clone())
    })
}

/// Walk up the scope tree from `scope_id` to find the first `node_to_restore`
/// that is still connected to the DOM.
///
/// If this scope's own `node_to_restore` is connected, returns it. Otherwise,
/// walks to parent scopes until a connected `node_to_restore` is found.
/// This handles the case where `node_to_restore` was removed from the DOM
/// (e.g., a button in a dynamically updated list).
///
/// Matches react-aria's connected-node walk in `useRestoreFocus` cleanup.
pub fn find_connected_node_to_restore(scope_id: ScopeId) -> Option<web_sys::Element> {
    TREE.with_borrow(|tree| {
        let mut current = Some(scope_id);
        while let Some(id) = current {
            if let Some(node) = tree.nodes.get(&id) {
                if let Some(ref el) = node.node_to_restore {
                    if el.is_connected() {
                        return Some(el.clone());
                    }
                }
                current = node.parent;
            } else {
                break;
            }
        }
        None
    })
}

/// Check whether this scope should actually perform focus restoration.
///
/// Walks from the active scope up to `scope_id`. If any intermediate scope
/// has a `node_to_restore`, that scope is "closer" to the active scope and
/// should handle restoration instead — so this scope should NOT restore.
///
/// This prevents nested scopes from competing: when a menu inside a modal
/// both have `restore_focus=true`, the innermost scope that has a
/// `node_to_restore` handles it; the outer scope defers.
///
/// Matches react-aria's `shouldRestoreFocus` function.
pub fn should_restore_focus(scope_id: ScopeId) -> bool {
    TREE.with_borrow(|tree| {
        let Some(active_id) = tree.active_scope else {
            return false;
        };

        // Walk from active scope up toward scope_id.
        let mut current_id = Some(active_id);
        while let Some(id) = current_id {
            // Reached the target scope — no intermediate restorer found.
            if id == scope_id {
                return true;
            }
            let Some(node) = tree.nodes.get(&id) else {
                break;
            };
            // Intermediate scope with node_to_restore → it will handle restoration.
            if node.node_to_restore.is_some() {
                return false;
            }
            current_id = node.parent;
        }

        // Active scope is not a descendant of (or equal to) this scope.
        false
    })
}

/// Get the currently active scope.
pub fn get_active_scope() -> Option<ScopeId> {
    TREE.with_borrow(|tree| tree.active_scope)
}

/// Set the active scope (called when focus enters a scope).
pub fn set_active_scope(id: ScopeId) {
    TREE.with_borrow_mut(|tree| {
        tree.active_scope = Some(id);
    });
}

/// Set the active scope, but only if `element` is directly within this scope
/// and NOT within any child scope's DOM subtree.
///
/// This prevents a parent scope from overwriting the active scope when a
/// `focusin` event bubbles up from a child scope.
pub fn set_active_scope_if_deepest(scope_id: ScopeId, element: &web_sys::Element) {
    TREE.with_borrow_mut(|tree| {
        let Some(node) = tree.nodes.get(&scope_id) else {
            return;
        };

        // If the element is within any child scope's DOM, this scope is not
        // the deepest — let the child scope handle it.
        let in_child = node.children.iter().any(|&child_id| {
            tree.nodes
                .get(&child_id)
                .and_then(|child| (child.get_element)())
                .and_then(|child_el| child_el.dyn_ref::<web_sys::Node>().cloned())
                .is_some_and(|child_node| {
                    element
                        .dyn_ref::<web_sys::Node>()
                        .is_some_and(|el_node| child_node.contains(Some(el_node)))
                })
        });

        if !in_child {
            tree.active_scope = Some(scope_id);
        }
    });
}

/// Check if `element` is inside the DOM subtree of `scope_id` or any of its
/// descendant scopes.
///
/// This is the key query for focus containment: when an outer scope wants to
/// know if focus is "within" it, focus inside a nested child scope counts as
/// within.
pub fn is_element_in_scope_or_descendant(element: &web_sys::Element, scope_id: ScopeId) -> bool {
    TREE.with_borrow(|tree| is_in_scope_recursive(tree, element, scope_id))
}

fn is_in_scope_recursive(
    tree: &FocusScopeTree,
    element: &web_sys::Element,
    scope_id: ScopeId,
) -> bool {
    let Some(node) = tree.nodes.get(&scope_id) else {
        return false;
    };

    // Allow focus on top-layer elements (e.g., toasts in portals) without
    // triggering containment recapture. Matches react-aria's
    // `isElementInChildScope` check for `data-react-aria-top-layer`.
    if element
        .closest("[data-leptonic-top-layer]")
        .ok()
        .flatten()
        .is_some()
    {
        return true;
    }

    // Check this scope's DOM element.
    if let Some(scope_el) = (node.get_element)() {
        if let Some(scope_node) = scope_el.dyn_ref::<web_sys::Node>() {
            if let Some(el_node) = element.dyn_ref::<web_sys::Node>() {
                if scope_node.contains(Some(el_node)) {
                    return true;
                }
            }
        }
    }

    // Check descendant scopes.
    for &child_id in &node.children {
        if is_in_scope_recursive(tree, element, child_id) {
            return true;
        }
    }

    false
}

/// Returns `true` if `scope_id` has `contain=true` and the active scope is
/// this scope or one of its descendants.
///
/// Only the "most specific" containing scope should recapture focus. If a
/// child scope is active and also contains, the parent should not interfere.
pub fn should_contain_focus(scope_id: ScopeId) -> bool {
    TREE.with_borrow(|tree| {
        let Some(node) = tree.nodes.get(&scope_id) else {
            return false;
        };

        if !node.contain {
            return false;
        }

        let Some(active) = tree.active_scope else {
            return false;
        };

        // Active scope must be this scope or a descendant.
        if active == scope_id {
            return true;
        }

        is_descendant_of(tree, active, scope_id)
    })
}

/// Returns `true` if this scope is the most specific (innermost) containing
/// scope for the current active scope.
///
/// Use this for Tab interception: only the innermost containing scope should
/// handle Tab/Shift+Tab. Outer containing scopes must defer to inner ones.
///
/// The check is: this scope has `contain=true`, the active scope is this scope
/// or a descendant, and no scope between the active scope and this scope also
/// has `contain=true`.
pub fn is_innermost_container(scope_id: ScopeId) -> bool {
    TREE.with_borrow(|tree| {
        let Some(node) = tree.nodes.get(&scope_id) else {
            return false;
        };

        if !node.contain {
            return false;
        }

        let Some(active) = tree.active_scope else {
            return false;
        };

        // If we ARE the active scope, we're the innermost container.
        if active == scope_id {
            return true;
        }

        // Active must be a descendant of this scope.
        if !is_descendant_of(tree, active, scope_id) {
            return false;
        }

        // Walk from active toward this scope. If any intermediate scope
        // (including the active scope itself) has contain=true, that scope
        // is more specific and should handle Tab instead of us.
        let mut current = active;
        loop {
            if current == scope_id {
                break;
            }
            if let Some(cur_node) = tree.nodes.get(&current) {
                if cur_node.contain {
                    return false;
                }
                match cur_node.parent {
                    Some(parent) => current = parent,
                    None => break,
                }
            } else {
                break;
            }
        }

        true
    })
}

/// Check if `candidate` is a descendant of `ancestor` in the scope tree.
fn is_descendant_of(tree: &FocusScopeTree, candidate: ScopeId, ancestor: ScopeId) -> bool {
    let Some(node) = tree.nodes.get(&candidate) else {
        return false;
    };

    match node.parent {
        Some(parent) if parent == ancestor => true,
        Some(parent) => is_descendant_of(tree, parent, ancestor),
        None => false,
    }
}

/// Find the innermost containing scope that is an ancestor of (or is)
/// `scope_id`. Used to determine which scope should actually recapture.
pub fn innermost_containing_ancestor(scope_id: ScopeId) -> Option<ScopeId> {
    TREE.with_borrow(|tree| {
        // First check if any child of scope_id (recursively) is containing and active.
        find_innermost_containing(tree, scope_id)
    })
}

fn find_innermost_containing(tree: &FocusScopeTree, scope_id: ScopeId) -> Option<ScopeId> {
    let node = tree.nodes.get(&scope_id)?;

    // Check children first (deeper = more specific).
    for &child_id in &node.children {
        if let Some(inner) = find_innermost_containing(tree, child_id) {
            return Some(inner);
        }
    }

    // Then check self.
    if node.contain {
        return Some(scope_id);
    }

    None
}
