// Upstream: react-stately/src/table/TableCollection.ts @ 99e6102368
use std::{collections::HashMap, sync::Arc};

use super::table_utils::{ColumnBound, ColumnSize};
use crate::hooks::collections::{
    Collection, CollectionBuilder, HeaderCell, ItemBuilder, Key, Node, NodeKind, RowBuilder,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Built with a typed builder (`TableCollection::build`) instead of JSX children: columns
//   (`column`, `column_group`) and rows (`row` with cells).
// - The rows (header rows first, then body rows) are an ordinary `Collection`; the column
//   structure is kept next to it (`Column`). Generated nodes (header rows, placeholders, the
//   selection checkbox column) get keys that never equal keys created from user values.
// - Rows need a text value (react-aria derives one from the row header cells when missing).
//
// ## OMITTED FEATURES
// - Drag button columns (`showDragButtons`): with drag and drop (PLAN.md, step 11).
//
// =============================================================================

/// What a column is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnKind {
    /// A column of data cells.
    Data,
    /// A group of columns: a header spanning its columns, in the header row above them.
    Group,
    /// The column of row selection checkboxes (`TableOptions::show_selection_checkboxes`).
    SelectionCheckbox,
}

/// A column (or column group) of a [`TableCollection`].
#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub key: Key,
    pub text_value: Arc<str>,
    pub kind: ColumnKind,
    /// The cells of this column label their rows (`role="rowheader"`).
    pub is_row_header: bool,
    /// The table can be sorted by this column.
    pub allows_sorting: bool,
    /// The column can be resized (in a table with column resizing).
    pub allows_resizing: bool,
    /// The column's initial width (with column resizing).
    pub default_width: Option<ColumnSize>,
    /// The column's minimum width (with column resizing).
    pub min_width: Option<ColumnBound>,
    /// The column's maximum width (with column resizing).
    pub max_width: Option<ColumnBound>,
    /// The (first) data column this column covers.
    pub index: usize,
    /// The number of data columns this column covers (`1` unless it is a group).
    pub col_span: usize,
    /// The header row this column's header is in.
    pub level: usize,
    /// The column group containing this column.
    pub parent: Option<Key>,
    /// The columns of a column group.
    pub children: Vec<Key>,
}

/// Options for building a [`TableCollection`].
#[derive(Debug, Clone, Copy, Default)]
pub struct TableOptions {
    /// Add a first column of checkboxes selecting the rows (and selecting all, in its header).
    pub show_selection_checkboxes: bool,
}

/// The rows and columns of a table.
///
/// Its [`collection`](TableCollection::collection) holds the header rows (of column headers and
/// placeholders, see [`NodeKind::HeaderRow`]) followed by the body rows; the column structure is
/// available through [`columns`](TableCollection::columns) and [`column`](TableCollection::column).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TableCollection {
    collection: Arc<Collection>,
    /// The data columns (no groups), in order.
    columns: Vec<Key>,
    column_info: HashMap<Key, Column>,
    header_rows: Vec<Key>,
    row_header_columns: Vec<Key>,
}

impl TableCollection {
    /// Build a table: its columns and rows.
    ///
    /// ```
    /// # use leptonic::hooks::TableCollection;
    /// let table = TableCollection::build(|t| {
    ///     t.column("name", "Name").row_header().allows_sorting();
    ///     t.column_group("contact", "Contact", |g| {
    ///         g.column("email", "Email");
    ///         g.column("phone", "Phone");
    ///     });
    ///     t.row("alice", "Alice", |r| {
    ///         r.cell("Alice");
    ///         r.cell("alice@example.com");
    ///         r.cell("555-0100");
    ///     });
    /// });
    /// assert_eq!(table.size(), 1);
    /// assert_eq!(table.header_rows().len(), 2);
    /// ```
    pub fn build(f: impl FnOnce(&mut TableBuilder)) -> Self {
        Self::build_with(TableOptions::default(), f)
    }

