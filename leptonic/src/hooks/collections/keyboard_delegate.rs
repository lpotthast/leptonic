// Upstream: react-aria/src/selection/ListKeyboardDelegate.ts @ 99e6102368
use std::sync::Arc;

use leptos::prelude::{Memo, Signal, WithUntracked};

use super::{CollectionMemo, Key, LayoutDelegate, Rect, SelectionManager};
use crate::utils::{filter::Collator, i18n::WritingDirection, orientation::Orientation};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `KeyboardDelegate` is a trait whose methods default to "no such key". react-aria deletes
//   `getKeyLeftOf`/`getKeyRightOf` from vertical stacks at runtime; returning `None` has the same
//   effect (the key press isn't handled).
// - `{includeDisabled}` option objects are `NavigationOptions`.
// - All geometry (item rects, scrollability, reversed layouts) goes through `LayoutDelegate`,
//   instead of the delegate querying the DOM itself.
// - Disabled items are determined by the `SelectionManager` (one source of truth).
//
// =============================================================================

/// Options for keyboard navigation queries.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NavigationOptions {
    /// Also land on disabled items (e.g. as drop targets during drag and drop).
    pub include_disabled: bool,
}

/// Answers "which item is next?" for keyboard navigation. Each layout (list, grid, table, tabs)
/// has its own implementation. Every method returns `None` when there is no such item.
pub trait KeyboardDelegate: Send + Sync {
    /// The item visually below `key`.
    fn key_below(&self, _key: &Key, _options: NavigationOptions) -> Option<Key> {
        None
    }
    /// The item visually above `key`.
    fn key_above(&self, _key: &Key, _options: NavigationOptions) -> Option<Key> {
        None
    }
    /// The item visually left of `key`.
    fn key_left_of(&self, _key: &Key, _options: NavigationOptions) -> Option<Key> {
        None
    }
    /// The item visually right of `key`.
    fn key_right_of(&self, _key: &Key, _options: NavigationOptions) -> Option<Key> {
        None
    }
    /// The item one page below `key`.
    fn key_page_below(&self, _key: &Key) -> Option<Key> {
        None
    }
    /// The item one page above `key`.
    fn key_page_above(&self, _key: &Key) -> Option<Key> {
        None
    }
    /// The first item. `global` (Ctrl+Home in grids) means the first item of the whole collection
    /// instead of the first item of `from`'s row.
    fn first_key(&self, _from: Option<&Key>, _global: bool) -> Option<Key> {
        None
    }
    /// The last item, see [`first_key`](Self::first_key).
    fn last_key(&self, _from: Option<&Key>, _global: bool) -> Option<Key> {
        None
    }
    /// The first item, starting at `from`, whose text starts with `search` (type-ahead).
    fn key_for_search(&self, _search: &str, _from: Option<&Key>) -> Option<Key> {
        None
    }
}

/// A keyboard delegate as collection hooks take it: `build` runs again only when a signal it reads
/// (the locale's reading direction, an orientation, ...) changes, not on every read.
pub fn keyboard_delegate_memo(
    build: impl Fn() -> Arc<dyn KeyboardDelegate> + Send + Sync + 'static,
) -> Signal<Arc<dyn KeyboardDelegate>> {
    // A rebuilt delegate always counts as changed (delegates can't be compared).
    Memo::new_with_compare(move |_| build(), |_, _| true).into()
}

/// How the items of a list are laid out.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ListLayout {
    /// One item per row (or column, for horizontal lists).
    #[default]
    Stack,
    /// Items wrap into rows (e.g. a card view). Up/down find the item in the same column.
    Grid,
}

/// Keyboard navigation for lists: listboxes, menus, grid lists, tag groups, ...
#[derive(Clone)]
pub struct ListKeyboardDelegate {
    collection: CollectionMemo,
    selection: SelectionManager,
    layout: ListLayout,
    orientation: Orientation,
    direction: WritingDirection,
    layout_delegate: Arc<dyn LayoutDelegate>,
    collator: Option<Arc<Collator>>,
}

