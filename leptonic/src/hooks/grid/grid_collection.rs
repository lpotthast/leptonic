use std::{collections::HashMap, hash::Hash};

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-types/grid/src/index.d.ts
// Specifically the `GridCollection<T>` interface.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Standalone struct wrapping `Vec<GridRow<K>>` instead of extending a generic
//   `Collection` trait with node-tree traversal.
//
// ## OMITTED FEATURES
// - No column span support yet.
// - No virtualization support.
//
// =============================================================================

/// A row in the grid, containing a key and its child cell keys.
#[derive(Debug, Clone)]
pub struct GridRow<K> {
    /// The unique key for this row.
    pub key: K,
    /// The keys of cells in this row, in order.
    pub cells: Vec<K>,
}

/// An indexed grid collection providing O(1) lookups for navigation.
///
/// Wraps `Vec<GridRow<K>>` with pre-computed indexes for fast key resolution.
#[derive(Debug, Clone)]
pub struct GridCollection<K: Hash + Eq> {
    rows: Vec<GridRow<K>>,
    /// Maps row key to its index in `rows`.
    row_index: HashMap<K, usize>,
    /// Maps cell key to `(row_index, cell_index_within_row)`.
    cell_index: HashMap<K, (usize, usize)>,
}

impl<K: Hash + Eq + Clone> GridCollection<K> {
    /// Build a collection from rows, pre-computing indexes.
    #[must_use]
    pub fn new(rows: Vec<GridRow<K>>) -> Self {
        let mut row_index = HashMap::with_capacity(rows.len());
        let mut cell_index = HashMap::new();

        for (ri, row) in rows.iter().enumerate() {
            row_index.insert(row.key.clone(), ri);
            for (ci, cell_key) in row.cells.iter().enumerate() {
                cell_index.insert(cell_key.clone(), (ri, ci));
            }
        }

        Self {
            rows,
            row_index,
            cell_index,
        }
    }

    /// The maximum number of cells across all rows.
    #[must_use]
    pub fn column_count(&self) -> usize {
        self.rows.iter().map(|r| r.cells.len()).max().unwrap_or(0)
    }

    /// All rows in the collection.
    #[must_use]
    pub fn rows(&self) -> &[GridRow<K>] {
        &self.rows
    }

    /// Whether the given key is a row key.
    #[must_use]
    pub fn is_row_key(&self, key: &K) -> bool {
        self.row_index.contains_key(key)
    }

    /// Whether the given key is a cell key.
    #[must_use]
    pub fn is_cell_key(&self, key: &K) -> bool {
        self.cell_index.contains_key(key)
    }

    /// Look up a row by its row key.
    #[must_use]
    pub fn get_row(&self, key: &K) -> Option<&GridRow<K>> {
        self.row_index.get(key).map(|&idx| &self.rows[idx])
    }

    /// Find the parent row and cell-index for a given cell key.
    #[must_use]
    pub fn get_row_for_cell(&self, cell_key: &K) -> Option<(&GridRow<K>, usize)> {
        self.cell_index
            .get(cell_key)
            .map(|&(ri, ci)| (&self.rows[ri], ci))
    }

    /// The row before the one identified by `row_key`, or `None`.
    #[must_use]
    pub fn row_before(&self, row_key: &K) -> Option<&GridRow<K>> {
        self.row_index
            .get(row_key)
            .and_then(|&idx| idx.checked_sub(1))
            .map(|idx| &self.rows[idx])
    }

    /// The row after the one identified by `row_key`, or `None`.
    #[must_use]
    pub fn row_after(&self, row_key: &K) -> Option<&GridRow<K>> {
        self.row_index.get(row_key).and_then(|&idx| {
            let next = idx + 1;
            if next < self.rows.len() {
                Some(&self.rows[next])
            } else {
                None
            }
        })
    }

    /// The first row, or `None` if empty.
    #[must_use]
    pub fn first_row(&self) -> Option<&GridRow<K>> {
        self.rows.first()
    }

    /// The last row, or `None` if empty.
    #[must_use]
    pub fn last_row(&self) -> Option<&GridRow<K>> {
        self.rows.last()
    }

    /// Total number of rows.
    #[must_use]
    pub fn size(&self) -> usize {
        self.rows.len()
    }
}

impl<K: Hash + Eq + Clone> Default for GridCollection<K> {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn make_collection() -> GridCollection<String> {
        GridCollection::new(vec![
            GridRow {
                key: "row-0".into(),
                cells: vec!["0-0".into(), "0-1".into(), "0-2".into()],
            },
            GridRow {
                key: "row-1".into(),
                cells: vec!["1-0".into(), "1-1".into(), "1-2".into()],
            },
            GridRow {
                key: "row-2".into(),
                cells: vec!["2-0".into(), "2-1".into()],
            },
        ])
    }

    #[test]
    fn column_count_returns_max_cells() {
        let c = make_collection();
        assert_that(c.column_count()).is_equal_to(3);
    }

    #[test]
    fn is_row_key_and_is_cell_key() {
        let c = make_collection();
        assert_that(c.is_row_key(&"row-0".into())).is_true();
        assert_that(c.is_row_key(&"0-0".into())).is_false();
        assert_that(c.is_cell_key(&"0-0".into())).is_true();
        assert_that(c.is_cell_key(&"row-0".into())).is_false();
    }

    #[test]
    fn get_row_for_cell_works() {
        let c = make_collection();
        let (row, ci) = c.get_row_for_cell(&"1-2".into()).unwrap();
        assert_that(row.key.as_str()).is_equal_to("row-1");
        assert_that(ci).is_equal_to(2);
    }

    #[test]
    fn row_before_and_after() {
        let c = make_collection();
        assert_that(c.row_before(&"row-0".into())).is_none();
        assert_that(c.row_before(&"row-1".into()).unwrap().key.as_str()).is_equal_to("row-0");
        assert_that(c.row_after(&"row-2".into())).is_none();
        assert_that(c.row_after(&"row-1".into()).unwrap().key.as_str()).is_equal_to("row-2");
    }

    #[test]
    fn first_and_last_row() {
        let c = make_collection();
        assert_that(c.first_row().unwrap().key.as_str()).is_equal_to("row-0");
        assert_that(c.last_row().unwrap().key.as_str()).is_equal_to("row-2");
    }

    #[test]
    fn empty_collection() {
        let c: GridCollection<String> = GridCollection::default();
        assert_that(c.size()).is_equal_to(0);
        assert_that(c.column_count()).is_equal_to(0);
        assert_that(c.first_row()).is_none();
        assert_that(c.last_row()).is_none();
    }
}
