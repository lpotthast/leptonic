// Upstream: react-stately/src/grid/useGridState.ts @ 99e6102368
// Upstream: react-aria/test/grid/useGrid.test.js @ 99e6102368
// Upstream: react-aria-components/test/Table.test.js @ 99e6102368
use std::sync::Arc;

use leptos::prelude::*;

use super::GridFocusMode;
use crate::hooks::collections::{
    Collection, CollectionMemo, ItemElements, Key, ListState, Node, NodeKind, SelectionManager,
    SelectionOptions,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The grid is a collection of rows (`CollectionBuilder::row`) whose children are cells.
// - In cell focus mode, the selection manager itself focuses a row's first/last cell when the
//   row is focused (react-aria wraps `setFocusedKey`).
//
// =============================================================================

/// Input of [`use_grid_state`].
#[derive(Debug, Clone)]
pub struct UseGridStateInput {
    /// The rows and their cells.
    pub collection: CollectionMemo,
    pub selection: SelectionOptions,
    pub focus_mode: GridFocusMode,
}

/// The state of a grid: its rows and cells, selection and focus.
#[derive(Debug, Clone, Copy)]
pub struct GridState {
    pub list: ListState,
    pub focus_mode: GridFocusMode,
    /// Set while e.g. a table column is resized with the arrow keys.
    keyboard_navigation_disabled: RwSignal<bool>,
    /// Whether the grid handles navigation keys: the flag above, for tables also while empty.
    navigation_disabled: Signal<bool>,
}

impl GridState {
    /// Whether the grid ignores navigation keys (react-stately's
    /// `isKeyboardNavigationDisabled`): while set with
    /// [`set_keyboard_navigation_disabled`](Self::set_keyboard_navigation_disabled), and for
    /// tables also while they have no rows.
    pub fn is_keyboard_navigation_disabled(&self) -> Signal<bool> {
        self.navigation_disabled
    }

    /// Let the grid ignore navigation keys (e.g. while the arrow keys resize a table column), or
    /// handle them again.
    pub fn set_keyboard_navigation_disabled(&self, disabled: bool) {
        self.keyboard_navigation_disabled.set(disabled);
    }

    /// The state, with keyboard navigation also disabled while the grid has no rows (react-stately's
    /// `useTableState`).
    pub(crate) fn without_navigation_while_empty(self) -> Self {
        let flag = self.keyboard_navigation_disabled;
        let collection = self.list.collection;
        Self {
            navigation_disabled: Signal::derive(move || {
                flag.get() || collection.with(|c| c.size() == 0)
            }),
            ..self
        }
    }
}

/// Creates the state of a grid (see [`GridState`]).
pub fn use_grid_state(input: UseGridStateInput) -> GridState {
    let UseGridStateInput {
        collection,
        selection,
        focus_mode,
    } = input;
    let mut manager = SelectionManager::new(collection, selection);
    if focus_mode == GridFocusMode::Cell {
        manager = manager.with_cell_focus();
    }

    // When the focused row (or a cell of it) disappears, focus moves to the row that took its
    // place (or the closest one before), to the same column. A row collapsed out of view moves
    // focus to its closest ancestor row shown.
    Effect::new(move |previous: Option<Arc<Collection>>| {
        let current = collection.get();
        let focused = untrack(|| manager.focused_key());
        if let (Some(previous), Some(focused)) = (&previous, focused)
            && !current.contains_key(&focused)
        {
            let next = untrack(|| refocus(previous, &current, &focused, &manager));
            manager.set_focused_key(next, None);
        }
        current
    });

    let keyboard_navigation_disabled = RwSignal::new(false);
    GridState {
        list: ListState {
            collection,
            selection: manager,
            item_elements: ItemElements::new(),
        },
        focus_mode,
        keyboard_navigation_disabled,
        navigation_disabled: keyboard_navigation_disabled.into(),
    }
}

/// The key to focus after the focused `key` was removed (or collapsed out of view).
fn refocus(
    previous: &Collection,
    current: &Collection,
    key: &Key,
    manager: &SelectionManager,
) -> Option<Key> {
    let node = previous.get(key)?;
    // A cell or column header: its row (a header row), and the column to focus in the new row.
    let (row, column) = match node.kind {
        NodeKind::Cell => (previous.get(node.parent_key.as_ref()?)?, Some(node.index)),
        NodeKind::Column => (
            previous.get(node.parent_key.as_ref()?)?,
            Some(node.col_index.unwrap_or(node.index)),
        ),
        _ => (node, None),
    };
    // A row collapsed out of view (tree grids): its closest ancestor row still shown, to the
    // same column.
    let mut ancestor = row.parent_key.as_ref();
    while let Some(parent) = ancestor {
        if current.get(parent).is_some_and(Node::is_item) {
            if let Some(column) = column
                && let Some(cell) = current.cells(parent).nth(column)
            {
                return Some(cell.key.clone());
            }
            return Some(parent.clone());
        }
        ancestor = previous.get(parent).and_then(|n| n.parent_key.as_ref());
    }
    let previous_rows: Vec<&Key> = previous.items().map(|n| &n.key).collect();
    let rows: Vec<Key> = current.items().map(|n| n.key.clone()).collect();
    if rows.is_empty() {
        return None;
    }
    // A column header (of a header row, before every body row) moves to the first body row
    // (react-stately: the header row's index, skipping header rows).
    let row_index = if row.kind == NodeKind::HeaderRow {
        0
    } else {
        previous_rows.iter().position(|k| **k == row.key)?
    };
    let removed = previous_rows.len().saturating_sub(rows.len());
    let index = if removed > 1 {
        (row_index + 1).saturating_sub(removed)
    } else {
        row_index
    }
    .min(rows.len() - 1);
    let usable = |k: &Key| !manager.is_disabled(k);
    let new_row = rows[index..]
        .iter()
        .find(|k| usable(k))
        .or_else(|| rows[..index].iter().rev().find(|k| usable(k)))?;
    if let Some(column) = column
        && let Some(cell) = current.cells(new_row).nth(column)
    {
        return Some(cell.key.clone());
    }
    Some(new_row.clone())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use assertr::prelude::*;

    use super::*;
    use crate::{
        hooks::collections::{CollectionBuilder, ItemBuilder, SelectionMode},
        testing::{flush_effects, with_owner},
    };

    /// A grid of `rows`, each with the cells `name` and `role`; rows in `disabled` are disabled.
    fn grid(
        rows: RwSignal<Vec<&'static str>>,
        disabled: &[&'static str],
        focus_mode: GridFocusMode,
    ) -> GridState {
        let collection: CollectionMemo = Memo::new(move |_| {
            Arc::new(Collection::build(|b| {
                for row in rows.get() {
                    b.row(row, row, |r| {
                        r.cell(row);
                        r.cell("role");
                    });
                }
            }))
        });
        use_grid_state(UseGridStateInput {
            collection,
            selection: SelectionOptions {
                selection_mode: Signal::stored(SelectionMode::Multiple),
                disabled_keys: Signal::stored(disabled.iter().copied().map(Key::from).collect()),
                ..Default::default()
            },
            focus_mode,
        })
    }

    fn focus(state: &GridState, key: Key) {
        state.list.selection.set_focused_key(Some(key), None);
        flush_effects();
    }

    fn focused(state: &GridState) -> Option<Key> {
        state.list.selection.focused_key()
    }

    fn remove(rows: RwSignal<Vec<&'static str>>, removed: &[&str]) {
        let removed: HashSet<&str> = removed.iter().copied().collect();
        rows.update(|rows| rows.retain(|row| !removed.contains(row)));
        flush_effects();
    }

    /// A tree grid: docs > (cv, work > (letter)), photos; the rows in `expanded` show their
    /// child rows.
    fn tree_grid(expanded: RwSignal<HashSet<Key>>, focus_mode: GridFocusMode) -> GridState {
        let full = Collection::build(|b| {
            fn row<'b>(b: &'b mut CollectionBuilder, key: &'static str) -> ItemBuilder<'b> {
                b.row(key, key, |r| {
                    r.cell(key);
                    r.cell("kind");
                })
            }
            let _ = row(b, "docs").children(|b| {
                row(b, "cv");
                let _ = row(b, "work").children(|b| {
                    row(b, "letter");
                });
            });
            row(b, "photos");
        });
        let collection: CollectionMemo =
            Memo::new(move |_| Arc::new(expanded.with(|expanded| full.with_expanded(expanded))));
        use_grid_state(UseGridStateInput {
            collection,
            selection: SelectionOptions::default(),
            focus_mode,
        })
    }

    #[test]
    fn a_row_collapsed_out_of_view_moves_focus_to_its_closest_shown_ancestor() {
        with_owner(|| {
            let expanded = RwSignal::new(HashSet::from([Key::from("docs"), Key::from("work")]));
            let state = tree_grid(expanded, GridFocusMode::Row);
            flush_effects();
            focus(&state, Key::from("letter"));

            // Collapsing `docs` (e.g. through bound expanded keys) hides `work` and `letter`.
            expanded.set(HashSet::from([Key::from("work")]));
            flush_effects();
            assert_that!(focused(&state)).is_equal_to(Some(Key::from("docs")));
        });
    }

    #[test]
    fn a_cell_collapsed_out_of_view_moves_focus_to_the_same_column_of_its_ancestor() {
        with_owner(|| {
            let expanded = RwSignal::new(HashSet::from([Key::from("docs")]));
            let state = tree_grid(expanded, GridFocusMode::Cell);
            flush_effects();
            focus(&state, Key::cell(&Key::from("cv"), 1));

            expanded.set(HashSet::new());
            flush_effects();
            assert_that!(focused(&state)).is_equal_to(Some(Key::cell(&Key::from("docs"), 1)));
        });
    }

    #[test]
    fn focus_moves_to_the_row_that_took_the_removed_rows_place() {
        with_owner(|| {
            let rows = RwSignal::new(vec!["alice", "bob", "carol"]);
            let state = grid(rows, &[], GridFocusMode::Row);
            flush_effects();
            focus(&state, Key::from("bob"));

            remove(rows, &["bob"]);
            assert_that!(focused(&state)).is_equal_to(Some(Key::from("carol")));
        });
    }

    #[test]
    fn a_removed_cell_moves_focus_to_the_same_column_of_the_next_row() {
        with_owner(|| {
            let rows = RwSignal::new(vec!["alice", "bob", "carol"]);
            let state = grid(rows, &[], GridFocusMode::Cell);
            flush_effects();
            focus(&state, Key::cell(&Key::from("bob"), 1));

            remove(rows, &["bob"]);
            assert_that!(focused(&state)).is_equal_to(Some(Key::cell(&Key::from("carol"), 1)));
        });
    }

    #[test]
    fn removing_the_last_row_moves_focus_back() {
        with_owner(|| {
            let rows = RwSignal::new(vec!["alice", "bob", "carol"]);
            let state = grid(rows, &[], GridFocusMode::Row);
            flush_effects();
            focus(&state, Key::from("carol"));

            remove(rows, &["carol"]);
            assert_that!(focused(&state)).is_equal_to(Some(Key::from("bob")));
        });
    }

    #[test]
    fn disabled_rows_are_skipped() {
        with_owner(|| {
            let rows = RwSignal::new(vec!["alice", "bob", "carol", "dave"]);
            let state = grid(rows, &["carol"], GridFocusMode::Row);
            flush_effects();
            focus(&state, Key::from("bob"));

            remove(rows, &["bob"]);
            assert_that!(focused(&state)).is_equal_to(Some(Key::from("dave")));
        });
    }

    #[test]
    fn removing_several_rows_moves_focus_to_the_first_row_after_them() {
        // react-stately: `diff > 1 ? max(index - diff + 1, 0) : index`.
        with_owner(|| {
            let rows = RwSignal::new(vec!["a", "b", "c", "d", "e"]);
            let state = grid(rows, &[], GridFocusMode::Row);
            flush_effects();
            focus(&state, Key::from("d"));

            remove(rows, &["b", "c", "d"]);
            assert_that!(focused(&state)).is_equal_to(Some(Key::from("e")));
        });
    }

    #[test]
    fn removing_every_row_clears_the_focus() {
        with_owner(|| {
            let rows = RwSignal::new(vec!["alice", "bob"]);
            let state = grid(rows, &[], GridFocusMode::Row);
            flush_effects();
            focus(&state, Key::from("bob"));

            remove(rows, &["alice", "bob"]);
            assert_that!(focused(&state)).is_none();
        });
    }

    /// react-stately's `useGridState` moves focus from a removed column header to the same column
    /// of the nearest body row.
    #[test]
    fn a_removed_column_header_moves_focus_to_the_same_column_of_the_first_row() {
        with_owner(|| {
            let columns = RwSignal::new(vec!["name", "role", "city"]);
            let collection: CollectionMemo = Memo::new(move |_| {
                let columns = columns.get();
                crate::hooks::table::TableCollection::build(|t| {
                    for column in &columns {
                        t.column(*column, *column);
                    }
                    for row in ["alice", "bob"] {
                        t.row(row, row, |r| {
                            for column in &columns {
                                r.cell(*column);
                            }
                        });
                    }
                })
                .collection()
                .clone()
            });
            let state = use_grid_state(UseGridStateInput {
                collection,
                selection: SelectionOptions::default(),
                focus_mode: GridFocusMode::Cell,
            });
            flush_effects();
            focus(&state, Key::from("role"));

            columns.set(vec!["name", "city"]);
            flush_effects();
            assert_that!(focused(&state)).is_equal_to(Some(Key::cell(&Key::from("alice"), 1)));
        });
    }

    #[test]
    fn other_changes_keep_the_focus() {
        with_owner(|| {
            let rows = RwSignal::new(vec!["alice", "bob", "carol"]);
            let state = grid(rows, &[], GridFocusMode::Row);
            flush_effects();
            focus(&state, Key::from("bob"));

            remove(rows, &["alice"]);
            assert_that!(focused(&state)).is_equal_to(Some(Key::from("bob")));
        });
    }
}
