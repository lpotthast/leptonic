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
// ## DIFFERENT BEHAVIOR
// - A group shared with a column whose top is below the group still counts that column for its
//   outer groups: react-stately's `buildHeaderRows` stops counting there, so outer groups three
//   or more levels up span too few columns.
//
// ## OMITTED FEATURES
// - Drag button columns (`showDragButtons`): not built yet (drag and drop exists; the table atoms
//   don't support dragging rows yet).
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

        // Each data column's stack of header cells, bottom to top: the column, then its groups
        // (innermost first), `None` where it has no header cell. A group shared with a later,
        // taller column moves up to that column's level (react-stately's `buildHeaderRows`).
        let mut stacks: Vec<Vec<Option<Key>>> = Vec::with_capacity(leaves.len());
        // Where each group's header cell is: its stack and position in it.
        let mut seen: HashMap<Key, (usize, usize)> = HashMap::new();
        for (def, ancestors) in &leaves {
            let mut stack = vec![Some(def.key.clone())];
            // Once a shared group is above this column's top, its outer groups are too: they only
            // span this column as well (react-stately stops there and doesn't count them).
            let mut above = false;
            for parent in ancestors {
                if let Some(&(earlier, position)) = seen.get(parent) {
                    if let Some(group) = column_info.get_mut(parent) {
                        group.col_span += 1;
                    }
                    if above || position > stack.len() {
                        above = true;
                        continue;
                    }
                    // Shift the group (and what is above it) up to this column's level.
                    let shift = stack.len() - position;
                    let earlier_stack = &mut stacks[earlier];
                    earlier_stack.splice(position..position, std::iter::repeat_n(None, shift));
                    for (moved, entry) in earlier_stack.iter().enumerate().skip(stack.len()) {
                        if let Some(key) = entry
                            && let Some(place) = seen.get_mut(key)
                        {
                            place.1 = moved;
                        }
                    }
                } else {
                    if let Some(group) = column_info.get_mut(parent) {
                        group.col_span = 1;
                    }
                    stack.push(Some(parent.clone()));
                    seen.insert(parent.clone(), (stacks.len(), stack.len() - 1));
                }
            }
            stacks.push(stack);
        }
        let header_row_count = stacks.iter().map(Vec::len).max().unwrap_or(0);

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

        // Header rows, top to bottom: data columns are in the bottom row, each group above its
        // columns. Placeholders fill the gaps (adjacent gaps are one placeholder).
        let mut cells: Vec<Vec<HeaderCell>> = (0..header_row_count).map(|_| Vec::new()).collect();
        // The number of columns each row covers so far.
        let mut covered = vec![0_usize; header_row_count];
        let mut placeholders = 0;
        let mut placeholder = |col_span: usize| {
            let cell = HeaderCell {
                key: Key::generated("placeholder", placeholders),
                kind: NodeKind::Placeholder,
                text_value: Arc::from(""),
                col_span: (col_span > 1).then_some(col_span),
            };
            placeholders += 1;
            cell
        };
        for (index, stack) in stacks.iter().enumerate() {
            for (height, entry) in stack.iter().enumerate() {
                let Some(key) = entry else {
                    continue;
                };
                let level = header_row_count - 1 - height;
                if covered[level] < index {
                    cells[level].push(placeholder(index - covered[level]));
                    covered[level] = index;
                }
                let column = column_info.get_mut(key).expect("flattened above");
                column.level = level;
                column.index = index;
                cells[level].push(HeaderCell {
                    key: key.clone(),
                    kind: NodeKind::Column,
                    text_value: column.text_value.clone(),
                    col_span: (column.col_span > 1).then_some(column.col_span),
                });
                covered[level] += column.col_span;
            }
        }
        let mut rows = CollectionBuilder::default();
        let mut header_rows = Vec::with_capacity(header_row_count);
        for (level, mut cells) in cells.into_iter().enumerate() {
            if covered[level] < leaves.len() {
                cells.push(placeholder(leaves.len() - covered[level]));
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

    fn header_rows(table: &TableCollection) -> Vec<Vec<(String, NodeKind, Option<usize>)>> {
        (0..table.header_rows().len())
            .map(|level| header_row(table, level))
            .collect()
    }

    fn column(key: &str) -> (String, NodeKind, Option<usize>) {
        (key.to_owned(), NodeKind::Column, None)
    }

    fn group(key: &str, span: usize) -> (String, NodeKind, Option<usize>) {
        (key.to_owned(), NodeKind::Column, Some(span))
    }

    fn placeholder(n: usize, span: Option<usize>) -> (String, NodeKind, Option<usize>) {
        (format!("placeholder-{n}"), NodeKind::Placeholder, span)
    }

    #[test]
    fn a_group_shared_with_a_shorter_column_keeps_its_level() {
        // |      Group 1      |
        // | Group 2 |         |
        // |    A    |    B    |
        let table = TableCollection::build(|t| {
            t.column_group("g1", "Group 1", |g| {
                g.column_group("g2", "Group 2", |g| {
                    g.column("a", "A");
                });
                g.column("b", "B");
            });
        });
        assert_that!(header_rows(&table)).is_equal_to(vec![
            vec![group("g1", 2)],
            vec![column("g2"), placeholder(0, None)],
            vec![column("a"), column("b")],
        ]);
        let g1 = table.column(&k("g1")).expect("group");
        assert_that!((g1.index, g1.col_span, g1.level)).is_equal_to((0, 2, 0));
        let g2 = table.column(&k("g2")).expect("group");
        assert_that!((g2.index, g2.col_span, g2.level)).is_equal_to((0, 1, 1));
    }

    #[test]
    fn a_group_shared_with_a_taller_column_moves_up() {
        // |      Group 1      |
        // |         | Group 2 |
        // |    A    |    B    |
        let table = TableCollection::build(|t| {
            t.column_group("g1", "Group 1", |g| {
                g.column("a", "A");
                g.column_group("g2", "Group 2", |g| {
                    g.column("b", "B");
                });
            });
            t.column("c", "C");
        });
        assert_that!(header_rows(&table)).is_equal_to(vec![
            vec![group("g1", 2), placeholder(1, None)],
            vec![placeholder(0, None), column("g2"), placeholder(2, None)],
            vec![column("a"), column("b"), column("c")],
        ]);
        let g1 = table.column(&k("g1")).expect("group");
        assert_that!((g1.index, g1.col_span, g1.level)).is_equal_to((0, 2, 0));
        let g2 = table.column(&k("g2")).expect("group");
        assert_that!((g2.index, g2.col_span, g2.level)).is_equal_to((1, 1, 1));
        // Header cells know their column.
        assert_that!(table.collection().get(&k("g2")).and_then(|n| n.col_index))
            .is_equal_to(Some(1));
    }

    #[test]
    fn outer_groups_span_every_column_under_them() {
        // |        Group 0        |
        // |        Group 1        |
        // | Group 2 |             |
        // |    A    |      B      |
        let table = TableCollection::build(|t| {
            t.column_group("g0", "Group 0", |g| {
                g.column_group("g1", "Group 1", |g| {
                    g.column_group("g2", "Group 2", |g| {
                        g.column("a", "A");
                    });
                    g.column("b", "B");
                });
            });
        });
        assert_that!(header_rows(&table)).is_equal_to(vec![
            vec![group("g0", 2)],
            vec![group("g1", 2)],
            vec![column("g2"), placeholder(0, None)],
            vec![column("a"), column("b")],
        ]);
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
