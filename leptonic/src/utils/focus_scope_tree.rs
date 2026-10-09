// Upstream: react-aria/src/focus/FocusScope.tsx @ 99e6102368
// Upstream: react-aria/test/focus/FocusScope.test.js @ 99e6102368
//! Thread-local tree tracking parent-child relationships between `FocusScope`s (react-aria's
//! `focusScopeTree`, `activeScope` and the scope queries of `FocusScope.tsx`).
//!
//! Each `FocusScope` registers itself in this tree on mount and unregisters on
//! cleanup. The tree enables nested-scope queries: "is this element inside this
//! scope or any of its descendant scopes?" — which is critical for letting an
//! inner containing scope (e.g., a dropdown inside a modal) hold focus without
//! the outer scope yanking it back.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
// - One capturing `focusin` listener on the document tracks the active scope for all scopes
//   (react-aria: listeners per scope, whose order decides). Focus moving within the active
//   scope's tree makes the deepest scope around it active (react-aria: a containing or restoring
//   scope becomes active when the active one is its ancestor); focus moving outside it leaves the
//   active scope while a scope on its way up contains focus (that scope pulls focus back), else
//   the deepest scope around the focus (or, outside every scope, none, unless the active scope
//   contains or restores focus) becomes active. React-aria keeps an unrelated plain scope active
//   while focus is in a containing one, which still contains it.
// - Only the active scope and its ancestors may contain focus. A containing sibling does not
//   recapture focus from a plain or restoring scope, and no scope traps focus before focus enters
//   one. This follows the centralized active-scope tracker rather than upstream's per-scope
//   listener ordering.
//
// =============================================================================

use std::{cell::RefCell, collections::HashMap};

#[cfg(all(feature = "atoms", not(feature = "ssr")))]
use wasm_bindgen::JsCast;

#[cfg(all(feature = "atoms", not(feature = "ssr")))]
use super::focusability::is_in_top_layer;
use super::shadow_dom::node_contains;

/// Unique identifier for a `FocusScope` instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(u64);

