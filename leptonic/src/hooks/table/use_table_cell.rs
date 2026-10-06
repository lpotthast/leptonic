// Upstream: react-aria/src/table/useTableCell.ts @ 99e6102368
use leptos::prelude::*;

use super::TableData;
use crate::{
    hooks::{
        CellFocusMode, PropsWithStyles, UseGridCellInput, UseGridCellProps, UseGridCellReturn,
        collections::Key, use_grid_cell,
    },
    utils::aria::AriaRole,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input of [`use_table_cell`].
#[derive(Debug, Clone)]
pub struct UseTableCellInput {
    /// The table (from `use_table`).
    pub table: TableData,
    /// The cell's key (`Key::cell(row, column_index)`).
    pub key: Key,
    /// See `UseGridCellInput::focus_mode`.
    pub focus_mode: Option<CellFocusMode>,
    /// See `UseGridCellInput::allows_arrow_navigation`.
    pub allows_arrow_navigation: bool,
    /// Select when the press ends instead of when it starts.
    pub should_select_on_press_up: bool,
}

impl UseTableCellInput {
    pub fn new(table: TableData, key: Key) -> Self {
        Self {
            table,
            key,
            focus_mode: None,
            allows_arrow_navigation: false,
            should_select_on_press_up: false,
        }
    }
}

/// Return value of [`use_table_cell`].
pub struct UseTableCellReturn {
    pub grid_cell_props: PropsWithStyles<UseGridCellProps>,
    pub is_pressed: Signal<bool>,
}

/// A body cell of a table. Cells of row header columns get `role="rowheader"` and the id the
/// row's `aria-labelledby` refers to.
pub fn use_table_cell(input: UseTableCellInput) -> UseTableCellReturn {
    let UseTableCellInput {
        table,
        key,
        focus_mode,
        allows_arrow_navigation,
        should_select_on_press_up,
    } = input;
    let row_header = table.state.table.with_untracked(|t| {
        let column = t.cell_column(&key)?;
        let row = t.collection().get(&key)?.parent_key.clone()?;
        t.row_header_columns()
            .contains(&column.key)
            .then(|| (row, column.key.clone()))
    });
    let id = row_header
        .as_ref()
        .map(|(row, column)| table.cell_id(row, column));
    let UseGridCellReturn {
        grid_cell_props,
        is_pressed,
    } = use_grid_cell(UseGridCellInput {
        id,
        focus_mode,
        allows_arrow_navigation,
        should_select_on_press_up,
        ..UseGridCellInput::new(table.grid, key)
    });
    let (mut cell, styles) = grid_cell_props.into_inner();
    if row_header.is_some() {
        cell.role = AriaRole::Rowheader;
    }
    UseTableCellReturn {
        grid_cell_props: PropsWithStyles::new(cell, styles),
        is_pressed,
    }
}
