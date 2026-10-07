// Upstream: react-aria/src/focus/FocusScope.tsx @ 99e6102368
//! Thread-local tree tracking parent-child relationships between `FocusScope`s (react-aria's
//! `focusScopeTree`, `activeScope` and the scope queries of `FocusScope.tsx`).
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
    /// Whether the scope restores focus when it unmounts.
    restore: bool,
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
    /// Whether the document-level active scope tracking is installed.
    static TRACKING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Tracks the active scope (react-aria: `useActiveScopeTracker`): on every `focusin`, the deepest
/// registered scope containing the focused element becomes the active scope; focus outside all
/// scopes ends the activity of a scope that neither contains nor restores focus. One capturing
/// listener on the document, installed by the first scope that mounts, so it never depends on
/// the order of the scopes' effects.
#[cfg(not(feature = "ssr"))]
pub fn ensure_active_scope_tracking() {
    use wasm_bindgen::closure::Closure;

    if TRACKING.with(|tracking| tracking.replace(true)) {
        return;
    }
    let Some(document) = leptos_use::use_document().as_ref().cloned() else {
        TRACKING.with(|tracking| tracking.set(false));
        return;
    };
    let on_focusin = Closure::<dyn Fn(web_sys::FocusEvent)>::new(|e: web_sys::FocusEvent| {
        if let Some(target) = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            set_active_scope_to_deepest_containing(&target);
        }
    });
    let _ = document.add_event_listener_with_callback_and_bool(
        "focusin",
        on_focusin.as_ref().unchecked_ref(),
        true,
    );
    // The listener lives as long as the page.
    on_focusin.forget();
}

/// Makes the deepest registered scope whose element contains `element` the active scope.
pub fn set_active_scope_to_deepest_containing(element: &web_sys::Element) {
    TREE.with_borrow_mut(|tree| {
        let depth = |mut id: ScopeId| {
            let mut depth = 0;
            while let Some(parent) = tree.nodes.get(&id).and_then(|node| node.parent) {
                depth += 1;
                id = parent;
            }
            depth
        };
        let deepest = tree
            .nodes
            .iter()
            .filter(|(_, node)| {
                (node.get_element)().is_some_and(|scope_el| {
                    scope_el.contains(Some(element.unchecked_ref::<web_sys::Node>()))
                })
            })
            .map(|(id, _)| *id)
            .max_by_key(|id| depth(*id));
        if let Some(deepest) = deepest {
            tree.active_scope = Some(deepest);
        } else {
            // Focus outside every scope ends a plain scope's activity (react-aria's
            // `useActiveScopeTracker`); a containing or restoring one stays active.
            let plain = tree
                .active_scope
                .and_then(|active| tree.nodes.get(&active))
                .is_some_and(|node| !node.contain && !node.restore);
            if plain {
                tree.active_scope = None;
            }
        }
    });
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
    restore: bool,
) {
    TREE.with_borrow_mut(|tree| {
        // A scope mounting outside the active scope (e.g. a dialog opened from a menu, rendered
        // elsewhere) gets the active scope as its parent, so restoring and containing chain
        // through it (react-aria's `FocusScope`). Scopes inside the active one keep their parent
        // (the descendant check); a parent that isn't registered (yet) is kept as well.
        let parent_registered = parent.is_none_or(|parent| tree.nodes.contains_key(&parent));
        let parent = match (parent, tree.active_scope) {
            (parent, Some(active))
                if parent_registered
                    && tree.nodes.contains_key(&active)
                    && !parent.is_some_and(|parent| is_descendant_of(tree, parent, active)) =>
            {
                Some(active)
            }
            (parent, _) => parent,
        };

        // Add as child of parent.
        if let Some(parent_id) = parent
            && let Some(parent_node) = tree.nodes.get_mut(&parent_id)
        {
            parent_node.children.push(id);
        }

        tree.nodes.insert(
            id,
            ScopeNode {
                parent,
                children: Vec::new(),
                get_element: Box::new(get_element),
                contain,
                restore,
                node_to_restore: None,
            },
        );
    });
}

