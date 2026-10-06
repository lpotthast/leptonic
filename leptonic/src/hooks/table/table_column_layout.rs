// Upstream: react-stately/src/table/TableColumnLayout.ts @ 99e6102368
use std::collections::HashMap;

use super::{
    table_collection::{Column, TableCollection},
    table_utils::{
        ColumnBound, ColumnSize, ColumnSizing, calculate_column_sizes, max_width, min_width,
    },
};
use crate::hooks::collections::Key;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Functions over an immutable `ColumnWidths` instead of a class that caches the last
//   computed widths in mutable fields.
// - No controlled column widths (`width` props): every column's width is state (see
//   `use_table_column_resize_state`).
//
// =============================================================================

/// The default width of a column without one: what the column's builder or the table says.
pub type DefaultWidth = dyn Fn(&Column) -> Option<ColumnSize> + Send + Sync;

/// The default minimum width of a column without one.
pub type DefaultMinWidth = dyn Fn(&Column) -> Option<ColumnBound> + Send + Sync;

/// The minimum width of columns that don't specify one (react-aria's default).
pub const DEFAULT_MIN_WIDTH: ColumnBound = ColumnBound::Px(75.0);

/// The computed pixel widths of a table's columns, with their bounds.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ColumnWidths {
    widths: HashMap<Key, f64>,
    min_widths: HashMap<Key, f64>,
    max_widths: HashMap<Key, f64>,
}

impl ColumnWidths {
    /// The width of `column` in pixels (0 for an unknown column).
    pub fn width(&self, column: &Key) -> f64 {
        self.widths.get(column).copied().unwrap_or_default()
    }

    /// The minimum width of `column` in pixels.
    pub fn min_width(&self, column: &Key) -> f64 {
        self.min_widths.get(column).copied().unwrap_or_default()
    }

    /// The maximum width of `column` in pixels.
    pub fn max_width(&self, column: &Key) -> f64 {
        self.max_widths.get(column).copied().unwrap_or_default()
    }

    /// The pixel widths of all columns.
    pub fn widths(&self) -> &HashMap<Key, f64> {
        &self.widths
    }
}

/// The width of `column` before it is resized: its own default width, else `default_width`'s,
/// else `1fr`.
pub(crate) fn initial_width(column: &Column, default_width: &DefaultWidth) -> ColumnSize {
    column
        .default_width
        .or_else(|| default_width(column))
        .unwrap_or_default()
}

/// The pixel widths of the data columns of `table` in a table `table_width` wide, for the column
/// sizes `widths`.
pub(crate) fn build_column_widths(
    table_width: f64,
    table: &TableCollection,
    widths: &HashMap<Key, ColumnSize>,
    default_width: &DefaultWidth,
    default_min_width: &DefaultMinWidth,
) -> ColumnWidths {
    let columns: Vec<&Column> = table.columns().collect();
    let sizing: Vec<ColumnSizing> = columns
        .iter()
        .map(|column| ColumnSizing {
            key: column.key.clone(),
            default_width: column.default_width,
            min_width: column.min_width,
            max_width: column.max_width,
        })
        .collect();
    let sizes = calculate_column_sizes(
        table_width,
        &sizing,
        widths,
        |index| default_width(columns[index]),
        |index| default_min_width(columns[index]),
    );

    let mut result = ColumnWidths::default();
    for (column, width) in columns.iter().zip(sizes) {
        let key = column.key.clone();
        result.widths.insert(key.clone(), width);
        result.min_widths.insert(
            key.clone(),
            min_width(
                column.min_width.or_else(|| default_min_width(column)),
                table_width,
            ),
        );
        result
            .max_widths
            .insert(key, max_width(column.max_width, table_width));
    }
    result
}

