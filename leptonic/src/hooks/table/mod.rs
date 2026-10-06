//! Table hooks: a grid with column headers above its body rows. Build the columns and rows with
//! [`TableCollection::build`], create the state with [`use_table_state`], and render the table
//! with [`use_table`], header rows with [`use_table_header_row`], column headers with
//! [`use_table_column_header`], body rows with [`use_table_row`] and cells with
//! [`use_table_cell`]. Group rows (`<thead>`, `<tbody>`) with `use_grid_row_group`.

mod messages;

/// The rows and columns of a table.
pub mod table_collection;

/// Column widths: the layout of resizable columns.
pub mod table_column_layout;

/// Table navigation: the grid's, plus the column headers.
pub mod table_keyboard_delegate;

/// Column sizes and the flexbox-like algorithm distributing a table's width among its columns.
pub mod table_utils;

/// The table element.
pub mod use_table;

/// A body cell (`gridcell` or `rowheader`).
pub mod use_table_cell;

/// A column header: focus, sorting.
pub mod use_table_column_header;

/// A column's resizer.
pub mod use_table_column_resize;

/// The widths of resizable columns.
pub mod use_table_column_resize_state;

/// A row of column headers, and its placeholders.
pub mod use_table_header_row;

/// A body row.
pub mod use_table_row;

/// Checkboxes selecting a row, or all rows.
pub mod use_table_selection_checkbox;

/// Table state: rows, columns, selection, focus and sorting.
pub mod use_table_state;

pub use table_collection::*;
pub use table_column_layout::{ColumnWidths, DEFAULT_MIN_WIDTH, DefaultMinWidth, DefaultWidth};
pub use table_keyboard_delegate::*;
pub use table_utils::{ColumnBound, ColumnSize, UNBOUNDED_WIDTH};
pub use use_table::*;
pub use use_table_cell::*;
pub use use_table_column_header::*;
pub use use_table_column_resize::*;
pub use use_table_column_resize_state::*;
pub use use_table_header_row::*;
pub use use_table_row::*;
pub use use_table_selection_checkbox::*;
pub use use_table_state::*;
