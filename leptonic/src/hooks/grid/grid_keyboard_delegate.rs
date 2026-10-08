// Upstream: react-aria/src/grid/GridKeyboardDelegate.ts @ 99e6102368
use std::sync::Arc;

use leptos::prelude::*;

use crate::{
    hooks::collections::{
        Collection, CollectionMemo, Key, KeyboardDelegate, LayoutDelegate, NavigationOptions, Node,
        NodeKind, SelectionManager,
    },
    utils::{filter::Collator, locale::WritingDirection},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation (the deprecated `layout`
// option is not offered).
//
// =============================================================================

/// What arrow keys focus in a grid.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum GridFocusMode {
    /// Rows; ArrowLeft/ArrowRight move into a row's cells.
    #[default]
    Row,
    /// Cells only.
    Cell,
}

/// Keyboard navigation for grids: rows and cells in two dimensions.
#[derive(Clone)]
pub struct GridKeyboardDelegate {
    collection: CollectionMemo,
    selection: SelectionManager,
    layout_delegate: Arc<dyn LayoutDelegate>,
    direction: WritingDirection,
    collator: Option<Arc<Collator>>,
    focus_mode: GridFocusMode,
}

impl std::fmt::Debug for GridKeyboardDelegate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GridKeyboardDelegate")
            .field("direction", &self.direction)
            .field("focus_mode", &self.focus_mode)
            .finish_non_exhaustive()
    }
}

impl GridKeyboardDelegate {
    /// The key a page above `from`, stepping up with `key_above` (react-aria's
    /// `getKeyPageAbove`, which steps with the possibly overridden `getKeyAbove`: a table's
    /// delegate reaches its column headers this way).
    pub fn key_page_above_with(
        &self,
        from: &Key,
        key_above: impl Fn(&Key) -> Option<Key>,
    ) -> Option<Key> {
        let mut key = from.clone();
        let mut rect = self.layout_delegate.item_rect(&key)?;
        let page_y = (rect.y + rect.height - self.layout_delegate.visible_rect().height).max(0.0);
        while rect.y > page_y {
            let Some(above) = key_above(&key) else {
                break;
            };
            key = above;
            match self.layout_delegate.item_rect(&key) {
                Some(r) => rect = r,
                None => break,
            }
        }
        Some(key)
    }

    /// The key a page below `from`, stepping down with `key_below` (see
    /// [`GridKeyboardDelegate::key_page_above_with`]).
    pub fn key_page_below_with(
        &self,
        from: &Key,
        key_below: impl Fn(&Key) -> Option<Key>,
    ) -> Option<Key> {
        let mut key = from.clone();
        let mut rect = self.layout_delegate.item_rect(&key)?;
        let page_height = self.layout_delegate.visible_rect().height;
        let page_y = self
            .layout_delegate
            .content_size()
            .height
            .min(rect.y + page_height);
        while rect.y + rect.height < page_y {
            let Some(below) = key_below(&key) else {
                break;
            };
            let Some(below_rect) = self.layout_delegate.item_rect(&below) else {
                key = below;
                break;
            };
            rect = below_rect;
            key = below;
        }
        Some(key)
    }

    pub fn new(
        collection: CollectionMemo,
        selection: SelectionManager,
        layout_delegate: Arc<dyn LayoutDelegate>,
    ) -> Self {
        Self {
            collection,
            selection,
            layout_delegate,
            direction: WritingDirection::Ltr,
            collator: None,
            focus_mode: GridFocusMode::Row,
        }
    }

