// Upstream: react-aria/src/table/useTableCell.ts @ 99e6102368
// Upstream: react-aria-components/test/Table.test.js @ 99e6102368
use leptos::prelude::*;

use super::TableData;
use crate::{
    PropsWithStyles,
    hooks::{
        collections::Key,
        grid::{
            CellFocusMode, UseGridCellInput, UseGridCellProps, UseGridCellReturn, use_grid_cell,
        },
    },
    utils::aria::AriaRole,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
// - Every cell gets the id `TableData::cell_id(row, column)` (react-aria: row header cells), so
//   that the id stays when the row header columns change.
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

/// Return value of [`use_table_cell`].
pub struct UseTableCellReturn {
    pub grid_cell_props: PropsWithStyles<UseGridCellProps>,
    pub is_pressed: Signal<bool>,
}

/// A body cell of a table. Cells of row header columns get `role="rowheader"`; their ids are
/// what the row's `aria-labelledby` refers to. The cell's role follows the table's row header
/// columns; for a cell moving to another column, create the cell again with its new key.
pub fn use_table_cell(input: UseTableCellInput) -> UseTableCellReturn {
    let UseTableCellInput {
        table,
        key,
        focus_mode,
        allows_arrow_navigation,
        should_select_on_press_up,
    } = input;
    // Every cell gets its row and column's id (react-aria: only row header cells, which label
    // their row), so that a cell becoming a row header keeps its id.
    let (row, column) = table.state.table.with_untracked(|t| {
        (
            t.collection().get(&key).and_then(|n| n.parent_key.clone()),
            t.cell_column(&key).map(|c| c.key.clone()),
        )
    });
    let id = row
        .as_ref()
        .zip(column.as_ref())
        .map(|(row, column)| table.cell_id(row, column));
    let state = table.state;
    let is_row_header = {
        let key = key.clone();
        Memo::new(move |_| {
            state.table.with(|t| {
                t.cell_column(&key)
                    .is_some_and(|column| t.row_header_columns().contains(&column.key))
            })
        })
    };
    let UseGridCellReturn {
        grid_cell_props,
        is_pressed,
    } = use_grid_cell(UseGridCellInput {
        id,
        focus_mode,
        allows_arrow_navigation,
        should_select_on_press_up,
        grid: table.grid,
        key,
    });
    let (mut cell, styles) = grid_cell_props.into_inner();
    cell.role = Signal::derive(move || {
        if is_row_header.get() {
            AriaRole::Rowheader
        } else {
            AriaRole::Gridcell
        }
    });
    UseTableCellReturn {
        grid_cell_props: PropsWithStyles::new(cell, styles),
        is_pressed,
    }
}
