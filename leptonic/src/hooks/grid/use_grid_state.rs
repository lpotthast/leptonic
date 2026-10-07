// Upstream: react-stately/src/grid/useGridState.ts @ 99e6102368
use std::sync::Arc;

use leptos::prelude::*;

use super::GridFocusMode;
use crate::hooks::collections::{
    Collection, CollectionMemo, ItemElements, Key, ListState, NodeKind, SelectionManager,
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
    /// While `true` (e.g. while resizing a table column), the grid doesn't handle navigation
    /// keys.
    pub is_keyboard_navigation_disabled: RwSignal<bool>,
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
    // place (or the closest one before), to the same column.
    Effect::new(move |previous: Option<Arc<Collection>>| {
        let current = collection.get();
        let focused = untrack(|| manager.focused_key());
        if let (Some(previous), Some(focused)) = (&previous, focused)
            && !current.contains_key(&focused)
        {
            let next = untrack(|| refocus(previous, &current, &focused, manager));
            manager.set_focused_key(next, None);
        }
        current
    });

    GridState {
        list: ListState {
            collection,
            selection: manager,
            item_elements: ItemElements::new(),
        },
        focus_mode,
        is_keyboard_navigation_disabled: RwSignal::new(false),
    }
}

/// The key to focus after the focused `key` was removed.
fn refocus(
    previous: &Collection,
    current: &Collection,
    key: &Key,
    manager: SelectionManager,
) -> Option<Key> {
    let node = previous.get(key)?;
    let row = if matches!(node.kind, NodeKind::Cell) {
        previous.get(node.parent_key.as_ref()?)?
    } else {
        node
    };
    let previous_rows: Vec<&Key> = previous.items().map(|n| &n.key).collect();
    let rows: Vec<Key> = current.items().map(|n| n.key.clone()).collect();
    if rows.is_empty() {
        return None;
    }
    let row_index = previous_rows.iter().position(|k| **k == row.key)?;
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
    if node.kind == NodeKind::Cell {
        let cells: Vec<Key> = current.children(new_row).map(|n| n.key.clone()).collect();
        if let Some(cell) = cells.get(node.index) {
            return Some(cell.clone());
        }
    }
    Some(new_row.clone())
}
