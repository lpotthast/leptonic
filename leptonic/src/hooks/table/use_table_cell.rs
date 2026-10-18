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
    utils::{aria::AriaRole, part::Part},
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
    /// The cell's key: `Key::cell(row, column)` with its column's key (see
    /// `TableBuilder::row`).
    pub key: Key,
    /// See `UseGridCellInput::focus_mode`.
    pub focus_mode: Option<CellFocusMode>,
    /// See `UseGridCellInput::allows_arrow_navigation`.
    pub allows_arrow_navigation: bool,
    /// Select when the press ends instead of when it starts.
    pub should_select_on_press_up: bool,
}

/// The return value of [`use_table_cell`].
#[derive(Debug)]
pub struct UseTableCellReturn {
    /// The cell element: `let (attrs, styles) = cell.attrs();`.
    pub cell: Part<PropsWithStyles<UseGridCellProps>>,
    pub is_pressed: Signal<bool>,
}

/// A body cell of a table. Cells of row header columns get `role="rowheader"`; their ids are
/// what the row's `aria-labelledby` refers to. The cell's role follows the table's row header
/// columns.
pub fn use_table_cell(input: UseTableCellInput) -> UseTableCellReturn {
    crate::metrics::subject!();
    let UseTableCellInput {
        table,
        key,
        focus_mode,
        allows_arrow_navigation,
        should_select_on_press_up,
    } = input;
    // The cell's row and column are in its key. Every cell gets its row and column's id
    // (react-aria: only row header cells, which label their row), so that a cell becoming a
    // row header keeps its id.
    let id = key
        .as_cell()
        .map(|(row, column)| table.cell_id(row, column));
    let column = key.as_cell().map(|(_, column)| column.clone());
    // Only the row header columns, not the rows (`performance.md` R1).
    let row_header_columns = table.state.row_header_columns;
    let is_row_header = move || {
        column
            .as_ref()
            .is_some_and(|column| row_header_columns.with(|columns| columns.contains(column)))
    };
    let UseGridCellReturn {
        cell: grid_cell_props,
        is_pressed,
    } = use_grid_cell(UseGridCellInput {
        id,
        focus_mode,
        allows_arrow_navigation,
        should_select_on_press_up,
        grid: table.grid,
        key,
    });
    let (mut cell, styles) = grid_cell_props.into_props().into_inner();
    cell.role = Signal::derive(move || {
        if is_row_header() {
            AriaRole::Rowheader
        } else {
            AriaRole::Gridcell
        }
    });
    UseTableCellReturn {
        cell: Part::stored(PropsWithStyles::new(cell, styles)),
        is_pressed,
    }
}