    /// [`TableCollection::build`] with options.
    pub fn build_with(options: TableOptions, f: impl FnOnce(&mut TableBuilder)) -> Self {
        let mut builder = TableBuilder {
            columns: ColumnsBuilder::default(),
            rows: CollectionBuilder::default(),
            show_selection_checkboxes: options.show_selection_checkboxes,
        };
        f(&mut builder);
        builder.finish()
    }

    /// The header rows and the body rows.
    pub fn collection(&self) -> &Arc<Collection> {
        &self.collection
    }

    /// The number of body rows.
    pub fn size(&self) -> usize {
        self.collection.size()
    }

    /// The body rows.
    pub fn rows(&self) -> impl Iterator<Item = &Node> {
        self.collection.items()
    }

    /// The data columns (including the selection checkbox column, not column groups), in order.
    pub fn columns(&self) -> impl Iterator<Item = &Column> {
        self.columns
            .iter()
            .filter_map(|key| self.column_info.get(key))
    }

    /// The number of data columns.
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// The column (or column group) with `key`.
    pub fn column(&self, key: &Key) -> Option<&Column> {
        self.column_info.get(key)
    }

    /// The data column at `index`.
    pub fn column_at(&self, index: usize) -> Option<&Column> {
        self.column_info.get(self.columns.get(index)?)
    }

    /// The column of a body cell.
    pub fn cell_column(&self, cell: &Key) -> Option<&Column> {
        let node = self.collection.get(cell)?;
        (node.kind == NodeKind::Cell)
            .then(|| self.column_at(node.col_index.unwrap_or(node.index)))
            .flatten()
    }

    /// The keys of the header rows, top to bottom.
    pub fn header_rows(&self) -> &[Key] {
        &self.header_rows
    }

    /// The columns whose cells label their rows. When no column is marked as row header, the
    /// first data column is.
    pub fn row_header_columns(&self) -> &[Key] {
        &self.row_header_columns
    }
}

/// Builds a [`TableCollection`], see [`TableCollection::build`].
#[derive(Debug)]
pub struct TableBuilder {
    columns: ColumnsBuilder,
    rows: CollectionBuilder,
    show_selection_checkboxes: bool,
}

