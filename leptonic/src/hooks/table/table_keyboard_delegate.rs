// Upstream: react-aria/src/table/TableKeyboardDelegate.ts @ 99e6102368
use std::sync::Arc;

use leptos::prelude::*;

use super::TableCollection;
use crate::{
    hooks::{
        GridKeyboardDelegate,
        collections::{Key, KeyboardDelegate, NavigationOptions, Node, NodeKind},
    },
    utils::{filter::Collator, locale::WritingDirection},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Wraps a `GridKeyboardDelegate` (react-aria subclasses it).
// - Home/End on a column header move within its header row; ArrowDown from a column header
//   always goes to the first body row's cell of that column (also in cell focus mode).
//
// =============================================================================

/// Keyboard navigation for tables: the grid's navigation, plus the column headers above the
/// body rows.
#[derive(Clone)]
pub struct TableKeyboardDelegate {
    grid: GridKeyboardDelegate,
    table: Memo<Arc<TableCollection>>,
    direction: WritingDirection,
    collator: Option<Arc<Collator>>,
}

impl std::fmt::Debug for TableKeyboardDelegate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TableKeyboardDelegate")
            .field("grid", &self.grid)
            .finish_non_exhaustive()
    }
}

impl TableKeyboardDelegate {
    /// Navigation for `table`. `grid` must be configured with the same direction and collator as
    /// given here.
    pub fn new(
        grid: GridKeyboardDelegate,
        table: Memo<Arc<TableCollection>>,
        direction: WritingDirection,
        collator: Option<Arc<Collator>>,
    ) -> Self {
        Self {
            grid,
            table,
            direction,
            collator,
        }
    }

    fn with<R>(&self, f: impl FnOnce(&TableCollection) -> R) -> R {
        self.table.with_untracked(|t| f(t))
    }

    fn node(&self, key: &Key) -> Option<Node> {
        self.with(|t| t.collection().get(key).cloned())
    }

    /// The column headers in the header row of `column`.
    fn header_row_columns(&self, column: &Node) -> Vec<Key> {
        self.with(|t| {
            column
                .parent_key
                .as_ref()
                .map(|row| {
                    t.collection()
                        .children(row)
                        .filter(|n| n.kind == NodeKind::Column)
                        .map(|n| n.key.clone())
                        .collect()
                })
                .unwrap_or_default()
        })
    }

    /// The next (or previous) column header in the header row, wrapping around.
    fn neighbor_column(&self, column: &Node, forward: bool) -> Option<Key> {
        let columns = self.header_row_columns(column);
        let position = columns.iter().position(|k| *k == column.key)?;
        let neighbor = if forward {
            columns.get(position + 1).or_else(|| columns.first())
        } else {
            position
                .checked_sub(1)
                .and_then(|p| columns.get(p))
                .or_else(|| columns.last())
        };
        neighbor.cloned()
    }

    fn first_row(&self) -> Option<Key> {
        self.with(|t| {
            t.rows()
                .find(|n| !self.grid.is_disabled(&n.key))
                .map(|n| n.key.clone())
        })
    }

    /// The enabled body row after `row`.
    fn next_row(&self, row: &Key) -> Option<Key> {
        self.with(|t| {
            t.rows()
                .skip_while(|n| n.key != *row)
                .skip(1)
                .find(|n| !self.grid.is_disabled(&n.key))
                .map(|n| n.key.clone())
        })
    }

    fn first_cell_of(&self, row: &Key) -> Option<Key> {
        self.with(|t| t.collection().children(row).next().map(|n| n.key.clone()))
    }

    fn last_cell_of(&self, row: &Key) -> Option<Key> {
        self.with(|t| t.collection().children(row).last().map(|n| n.key.clone()))
    }
}

impl KeyboardDelegate for TableKeyboardDelegate {
    fn key_below(&self, key: &Key, options: NavigationOptions) -> Option<Key> {
        let node = self.node(key)?;
        if node.kind == NodeKind::Column {
            // A column group: its first column. A column: its cell in the first row.
            let (child, index) = self.with(|t| {
                let column = t.column(key);
                (
                    column.and_then(|c| c.children.first().cloned()),
                    column.map_or(0, |c| c.index),
                )
            });
            if child.is_some() {
                return child;
            }
            let row = self.first_row()?;
            return self.grid.key_for_item_in_row_by_index(&row, index);
        }
        self.grid.key_below(key, options)
    }

    fn key_above(&self, key: &Key, options: NavigationOptions) -> Option<Key> {
        let node = self.node(key)?;
        if node.kind == NodeKind::Column {
            return self.with(|t| t.column(key).and_then(|c| c.parent.clone()));
        }
        if let Some(above) = self.grid.key_above(key, options) {
            return Some(above);
        }
        // From the first row: the column headers.
        self.with(|t| {
            let index = match node.kind {
                NodeKind::Cell => node.col_index.unwrap_or(node.index),
                _ => 0,
            };
            t.column_at(index).map(|c| c.key.clone())
        })
    }

