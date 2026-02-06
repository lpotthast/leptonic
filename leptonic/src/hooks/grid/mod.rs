//! Grid hooks for building accessible grid and grid-list patterns.
//!
//! Two usage patterns are supported:
//!
//! **2D Grid** — A multi-column grid with keyboard navigation across rows and cells.
//! Use [`use_grid`] as the container, [`use_grid_row`] for rows,
//! [`use_grid_cell`] for cells, and optionally [`use_grid_row_group`] to group rows.
//!
//! **1D Grid List** — A single-column list with grid semantics (vertical navigation only).
//! Use [`use_grid_list`] as the container and [`use_grid_list_item`] for items.
//!
//! Both patterns delegate selection to `use_selection_state` and share the
//! [`GridCollection`] data structure for O(1) key lookups.

/// Data structure representing the grid's row/cell layout with O(1) key lookups.
pub mod grid_collection;

/// Implements [`KeyboardDelegate`] for 2D grid navigation (row mode and cell mode).
pub mod grid_keyboard_delegate;

/// Container hook for a 2D grid. Manages keyboard navigation, selection, and focus.
/// Produces shared state consumed by [`use_grid_row`], [`use_grid_cell`], and [`use_grid_row_group`].
pub mod use_grid;

/// Cell hook for a 2D grid. Provides selection, focus, and within-cell child navigation.
pub mod use_grid_cell;

/// Container hook for a 1D grid list. Manages vertical navigation, selection, and focus.
/// Produces shared state consumed by [`use_grid_list_item`].
pub mod use_grid_list;

/// Item hook for a 1D grid list. Renders as a row with a single gridcell and handles
/// within-item child navigation.
pub mod use_grid_list_item;

/// Row hook for a 2D grid. Provides selection, focus, and row-level interaction.
pub mod use_grid_row;

/// Structural container for grouping rows (analogous to `<tbody>`). Provides `role="rowgroup"`.
pub mod use_grid_row_group;

pub use grid_collection::*;
pub use grid_keyboard_delegate::*;
pub use use_grid::*;
pub use use_grid_cell::*;
pub use use_grid_list::*;
pub use use_grid_list_item::*;
pub use use_grid_row::*;
pub use use_grid_row_group::*;