    #[must_use]
    pub fn with_direction(mut self, direction: WritingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Enable type-ahead (on rows' text), comparing with `collator`.
    #[must_use]
    pub fn with_collator(mut self, collator: Arc<Collator>) -> Self {
        self.collator = Some(collator);
        self
    }

    #[must_use]
    pub fn with_focus_mode(mut self, focus_mode: GridFocusMode) -> Self {
        self.focus_mode = focus_mode;
        self
    }

    fn rtl(&self) -> bool {
        self.direction == WritingDirection::Rtl
    }

    fn with<R>(&self, f: impl FnOnce(&Collection) -> R) -> R {
        self.collection.with_untracked(|c| f(c))
    }

    pub(crate) fn is_disabled(&self, key: &Key) -> bool {
        untrack(|| self.selection.is_disabled(key))
    }

    fn is_cell(node: &Node) -> bool {
        node.kind == NodeKind::Cell
    }

    fn is_row(node: &Node) -> bool {
        node.kind == NodeKind::Item
    }

    /// From `from` (exclusive; the end when `None`), the previous key passing `pred`.
    fn find_previous_key(
        &self,
        from: Option<&Key>,
        pred: impl Fn(&Node) -> bool,
        include_disabled: bool,
    ) -> Option<Key> {
        self.with(|c| {
            let mut key = match from {
                Some(from) => c.key_before(from).cloned(),
                None => c.last_key().cloned(),
            };
            while let Some(k) = key {
                let node = c.get(&k)?;
                if (include_disabled || !self.is_disabled(&k)) && pred(node) {
                    return Some(k);
                }
                key = c.key_before(&k).cloned();
            }
            None
        })
    }

    /// From `from` (exclusive; the start when `None`), the next key passing `pred`.
    fn find_next_key(
        &self,
        from: Option<&Key>,
        pred: impl Fn(&Node) -> bool,
        include_disabled: bool,
    ) -> Option<Key> {
        self.with(|c| {
            let mut key = match from {
                Some(from) => c.key_after(from).cloned(),
                None => c.first_key().cloned(),
            };
            while let Some(k) = key {
                let node = c.get(&k)?;
                if (include_disabled || !self.is_disabled(&k)) && pred(node) {
                    return Some(k);
                }
                key = c.key_after(&k).cloned();
            }
            None
        })
    }

    /// The first enabled row after `from` (from the start when `None`), among the rows shown (a
    /// tree's expanded rows).
    pub(crate) fn row_after(&self, from: Option<&Key>) -> Option<Key> {
        self.find_next_key(from, Self::is_row, false)
    }

    fn cells(c: &Collection, row: &Key) -> Vec<Key> {
        c.cells(row).map(|n| n.key.clone()).collect()
    }

    fn first_cell(&self, row: &Key) -> Option<Key> {
        self.with(|c| Self::cells(c, row).into_iter().next())
    }

    fn last_cell(&self, row: &Key) -> Option<Key> {
        self.with(|c| Self::cells(c, row).pop())
    }

    /// The cell of `row` covering column `index` (respecting column spans).
    pub(crate) fn key_for_item_in_row_by_index(&self, row: &Key, index: usize) -> Option<Key> {
        self.with(|c| {
            let mut i = 0;
            for child in c.cells(row) {
                if let Some(span) = child.col_span
                    && span + i > index
                {
                    return Some(child.key.clone());
                }
                if let Some(span) = child.col_span {
                    i += span - 1;
                }
                if i == index {
                    return Some(child.key.clone());
                }
                i += 1;
            }
            None
        })
    }

    fn vertical(&self, from: &Key, options: NavigationOptions, below: bool) -> Option<Key> {
        let start = self.with(|c| c.get(from).cloned())?;
        let row = if Self::is_cell(&start) {
            start.parent_key.clone()?
        } else {
            from.clone()
        };
        let is_item = |n: &Node| n.kind == NodeKind::Item;
        let key = if below {
            self.find_next_key(Some(&row), is_item, options.include_disabled)
        } else {
            self.find_previous_key(Some(&row), is_item, options.include_disabled)
        }?;
        if Self::is_cell(&start) {
            let index = start.col_index.unwrap_or(start.index);
            return self.key_for_item_in_row_by_index(&key, index);
        }
        (self.focus_mode == GridFocusMode::Row).then_some(key)
    }

    fn horizontal(&self, key: &Key, right: bool) -> Option<Key> {
        let node = self.with(|c| c.get(key).cloned())?;
        // Moving right means forward in left-to-right text.
        let forward = right != self.rtl();
        if Self::is_row(&node) {
            return if forward {
                self.first_cell(key)
            } else {
                self.last_cell(key)
            };
        }
        if Self::is_cell(&node) {
            let parent = node.parent_key.clone()?;
            let neighbor = self.with(|c| {
                let cells = Self::cells(c, &parent);
                let index = if forward {
                    node.index.checked_add(1)
                } else {
                    node.index.checked_sub(1)
                }?;
                cells.get(index).cloned()
            });
            if neighbor.is_some() {
                return neighbor;
            }
            if self.focus_mode == GridFocusMode::Row {
                return Some(parent);
            }
            return if forward {
                self.last_key(Some(key), false)
            } else {
                self.first_key(Some(key), false)
            };
        }
        None
    }
}

impl KeyboardDelegate for GridKeyboardDelegate {
    fn key_below(&self, key: &Key, options: NavigationOptions) -> Option<Key> {
        self.vertical(key, options, true)
    }

    fn key_above(&self, key: &Key, options: NavigationOptions) -> Option<Key> {
        self.vertical(key, options, false)
    }

    fn key_right_of(&self, key: &Key, _options: NavigationOptions) -> Option<Key> {
        self.horizontal(key, true)
    }

    fn key_left_of(&self, key: &Key, _options: NavigationOptions) -> Option<Key> {
        self.horizontal(key, false)
    }

    fn first_key(&self, from: Option<&Key>, global: bool) -> Option<Key> {
        let from_node = from.and_then(|k| self.with(|c| c.get(k).cloned()));
        if let Some(node) = &from_node
            && Self::is_cell(node)
            && !global
        {
            return self.first_cell(node.parent_key.as_ref()?);
        }
        let key = self.find_next_key(None, |n| n.kind == NodeKind::Item, false)?;
        let to_cell = from_node
            .as_ref()
            .is_some_and(|n| Self::is_cell(n) && global)
            || self.focus_mode == GridFocusMode::Cell;
        if to_cell {
            return self.first_cell(&key);
        }
        Some(key)
    }

    fn last_key(&self, from: Option<&Key>, global: bool) -> Option<Key> {
        let from_node = from.and_then(|k| self.with(|c| c.get(k).cloned()));
        if let Some(node) = &from_node
            && Self::is_cell(node)
            && !global
        {
            return self.last_cell(node.parent_key.as_ref()?);
        }
        let key = self.find_previous_key(None, |n| n.kind == NodeKind::Item, false)?;
        let to_cell = from_node
            .as_ref()
            .is_some_and(|n| Self::is_cell(n) && global)
            || self.focus_mode == GridFocusMode::Cell;
        if to_cell {
            return self.last_cell(&key);
        }
        Some(key)
    }

    fn key_page_above(&self, from: &Key) -> Option<Key> {
        self.key_page_above_with(from, |key| {
            self.key_above(key, NavigationOptions::default())
        })
    }

    fn key_page_below(&self, from: &Key) -> Option<Key> {
        self.key_page_below_with(from, |key| {
            self.key_below(key, NavigationOptions::default())
        })
    }

    fn key_for_search(&self, search: &str, from: Option<&Key>) -> Option<Key> {
        let collator = self.collator.as_ref()?;
        let start = match from {
            Some(from) => from.clone(),
            None => self.first_key(None, false)?,
        };
        let mut key = self.with(|c| {
            let node = c.get(&start)?;
            if Self::is_cell(node) {
                node.parent_key.clone()
            } else {
                Some(start.clone())
            }
        });
        let search_len = search.chars().count();
        let mut has_wrapped = false;
        while let Some(k) = key {
            let node = self.with(|c| c.get(&k).cloned())?;
            if !node.text_value.is_empty() {
                let prefix: String = node.text_value.chars().take(search_len).collect();
                if collator.compare(&prefix, search) == std::cmp::Ordering::Equal {
                    if Self::is_row(&node) && self.focus_mode == GridFocusMode::Cell {
                        return self.first_cell(&k);
                    }
                    return Some(k);
                }
            }
            key = self.find_next_key(Some(&k), |n| n.kind == NodeKind::Item, false);
            if key.is_none() && !has_wrapped {
                key = self.first_key(None, false);
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
        hooks::collections::{Rect, SelectionMode, SelectionOptions, Size},
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

    /// | Alice | 30 | Admin |
    /// | Bob   | 25 | User  |   (disabled with `disable_bob`)
    /// | Carol | wide (2)   |
    /// | Dave  | 40 | User  |
    fn delegate(focus_mode: GridFocusMode, disable_bob: bool) -> GridKeyboardDelegate {
        let collection: CollectionMemo = Memo::new(|_| {
            Arc::new(Collection::build(|b| {
                for (key, age, role) in [("alice", "30", "Admin"), ("bob", "25", "User")] {
                    b.row(key, key, |r| {
                        r.cell(key);
                        r.cell(age);
                        r.cell(role);
                    });
                }
                b.row("carol", "carol", |r| {
                    r.cell("carol");
                    r.cell("wide").col_span(2);
                });
                b.row("dave", "dave", |r| {
                    r.cell("dave");
                    r.cell("40");
                    r.cell("User");
                });
            }))
        });
        let disabled = if disable_bob {
            std::iter::once(Key::from("bob")).collect()
        } else {
            std::collections::HashSet::new()
        };
        let selection = SelectionManager::new(
            collection,
            SelectionOptions {
                selection_mode: Signal::stored(SelectionMode::Multiple),
                disabled_keys: Signal::stored(disabled),
                ..Default::default()
            },
        );
        GridKeyboardDelegate::new(collection, selection, Arc::new(NoLayout))
            .with_focus_mode(focus_mode)
            .with_collator(Arc::new(Collator::new(
                &Locale::default(),
                &CollatorOptions::default(),
            )))
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
    fn row_mode_moves_between_rows_and_into_cells() {
        Owner::new().with(|| {
            let d = delegate(GridFocusMode::Row, false);
            assert_that!(d.key_below(&k("alice"), NAV)).is_equal_to(Some(k("bob")));
            assert_that!(d.key_above(&k("alice"), NAV)).is_none();
            // Right enters the row at its first cell, left at its last.
            assert_that!(d.key_right_of(&k("alice"), NAV)).is_equal_to(Some(cell("alice", 0)));
            assert_that!(d.key_left_of(&k("alice"), NAV)).is_equal_to(Some(cell("alice", 2)));
            // Past the row's ends, focus returns to the row.
            assert_that!(d.key_right_of(&cell("alice", 2), NAV)).is_equal_to(Some(k("alice")));
            assert_that!(d.key_left_of(&cell("alice", 0), NAV)).is_equal_to(Some(k("alice")));
            assert_that!(d.first_key(None, false)).is_equal_to(Some(k("alice")));
            assert_that!(d.last_key(None, false)).is_equal_to(Some(k("dave")));
        });
    }

    #[test]
    fn vertical_moves_keep_the_column_and_skip_disabled_rows() {
        Owner::new().with(|| {
            let d = delegate(GridFocusMode::Row, true);
            assert_that!(d.key_below(&k("alice"), NAV)).is_equal_to(Some(k("carol")));
            assert_that!(d.key_below(&cell("alice", 1), NAV)).is_equal_to(Some(cell("carol", 1)));
            // Carol's second cell spans columns 1 and 2.
            assert_that!(d.key_above(&cell("dave", 2), NAV)).is_equal_to(Some(cell("carol", 1)));
            assert_that!(d.key_below(&cell("carol", 1), NAV)).is_equal_to(Some(cell("dave", 1)));
        });
    }

    #[test]
    fn cell_mode_stays_on_cells() {
        Owner::new().with(|| {
            let d = delegate(GridFocusMode::Cell, false);
            assert_that!(d.first_key(None, false)).is_equal_to(Some(cell("alice", 0)));
            assert_that!(d.last_key(None, false)).is_equal_to(Some(cell("dave", 2)));
            // Home/End move within the row; with Ctrl, to the first/last row.
            assert_that!(d.first_key(Some(&cell("bob", 2)), false))
                .is_equal_to(Some(cell("bob", 0)));
            assert_that!(d.last_key(Some(&cell("bob", 0)), true))
                .is_equal_to(Some(cell("dave", 2)));
            // At the end of a row, right stays at the row's last cell.
            assert_that!(d.key_right_of(&cell("bob", 2), NAV)).is_equal_to(Some(cell("bob", 2)));
            // Rows are not focus targets in cell mode.
            assert_that!(d.key_below(&k("alice"), NAV)).is_none();
        });
    }

    #[test]
    fn right_to_left_mirrors_horizontal_moves() {
        Owner::new().with(|| {
            let d = delegate(GridFocusMode::Row, false).with_direction(WritingDirection::Rtl);
            assert_that!(d.key_left_of(&k("alice"), NAV)).is_equal_to(Some(cell("alice", 0)));
            assert_that!(d.key_left_of(&cell("alice", 0), NAV)).is_equal_to(Some(cell("alice", 1)));
            assert_that!(d.key_right_of(&cell("alice", 1), NAV))
                .is_equal_to(Some(cell("alice", 0)));
        });
    }

    #[test]
    fn type_ahead_matches_row_text() {
        Owner::new().with(|| {
            let d = delegate(GridFocusMode::Row, false);
            assert_that!(d.key_for_search("ca", None)).is_equal_to(Some(k("carol")));
            assert_that!(d.key_for_search("a", Some(&k("bob")))).is_equal_to(Some(k("alice")));
            let d = delegate(GridFocusMode::Cell, false);
            assert_that!(d.key_for_search("d", None)).is_equal_to(Some(cell("dave", 0)));
        });
    }
}
