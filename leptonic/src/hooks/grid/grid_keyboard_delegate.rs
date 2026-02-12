use std::{collections::HashSet, hash::Hash};

use leptos::prelude::*;

use super::grid_collection::GridCollection;
use crate::hooks::selection::keyboard_delegate::KeyboardDelegate;

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/grid/src/GridKeyboardDelegate.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
// - Reads signals lazily via `get_untracked()` instead of react-aria's `useMemo`
//   recreation pattern. This is always up-to-date but uses untracked reads in
//   event handlers (appropriate since these run outside the reactive graph).
//
// ## OMITTED FEATURES
// - No RTL support — `get_key_left_of`/`get_key_right_of` don't swap direction
//   based on locale. Can be added later.
// - No `ref` / scrollable element for page-up/page-down calculation.
//
// =============================================================================

/// Controls how focus moves within a grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GridFocusMode {
    /// Arrow up/down navigate between rows. Right enters cells, left exits to row.
    #[default]
    Row,
    /// All arrows navigate cells in 2D.
    Cell,
}

/// A keyboard delegate for 2D grid navigation.
///
/// Implements `KeyboardDelegate<K>` using a `GridCollection<K>` to resolve
/// navigation targets. Reads `Signal`s lazily via `get_untracked()` — the
/// delegate does not need to be recreated when the collection changes.
pub struct GridKeyboardDelegate<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    collection: Signal<GridCollection<K>>,
    disabled_keys: Signal<HashSet<K>>,
    focus_mode: GridFocusMode,
}

// Manual impls avoid requiring K: Copy/Clone for these traits,
// since Signal<T> is Copy regardless of T.
#[allow(clippy::expl_impl_clone_on_copy)]
impl<K: Hash + Eq + Clone + Send + Sync + 'static> Clone for GridKeyboardDelegate<K> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<K: Hash + Eq + Clone + Send + Sync + 'static> Copy for GridKeyboardDelegate<K> {}

impl<K: Hash + Eq + Clone + Send + Sync + 'static> std::fmt::Debug for GridKeyboardDelegate<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GridKeyboardDelegate")
            .field("focus_mode", &self.focus_mode)
            .finish_non_exhaustive()
    }
}

impl<K> GridKeyboardDelegate<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// Create a new grid keyboard delegate.
    #[must_use]
    pub fn new(
        collection: Signal<GridCollection<K>>,
        disabled_keys: Signal<HashSet<K>>,
        focus_mode: GridFocusMode,
    ) -> Self {
        Self {
            collection,
            disabled_keys,
            focus_mode,
        }
    }

    /// Given a row key, find the next non-disabled row after it.
    fn next_non_disabled_row_after(&self, row_key: &K) -> Option<K> {
        let collection = self.collection.get_untracked();
        let disabled = self.disabled_keys.get_untracked();
        let mut current = collection.row_after(row_key).map(|r| r.key.clone());
        while let Some(ref k) = current {
            if !disabled.contains(k) {
                return current;
            }
            current = collection.row_after(k).map(|r| r.key.clone());
        }
        None
    }

    /// Given a row key, find the previous non-disabled row before it.
    fn prev_non_disabled_row_before(&self, row_key: &K) -> Option<K> {
        let collection = self.collection.get_untracked();
        let disabled = self.disabled_keys.get_untracked();
        let mut current = collection.row_before(row_key).map(|r| r.key.clone());
        while let Some(ref k) = current {
            if !disabled.contains(k) {
                return current;
            }
            current = collection.row_before(k).map(|r| r.key.clone());
        }
        None
    }
}

impl<K> KeyboardDelegate<K> for GridKeyboardDelegate<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    fn get_key_below(&self, key: &K) -> Option<K> {
        let collection = self.collection.get_untracked();