/// Leptos context provided by each `FocusScope` so that nested scopes can
/// discover their parent.
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
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
    #[cfg(all(feature = "atoms", not(feature = "ssr")))]
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
    #[cfg(all(feature = "atoms", not(feature = "ssr")))]
    static TRACKING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Tracks the active scope (react-aria: `useActiveScopeTracker` and the scopes' `focusin`
/// handlers), see [`track_focus`]. One capturing listener on the document, installed by the
/// first scope that mounts, so it runs before the scopes' own listeners and never depends on the
/// order of their effects.
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
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
        if let Some(target) = super::shadow_dom::get_event_target(&e)
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            track_focus(&target);
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

/// Whether the element of a scope contains `element` (across shadow roots).
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
fn scope_contains(node: &ScopeNode, element: &web_sys::Element) -> bool {
    (node.get_element)().is_some_and(|scope| node_contains(&scope, element))
}

/// The depth of a scope in the tree (roots: 0).
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
fn depth(tree: &FocusScopeTree, mut id: ScopeId) -> usize {
    let mut depth = 0;
    while let Some(parent) = tree.nodes.get(&id).and_then(|node| node.parent) {
        depth += 1;
        id = parent;
    }
    depth
}

/// The deepest registered scope whose element contains `element`, among `candidates`.
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
fn deepest_containing(
    tree: &FocusScopeTree,
    element: &web_sys::Element,
    candidates: impl Iterator<Item = ScopeId>,
) -> Option<ScopeId> {
    candidates
        .filter(|id| {
            tree.nodes
                .get(id)
                .is_some_and(|node| scope_contains(node, element))
        })
        .max_by_key(|id| depth(tree, *id))
}

/// `id` and every scope below it in the tree.
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
fn subtree(tree: &FocusScopeTree, id: ScopeId) -> Vec<ScopeId> {
    let mut ids = vec![id];
    let mut i = 0;
    while let Some(current) = ids.get(i).copied() {
        if let Some(node) = tree.nodes.get(&current) {
            ids.extend(node.children.iter().copied());
        }
        i += 1;
    }
    ids
}

/// Updates the active scope for focus moving to `element`:
/// - into the active scope's tree (or with no active scope): the deepest scope around it there
///   becomes active ("If focusing an element in a child scope of the currently active scope, the
///   child becomes active");
/// - outside it, while the active scope or one of its ancestors contains focus: nothing ("Moving
///   out of the active scope to an ancestor is not allowed"; the containing scope pulls focus
///   back);
/// - outside it otherwise: the deepest scope around it, or, outside every scope, none, unless the
///   active scope contains or restores focus (react-aria's `useActiveScopeTracker`).
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
pub fn track_focus(element: &web_sys::Element) {
    TREE.with_borrow_mut(|tree| {
        let active = tree
            .active_scope
            .filter(|active| tree.nodes.contains_key(active));
        let next = match active {
            None => deepest_containing(tree, element, tree.nodes.keys().copied()),
            Some(active) if is_in_scope_recursive(tree, element, active) => {
                deepest_containing(tree, element, subtree(tree, active).into_iter())
                    .or(Some(active))
            }
            Some(active) if !contains_on_the_way_up(tree, active, None) => {
                match deepest_containing(tree, element, tree.nodes.keys().copied()) {
                    Some(scope) => Some(scope),
                    // Outside every scope: a plain scope's activity ends.
                    None => tree
                        .nodes
                        .get(&active)
                        .filter(|node| node.contain || node.restore)
                        .map(|_| active),
                }
            }
            Some(active) => Some(active),
        };
        tree.active_scope = next;
    });
}

/// Whether a scope from `from` up to (excluding) `until` contains focus (`until: None`: up to the
/// root).
fn contains_on_the_way_up(tree: &FocusScopeTree, from: ScopeId, until: Option<ScopeId>) -> bool {
    let mut current = Some(from);
    while let Some(id) = current.filter(|id| Some(*id) != until) {
        let Some(node) = tree.nodes.get(&id) else {
            return false;
        };
        if node.contain {
            return true;
        }
        current = node.parent;
    }
    false
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
    // Native tests exercise tree relationships without the client restoration listeners.
    #[cfg(not(all(feature = "atoms", not(feature = "ssr"))))]
    let _ = restore;
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
                #[cfg(all(feature = "atoms", not(feature = "ssr")))]
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

/// Changes whether a registered scope restores focus when it unmounts.
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
pub fn set_restore(id: ScopeId, restore: bool) {
    TREE.with_borrow_mut(|tree| {
        if let Some(node) = tree.nodes.get_mut(&id) {
            node.restore = restore;
        }
    });
}

/// Whether a registered scope restores focus when it unmounts.
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
pub fn restores(id: ScopeId) -> bool {
    TREE.with_borrow(|tree| tree.nodes.get(&id).is_some_and(|node| node.restore))
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
        {
            for (other_id, other_node) in &mut tree.nodes {
                if *other_id == id {
                    continue;
                }
                if let Some(ref restore_el) = other_node.node_to_restore
                    && node_contains(&my_el, restore_el)
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
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
pub fn set_node_to_restore(id: ScopeId, element: Option<web_sys::Element>) {
    TREE.with_borrow_mut(|tree| {
        if let Some(node) = tree.nodes.get_mut(&id) {
            node.node_to_restore = element;
        }
    });
}

/// Get the `node_to_restore` for a scope.
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
pub fn get_node_to_restore(id: ScopeId) -> Option<web_sys::Element> {
    TREE.with_borrow(|tree| {
        tree.nodes
            .get(&id)
            .and_then(|node| node.node_to_restore.clone())
    })
}

/// The nodes to restore focus to when `scope_id` unmounts: its own `node_to_restore`, then those
/// of its parent scopes (used when a node was removed from the DOM in the meantime).
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
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
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
pub fn is_element_in_any_scope(element: &web_sys::Element) -> bool {
    TREE.with_borrow(|tree| {
        tree.nodes
            .values()
            .any(|node| scope_contains(node, element))
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
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
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

/// The element of the active scope.
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
pub fn active_scope_element() -> Option<web_sys::Element> {
    TREE.with_borrow(|tree| {
        tree.active_scope
            .and_then(|id| tree.nodes.get(&id))
            .and_then(|node| (node.get_element)())
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
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
pub fn set_active_scope_if_deepest(scope_id: ScopeId, element: &web_sys::Element) {
    TREE.with_borrow_mut(|tree| {
        let Some(node) = tree.nodes.get(&scope_id) else {
            return;
        };

        // If the element is within any child scope's DOM, this scope is not
        // the deepest — let the child scope handle it.
        let in_child = node.children.iter().any(|child_id| {
            tree.nodes
                .get(child_id)
                .is_some_and(|child| scope_contains(child, element))
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
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
pub fn is_element_in_scope_or_descendant(element: &web_sys::Element, scope_id: ScopeId) -> bool {
    TREE.with_borrow(|tree| is_in_scope_recursive(tree, element, scope_id))
}

#[cfg(all(feature = "atoms", not(feature = "ssr")))]
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
    if is_in_top_layer(element) {
        return true;
    }

    // Check this scope's DOM element.
    if scope_contains(node, element) {
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

/// Whether `scope_id` is the active scope or an ancestor without another containing scope between
/// them. Only the innermost containing scope around the active one holds focus; callers check that
/// `scope_id` contains focus itself. With no active scope, nothing contains focus.
pub fn should_contain_focus(scope_id: ScopeId) -> bool {
    TREE.with_borrow(|tree| {
        tree.active_scope.is_some_and(|active| {
            (active == scope_id || is_descendant_of(tree, active, scope_id))
                && !contains_on_the_way_up(tree, active, Some(scope_id))
        })
    })
}

/// Whether `scope_id` is registered with `contain` and should contain focus now (the scopes'
/// containment handlers run only then).
pub fn is_containing(scope_id: ScopeId) -> bool {
    TREE.with_borrow(|tree| tree.nodes.get(&scope_id).is_some_and(|node| node.contain))
        && should_contain_focus(scope_id)
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

/// Whether `element` is in the active scope or one of its descendant scopes, or in a top layer
/// (react-aria's `isElementInChildOfActiveScope`; without an active scope: in any scope).
///
/// This is used by `use_overlay` to avoid closing an outer overlay when focus moves into a nested
/// scope (e.g., a menu opening inside a dialog).
#[cfg(all(feature = "atoms", not(feature = "ssr")))]
pub fn is_element_in_child_of_active_scope(element: &web_sys::Element) -> bool {
    TREE.with_borrow(|tree| match tree.active_scope {
        Some(active) => is_in_scope_recursive(tree, element, active),
        None => {
            is_in_top_layer(element)
                || tree
                    .nodes
                    .values()
                    .any(|node| scope_contains(node, element))
        }
    })
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

    fn set_no_active_scope() {
        TREE.with_borrow_mut(|tree| tree.active_scope = None);
    }

    fn active() -> Option<ScopeId> {
        TREE.with_borrow(|tree| tree.active_scope)
    }

    fn parent_of(id: ScopeId) -> Option<ScopeId> {
        TREE.with_borrow(|tree| tree.nodes.get(&id).and_then(|node| node.parent))
    }

    /// react-aria's `shouldContainFocus`: a containing scope holds focus unless a containing scope
    /// sits between it and the active scope.
    #[test]
    fn the_innermost_containing_scope_around_the_active_one_holds_focus() {
        let outer = scope(None, true);
        let middle = scope(Some(outer), false);
        let inner = scope(Some(middle), true);
        // No active scope: containment begins only after focus enters a scope.
        set_no_active_scope();
        assert_that!(is_containing(outer)).is_false();

        set_active_scope(middle);
        assert_that!(is_containing(outer)).is_true();
        // Not a scope on the way up from the active one: the outer one holds focus.
        assert_that!(is_containing(inner)).is_false();
        assert_that!(is_containing(middle)).is_false();

        set_active_scope(inner);
        assert_that!(is_containing(outer)).is_false();
        assert_that!(is_containing(inner)).is_true();

        // Containment stops at the nearest containing scope.
        set_contain(inner, false);
        assert_that!(is_containing(outer)).is_true();
    }

    /// FocusScope.test.js "should work with multiple focus scopes": a containing scope next to
    /// the active one doesn't hold focus, the active one does.
    #[test]
    fn a_containing_scope_next_to_the_active_one_holds_no_focus() {
        let first = scope(None, true);
        let second = scope(None, true);
        set_active_scope(first);
        assert_that!(is_containing(first)).is_true();
        assert_that!(is_containing(second)).is_false();

        // A plain active scope next to it stays usable too.
        set_no_active_scope();
        let plain = scope(None, false);
        set_active_scope(plain);
        assert_that!(is_containing(second)).is_false();
        set_active_scope(second);
        assert_that!(is_containing(second)).is_true();
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