impl TableBuilder {
    /// Add a column.
    pub fn column(
        &mut self,
        key: impl Into<Key>,
        text_value: impl Into<Arc<str>>,
    ) -> ColumnBuilder<'_> {
        self.columns.column(key, text_value)
    }

    /// Add a group of columns, filled by `columns`.
    pub fn column_group(
        &mut self,
        key: impl Into<Key>,
        text_value: impl Into<Arc<str>>,
        columns: impl FnOnce(&mut ColumnsBuilder),
    ) {
        self.columns.column_group(key, text_value, columns);
    }

    /// Add a body row with one cell per data column. Cell keys are generated:
    /// `Key::cell(row, i)`, counting the selection checkbox cell (if any) as cell `0`.
    pub fn row(
        &mut self,
        key: impl Into<Key>,
        text_value: impl Into<Arc<str>>,
        cells: impl FnOnce(&mut RowBuilder),
    ) -> ItemBuilder<'_> {
        let show_selection_checkboxes = self.show_selection_checkboxes;
        self.rows.row(key, text_value, |r| {
            if show_selection_checkboxes {
                r.cell("");
            }
            cells(r);
        })
    }

    fn finish(self) -> TableCollection {
        let mut leaves: Vec<(ColumnDef, Vec<Key>)> = Vec::new();
        let mut column_info: HashMap<Key, Column> = HashMap::new();
        if self.show_selection_checkboxes {
            let key = Key::generated("selection-column", 0);
            leaves.push((
                ColumnDef::new(key, Arc::from(""), ColumnKind::SelectionCheckbox),
                Vec::new(),
            ));
        }
        for def in self.columns.defs {
            flatten(def, &mut Vec::new(), &mut leaves, &mut column_info);
        }

        let header_row_count = leaves
            .iter()
            .map(|(_, ancestors)| ancestors.len() + 1)
            .max()
            .unwrap_or(0);

        let mut columns = Vec::with_capacity(leaves.len());
        for (index, (def, ancestors)) in leaves.iter().enumerate() {
            columns.push(def.key.clone());
            column_info.insert(
                def.key.clone(),
                Column {
                    key: def.key.clone(),
                    text_value: def.text_value.clone(),
                    kind: def.kind,
                    is_row_header: def.is_row_header,
                    allows_sorting: def.allows_sorting,
                    allows_resizing: def.allows_resizing,
                    default_width: def.default_width,
                    min_width: def.min_width,
                    max_width: def.max_width,
                    index,
                    col_span: 1,
                    level: header_row_count - 1,
                    parent: ancestors.first().cloned(),
                    children: Vec::new(),
                },
            );
        }

        // Header rows, top to bottom: data columns are in the bottom row, each group in the row
        // above its columns. Where a column has no group, a placeholder fills the gap (adjacent
        // gaps are one placeholder).
        let mut rows = CollectionBuilder::default();
        let mut header_rows = Vec::with_capacity(header_row_count);
        let mut placeholders = 0;
        for level in 0..header_row_count {
            let from_bottom = header_row_count - 1 - level;
            let mut cells: Vec<HeaderCell> = Vec::new();
            let mut previous: Option<Option<Key>> = None;
            for (index, (def, ancestors)) in leaves.iter().enumerate() {
                let here = match from_bottom {
                    0 => Some(def.key.clone()),
                    n => ancestors.get(n - 1).cloned(),
                };
                let same_group = here.is_some() && previous.as_ref() == Some(&here);
                let same_gap = here.is_none() && previous == Some(None);
                if same_group || same_gap {
                    if let Some(last) = cells.last_mut() {
                        last.col_span = Some(last.col_span.unwrap_or(1) + 1);
                    }
                    if let Some(group) = here.as_ref().and_then(|k| column_info.get_mut(k)) {
                        group.col_span += 1;
                    }
                } else if let Some(key) = &here {
                    let column = column_info.get_mut(key).expect("flattened above");
                    column.level = level;
                    if column.kind == ColumnKind::Group {
                        column.index = index;
                        column.col_span = 1;
                    }
                    cells.push(HeaderCell {
                        key: key.clone(),
                        kind: NodeKind::Column,
                        text_value: column.text_value.clone(),
                        col_span: None,
                    });
                } else {
                    cells.push(HeaderCell {
                        key: Key::generated("placeholder", placeholders),
                        kind: NodeKind::Placeholder,
                        text_value: Arc::from(""),
                        col_span: None,
                    });
                    placeholders += 1;
                }
                previous = Some(here);
            }
            // Only spans of more than one column are spans.
            for cell in &mut cells {
                if cell.col_span == Some(1) {
                    cell.col_span = None;
                }
            }
            let key = Key::generated("headerrow", level);
            header_rows.push(key.clone());
            rows.header_row(key, cells);
        }
        rows.append(self.rows);

        let mut row_header_columns: Vec<Key> = columns
            .iter()
            .filter(|key| column_info.get(*key).is_some_and(|c| c.is_row_header))
            .cloned()
            .collect();
        if row_header_columns.is_empty()
            && let Some(first) = columns.iter().find(|key| {
                column_info
                    .get(*key)
                    .is_some_and(|c| c.kind == ColumnKind::Data)
            })
        {
            row_header_columns.push(first.clone());
        }

        TableCollection {
            collection: Arc::new(rows.build()),
            columns,
            column_info,
            header_rows,
            row_header_columns,
        }
    }
}

/// Adds columns: to the table ([`TableBuilder`]) or to a column group.
#[derive(Debug, Default)]
pub struct ColumnsBuilder {
    defs: Vec<ColumnDef>,
}

