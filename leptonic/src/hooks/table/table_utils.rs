// Upstream: react-stately/src/table/TableUtils.ts @ 99e6102368
use crate::hooks::collections::Key;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Column sizes are enums (`ColumnSize`, `ColumnBound`) instead of numbers and strings
//   (`"50%"`, `"2fr"`), so invalid sizes can't be expressed.
//
// =============================================================================

/// The width of a column: fixed, a share of the table width, or a fraction of the remaining
/// space (like CSS grid's `fr`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColumnSize {
    /// Pixels.
    Px(f64),
    /// Percent of the table width.
    Percent(f64),
    /// A share of the space left by the fixed columns.
    Fr(f64),
}

impl Default for ColumnSize {
    fn default() -> Self {
        Self::Fr(1.0)
    }
}

/// A minimum or maximum column width.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColumnBound {
    /// Pixels.
    Px(f64),
    /// Percent of the table width.
    Percent(f64),
}

impl ColumnBound {
    /// The bound in pixels for a table `table_width` wide.
    pub fn resolve(self, table_width: f64) -> f64 {
        match self {
            Self::Px(px) => px,
            Self::Percent(percent) => table_width * (percent / 100.0),
        }
    }
}

/// The sizes of a column for [`calculate_column_sizes`].
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnSizing {
    pub key: Key,
    /// The column's initial width.
    pub default_width: Option<ColumnSize>,
    pub min_width: Option<ColumnBound>,
    pub max_width: Option<ColumnBound>,
}

/// The maximum width of an unbounded column (JavaScript's `Number.MAX_SAFE_INTEGER`, as in
/// react-aria; it is what the resizer input reports as its `max`).
pub const UNBOUNDED_WIDTH: f64 = 9_007_199_254_740_991.0;

/// The maximum width of a column: `max_width`, or [`UNBOUNDED_WIDTH`].
pub fn max_width(max_width: Option<ColumnBound>, table_width: f64) -> f64 {
    max_width.map_or(UNBOUNDED_WIDTH, |bound| bound.resolve(table_width))
}

/// The minimum width of a column: `min_width`, or 0.
pub fn min_width(min_width: Option<ColumnBound>, table_width: f64) -> f64 {
    min_width.map_or(0.0, |bound| bound.resolve(table_width))
}

#[derive(Debug, Clone, Copy)]
struct FlexItem {
    frozen: bool,
    base_size: f64,
    min: f64,
    max: f64,
    flex: f64,
    target_main_size: f64,
    violation: f64,
}

