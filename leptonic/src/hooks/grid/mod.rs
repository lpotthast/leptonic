//! Grid hooks: rows of cells with keyboard navigation in two dimensions. Create the state with
//! [`use_grid_state`] from a collection of rows (`CollectionBuilder::row`), render the grid with
//! [`use_grid`], rows with [`use_grid_row`], cells with [`use_grid_cell`], and optionally group
//! rows with [`use_grid_row_group`]. (Single-column grid lists live in `hooks::gridlist`.)

/// Two-dimensional keyboard navigation (row and cell focus modes).
pub(crate) mod grid_keyboard_delegate;

/// The grid element: keyboard navigation, selection and focus.
pub(crate) mod use_grid;

/// A cell: focus of the cell or its children, arrow keys between children.
pub(crate) mod use_grid_cell;

/// A row: selection and actions.
pub(crate) mod use_grid_row;

/// A group of rows (like `<tbody>`).
pub(crate) mod use_grid_row_group;

/// Announces selection changes (many screen readers don't).
pub(crate) mod use_grid_selection_announcement;

/// A checkbox selecting a row.
pub(crate) mod use_grid_selection_checkbox;

/// Grid state: rows and cells, selection, focus (and refocusing when rows disappear).
pub(crate) mod use_grid_state;

/// How to select items whose press performs their action, on touch devices.
pub(crate) mod use_highlight_selection_description;

pub use grid_keyboard_delegate::*;
pub use use_grid::*;
pub use use_grid_cell::*;
pub use use_grid_row::*;
pub use use_grid_row_group::*;
pub use use_grid_selection_announcement::*;
pub use use_grid_selection_checkbox::*;
pub use use_grid_state::*;
pub use use_highlight_selection_description::*;