/// The column sizes after resizing `column` to `width` (within its bounds): the columns before it
/// keep their current pixel widths, the columns after it keep their sizes from `widths` (so
/// fractional columns share what is left).
pub(crate) fn resize_column_width(
    table: &TableCollection,
    current: &ColumnWidths,
    widths: &HashMap<Key, ColumnSize>,
    column: &Key,
    width: f64,
) -> HashMap<Key, ColumnSize> {
    let width = current
        .min_width(column)
        .max(current.max_width(column).min(width.floor()));
    let mut freeze = true;
    table
        .columns()
        .map(|c| {
            let size = if c.key == *column {
                freeze = false;
                ColumnSize::Px(width)
            } else if freeze {
                ColumnSize::Px(current.width(&c.key))
            } else {
                widths.get(&c.key).copied().unwrap_or_default()
            };
            (c.key.clone(), size)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn table(columns: &[(&str, Option<ColumnSize>)]) -> TableCollection {
        TableCollection::build(|t| {
            for (key, width) in columns {
                let column = t.column(*key, *key);
                if let Some(width) = width {
                    column.default_width(*width);
                }
            }
        })
    }

    fn sizes(widths: &[(&str, ColumnSize)]) -> HashMap<Key, ColumnSize> {
        widths.iter().map(|(k, w)| (Key::from(*k), *w)).collect()
    }

    fn pixels(widths: &ColumnWidths, keys: &[&str]) -> Vec<f64> {
        keys.iter().map(|k| widths.width(&Key::from(*k))).collect()
    }

    const KEYS: [&str; 5] = ["name", "type", "height", "weight", "level"];

    #[test]
    fn generates_column_widths_with_defaults_if_none_are_provided() {
        let table = table(&KEYS.map(|k| (k, None)));
        let widths = build_column_widths(1000.0, &table, &HashMap::new(), &|_| None, &|_| {
            Some(DEFAULT_MIN_WIDTH)
        });
        assert_that!(pixels(&widths, &KEYS)).is_equal_to(vec![200.0; 5]);
    }

    #[test]
    fn can_resize_columns() {
        let px = ColumnSize::Px;
        let fr = ColumnSize::Fr;
        let table = table(&[
            ("name", Some(fr(1.0))),
            ("type", Some(fr(1.0))),
            ("height", None),
            ("weight", None),
            ("level", Some(fr(5.0))),
        ]);
        let default_width = |_: &Column| Some(ColumnSize::Px(150.0));
        let default_min_width = |_: &Column| Some(ColumnBound::Px(50.0));
        let build = |widths: &HashMap<Key, ColumnSize>| {
            build_column_widths(1000.0, &table, widths, &default_width, &default_min_width)
        };

        let state = sizes(&[
            ("name", fr(1.0)),
            ("type", fr(1.0)),
            ("height", px(150.0)),
            ("weight", px(150.0)),
            ("level", fr(5.0)),
        ]);
        let widths = build(&state);
        assert_that!(pixels(&widths, &KEYS)).is_equal_to(vec![100.0, 100.0, 150.0, 150.0, 500.0]);

        let state = resize_column_width(&table, &widths, &state, &Key::from("height"), 200.0);
        assert_that!(state.clone()).is_equal_to(sizes(&[
            ("name", px(100.0)),
            ("type", px(100.0)),
            ("height", px(200.0)),
            ("weight", px(150.0)),
            ("level", fr(5.0)),
        ]));
        let widths = build(&state);
        assert_that!(pixels(&widths, &KEYS)).is_equal_to(vec![100.0, 100.0, 200.0, 150.0, 450.0]);

        let state = resize_column_width(&table, &widths, &state, &Key::from("type"), 50.0);
        assert_that!(state.clone()).is_equal_to(sizes(&[
            ("name", px(100.0)),
            ("type", px(50.0)),
            ("height", px(200.0)),
            ("weight", px(150.0)),
            ("level", fr(5.0)),
        ]));
        let widths = build(&state);
        assert_that!(pixels(&widths, &KEYS)).is_equal_to(vec![100.0, 50.0, 200.0, 150.0, 500.0]);
    }

    #[test]
    fn can_resize_to_bigger_than_the_table() {
        let px = ColumnSize::Px;
        let fr = ColumnSize::Fr;
        let table = table(&[
            ("name", Some(fr(1.0))),
            ("type", Some(fr(1.0))),
            ("height", None),
            ("weight", None),
            ("level", Some(fr(5.0))),
        ]);
        let default_width = |_: &Column| Some(ColumnSize::Px(150.0));
        let default_min_width = |_: &Column| Some(ColumnBound::Px(50.0));
        let state = sizes(&[
            ("name", fr(1.0)),
            ("type", fr(1.0)),
            ("height", px(150.0)),
            ("weight", px(150.0)),
            ("level", fr(5.0)),
        ]);
        let widths =
            build_column_widths(1000.0, &table, &state, &default_width, &default_min_width);
        let state = resize_column_width(&table, &widths, &state, &Key::from("height"), 1000.0);
        assert_that!(state.get(&Key::from("height")).copied()).is_equal_to(Some(px(1000.0)));
        let widths =
            build_column_widths(1000.0, &table, &state, &default_width, &default_min_width);
        assert_that!(pixels(&widths, &KEYS)).is_equal_to(vec![100.0, 100.0, 1000.0, 150.0, 50.0]);
    }
}