    fn key_right_of(&self, key: &Key, options: NavigationOptions) -> Option<Key> {
        let node = self.node(key)?;
        if node.kind == NodeKind::Column {
            return self.neighbor_column(&node, self.direction != WritingDirection::Rtl);
        }
        self.grid.key_right_of(key, options)
    }

    fn key_left_of(&self, key: &Key, options: NavigationOptions) -> Option<Key> {
        let node = self.node(key)?;
        if node.kind == NodeKind::Column {
            return self.neighbor_column(&node, self.direction == WritingDirection::Rtl);
        }
        self.grid.key_left_of(key, options)
    }

    fn first_key(&self, from: Option<&Key>, global: bool) -> Option<Key> {
        if let Some(column) = from
            .and_then(|k| self.node(k))
            .filter(|n| n.kind == NodeKind::Column)
        {
            if !global {
                return self.header_row_columns(&column).into_iter().next();
            }
            let row = self.grid.first_key(None, false)?;
            return self.first_cell_of(&row).or(Some(row));
        }
        self.grid.first_key(from, global)
    }

    fn last_key(&self, from: Option<&Key>, global: bool) -> Option<Key> {
        if let Some(column) = from
            .and_then(|k| self.node(k))
            .filter(|n| n.kind == NodeKind::Column)
        {
            if !global {
                return self.header_row_columns(&column).pop();
            }
            let row = self.grid.last_key(None, false)?;
            return self.last_cell_of(&row).or(Some(row));
        }
        self.grid.last_key(from, global)
    }

    // Paging steps with the table's own `key_above`/`key_below` (react-aria's base class calls
    // the overridden methods), so PageUp reaches the column headers.
    fn key_page_above(&self, key: &Key) -> Option<Key> {
        self.grid
            .key_page_above_with(key, |k| self.key_above(k, NavigationOptions::default()))
    }

    fn key_page_below(&self, key: &Key) -> Option<Key> {
        self.grid
            .key_page_below_with(key, |k| self.key_below(k, NavigationOptions::default()))
    }