impl std::fmt::Debug for ListKeyboardDelegate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListKeyboardDelegate")
            .field("layout", &self.layout)
            .field("orientation", &self.orientation)
            .field("direction", &self.direction)
            .finish_non_exhaustive()
    }
}

impl ListKeyboardDelegate {
    /// A vertical stack in LTR, without type-ahead.
    pub fn new(
        collection: CollectionMemo,
        selection: SelectionManager,
        layout_delegate: Arc<dyn LayoutDelegate>,
    ) -> Self {
        Self {
            collection,
            selection,
            layout: ListLayout::Stack,
            orientation: Orientation::Vertical,
            direction: WritingDirection::Ltr,
            layout_delegate,
            collator: None,
        }
    }

    #[must_use]
    pub fn with_layout(mut self, layout: ListLayout) -> Self {
        self.layout = layout;
        self
    }

    #[must_use]
    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    #[must_use]
    pub fn with_direction(mut self, direction: WritingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Enable type-ahead, comparing text with `collator` (usually base sensitivity, so that
    /// case and accents are ignored).
    #[must_use]
    pub fn with_collator(mut self, collator: Arc<Collator>) -> Self {
        self.collator = Some(collator);
        self
    }

    fn is_disabled(&self, key: &Key) -> bool {
        leptos::prelude::untrack(|| self.selection.is_disabled(key))
    }

    /// Starting at `key` (inclusive), the first item that can be navigated to.
    fn find_next_non_disabled(
        &self,
        key: Option<Key>,
        step: impl Fn(&Key) -> Option<Key>,
        options: NavigationOptions,
    ) -> Option<Key> {
        let mut key = key;
        while let Some(k) = key {
            let is_item = self
                .collection
                .with_untracked(|c| c.get(&k).is_some_and(super::Node::is_item));
            if is_item && (options.include_disabled || !self.is_disabled(&k)) {
                return Some(k);
            }
            key = step(&k);
        }
        None
    }

    fn after(&self, key: &Key) -> Option<Key> {
        self.collection
            .with_untracked(|c| c.key_after(key).cloned())
    }

    fn before(&self, key: &Key) -> Option<Key> {
        self.collection
            .with_untracked(|c| c.key_before(key).cloned())
    }

    /// The next item in collection order.
    pub fn next_key(&self, key: &Key, options: NavigationOptions) -> Option<Key> {
        self.find_next_non_disabled(self.after(key), |k| self.after(k), options)
    }

    /// The previous item in collection order.
    pub fn previous_key(&self, key: &Key, options: NavigationOptions) -> Option<Key> {
        self.find_next_non_disabled(self.before(key), |k| self.before(k), options)
    }

    /// Step with `next` until the item no longer `skip`s relative to `key`'s rect, e.g. to find
    /// the item in the same column of the next row.
    fn find_key(
        &self,
        key: &Key,
        next: impl Fn(&Key) -> Option<Key>,
        skip: impl Fn(&Rect, &Rect) -> bool,
    ) -> Option<Key> {
        let start = self.layout_delegate.item_rect(key)?;
        let mut candidate = next(key);
        while let Some(k) = &candidate {
            match self.layout_delegate.item_rect(k) {
                Some(rect) if skip(&start, &rect) => candidate = next(k),
                _ => break,
            }
        }
        candidate
    }

    fn is_same_row(a: &Rect, b: &Rect) -> bool {
        a.y == b.y || a.x != b.x
    }

    fn is_same_column(a: &Rect, b: &Rect) -> bool {
        a.x == b.x || a.y != b.y
    }

    /// Whether the list is laid out bottom to top (`flex-direction: column-reverse`).
    fn is_reversed(&self, key: &Key) -> bool {
        let rect = |k: &Key| self.layout_delegate.item_rect(k);
        let options = NavigationOptions::default();
        if let Some(next) = self.next_key(key, options) {
            return matches!((rect(key), rect(&next)), (Some(a), Some(b)) if a.y > b.y);
        }
        if let Some(previous) = self.previous_key(key, options) {
            return matches!((rect(&previous), rect(key)), (Some(a), Some(b)) if a.y > b.y);
        }
        false
    }

    /// Previous or next item, depending on the writing direction.
    fn next_column(
        &self,
        key: &Key,
        towards_start: bool,
        options: NavigationOptions,
    ) -> Option<Key> {
        if towards_start {
            self.previous_key(key, options)
        } else {
            self.next_key(key, options)
        }
    }

    fn horizontal(
        &self,
        key: &Key,
        towards_start: bool,
        options: NavigationOptions,
    ) -> Option<Key> {
        match (self.layout, self.orientation) {
            (ListLayout::Grid, Orientation::Vertical)
            | (ListLayout::Stack, Orientation::Horizontal) => {
                self.next_column(key, towards_start, options)
            }
            (ListLayout::Grid, Orientation::Horizontal) => self.find_key(
                key,
                |k| self.next_column(k, towards_start, options),
                Self::is_same_column,
            ),
            (ListLayout::Stack, Orientation::Vertical) => None,
        }
    }
}

impl KeyboardDelegate for ListKeyboardDelegate {
    fn key_below(&self, key: &Key, options: NavigationOptions) -> Option<Key> {
        match (self.layout, self.orientation) {
            (ListLayout::Grid, Orientation::Vertical) => {
                self.find_key(key, |k| self.next_key(k, options), Self::is_same_row)
            }
            (_, Orientation::Vertical) if self.is_reversed(key) => self.previous_key(key, options),
            _ => self.next_key(key, options),
        }
    }

    fn key_above(&self, key: &Key, options: NavigationOptions) -> Option<Key> {
        match (self.layout, self.orientation) {
            (ListLayout::Grid, Orientation::Vertical) => {
                self.find_key(key, |k| self.previous_key(k, options), Self::is_same_row)
            }
            (_, Orientation::Vertical) if self.is_reversed(key) => self.next_key(key, options),
            _ => self.previous_key(key, options),
        }
    }

    fn key_right_of(&self, key: &Key, options: NavigationOptions) -> Option<Key> {
        self.horizontal(key, self.direction == WritingDirection::Rtl, options)
    }

    fn key_left_of(&self, key: &Key, options: NavigationOptions) -> Option<Key> {
        self.horizontal(key, self.direction == WritingDirection::Ltr, options)
    }

    fn first_key(&self, _from: Option<&Key>, _global: bool) -> Option<Key> {
        let first = self.collection.with_untracked(|c| c.first_key().cloned());
        self.find_next_non_disabled(first, |k| self.after(k), NavigationOptions::default())
    }

    fn last_key(&self, _from: Option<&Key>, _global: bool) -> Option<Key> {
        let last = self.collection.with_untracked(|c| c.last_key().cloned());
        self.find_next_non_disabled(last, |k| self.before(k), NavigationOptions::default())
    }

    fn key_page_above(&self, key: &Key) -> Option<Key> {
        let mut rect = self.layout_delegate.item_rect(key)?;
        let reversed = self.is_reversed(key);
        if !self.layout_delegate.is_scrollable() {
            return self.first_key(None, false);
        }
        let visible = self.layout_delegate.visible_rect();
        let options = NavigationOptions::default();
        let mut current = Some(key.clone());
        if self.orientation == Orientation::Horizontal {
            let page_x = (rect.x + rect.width - visible.width).max(0.0);
            while rect.x > page_x {
                current = self.key_above(current.as_ref()?, options);
                match current
                    .as_ref()
                    .and_then(|k| self.layout_delegate.item_rect(k))
                {
                    Some(r) => rect = r,
                    None => break,
                }
            }
        } else {
            let page_y = if reversed {
                rect.y - visible.height
            } else {
                (rect.y + rect.height - visible.height).max(0.0)
            };
            while rect.y > page_y {
                current = self.key_above(current.as_ref()?, options);
                match current
                    .as_ref()
                    .and_then(|k| self.layout_delegate.item_rect(k))
                {
                    Some(r) => rect = r,
                    None => break,
                }
            }
        }
        current.or_else(|| {
            if reversed {
                self.last_key(None, false)
            } else {
                self.first_key(None, false)
            }
        })
    }

    fn key_page_below(&self, key: &Key) -> Option<Key> {
        let mut rect = self.layout_delegate.item_rect(key)?;
        let reversed = self.is_reversed(key);
        if !self.layout_delegate.is_scrollable() {
            return self.last_key(None, false);
        }
        let visible = self.layout_delegate.visible_rect();
        let content = self.layout_delegate.content_size();
        let options = NavigationOptions::default();
        let mut current = Some(key.clone());
        if self.orientation == Orientation::Horizontal {
            let page_x = content.width.min(rect.x - rect.width + visible.width);
            while rect.x < page_x {
                current = self.key_below(current.as_ref()?, options);
                match current
                    .as_ref()
                    .and_then(|k| self.layout_delegate.item_rect(k))
                {
                    Some(r) => rect = r,
                    None => break,
                }
            }
        } else {
            let page_y = content.height.min(rect.y - rect.height + visible.height);
            while rect.y < page_y {
                current = self.key_below(current.as_ref()?, options);
                match current
                    .as_ref()
                    .and_then(|k| self.layout_delegate.item_rect(k))
                {
                    Some(r) => rect = r,
                    None => break,
                }
            }
        }
        current.or_else(|| {
            if reversed {
                self.first_key(None, false)
            } else {
                self.last_key(None, false)
            }
        })
    }

    fn key_for_search(&self, search: &str, from: Option<&Key>) -> Option<Key> {
        let collator = self.collator.as_ref()?;
        let search_len = search.chars().count();
        let mut key = from.cloned().or_else(|| self.first_key(None, false));
        while let Some(k) = key {
            let matches = self.collection.with_untracked(|c| {
                c.get(&k).is_some_and(|node| {
                    let prefix: String = node.text_value.chars().take(search_len).collect();
                    !node.text_value.is_empty()
                        && collator.compare(&prefix, search) == std::cmp::Ordering::Equal
                })
            });
            if matches {
                return Some(k);
            }
            key = self.next_key(&k, NavigationOptions::default());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use assertr::prelude::*;

    use super::*;
    use crate::{
        hooks::collections::{Collection, SelectionMode, SelectionOptions, Size},
        utils::{filter::CollatorOptions, i18n::Locale},
    };

    /// Items laid out in a grid with `columns` columns of 100x20 rects, in a viewport of the
    /// given height.
    struct FakeLayout {
        rects: HashMap<Key, Rect>,
        viewport_height: f64,
        content_height: f64,
    }

    impl FakeLayout {
        fn grid(keys: &[&str], columns: usize, viewport_height: f64) -> Self {
            let rects: HashMap<Key, Rect> = keys
                .iter()
                .enumerate()
                .map(|(i, k)| {
                    let (row, col) = (i / columns, i % columns);
                    #[allow(clippy::cast_precision_loss)]
                    let rect = Rect {
                        x: col as f64 * 100.0,
                        y: row as f64 * 20.0,
                        width: 100.0,
                        height: 20.0,
                    };
                    (Key::from(*k), rect)
                })
                .collect();
            #[allow(clippy::cast_precision_loss)]
            let content_height = keys.len().div_ceil(columns) as f64 * 20.0;
            Self {
                rects,
                viewport_height,
                content_height,
            }
        }
    }

    impl LayoutDelegate for FakeLayout {
        fn item_rect(&self, key: &Key) -> Option<Rect> {
            self.rects.get(key).copied()
        }
        fn visible_rect(&self) -> Rect {
            Rect {
                x: 0.0,
                y: 0.0,
                width: 300.0,
                height: self.viewport_height,
            }
        }
        fn content_size(&self) -> Size {
            Size {
                width: 300.0,
                height: self.content_height,
            }
        }
    }

    const KEYS: [&str; 6] = ["apple", "banana", "cherry", "durian", "elderberry", "fig"];

    /// Items flowing top to bottom, then into the next column (a horizontal grid), `rows` per
    /// column.
    fn column_layout(keys: &[&str], rows: usize) -> FakeLayout {
        let mut layout = FakeLayout::grid(keys, 1, 200.0);
        for (i, key) in keys.iter().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let rect = Rect {
                x: (i / rows) as f64 * 100.0,
                y: (i % rows) as f64 * 20.0,
                width: 100.0,
                height: 20.0,
            };
            layout.rects.insert(Key::from(*key), rect);
        }
        layout
    }

    fn delegate(columns: usize, viewport_height: f64, disabled: &[&str]) -> ListKeyboardDelegate {
        let collection: CollectionMemo = Memo::new(|_| {
            Arc::new(Collection::build(|b| {
                for key in KEYS {
                    let mut text = key.to_owned();
                    text[..1].make_ascii_uppercase();
                    b.item(key, text);
                }
            }))
        });
        let disabled: std::collections::HashSet<Key> =
            disabled.iter().map(|k| Key::from(*k)).collect();
        let selection = SelectionManager::new(
            collection,
            SelectionOptions {
                selection_mode: Signal::stored(SelectionMode::Multiple),
                disabled_keys: Signal::stored(disabled),
                ..Default::default()
            },
        );
        let layout = Arc::new(FakeLayout::grid(&KEYS, columns, viewport_height));
        ListKeyboardDelegate::new(collection, selection, layout).with_collator(Arc::new(
            Collator::new(&Locale::default(), &CollatorOptions::default()),
        ))
    }

    fn k(s: &str) -> Key {
        Key::from(s)
    }

    const NAV: NavigationOptions = NavigationOptions {
        include_disabled: false,
    };

    #[test]
    fn vertical_stack_skips_disabled_items_and_has_no_horizontal_navigation() {
        crate::testing::with_owner(|| {
            let d = delegate(1, 200.0, &["banana"]);
            assert_that!(d.key_below(&k("apple"), NAV)).is_equal_to(Some(k("cherry")));
            assert_that!(d.key_above(&k("cherry"), NAV)).is_equal_to(Some(k("apple")));
            assert_that!(d.key_below(&k("fig"), NAV)).is_none();
            assert_that!(d.key_right_of(&k("apple"), NAV)).is_none();
            let including = NavigationOptions {
                include_disabled: true,
            };
            assert_that!(d.key_below(&k("apple"), including)).is_equal_to(Some(k("banana")));
        });
    }

    #[test]
    fn first_and_last_skip_disabled_items() {
        crate::testing::with_owner(|| {
            let d = delegate(1, 200.0, &["apple", "fig"]);
            assert_that!(d.first_key(None, false)).is_equal_to(Some(k("banana")));
            assert_that!(d.last_key(None, false)).is_equal_to(Some(k("elderberry")));
        });
    }

    // Upstream: ListBox.test.js "should not throw TypeError at boundaries of vertical grid
    // layout when keyboard navigating (up/down)".
    #[test]
    fn grid_layout_stops_at_its_edges() {
        crate::testing::with_owner(|| {
            // apple  banana     cherry
            // durian elderberry fig
            let d = delegate(3, 200.0, &[]).with_layout(ListLayout::Grid);
            assert_that!(d.key_above(&k("banana"), NAV)).is_none();
            assert_that!(d.key_below(&k("elderberry"), NAV)).is_none();
            assert_that!(d.key_below(&k("fig"), NAV)).is_none();
        });
    }

    // Upstream: ListBox.test.js "should support horizontal grid layout", "should not throw
    // TypeError at boundaries of horizontal grid layout when keyboard navigating (left/right)".
    #[test]
    fn horizontal_grid_layout_moves_by_row_and_column() {
        crate::testing::with_owner(|| {
            // apple  cherry elderberry
            // banana durian fig
            let base = delegate(1, 200.0, &[]);
            let d = ListKeyboardDelegate::new(
                base.collection,
                base.selection,
                Arc::new(column_layout(&KEYS, 2)),
            )
            .with_layout(ListLayout::Grid)
            .with_orientation(Orientation::Horizontal);
            assert_that!(d.key_right_of(&k("apple"), NAV)).is_equal_to(Some(k("cherry")));
            assert_that!(d.key_right_of(&k("durian"), NAV)).is_equal_to(Some(k("fig")));
            assert_that!(d.key_left_of(&k("durian"), NAV)).is_equal_to(Some(k("banana")));
            assert_that!(d.key_below(&k("apple"), NAV)).is_equal_to(Some(k("banana")));
            assert_that!(d.key_above(&k("cherry"), NAV)).is_equal_to(Some(k("banana")));
            // The edges.
            assert_that!(d.key_right_of(&k("elderberry"), NAV)).is_none();
            assert_that!(d.key_left_of(&k("banana"), NAV)).is_none();
            assert_that!(d.key_above(&k("apple"), NAV)).is_none();
            assert_that!(d.key_below(&k("fig"), NAV)).is_none();
        });
    }

    #[test]
    fn grid_layout_moves_by_column_and_row() {
        crate::testing::with_owner(|| {
            // apple  banana     cherry
            // durian elderberry fig
            let d = delegate(3, 200.0, &[]).with_layout(ListLayout::Grid);
            assert_that!(d.key_below(&k("banana"), NAV)).is_equal_to(Some(k("elderberry")));
            assert_that!(d.key_above(&k("fig"), NAV)).is_equal_to(Some(k("cherry")));
            assert_that!(d.key_right_of(&k("apple"), NAV)).is_equal_to(Some(k("banana")));
            assert_that!(d.key_left_of(&k("banana"), NAV)).is_equal_to(Some(k("apple")));
            let rtl = d.with_direction(WritingDirection::Rtl);
            assert_that!(rtl.key_right_of(&k("banana"), NAV)).is_equal_to(Some(k("apple")));
        });
    }

    #[test]
    fn horizontal_stack_uses_left_and_right() {
        crate::testing::with_owner(|| {
            let d = delegate(6, 200.0, &[]).with_orientation(Orientation::Horizontal);
            assert_that!(d.key_right_of(&k("apple"), NAV)).is_equal_to(Some(k("banana")));
            assert_that!(d.key_left_of(&k("banana"), NAV)).is_equal_to(Some(k("apple")));
        });
    }

    #[test]
    fn page_up_and_down_move_by_a_viewport() {
        crate::testing::with_owner(|| {
            // 6 rows of 20px in a 50px viewport: a page is about two items.
            let d = delegate(1, 50.0, &[]);
            assert_that!(d.key_page_below(&k("apple"))).is_equal_to(Some(k("cherry")));
            assert_that!(d.key_page_above(&k("fig"))).is_equal_to(Some(k("durian")));
            // Not scrollable: jump to the ends.
            let d = delegate(1, 500.0, &[]);
            assert_that!(d.key_page_below(&k("apple"))).is_equal_to(Some(k("fig")));
            assert_that!(d.key_page_above(&k("fig"))).is_equal_to(Some(k("apple")));
        });
    }

    #[test]
    fn type_ahead_matches_prefixes_ignoring_case_and_skips_disabled_items() {
        crate::testing::with_owner(|| {
            let d = delegate(1, 200.0, &["durian"]);
            assert_that!(d.key_for_search("ch", None)).is_equal_to(Some(k("cherry")));
            assert_that!(d.key_for_search("E", None)).is_equal_to(Some(k("elderberry")));
            assert_that!(d.key_for_search("d", None)).is_none();
            // Searching from a key starts there.
            assert_that!(d.key_for_search("a", Some(&k("banana")))).is_none();
        });
    }
}