/// The widths of `columns` in a table `available_width` wide, following CSS flexbox's
/// algorithm: fixed and percentage widths first, the remaining space shared by the fractional
/// widths, all within each column's minimum and maximum. `changed_widths` (e.g. from
/// resizing) take precedence over the columns' own widths; `default_width` and
/// `default_min_width` fill in what a column doesn't specify. The widths are whole pixels that
/// sum up to the table width.
#[allow(clippy::implicit_hasher)]
pub fn calculate_column_sizes(
    available_width: f64,
    columns: &[ColumnSizing],
    changed_widths: &std::collections::HashMap<Key, ColumnSize>,
    default_width: impl Fn(usize) -> Option<ColumnSize>,
    default_min_width: impl Fn(usize) -> Option<ColumnBound>,
) -> Vec<f64> {
    let original_width = available_width;
    let available_width = available_width.floor();
    let has_fractional_width = original_width - available_width > 0.0;
    let mut has_non_frozen_items = false;

    let mut items: Vec<FlexItem> = columns
        .iter()
        .enumerate()
        .map(|(index, column)| {
            let width = changed_widths
                .get(&column.key)
                .copied()
                .or(column.default_width)
                .or_else(|| default_width(index))
                .unwrap_or_default();
            let (mut frozen, base_size, flex) = match width {
                ColumnSize::Px(px) => (true, px, 0.0),
                ColumnSize::Percent(percent) => (true, available_width * (percent / 100.0), 0.0),
                ColumnSize::Fr(fr) => (fr <= 0.0, 0.0, fr),
            };
            let min = min_width(
                column.min_width.or_else(|| default_min_width(index)),
                available_width,
            );
            let max = max_width(column.max_width, available_width);
            let hypothetical_main_size = min.max(base_size.min(max));
            let mut target_main_size = 0.0;
            if frozen {
                target_main_size = hypothetical_main_size;
            } else if base_size > hypothetical_main_size {
                frozen = true;
                target_main_size = hypothetical_main_size;
            }
            if !frozen {
                has_non_frozen_items = true;
            }
            FlexItem {
                frozen,
                base_size,
                min,
                max,
                flex,
                target_main_size,
                violation: 0.0,
            }
        })
        .collect();

    while has_non_frozen_items {
        let mut used_width = 0.0;
        let mut flex_factors = 0.0;
        for item in &items {
            if item.frozen {
                used_width += item.target_main_size;
            } else {
                used_width += item.base_size;
                flex_factors += item.flex;
            }
        }
        let remaining_free_space = available_width - used_width;
        if remaining_free_space > 0.0 {
            for item in items.iter_mut().filter(|i| !i.frozen) {
                item.target_main_size =
                    item.base_size + item.flex / flex_factors * remaining_free_space;
            }
        }

        let mut total_violation = 0.0;
        for item in &mut items {
            item.violation = 0.0;
            if !item.frozen {
                let clamped = item.min.max(item.target_main_size.min(item.max));
                item.violation = clamped - item.target_main_size;
                item.target_main_size = clamped;
                total_violation += item.violation;
            }
        }

        has_non_frozen_items = false;
        for item in &mut items {
            #[allow(clippy::float_cmp)]
            let same_sign = total_violation.signum() == item.violation.signum();
            if total_violation == 0.0 || same_sign {
                item.frozen = true;
            } else if !item.frozen {
                has_non_frozen_items = true;
            }
        }
    }

    let mut sizes = cascade_rounding(&items);
    // Keep the columns flush with a fractional table width: the last column gets the fraction.
    // (Spliced as digits, as react-aria does, to avoid floating point noise.)
    if has_fractional_width && let Some(last) = sizes.last_mut() {
        let fraction = original_width.to_string();
        if let Some((_, digits)) = fraction.split_once('.')
            && let Ok(width) = format!("{last}.{digits}").parse()
        {
            *last = width;
        }
    }
    sizes
}

