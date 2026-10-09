// Upstream: react-aria/src/dnd/DropTargetKeyboardNavigation.ts @ 99e6102368
// Upstream: react-aria/test/dnd/DropTargetKeyboardNavigation.test.tsx @ 99e6102368
use super::types::{DropPosition, DropTarget, ItemDropTarget};
use crate::hooks::collections::{Collection, Key, KeyboardDelegate, NavigationOptions, NodeKind};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// The arrow key moving a keyboard drag's drop target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NavigationDirection {
    Left,
    Right,
    Up,
    Down,
}

/// The drop target after `target` when moving in `direction`: through the root, then before,
/// on and after each item (and into and out of nested items).
pub(crate) fn navigate(
    delegate: &dyn KeyboardDelegate,
    collection: &Collection,
    target: Option<&DropTarget>,
    direction: NavigationDirection,
    rtl: bool,
    wrap: bool,
) -> Option<DropTarget> {
    use NavigationDirection::{Down, Left, Right, Up};
    match direction {
        Left if rtl => next_drop_target(delegate, collection, target, wrap, Some(Left)),
        Left => previous_drop_target(delegate, collection, target, wrap, Some(Left)),
        Right if rtl => previous_drop_target(delegate, collection, target, wrap, Some(Right)),
        Right => next_drop_target(delegate, collection, target, wrap, Some(Right)),
        Up => previous_drop_target(delegate, collection, target, wrap, None),
        Down => next_drop_target(delegate, collection, target, wrap, None),
    }
}

const INCLUDE_DISABLED: NavigationOptions = NavigationOptions {
    include_disabled: true,
};

fn item(key: Key, drop_position: DropPosition) -> DropTarget {
    DropTarget::Item(ItemDropTarget { key, drop_position })
}

fn next_drop_target(
    delegate: &dyn KeyboardDelegate,
    collection: &Collection,
    target: Option<&DropTarget>,
    wrap: bool,
    horizontal: Option<NavigationDirection>,
) -> Option<DropTarget> {
    let Some(target) = target else {
        return Some(DropTarget::Root);
    };
    let target = match target {
        DropTarget::Root => {
            return delegate
                .first_key(None, false)
                .map(|key| item(key, DropPosition::Before));
        }
        DropTarget::Item(target) => target,
    };

    let next_key = match horizontal {
        Some(NavigationDirection::Right) => delegate.key_right_of(&target.key, INCLUDE_DISABLED),
        Some(_) => delegate.key_left_of(&target.key, INCLUDE_DISABLED),
        None => delegate.key_below(&target.key, INCLUDE_DISABLED),
    };
    let next_collection_key = next_item(collection, &target.key, |k| {
        collection.key_after(k).cloned()
    });
    if let Some(next_key) = &next_key
        && Some(next_key) != next_collection_key.as_ref()
    {
        return Some(item(next_key.clone(), target.drop_position));
    }

    match target.drop_position {
        DropPosition::Before => return Some(item(target.key.clone(), DropPosition::On)),
        DropPosition::On => {
            let target_node = collection.get(&target.key);
            let next_node = next_key.as_ref().and_then(|k| collection.get(k));
            if let (Some(target_node), Some(next_node)) = (target_node, next_node)
                && next_node.level >= target_node.level
            {
                return Some(item(next_node.key.clone(), DropPosition::Before));
            }
            return Some(item(target.key.clone(), DropPosition::After));
        }
        DropPosition::After => {
            let target_node = collection.get(&target.key);
            let mut next_in_level = target_node
                .and_then(|n| n.next_key.as_ref())
                .and_then(|k| collection.get(k));
            while let Some(node) = next_in_level
                && node.kind != NodeKind::Item
            {
                next_in_level = node.next_key.as_ref().and_then(|k| collection.get(k));
            }
            if let Some(target_node) = target_node
                && next_in_level.is_none()
                && let Some(parent) = target_node
                    .parent_key
                    .as_ref()
                    .and_then(|k| collection.get(k))
            {
                // After the last child: before the parent's next sibling, or after the parent.
                if let Some(next) = parent.next_key.as_ref().and_then(|k| collection.get(k))
                    && next.kind == NodeKind::Item
                {
                    return Some(item(next.key.clone(), DropPosition::Before));
                }
                if parent.kind == NodeKind::Item {
                    return Some(item(parent.key.clone(), DropPosition::After));
                }
            }
            if let Some(next) = next_in_level {
                return Some(item(next.key.clone(), DropPosition::On));
            }
        }
    }

    wrap.then_some(DropTarget::Root)
}