        if collection.is_cell_key(key) {
            // Cell: find same column in next non-disabled row.
            let (row, ci) = collection.get_row_for_cell(key)?;
            let row_key = row.key.clone();
            let next_row_key = self.next_non_disabled_row_after(&row_key)?;
            let next_row = collection.get_row(&next_row_key)?;
            // Clamp column index to the next row's cell count.
            let clamped_ci = ci.min(next_row.cells.len().saturating_sub(1));
            next_row.cells.get(clamped_ci).cloned()
        } else if collection.is_row_key(key) {
            // Row: next non-disabled row.
            self.next_non_disabled_row_after(key)
        } else {
            None
        }
    }

    fn get_key_above(&self, key: &K) -> Option<K> {
        let collection = self.collection.get_untracked();

        if collection.is_cell_key(key) {
            let (row, ci) = collection.get_row_for_cell(key)?;
            let row_key = row.key.clone();
            let prev_row_key = self.prev_non_disabled_row_before(&row_key)?;
            let prev_row = collection.get_row(&prev_row_key)?;
            let clamped_ci = ci.min(prev_row.cells.len().saturating_sub(1));
            prev_row.cells.get(clamped_ci).cloned()
        } else if collection.is_row_key(key) {
            self.prev_non_disabled_row_before(key)
        } else {
            None
        }
    }

    fn get_key_right_of(&self, key: &K) -> Option<K> {
        let collection = self.collection.get_untracked();

        if collection.is_row_key(key) {
            // Row mode: enter the row → first cell
            let row = collection.get_row(key)?;
            row.cells.first().cloned()
        } else if collection.is_cell_key(key) {
            let (row, ci) = collection.get_row_for_cell(key)?;
            let next_ci = ci + 1;
            if next_ci < row.cells.len() {
                // Next cell in row.
                Some(row.cells[next_ci].clone())
            } else if self.focus_mode == GridFocusMode::Row {
                // At end of row in Row mode: return to row key.
                Some(row.key.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_key_left_of(&self, key: &K) -> Option<K> {
        let collection = self.collection.get_untracked();

        if collection.is_row_key(key) {
            // Row mode: enter the row → last cell
            let row = collection.get_row(key)?;
            row.cells.last().cloned()
        } else if collection.is_cell_key(key) {
            let (row, ci) = collection.get_row_for_cell(key)?;
            if ci > 0 {
                // Previous cell in row.
                Some(row.cells[ci - 1].clone())
            } else if self.focus_mode == GridFocusMode::Row {
                // At start of row in Row mode: return to row key.
                Some(row.key.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_first_key(&self, from_key: Option<&K>, global: bool) -> Option<K> {
        let collection = self.collection.get_untracked();

        if !global {
            if let Some(fk) = from_key {
                if collection.is_cell_key(fk) {
                    // First cell in the same row.
                    let (row, _) = collection.get_row_for_cell(fk)?;
                    return row.cells.first().cloned();
                }
            }
        }

        // Global first: first row (or first cell if Cell mode).
        let first_row = collection.first_row()?;
        if self.focus_mode == GridFocusMode::Cell {
            first_row.cells.first().cloned()
        } else {
            Some(first_row.key.clone())
        }
    }

    fn get_last_key(&self, from_key: Option<&K>, global: bool) -> Option<K> {
        let collection = self.collection.get_untracked();

        if !global {
            if let Some(fk) = from_key {
                if collection.is_cell_key(fk) {
                    // Last cell in the same row.
                    let (row, _) = collection.get_row_for_cell(fk)?;
                    return row.cells.last().cloned();
                }
            }
        }

        // Global last: last row (or last cell if Cell mode).
        let last_row = collection.last_row()?;
        if self.focus_mode == GridFocusMode::Cell {
            last_row.cells.last().cloned()
        } else {
            Some(last_row.key.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::hooks::grid::grid_collection::GridRow;

    fn make_delegate(focus_mode: GridFocusMode) -> GridKeyboardDelegate<String> {
        let collection = GridCollection::new(vec![
            GridRow {
                key: "row-0".into(),
                cells: vec!["0-0".into(), "0-1".into(), "0-2".into()],
            },
            GridRow {
                key: "row-1".into(),
                cells: vec!["1-0".into(), "1-1".into(), "1-2".into()],
            },
            GridRow {
                key: "row-2".into(),
                cells: vec!["2-0".into(), "2-1".into()],
            },
        ]);

        // Note: these tests use Signal::stored which requires a reactive runtime.
        // For unit tests we'll test the collection logic directly via the methods.
        // Full integration tests would need a Leptos runtime.
        GridKeyboardDelegate {
            collection: Signal::stored(collection),
            disabled_keys: Signal::stored(HashSet::new()),
            focus_mode,
        }
    }

    #[test]
    fn cell_navigation_down() {
        let _owner = Owner::new();
        let d = make_delegate(GridFocusMode::Cell);

        assert_that(d.get_key_below(&"0-1".into()))
            .is_some()
            .is_equal_to("1-1");

        // At bottom-right, should clamp column
        assert_that(d.get_key_below(&"1-2".into()))
            .is_some()
            .is_equal_to("2-1");

        // Last row cell, no row below
        assert_that(d.get_key_below(&"2-0".into())).is_none();
    }

    #[test]
    fn cell_navigation_up() {
        let _owner = Owner::new();
        let d = make_delegate(GridFocusMode::Cell);

        assert_that(d.get_key_above(&"1-1".into()))
            .is_some()
            .is_equal_to("0-1");

        // First row, no row above
        assert_that(d.get_key_above(&"0-0".into())).is_none();
    }

    #[test]
    fn cell_navigation_right_left() {
        let _owner = Owner::new();
        let d = make_delegate(GridFocusMode::Cell);

        assert_that(d.get_key_right_of(&"0-0".into()))
            .is_some()
            .is_equal_to("0-1");

        // End of row in Cell mode: None
        assert_that(d.get_key_right_of(&"0-2".into())).is_none();

        assert_that(d.get_key_left_of(&"0-1".into()))
            .is_some()
            .is_equal_to("0-0");

        // Start of row in Cell mode: None
        assert_that(d.get_key_left_of(&"0-0".into())).is_none();
    }

    #[test]
    fn row_mode_right_enters_row() {
        let _owner = Owner::new();
        let d = make_delegate(GridFocusMode::Row);

        // Right from row key → first cell
        assert_that(d.get_key_right_of(&"row-0".into()))
            .is_some()
            .is_equal_to("0-0");

        // At end of row in Row mode → back to row key
        assert_that(d.get_key_right_of(&"0-2".into()))
            .is_some()
            .is_equal_to("row-0");
    }

    #[test]
    fn row_mode_left_enters_row() {
        let _owner = Owner::new();
        let d = make_delegate(GridFocusMode::Row);

        // Left from row key → last cell
        assert_that(d.get_key_left_of(&"row-0".into()))
            .is_some()
            .is_equal_to("0-2");

        // At start of row in Row mode → back to row key
        assert_that(d.get_key_left_of(&"0-0".into()))
            .is_some()
            .is_equal_to("row-0");
    }

    #[test]
    fn first_key_and_last_key() {
        let _owner = Owner::new();

        let d_cell = make_delegate(GridFocusMode::Cell);
        // Global first in Cell mode → first cell
        assert_that(d_cell.get_first_key(None, true))
            .is_some()
            .is_equal_to("0-0");

        // From a cell, non-global → first cell in same row
        assert_that(d_cell.get_first_key(Some(&"1-2".into()), false))
            .is_some()
            .is_equal_to("1-0");

        // Global last in Cell mode → last cell of last row
        assert_that(d_cell.get_last_key(None, true))
            .is_some()
            .is_equal_to("2-1");

        let d_row = make_delegate(GridFocusMode::Row);
        // Global first in Row mode → first row key
        assert_that(d_row.get_first_key(None, true))
            .is_some()
            .is_equal_to("row-0");

        // Global last in Row mode → last row key
        assert_that(d_row.get_last_key(None, true))
            .is_some()
            .is_equal_to("row-2");
    }

    #[test]
    fn row_navigation_up_down() {
        let _owner = Owner::new();
        let d = make_delegate(GridFocusMode::Row);

        assert_that(d.get_key_below(&"row-0".into()))
            .is_some()
            .is_equal_to("row-1");

        assert_that(d.get_key_above(&"row-2".into()))
            .is_some()
            .is_equal_to("row-1");

        assert_that(d.get_key_below(&"row-2".into())).is_none();
        assert_that(d.get_key_above(&"row-0".into())).is_none();
    }
}
