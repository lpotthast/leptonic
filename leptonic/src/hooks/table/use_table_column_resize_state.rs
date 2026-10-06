// Upstream: react-stately/src/table/useTableColumnResizeState.ts @ 99e6102368
use std::{collections::HashMap, sync::Arc};

use leptos::prelude::*;

use super::{
    table_column_layout::{
        ColumnWidths, DEFAULT_MIN_WIDTH, DefaultMinWidth, DefaultWidth, build_column_widths,
        initial_width, resize_column_width,
    },
    table_utils::ColumnSize,
    use_table_state::TableState,
};
use crate::hooks::collections::Key;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned state: no controlled column widths (react-aria's `width` column props, which
//   parents update from `onResize`). Instead, the state remembers resized widths across column
//   changes: a column keeps its width when other columns are added or removed, and gets it back
//   when it is removed and added again. (React-aria's uncontrolled widths reset when the columns
//   change; its controlled widths behave like ours.)
// - Column widths are read from the computed `column_widths` signal instead of getters backed by
//   a mutable layout object.
//
// =============================================================================

/// Input of [`use_table_column_resize_state`].
#[derive(Clone)]
pub struct UseTableColumnResizeStateInput {
    pub table_state: TableState,
    /// The width available to the columns, in pixels (e.g. of the table's scroll container).
    pub table_width: Signal<f64>,
    /// The width of columns without a `default_width`. Defaults to `ColumnSize::Fr(1.0)`.
    pub default_width: Option<Arc<DefaultWidth>>,
    /// The minimum width of columns without a `min_width`. Defaults to [`DEFAULT_MIN_WIDTH`].
    pub default_min_width: Option<Arc<DefaultMinWidth>>,
}

impl UseTableColumnResizeStateInput {
    pub fn new(table_state: TableState, table_width: Signal<f64>) -> Self {
        Self {
            table_state,
            table_width,
            default_width: None,
            default_min_width: None,
        }
    }
}

impl std::fmt::Debug for UseTableColumnResizeStateInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseTableColumnResizeStateInput")
            .finish_non_exhaustive()
    }
}

/// The widths of a table's columns, and which column is being resized.
#[derive(Debug, Clone, Copy)]
pub struct TableColumnResizeState {
    pub table_state: TableState,
    /// The column being resized.
    pub resizing_column: Signal<Option<Key>>,
    /// The columns' pixel widths and bounds.
    pub column_widths: Signal<ColumnWidths>,
    set_resizing_column: WriteSignal<Option<Key>>,
    /// The current size of every column.
    sizes: Memo<HashMap<Key, ColumnSize>>,
    /// The sizes of resized columns (and the columns next to them), by column.
    set_resized: WriteSignal<HashMap<Key, ColumnSize>>,
}

impl TableColumnResizeState {
    /// Resize `column` to `width` pixels (within its bounds). Returns the new size of every
    /// column: the columns before it keep their current pixel widths, the columns after it keep
    /// their sizes.
    pub fn update_resized_columns(&self, column: &Key, width: f64) -> HashMap<Key, ColumnSize> {
        let new_sizes = untrack(|| {
            self.table_state.table.with(|table| {
                self.column_widths.with(|current| {
                    self.sizes
                        .with(|sizes| resize_column_width(table, current, sizes, column, width))
                })
            })
        });
        self.set_resized
            .update(|resized| resized.extend(new_sizes.clone()));
        new_sizes
    }

    /// Mark `column` as being resized.
    pub fn start_resize(&self, column: Key) {
        self.set_resizing_column.set(Some(column));
    }

    /// Mark the resizing as finished.
    pub fn end_resize(&self) {
        self.set_resizing_column.set(None);
    }

    /// The width of `column` in pixels.
    pub fn column_width(&self, column: &Key) -> f64 {
        self.column_widths.with(|w| w.width(column))
    }

    /// The minimum width of `column` in pixels.
    pub fn column_min_width(&self, column: &Key) -> f64 {
        self.column_widths.with(|w| w.min_width(column))
    }

    /// The maximum width of `column` in pixels.
    pub fn column_max_width(&self, column: &Key) -> f64 {
        self.column_widths.with(|w| w.max_width(column))
    }
}