/// Round floats that sum up to an integer to integers with the same sum.
fn cascade_rounding(items: &[FlexItem]) -> Vec<f64> {
    let mut fp_total = 0.0;
    let mut int_total = 0.0;
    items
        .iter()
        .map(|item| {
            let float = item.target_main_size;
            let integer = (float + fp_total).round() - int_total;
            fp_total += float;
            int_total += integer;
            integer
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use assertr::prelude::*;

    use super::*;

    fn column(key: &str) -> ColumnSizing {
        ColumnSizing {
            key: Key::from(key),
            default_width: None,
            min_width: None,
            max_width: None,
        }
    }

    fn sized(key: &str, width: ColumnSize) -> ColumnSizing {
        ColumnSizing {
            default_width: Some(width),
            ..column(key)
        }
    }

    fn changed(widths: &[(&str, ColumnSize)]) -> HashMap<Key, ColumnSize> {
        widths.iter().map(|(k, w)| (Key::from(*k), *w)).collect()
    }

    #[test]
    fn real_life_case_1() {
        let widths = calculate_column_sizes(
            284.0 + 284.0 + 1140.0,
            &[
                sized("name", ColumnSize::Fr(0.998_242_530_755_711_7)),
                sized("type", ColumnSize::Px(286.0)),
                sized("level", ColumnSize::Fr(4.0)),
            ],
            &changed(&[
                ("name", ColumnSize::Fr(0.998_242_530_755_711_7)),
                ("type", ColumnSize::Px(286.0)),
                ("level", ColumnSize::Fr(4.0)),
            ]),
            |_| Some(ColumnSize::Px(150.0)),
            |_| Some(ColumnBound::Px(50.0)),
        );
        assert_that!(widths).is_equal_to(vec![284.0, 286.0, 1138.0]);
    }

    #[test]
    fn real_life_case_2() {
        let widths = calculate_column_sizes(
            284.0 + 284.0 + 1140.0,
            &[
                sized("name", ColumnSize::Fr(1.0)),
                sized("type", ColumnSize::Fr(1.0)),
                column("height"),
                column("weight"),
                sized("level", ColumnSize::Fr(4.0)),
            ],
            &changed(&[
                ("name", ColumnSize::Px(235.0)),
                ("type", ColumnSize::Px(235.0)),
                ("level", ColumnSize::Fr(4.0)),
                ("height", ColumnSize::Px(150.0)),
            ]),
            |_| Some(ColumnSize::Px(150.0)),
            |_| Some(ColumnBound::Px(50.0)),
        );
        assert_that!(widths).is_equal_to(vec![235.0, 235.0, 150.0, 150.0, 938.0]);
    }

    #[test]
    fn real_life_case_3() {
        let percent = |key: &str, max: Option<f64>, min: Option<f64>| ColumnSizing {
            max_width: max.map(ColumnBound::Percent),
            min_width: min.map(ColumnBound::Percent),
            ..column(key)
        };
        let columns = vec![
            percent("id", Some(5.0), None),
            percent("name", None, Some(20.0)),
            column("info"),
            percent("hp", Some(5.0), None),
            percent("attack", Some(5.0), None),
            percent("defense", Some(5.0), None),
            percent("speed", Some(5.0), None),
            percent("total", Some(5.0), None),
            percent("weight", Some(5.0), None),
            percent("height", Some(5.0), None),
            percent("abilities", None, Some(20.0)),
        ];
        let widths = calculate_column_sizes(
            1000.0,
            &columns,
            &HashMap::new(),
            |index| {
                Some(if index == 2 {
                    ColumnSize::Px(30.0)
                } else {
                    ColumnSize::Fr(1.0)
                })
            },
            |_| Some(ColumnBound::Px(25.0)),
        );
        assert_that!(widths).is_equal_to(vec![
            50.0, 285.0, 30.0, 50.0, 50.0, 50.0, 50.0, 50.0, 50.0, 50.0, 285.0,
        ]);
    }

    #[test]
    fn real_life_case_4() {
        let widths = calculate_column_sizes(
            300.0,
            &[
                sized("name", ColumnSize::Px(100.0)),
                sized("type", ColumnSize::Px(150.0)),
                sized("weight", ColumnSize::Px(200.0)),
                sized("level", ColumnSize::Fr(1.0)),
            ],
            &HashMap::new(),
            |_| Some(ColumnSize::Fr(1.0)),
            |_| Some(ColumnBound::Px(50.0)),
        );
        assert_that!(widths).is_equal_to(vec![100.0, 150.0, 200.0, 50.0]);
    }

    #[test]
    fn default_widths() {
        let widths = calculate_column_sizes(
            800.0,
            &[
                sized("name", ColumnSize::Fr(1.0)),
                sized("type", ColumnSize::Fr(1.0)),
                sized("level", ColumnSize::Fr(4.0)),
            ],
            &HashMap::new(),
            |_| Some(ColumnSize::Px(150.0)),
            |_| Some(ColumnBound::Px(50.0)),
        );
        assert_that!(widths).is_equal_to(vec![133.0, 134.0, 533.0]);
    }

    #[test]
    fn keeps_columns_flush_with_a_fractional_table_width() {
        for (table_width, expected) in [(1000.5, 500.5), (1000.7, 500.7)] {
            let widths = calculate_column_sizes(
                table_width,
                &[
                    sized("name", ColumnSize::Fr(1.0)),
                    sized("type", ColumnSize::Fr(1.0)),
                ],
                &HashMap::new(),
                |_| Some(ColumnSize::Px(150.0)),
                |_| Some(ColumnBound::Px(50.0)),
            );
            assert_that!(widths.clone()).is_equal_to(vec![500.0, expected]);
            #[allow(clippy::float_cmp)]
            let flush = widths.iter().sum::<f64>() == table_width;
            assert_that!(flush).is_true();
        }
    }
}