impl ColumnsBuilder {
    /// Add a column.
    pub fn column(
        &mut self,
        key: impl Into<Key>,
        text_value: impl Into<Arc<str>>,
    ) -> ColumnBuilder<'_> {
        self.defs.push(ColumnDef::new(
            key.into(),
            text_value.into(),
            ColumnKind::Data,
        ));
        ColumnBuilder {
            def: self.defs.last_mut().expect("just pushed"),
        }
    }

    /// Add a group of columns, filled by `columns`.
    pub fn column_group(
        &mut self,
        key: impl Into<Key>,
        text_value: impl Into<Arc<str>>,
        columns: impl FnOnce(&mut ColumnsBuilder),
    ) {
        let mut inner = ColumnsBuilder::default();
        columns(&mut inner);
        self.defs.push(ColumnDef {
            children: inner.defs,
            ..ColumnDef::new(key.into(), text_value.into(), ColumnKind::Group)
        });
    }
}

/// Configures a column added with [`ColumnsBuilder::column`].
pub struct ColumnBuilder<'a> {
    def: &'a mut ColumnDef,
}

#[allow(clippy::return_self_not_must_use)]
impl ColumnBuilder<'_> {
    /// The column's cells label their rows (`role="rowheader"`).
    pub fn row_header(self) -> Self {
        self.def.is_row_header = true;
        self
    }

    /// The table can be sorted by this column.
    pub fn allows_sorting(self) -> Self {
        self.def.allows_sorting = true;
        self
    }

    /// The column can be resized (in a table with column resizing).
    pub fn allows_resizing(self) -> Self {
        self.def.allows_resizing = true;
        self
    }

    /// The column's initial width (with column resizing). Defaults to `ColumnSize::Fr(1.0)`.
    pub fn default_width(self, width: ColumnSize) -> Self {
        self.def.default_width = Some(width);
        self
    }

    /// The column's minimum width (with column resizing). Defaults to 75 pixels.
    pub fn min_width(self, width: ColumnBound) -> Self {
        self.def.min_width = Some(width);
        self
    }

    /// The column's maximum width (with column resizing). Unbounded by default.
    pub fn max_width(self, width: ColumnBound) -> Self {
        self.def.max_width = Some(width);
        self
    }
}

#[derive(Debug, Clone)]
struct ColumnDef {
    key: Key,
    text_value: Arc<str>,
    kind: ColumnKind,
    is_row_header: bool,
    allows_sorting: bool,
    allows_resizing: bool,
    default_width: Option<ColumnSize>,
    min_width: Option<ColumnBound>,
    max_width: Option<ColumnBound>,
    children: Vec<ColumnDef>,
}

impl ColumnDef {
    fn new(key: Key, text_value: Arc<str>, kind: ColumnKind) -> Self {
        Self {
            key,
            text_value,
            kind,
            is_row_header: false,
            allows_sorting: false,
            allows_resizing: false,
            default_width: None,
            min_width: None,
            max_width: None,
            children: Vec::new(),
        }
    }
}