/// Changes whether a registered scope contains focus.
pub fn set_contain(id: ScopeId, contain: bool) {
    TREE.with_borrow_mut(|tree| {
        if let Some(node) = tree.nodes.get_mut(&id) {
            node.contain = contain;
        }
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

        if let Some(my_el) = my_element
            && my_node_to_restore.is_some()
            && let Some(my_node) = my_el.dyn_ref::<web_sys::Node>()
        {
            for (other_id, other_node) in &mut tree.nodes {
                if *other_id == id {
                    continue;
                }
                if let Some(ref restore_el) = other_node.node_to_restore
                    && let Some(restore_node) = restore_el.dyn_ref::<web_sys::Node>()
                    && my_node.contains(Some(restore_node))
                {
                    other_node.node_to_restore.clone_from(&my_node_to_restore);
                }
            }
        }

        // The active scope (this one, or one inside it) passes to the parent (react-aria).
        let was_active = tree
            .active_scope
            .is_some_and(|active| active == id || is_descendant_of(tree, active, id));

        // Remove from the parent's children; this scope's children move to the parent.
        let Some(node) = tree.nodes.remove(&id) else {
            return;
        };
        if let Some(parent_node) = node.parent.and_then(|parent| tree.nodes.get_mut(&parent)) {
            parent_node.children.retain(|child| *child != id);
            parent_node.children.extend(node.children.iter().copied());
        }
        for child in &node.children {
            if let Some(child_node) = tree.nodes.get_mut(child) {
                child_node.parent = node.parent;
            }
        }

        if was_active {
            tree.active_scope = node.parent.filter(|parent| tree.nodes.contains_key(parent));
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

/// The nodes to restore focus to when `scope_id` unmounts: its own `node_to_restore`, then those
/// of its parent scopes (used when a node was removed from the DOM in the meantime).
pub fn nodes_to_restore(scope_id: ScopeId) -> Vec<web_sys::Element> {
    TREE.with_borrow(|tree| {
        let mut nodes = Vec::new();
        let mut current = Some(scope_id);
        while let Some(node) = current.and_then(|id| tree.nodes.get(&id)) {
            nodes.extend(node.node_to_restore.clone());
            current = node.parent;
        }
        nodes
    })
}

/// Whether `element` is inside any registered scope.
pub fn is_element_in_any_scope(element: &web_sys::Element) -> bool {
    TREE.with_borrow(|tree| {
        tree.nodes.values().any(|node| {
            (node.get_element)()
                .is_some_and(|scope| scope.contains(Some(element.unchecked_ref::<web_sys::Node>())))
        })
    })
}

/// The parent scopes of `scope_id`, innermost first.
pub fn ancestors(scope_id: ScopeId) -> Vec<ScopeId> {
    TREE.with_borrow(|tree| {
        let mut ids = Vec::new();
        let mut current = tree.nodes.get(&scope_id).and_then(|node| node.parent);
        while let Some(id) = current {
            ids.push(id);
            current = tree.nodes.get(&id).and_then(|node| node.parent);
        }
        ids
    })
}

/// The element of a scope, if it is still registered.
pub fn scope_element(id: ScopeId) -> Option<web_sys::Element> {
    TREE.with_borrow(|tree| tree.nodes.get(&id).and_then(|node| (node.get_element)()))
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
    if let Some(scope_el) = (node.get_element)()
        && let Some(scope_node) = scope_el.dyn_ref::<web_sys::Node>()
        && let Some(el_node) = element.dyn_ref::<web_sys::Node>()
        && scope_node.contains(Some(el_node))
    {
        return true;
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

/// Check if `element` is inside any child scope of the currently active scope.
///
/// This is used by `use_overlay` to avoid closing an outer overlay when focus
/// moves into a nested scope (e.g., a menu opening inside a dialog).
/// Equivalent to react-aria's `isElementInChildOfActiveScope`.
///
/// Returns `true` if:
/// - The element is inside a `[data-leptonic-top-layer]` container, OR
/// - The element is within the DOM subtree of any scope that is a descendant
///   of the active scope.
pub fn is_element_in_child_of_active_scope(element: &web_sys::Element) -> bool {
    TREE.with_borrow(|tree| {
        let Some(active_id) = tree.active_scope else {
            return false;
        };

        is_element_in_child_scope(tree, element, active_id)
    })
}

/// Check if `element` is in any child scope of the given scope.
fn is_element_in_child_scope(
    tree: &FocusScopeTree,
    element: &web_sys::Element,
    scope_id: ScopeId,
) -> bool {
    // Allow focus on top-layer elements (e.g., toasts in portals).
    if element
        .closest("[data-leptonic-top-layer]")
        .ok()
        .flatten()
        .is_some()
    {
        return true;
    }

    let Some(node) = tree.nodes.get(&scope_id) else {
        return false;
    };

    // Check all descendant scopes (but not this scope itself).
    for &child_id in &node.children {
        if is_in_scope_recursive(tree, element, child_id) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    /// Registers a scope without an element (the tree logic doesn't need one).
    fn scope(parent: Option<ScopeId>, contain: bool) -> ScopeId {
        let id = allocate_id();
        register_scope(id, parent, || None, contain, false);
        id
    }

    fn active() -> Option<ScopeId> {
        TREE.with_borrow(|tree| tree.active_scope)
    }

    fn parent_of(id: ScopeId) -> Option<ScopeId> {
        TREE.with_borrow(|tree| tree.nodes.get(&id).and_then(|node| node.parent))
    }

    #[test]
    fn a_scope_contains_focus_while_it_or_a_descendant_is_active() {
        let outer = scope(None, true);
        let inner = scope(Some(outer), false);
        assert_that!(should_contain_focus(outer)).is_false();

        set_active_scope(inner);
        assert_that!(should_contain_focus(outer)).is_true();
        assert_that!(should_contain_focus(inner)).is_false();

        set_contain(outer, false);
        assert_that!(should_contain_focus(outer)).is_false();
    }

    #[test]
    fn only_the_innermost_containing_scope_handles_tab() {
        let outer = scope(None, true);
        let middle = scope(Some(outer), false);
        let inner = scope(Some(middle), true);

        set_active_scope(middle);
        assert_that!(is_innermost_container(outer)).is_true();
        assert_that!(is_innermost_container(inner)).is_false();

        set_active_scope(inner);
        assert_that!(is_innermost_container(outer)).is_false();
        assert_that!(is_innermost_container(inner)).is_true();

        // react-aria's `shouldContainFocus`: containment stops at the nearest containing scope.
        set_contain(inner, false);
        assert_that!(is_innermost_container(outer)).is_true();
    }

    #[test]
    fn a_scope_mounting_outside_the_active_scope_gets_it_as_its_parent() {
        let menu = scope(None, true);
        set_active_scope(menu);
        // E.g. a dialog opened from the menu, rendered elsewhere.
        let dialog = scope(None, true);
        assert_that!(parent_of(dialog)).is_equal_to(Some(menu));
        assert_that!(ancestors(dialog)).is_equal_to(vec![menu]);

        // A scope inside the active one keeps its parent.
        let item = scope(Some(menu), false);
        let nested = scope(Some(item), false);
        assert_that!(parent_of(nested)).is_equal_to(Some(item));
        assert_that!(ancestors(nested)).is_equal_to(vec![item, menu]);
    }

    #[test]
    fn unregistering_moves_children_and_activity_to_the_parent() {
        let root = scope(None, false);
        let middle = scope(Some(root), false);
        let leaf = scope(Some(middle), true);
        set_active_scope(leaf);

        // As react-aria: unmounting the active scope or one of its ancestors makes the parent
        // the active scope.
        unregister_scope(middle);
        assert_that!(parent_of(leaf)).is_equal_to(Some(root));
        assert_that!(active()).is_equal_to(Some(root));

        set_active_scope(leaf);
        unregister_scope(leaf);
        assert_that!(active()).is_equal_to(Some(root));
        assert_that!(ancestors(leaf)).is_empty();
    }

    #[test]
    fn the_active_scope_or_its_ancestors_restore_focus() {
        let outer = scope(None, false);
        let inner = scope(Some(outer), false);
        let other = scope(None, false);
        // No active scope: nothing restores.
        assert_that!(should_restore_focus(outer)).is_false();

        set_active_scope(inner);
        assert_that!(should_restore_focus(inner)).is_true();
        // No scope in between has a node to restore.
        assert_that!(should_restore_focus(outer)).is_true();

        // A scope outside the active one's ancestors doesn't restore.
        assert_that!(should_restore_focus(other)).is_false();
    }
}