fn previous_drop_target(
    delegate: &dyn KeyboardDelegate,
    collection: &Collection,
    target: Option<&DropTarget>,
    wrap: bool,
    horizontal: Option<NavigationDirection>,
) -> Option<DropTarget> {
    if target.is_none() || (wrap && target == Some(&DropTarget::Root)) {
        // After the outermost ancestor of the last item.
        let mut previous_key = None;
        let mut last_key = delegate.last_key(None, false);
        while let Some(key) = last_key.take() {
            let Some(node) = collection.get(&key) else {
                break;
            };
            if node.kind != NodeKind::Item {
                break;
            }
            last_key.clone_from(&node.parent_key);
            previous_key = Some(key);
        }
        return previous_key.map(|key| item(key, DropPosition::After));
    }
    let Some(DropTarget::Item(target)) = target else {
        return None;
    };

    let previous_key = match horizontal {
        Some(NavigationDirection::Left) => delegate.key_left_of(&target.key, INCLUDE_DISABLED),
        Some(_) => delegate.key_right_of(&target.key, INCLUDE_DISABLED),
        None => delegate.key_above(&target.key, INCLUDE_DISABLED),
    };
    let previous_collection_key = next_item(collection, &target.key, |k| {
        collection.key_before(k).cloned()
    });
    if let Some(previous_key) = &previous_key
        && Some(previous_key) != previous_collection_key.as_ref()
    {
        return Some(item(previous_key.clone(), target.drop_position));
    }

    match target.drop_position {
        DropPosition::Before => {
            if let Some(prev) = collection.get(&target.key).and_then(|n| n.prev_key.clone())
                && let Some(last_child) = last_child(collection, &prev)
            {
                return Some(last_child);
            }
            Some(match previous_key {
                Some(key) => item(key, DropPosition::On),
                None => DropTarget::Root,
            })
        }
        DropPosition::On => Some(item(target.key.clone(), DropPosition::Before)),
        DropPosition::After => Some(
            last_child(collection, &target.key)
                .unwrap_or_else(|| item(target.key.clone(), DropPosition::On)),
        ),
    }
}

/// "After the last child" of `key`, if it has (expanded) children.
fn last_child(collection: &Collection, key: &Key) -> Option<DropTarget> {
    let target_node = collection.get(key)?;
    let next_key = next_item(collection, key, |k| collection.key_after(k).cloned())?;
    let next_node = collection.get(&next_key)?;
    if next_node.level <= target_node.level {
        return None;
    }
    let mut last = target_node
        .last_child_key
        .as_ref()
        .and_then(|k| collection.get(k));
    while let Some(node) = last
        && node.kind != NodeKind::Item
        && node.prev_key.is_some()
    {
        last = node.prev_key.as_ref().and_then(|k| collection.get(k));
    }
    last.map(|node| item(node.key.clone(), DropPosition::After))
}