/// Collects the data columns of `def` (with their groups, innermost first) into `leaves`, and
/// registers its groups in `info`.
fn flatten(
    def: ColumnDef,
    ancestors: &mut Vec<Key>,
    leaves: &mut Vec<(ColumnDef, Vec<Key>)>,
    info: &mut HashMap<Key, Column>,
) {
    if def.kind != ColumnKind::Group {
        let mut chain = ancestors.clone();
        chain.reverse();
        leaves.push((def, chain));
        return;
    }
    let key = def.key.clone();
    info.insert(
        key.clone(),
        Column {
            key: key.clone(),
            text_value: def.text_value.clone(),
            kind: ColumnKind::Group,
            is_row_header: false,
            allows_sorting: false,
            allows_resizing: false,
            default_width: None,
            min_width: None,
            max_width: None,
            index: 0,
            col_span: 0,
            level: 0,
            parent: ancestors.last().cloned(),
            children: def.children.iter().map(|c| c.key.clone()).collect(),
        },
    );
    ancestors.push(key);
    for child in def.children {
        flatten(child, ancestors, leaves, info);
    }
    ancestors.pop();
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn k(s: &str) -> Key {
        Key::from(s)
    }

    fn header_row(table: &TableCollection, level: usize) -> Vec<(String, NodeKind, Option<usize>)> {
        table
            .collection()
            .children(&table.header_rows()[level])
            .map(|n| (n.key.to_string(), n.kind, n.col_span))
            .collect()
    }

    #[test]
    fn flat_columns_make_one_header_row() {
        let table = TableCollection::build(|t| {
            t.column("name", "Name");
            t.column("age", "Age").allows_sorting();
            t.row("alice", "Alice", |r| {
                r.cell("Alice");
                r.cell("30");
            });
        });
        assert_that!(table.header_rows().len()).is_equal_to(1);
        assert_that!(table.size()).is_equal_to(1);
        assert_that!(header_row(&table, 0)).is_equal_to(vec![
            ("name".to_owned(), NodeKind::Column, None),
            ("age".to_owned(), NodeKind::Column, None),
        ]);
        // The first column labels the rows when none is marked.
        assert_that!(table.row_header_columns().to_vec()).is_equal_to(vec![k("name")]);
        assert_that!(table.column(&k("age")).map(|c| c.allows_sorting)).is_equal_to(Some(true));
        let cell = Key::cell(&k("alice"), 1);
        assert_that!(table.cell_column(&cell).map(|c| c.key.clone())).is_equal_to(Some(k("age")));
        // Navigation from the last header goes to the body.
        assert_that!(table.collection().key_after(&k("age")).cloned())
            .is_equal_to(Some(k("alice")));
    }

    #[test]
    fn column_groups_span_their_columns_and_placeholders_fill_gaps() {
        // | Name |    Contact     |     |      <- placeholder over "Name" and "Notes"
        // | Name | Email | Phone | Notes |
        let table = TableCollection::build(|t| {
            t.column("name", "Name").row_header();
            t.column_group("contact", "Contact", |g| {
                g.column("email", "Email");
                g.column("phone", "Phone");
            });
            t.column("notes", "Notes");
        });
        assert_that!(header_row(&table, 0)).is_equal_to(vec![
            ("placeholder-0".to_owned(), NodeKind::Placeholder, None),
            ("contact".to_owned(), NodeKind::Column, Some(2)),
            ("placeholder-1".to_owned(), NodeKind::Placeholder, None),
        ]);
        assert_that!(header_row(&table, 1).len()).is_equal_to(4);
        let contact = table.column(&k("contact")).expect("group");
        assert_that!((contact.index, contact.col_span, contact.level)).is_equal_to((1, 2, 0));
        assert_that!(contact.children.clone()).is_equal_to(vec![k("email"), k("phone")]);
        assert_that!(table.column(&k("phone")).and_then(|c| c.parent.clone()))
            .is_equal_to(Some(k("contact")));
        // Header cells know their column.
        let notes_placeholder = Key::generated("placeholder", 1);
        assert_that!(
            table
                .collection()
                .get(&notes_placeholder)
                .and_then(|n| n.col_index)
        )
        .is_equal_to(Some(3));
        assert_that!(table.column_count()).is_equal_to(4);
    }

    #[test]
    fn selection_checkboxes_add_a_first_column_and_cell() {
        let table = TableCollection::build_with(
            TableOptions {
                show_selection_checkboxes: true,
            },
            |t| {
                t.column("name", "Name");
                t.row("alice", "Alice", |r| {
                    r.cell("Alice");
                });
            },
        );
        let first = table.column_at(0).expect("checkbox column");
        assert_that!(first.kind).is_equal_to(ColumnKind::SelectionCheckbox);
        assert_that!(table.row_header_columns().to_vec()).is_equal_to(vec![k("name")]);
        let name_cell = Key::cell(&k("alice"), 1);
        assert_that!(table.cell_column(&name_cell).map(|c| c.key.clone()))
            .is_equal_to(Some(k("name")));
        assert_that!(
            table
                .collection()
                .get(&name_cell)
                .map(|n| n.text_value.to_string())
        )
        .is_equal_to(Some("Alice".to_owned()));
    }
}