    /// Matches the rows' text, then the text of their row header cells.
    fn key_for_search(&self, search: &str, from: Option<&Key>) -> Option<Key> {
        let collator = self.collator.as_ref()?;
        let start = match from {
            Some(from) => from.clone(),
            None => self.first_key(None, false)?,
        };
        let from_cell = self.node(&start).is_some_and(|n| n.kind == NodeKind::Cell);
        let mut key = if from_cell {
            self.node(&start)?.parent_key
        } else {
            Some(start)
        };
        let search_len = search.chars().count();
        let matches = |text: &str| {
            let prefix: String = text.chars().take(search_len).collect();
            !text.is_empty() && collator.compare(&prefix, search) == std::cmp::Ordering::Equal
        };
        let mut has_wrapped = false;
        while let Some(k) = key {
            let row = self.node(&k)?;
            // (In cell focus mode, focusing the row focuses its first cell.)
            if matches(&row.text_value) {
                return Some(k);
            }
            let row_header_cell = self.with(|t| {
                t.collection()
                    .children(&k)
                    .find(|cell| {
                        t.cell_column(&cell.key)
                            .is_some_and(|c| t.row_header_columns().contains(&c.key))
                            && matches(&cell.text_value)
                    })
                    .map(|cell| cell.key.clone())
            });
            if let Some(cell) = row_header_cell {
                return Some(if from_cell { cell } else { k });
            }
            key = self.next_row(&k);
            if key.is_none() && !has_wrapped {
                key = self.first_row();
                has_wrapped = true;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::{
        hooks::{
            GridFocusMode,
            collections::{
                CollectionMemo, LayoutDelegate, Rect, SelectionManager, SelectionMode,
                SelectionOptions, Size,
            },
        },
        utils::{filter::CollatorOptions, i18n::Locale},
    };

    struct NoLayout;

    impl LayoutDelegate for NoLayout {
        fn item_rect(&self, _key: &Key) -> Option<Rect> {
            None
        }
        fn visible_rect(&self) -> Rect {
            Rect::default()
        }
        fn content_size(&self) -> Size {
            Size::default()
        }
    }

    /// |      |    Contact     |
    /// | Name | Email | Phone  |
    /// | Alice| a@... | 555-01 |
    /// | Bob  | b@... | 555-02 |
    fn delegate(focus_mode: GridFocusMode, direction: WritingDirection) -> TableKeyboardDelegate {
        delegate_with_layout(focus_mode, direction, Arc::new(NoLayout))
    }

    fn delegate_with_layout(
        focus_mode: GridFocusMode,
        direction: WritingDirection,
        layout: Arc<dyn LayoutDelegate>,
    ) -> TableKeyboardDelegate {
        let table = Memo::new(|_| {
            Arc::new(TableCollection::build(|t| {
                t.column("name", "Name");
                t.column_group("contact", "Contact", |g| {
                    g.column("email", "Email");
                    g.column("phone", "Phone");
                });
                for (name, email, phone) in [("alice", "a@x", "555-01"), ("bob", "b@x", "555-02")] {
                    t.row(name, name, |r| {
                        r.cell(name);
                        r.cell(email);
                        r.cell(phone);
                    });
                }
            }))
        });
        let collection: CollectionMemo = Memo::new(move |_| table.with(|t| t.collection().clone()));
        let selection = SelectionManager::new(
            collection,
            SelectionOptions {
                selection_mode: Signal::stored(SelectionMode::Multiple),
                ..Default::default()
            },
        );
        let collator = Arc::new(Collator::new(
            &Locale::default(),
            &CollatorOptions::default(),
        ));
        let grid = GridKeyboardDelegate::new(collection, selection, layout)
            .with_direction(direction)
            .with_collator(collator.clone())
            .with_focus_mode(focus_mode);
        TableKeyboardDelegate::new(grid, table, direction, Some(collator))
    }

    fn k(s: &str) -> Key {
        Key::from(s)
    }

    fn cell(row: &str, column: usize) -> Key {
        Key::cell(&k(row), column)
    }

    const NAV: NavigationOptions = NavigationOptions {
        include_disabled: false,
    };

    #[test]
    fn up_from_the_first_row_reaches_the_column_headers() {
        Owner::new().with(|| {
            let d = delegate(GridFocusMode::Row, WritingDirection::Ltr);
            assert_that!(d.key_above(&k("alice"), NAV)).is_equal_to(Some(k("name")));
            assert_that!(d.key_above(&cell("alice", 2), NAV)).is_equal_to(Some(k("phone")));
            assert_that!(d.key_above(&cell("bob", 1), NAV)).is_equal_to(Some(cell("alice", 1)));
            // From a column to its group, and down again.
            assert_that!(d.key_above(&k("phone"), NAV)).is_equal_to(Some(k("contact")));
            assert_that!(d.key_above(&k("contact"), NAV)).is_none();
            assert_that!(d.key_below(&k("contact"), NAV)).is_equal_to(Some(k("email")));
            assert_that!(d.key_below(&k("email"), NAV)).is_equal_to(Some(cell("alice", 1)));
            // The first key is the first body row.
            assert_that!(d.first_key(None, false)).is_equal_to(Some(k("alice")));
        });
    }

    #[test]
    fn left_and_right_wrap_within_a_header_row() {
        Owner::new().with(|| {
            let d = delegate(GridFocusMode::Row, WritingDirection::Ltr);
            assert_that!(d.key_right_of(&k("name"), NAV)).is_equal_to(Some(k("email")));
            assert_that!(d.key_right_of(&k("phone"), NAV)).is_equal_to(Some(k("name")));
            assert_that!(d.key_left_of(&k("name"), NAV)).is_equal_to(Some(k("phone")));
            // The group row has a placeholder and one group: it wraps onto itself.
            assert_that!(d.key_right_of(&k("contact"), NAV)).is_equal_to(Some(k("contact")));
            assert_that!(d.first_key(Some(&k("phone")), false)).is_equal_to(Some(k("name")));
            assert_that!(d.last_key(Some(&k("name")), true)).is_equal_to(Some(cell("bob", 2)));
            let rtl = delegate(GridFocusMode::Row, WritingDirection::Rtl);
            assert_that!(rtl.key_left_of(&k("name"), NAV)).is_equal_to(Some(k("email")));
        });
    }

    /// Rows of 20px: the group header row, the column header row, then Alice and Bob; a page
    /// is 100px.
    struct RowsLayout;

    impl LayoutDelegate for RowsLayout {
        fn item_rect(&self, key: &Key) -> Option<Rect> {
            // Cells (`row-index`) are in their row.
            let key = key.to_string();
            let y = match key.split('-').next().unwrap_or_default() {
                "contact" => 0.0,
                "name" | "email" | "phone" => 20.0,
                "alice" => 40.0,
                "bob" => 60.0,
                _ => return None,
            };
            Some(Rect::new(0.0, y, 300.0, 20.0))
        }
        fn visible_rect(&self) -> Rect {
            Rect::new(0.0, 0.0, 300.0, 100.0)
        }
        fn content_size(&self) -> Size {
            Size::new(300.0, 80.0)
        }
    }

    #[test]
    fn page_up_and_down_reach_the_column_headers() {
        Owner::new().with(|| {
            let d = delegate_with_layout(
                GridFocusMode::Row,
                WritingDirection::Ltr,
                Arc::new(RowsLayout),
            );
            // Up through the rows into the header row (react-aria's paging steps with the
            // table's own `getKeyAbove`).
            assert_that!(d.key_page_above(&k("bob"))).is_equal_to(Some(k("name")));
            // Down from a column header: its cells.
            assert_that!(d.key_page_below(&k("name"))).is_equal_to(Some(cell("bob", 0)));
        });
    }

    #[test]
    fn type_ahead_matches_rows_and_row_headers() {
        Owner::new().with(|| {
            let d = delegate(GridFocusMode::Cell, WritingDirection::Ltr);
            assert_that!(d.key_for_search("b", None)).is_equal_to(Some(k("bob")));
            // Searching from a cell finds the row header cell.
            assert_that!(d.key_for_search("ali", Some(&cell("bob", 2))))
                .is_equal_to(Some(k("alice")));
        });
    }
}