/// The next item (skipping other nodes) in the direction of `step`.
fn next_item(
    collection: &Collection,
    key: &Key,
    step: impl Fn(&Key) -> Option<Key>,
) -> Option<Key> {
    let mut next = step(key);
    while let Some(k) = &next {
        match collection.get(k) {
            Some(node) if node.kind != NodeKind::Item => next = step(k),
            _ => break,
        }
    }
    next
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, sync::Arc};

    use assertr::prelude::*;
    use leptos::prelude::*;

    use super::*;
    use crate::{
        Orientation,
        hooks::collections::{
            CollectionMemo, LayoutDelegate, ListKeyboardDelegate, Rect, SelectionManager,
            SelectionOptions, Size,
        },
        utils::i18n::WritingDirection,
    };

    struct NoLayout;

    impl LayoutDelegate for NoLayout {
        fn item_rect(&self, _key: &Key) -> Option<Rect> {
            None
        }
        fn visible_rect(&self) -> Rect {
            Rect::default()
        }
        fn content_size(&self) -> Size {
            Size::default()
        }
    }

    /// react-aria's test tree: projects and reports with nested items, fully expanded.
    fn collection() -> CollectionMemo {
        Memo::new(|_| {
            let tree = Collection::build(|b| {
                b.item("projects", "Projects").children(|b| {
                    b.item("project-1", "Project 1");
                    b.item("project-2", "Project 2").children(|b| {
                        b.item("project-2A", "Project 2A");
                        b.item("project-2B", "Project 2B");
                        b.item("project-2C", "Project 2C");
                    });
                    b.item("project-3", "Project 3");
                    b.item("project-4", "Project 4");
                    b.item("project-5", "Project 5").children(|b| {
                        b.item("project-5A", "Project 5A");
                        b.item("project-5B", "Project 5B");
                        b.item("project-5C", "Project 5C");
                    });
                });
                b.item("reports", "Reports").children(|b| {
                    b.item("reports-1", "Reports 1").children(|b| {
                        b.item("reports-1A", "Reports 1A").children(|b| {
                            b.item("reports-1AB", "Reports 1AB").children(|b| {
                                b.item("reports-1ABC", "Reports 1ABC");
                            });
                        });
                        b.item("reports-1B", "Reports 1B");
                        b.item("reports-1C", "Reports 1C");
                    });
                    b.item("reports-2", "Reports 2");
                });
            });
            let parents: HashSet<Key> = [
                "projects",
                "project-2",
                "project-5",
                "reports",
                "reports-1",
                "reports-1A",
                "reports-1AB",
            ]
            .into_iter()
            .map(Key::from)
            .collect();
            Arc::new(tree.with_expanded(&parents))
        })
    }

    fn expected() -> Vec<String> {
        let mut targets = vec!["root".to_owned()];
        for (key, positions) in [
            ("projects", "bo"),
            ("project-1", "bo"),
            ("project-2", "bo"),
            ("project-2A", "bo"),
            ("project-2B", "bo"),
            ("project-2C", "boa"),
            ("project-3", "bo"),
            ("project-4", "bo"),
            ("project-5", "bo"),
            ("project-5A", "bo"),
            ("project-5B", "bo"),
            ("project-5C", "boa"),
            ("project-5", "a"),
            ("reports", "bo"),
            ("reports-1", "bo"),
            ("reports-1A", "bo"),
            ("reports-1AB", "bo"),
            ("reports-1ABC", "boa"),
            ("reports-1AB", "a"),
            ("reports-1B", "bo"),
            ("reports-1C", "boa"),
            ("reports-2", "boa"),
            ("reports", "a"),
        ] {
            for p in positions.chars() {
                let position = match p {
                    'b' => "before",
                    'o' => "on",
                    _ => "after",
                };
                targets.push(format!("{key} {position}"));
            }
        }
        targets
    }

    fn describe(target: &DropTarget) -> String {
        match target {
            DropTarget::Root => "root".to_owned(),
            DropTarget::Item(item) => format!(
                "{} {}",
                item.key,
                match item.drop_position {
                    DropPosition::Before => "before",
                    DropPosition::On => "on",
                    DropPosition::After => "after",
                }
            ),
        }
    }

    fn collect(
        delegate: &dyn KeyboardDelegate,
        direction: NavigationDirection,
        rtl: bool,
    ) -> Vec<String> {
        let collection = delegate_collection();
        let mut results = Vec::new();
        let mut target = None;
        loop {
            target = collection
                .with_untracked(|c| navigate(delegate, c, target.as_ref(), direction, rtl, false));
            match &target {
                Some(t) => results.push(describe(t)),
                None => return results,
            }
        }
    }

    thread_local! {
        static COLLECTION: std::cell::Cell<Option<CollectionMemo>> = const { std::cell::Cell::new(None) };
    }

    fn delegate_collection() -> CollectionMemo {
        COLLECTION
            .with(std::cell::Cell::get)
            .expect("set by the test")
    }

    fn delegate(orientation: Orientation, direction: WritingDirection) -> ListKeyboardDelegate {
        let collection = collection();
        COLLECTION.with(|c| c.set(Some(collection)));
        let selection = SelectionManager::new(collection, SelectionOptions::default());
        ListKeyboardDelegate::new(collection, selection, Arc::new(NoLayout))
            .with_orientation(orientation)
            .with_direction(direction)
    }

    #[test]
    fn navigates_forward_vertically() {
        crate::testing::with_owner(|| {
            let d = delegate(Orientation::Vertical, WritingDirection::Ltr);
            assert_that!(collect(&d, NavigationDirection::Down, false)).is_equal_to(expected());
        });
    }

    #[test]
    fn navigates_backward_vertically() {
        crate::testing::with_owner(|| {
            let d = delegate(Orientation::Vertical, WritingDirection::Ltr);
            let mut results = collect(&d, NavigationDirection::Up, false);
            results.reverse();
            assert_that!(results).is_equal_to(expected());
        });
    }

    #[test]
    fn navigates_forward_horizontally_ltr() {
        crate::testing::with_owner(|| {
            let d = delegate(Orientation::Horizontal, WritingDirection::Ltr);
            assert_that!(collect(&d, NavigationDirection::Right, false)).is_equal_to(expected());
        });
    }

    #[test]
    fn navigates_forward_horizontally_rtl() {
        crate::testing::with_owner(|| {
            let d = delegate(Orientation::Horizontal, WritingDirection::Rtl);
            assert_that!(collect(&d, NavigationDirection::Left, true)).is_equal_to(expected());
        });
    }

    #[test]
    fn navigates_backward_horizontally_ltr() {
        crate::testing::with_owner(|| {
            let d = delegate(Orientation::Horizontal, WritingDirection::Ltr);
            let mut results = collect(&d, NavigationDirection::Left, false);
            results.reverse();
            assert_that!(results).is_equal_to(expected());
        });
    }

    #[test]
    fn navigates_backward_horizontally_rtl() {
        crate::testing::with_owner(|| {
            let d = delegate(Orientation::Horizontal, WritingDirection::Rtl);
            let mut results = collect(&d, NavigationDirection::Right, true);
            results.reverse();
            assert_that!(results).is_equal_to(expected());
        });
    }
}