/// Creates the state of resizable table columns: their widths in a table `table_width` wide.
/// Columns are sized by their builder's `default_width`, `min_width` and `max_width` until
/// they are resized.
pub fn use_table_column_resize_state(
    input: UseTableColumnResizeStateInput,
) -> TableColumnResizeState {
    let UseTableColumnResizeStateInput {
        table_state,
        table_width,
        default_width,
        default_min_width,
    } = input;
    let default_width: Arc<DefaultWidth> = default_width.unwrap_or_else(|| Arc::new(|_| None));
    let default_min_width: Arc<DefaultMinWidth> =
        default_min_width.unwrap_or_else(|| Arc::new(|_| Some(DEFAULT_MIN_WIDTH)));

    let (resizing_column, set_resizing_column) = signal(None);
    let (resized, set_resized) = signal(HashMap::<Key, ColumnSize>::new());
    let table = table_state.table;

    let sizes = {
        let default_width = default_width.clone();
        Memo::new(move |_| {
            table.with(|t| {
                resized.with(|resized| {
                    t.columns()
                        .map(|column| {
                            let size = resized
                                .get(&column.key)
                                .copied()
                                .unwrap_or_else(|| initial_width(column, &*default_width));
                            (column.key.clone(), size)
                        })
                        .collect()
                })
            })
        })
    };
    let column_widths = Memo::new(move |_| {
        let table_width = table_width.get();
        table.with(|t| {
            sizes.with(|sizes| {
                build_column_widths(table_width, t, sizes, &*default_width, &*default_min_width)
            })
        })
    });

    TableColumnResizeState {
        table_state,
        resizing_column: resizing_column.into(),
        column_widths: column_widths.into(),
        set_resizing_column,
        sizes,
        set_resized,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::hooks::{TableCollection, UseTableStateInput, use_table_state};

    fn widths(state: &TableColumnResizeState, keys: &[&str]) -> Vec<f64> {
        keys.iter()
            .map(|k| state.column_width(&Key::from(*k)))
            .collect()
    }

    #[test]
    fn keeps_resized_widths_when_columns_change() {
        Owner::new().with(|| {
            let with_type = RwSignal::new(true);
            let table = Memo::new(move |_| {
                let with_type = with_type.get();
                Arc::new(TableCollection::build(|t| {
                    t.column("name", "Name");
                    if with_type {
                        t.column("type", "Type");
                    }
                    t.column("level", "Level")
                        .default_width(ColumnSize::Fr(4.0));
                }))
            });
            let table_state = use_table_state(UseTableStateInput::new(table));
            let state = use_table_column_resize_state(UseTableColumnResizeStateInput::new(
                table_state,
                Signal::stored(600.0),
            ));
            assert_that!(widths(&state, &["name", "type", "level"]))
                .is_equal_to(vec![100.0, 100.0, 400.0]);

            let sizes = state.update_resized_columns(&Key::from("type"), 150.0);
            assert_that!(sizes.get(&Key::from("name")).copied())
                .is_equal_to(Some(ColumnSize::Px(100.0)));
            assert_that!(sizes.get(&Key::from("level")).copied())
                .is_equal_to(Some(ColumnSize::Fr(4.0)));
            assert_that!(widths(&state, &["name", "type", "level"]))
                .is_equal_to(vec![100.0, 150.0, 350.0]);

            with_type.set(false);
            assert_that!(widths(&state, &["name", "level"])).is_equal_to(vec![100.0, 500.0]);
            with_type.set(true);
            assert_that!(widths(&state, &["name", "type", "level"]))
                .is_equal_to(vec![100.0, 150.0, 350.0]);
        });
    }

    #[test]
    fn clamps_to_the_column_bounds() {
        Owner::new().with(|| {
            let table = Memo::new(move |_| {
                Arc::new(TableCollection::build(|t| {
                    t.column("name", "Name");
                    t.column("level", "Level");
                }))
            });
            let table_state = use_table_state(UseTableStateInput::new(table));
            let state = use_table_column_resize_state(UseTableColumnResizeStateInput::new(
                table_state,
                Signal::stored(400.0),
            ));
            state.update_resized_columns(&Key::from("name"), 10.0);
            assert_that!(widths(&state, &["name", "level"])).is_equal_to(vec![75.0, 325.0]);
            assert_that!(state.column_min_width(&Key::from("name"))).is_equal_to(75.0);
        });
    }
}
