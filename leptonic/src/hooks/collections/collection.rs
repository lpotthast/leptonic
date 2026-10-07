// Upstream: react-aria/src/collections/BaseCollection.ts @ 99e6102368
use std::{cmp::Ordering, collections::HashMap, sync::Arc};

use super::{ItemLink, Key, Node, NodeKind};

/// An immutable, ordered tree of [`Node`]s: the items of a listbox, menu, select, tree, ...
///
/// Collections are built from data the application owns (see [`Collection::build`] and
/// `use_collection`), not from rendered DOM. That makes them available during server-side
/// rendering, so everything derived from them (selected text, `aria-activedescendant`, hidden
/// `<option>`s, ...) is rendered on the server and hydrates without surprises.
///
/// Traversal follows react-aria's `BaseCollection`: [`key_after`](Self::key_after) and
/// [`key_before`](Self::key_before) walk all nodes in document order, descending into sections
/// (but not into items' children, which belong to trees). Keyboard delegates skip non-item
/// nodes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Collection {
    nodes: HashMap<Key, Node>,
    first_key: Option<Key>,
    last_key: Option<Key>,
    item_count: usize,
    /// Position of every node in document order, for O(1) order comparisons.
    order: HashMap<Key, usize>,
    /// A tree's visible items (see [`Collection::with_expanded`]): navigation also visits items'
    /// children.
    is_tree_view: bool,
}

impl Collection {
    /// Build a collection.
    ///
    /// ```
    /// use leptonic::hooks::Collection;
    ///
    /// let collection = Collection::build(|b| {
    ///     b.section("fruit", |s| {
    ///         s.header("fruit-header", "Fruit");
    ///         s.item("apple", "Apple");
    ///         s.item("banana", "Banana").disabled(true);
    ///     });
    ///     b.separator("sep");
    ///     b.item("bread", "Bread");
    /// });
    /// assert_eq!(collection.size(), 3);
    /// ```
    pub fn build(f: impl FnOnce(&mut CollectionBuilder)) -> Self {
        let mut builder = CollectionBuilder::default();
        f(&mut builder);
        builder.finish()
    }

    /// The number of items (sections, headers and separators don't count).
    pub fn size(&self) -> usize {
        self.item_count
    }

    pub fn is_empty(&self) -> bool {
        self.item_count == 0
    }

    pub fn get(&self, key: &Key) -> Option<&Node> {
        self.nodes.get(key)
    }

    pub fn contains_key(&self, key: &Key) -> bool {
        self.nodes.contains_key(key)
    }

    /// The keys of all nodes (sections, headers, items, cells, ...), in no particular order.
    pub fn keys(&self) -> impl Iterator<Item = &Key> {
        self.nodes.keys()
    }

    /// The first top-level node.
    pub fn first_key(&self) -> Option<&Key> {
        self.first_key.as_ref()
    }

    /// The last node in document order (descending into the last top-level node's children).
    pub fn last_key(&self) -> Option<&Key> {
        let mut node = self.get(self.last_key.as_ref()?)?;
        while let Some(last_child) = &node.last_child_key {
            node = self.get(last_child)?;
        }
        Some(&node.key)
    }

    /// The next node in document order. Descends into sections (and other non-item nodes), but
    /// not into items' children, except in a tree view ([`Collection::with_expanded`]).
    pub fn key_after(&self, key: &Key) -> Option<&Key> {
        let mut node = self.get(key)?;
        if (node.kind != NodeKind::Item || self.is_tree_view)
            && let Some(first_child) = &node.first_child_key
        {
            return Some(first_child);
        }
        loop {
            if let Some(next) = &node.next_key {
                return Some(next);
            }
            node = self.get(node.parent_key.as_ref()?)?;
        }
    }

    /// The previous node in document order. From the first node of a section, this is the
    /// section itself.
    pub fn key_before(&self, key: &Key) -> Option<&Key> {
        let node = self.get(key)?;
        match &node.prev_key {
            Some(prev) => {
                let mut node = self.get(prev)?;
                while (node.kind != NodeKind::Item || self.is_tree_view)
                    && let Some(last_child) = &node.last_child_key
                {
                    node = self.get(last_child)?;
                }
                Some(&node.key)
            }
            None => node.parent_key.as_ref(),
        }
    }

    /// The top-level nodes.
    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        self.siblings_from(self.first_key.as_ref())
    }

    /// The children of `parent`: a section's contents or a tree item's child items.
    pub fn children(&self, parent: &Key) -> impl Iterator<Item = &Node> {
        self.siblings_from(self.get(parent).and_then(|p| p.first_child_key.as_ref()))
    }

    /// All nodes in document order, as visited by [`key_after`](Self::key_after).
    pub fn nodes_in_order(&self) -> impl Iterator<Item = &Node> {
        let mut next = self.first_key.as_ref();
        std::iter::from_fn(move || {
            let node = self.get(next?)?;
            next = self.key_after(&node.key);
            Some(node)
        })
    }

    /// All items in document order (sections are traversed, items' children are not).
    pub fn items(&self) -> impl Iterator<Item = &Node> {
        self.nodes_in_order().filter(|node| node.is_item())
    }

    /// Compares the document order of two nodes (including nested tree items). `None` if either
    /// key is not part of the collection.
    pub fn compare_order(&self, a: &Key, b: &Key) -> Option<Ordering> {
        Some(self.order.get(a)?.cmp(self.order.get(b)?))
    }

    /// `keys` in collection order; keys missing from the collection come last (sorted by key).
    pub fn sorted_keys(&self, keys: impl IntoIterator<Item = Key>) -> Vec<Key> {
        let mut keys: Vec<Key> = keys.into_iter().collect();
        keys.sort_by(|a, b| match (self.order.get(a), self.order.get(b)) {
            (Some(a), Some(b)) => a.cmp(b),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => a.cmp(b),
        });
        keys
    }

    /// The keys of all items between `from` and `to` (inclusive, in document order, either
    /// direction), e.g. for range selection. Only items at the same tree level as `from` that are
    /// reachable by [`key_after`](Self::key_after)/[`key_before`](Self::key_before) are included.
    pub fn item_keys_between(&self, from: &Key, to: &Key) -> Vec<Key> {
        let Some(ordering) = self.compare_order(from, to) else {
            return Vec::new();
        };
        let (start, end) = match ordering {
            Ordering::Greater => (to, from),
            _ => (from, to),
        };
        let mut keys = Vec::new();
        let mut current = Some(start);
        while let Some(key) = current {
            if self.get(key).is_some_and(Node::is_item) {
                keys.push(key.clone());
            }
            if key == end {
                break;
            }
            current = self.key_after(key);
        }
        keys
    }

    /// The visible part of a tree: items' children only where the item is in `expanded`
    /// (sections always show their contents). Navigation (`key_after`, `key_before`) walks the
    /// visible items in order, children included. Collapsed items keep `has_child_nodes`.
    #[must_use]
    pub fn with_expanded(&self, expanded: &std::collections::HashSet<Key>) -> Self {
        let mut collection = CollectionBuilder {
            entries: self.expanded_entries(self.first_key.as_ref(), expanded),
        }
        .finish();
        collection.is_tree_view = true;
        collection
    }

    fn expanded_entries(
        &self,
        first: Option<&Key>,
        expanded: &std::collections::HashSet<Key>,
    ) -> Vec<Entry> {
        self.siblings_from(first)
            .map(|node| {
                let mut entry = Entry::from_node(node);
                if node.kind != NodeKind::Item || expanded.contains(&node.key) {
                    entry.children = self.expanded_entries(node.first_child_key.as_ref(), expanded);
                }
                entry
            })
            .collect()
    }

    /// A new collection with only the items for which `keep(text_value, node)` returns `true`.
    ///
    /// As in react-aria: an item that is kept keeps all its children; sections that end up
    /// without items are removed; a separator at the end of a level is removed.
    #[must_use]
    pub fn filter(&self, keep: impl Fn(&str, &Node) -> bool) -> Self {
        CollectionBuilder {
            entries: self.filtered_entries(self.first_key.as_ref(), &keep),
        }
        .finish()
    }

    fn filtered_entries(
        &self,
        first: Option<&Key>,
        keep: &impl Fn(&str, &Node) -> bool,
    ) -> Vec<Entry> {
        let mut entries = Vec::new();
        for node in self.siblings_from(first) {
            match node.kind {
                NodeKind::Item => {
                    if keep(&node.text_value, node) {
                        entries.push(self.to_entry(node));
                    }
                }
                NodeKind::Section => {
                    let children = self.filtered_entries(node.first_child_key.as_ref(), keep);
                    // A section with nothing left but its header disappears.
                    if children
                        .last()
                        .is_some_and(|last| last.kind != NodeKind::Header)
                    {
                        let mut entry = Entry::from_node(node);
                        entry.children = children;
                        entries.push(entry);
                    }
                }
                // Table header rows stay as they are.
                NodeKind::HeaderRow => entries.push(self.to_entry(node)),
                NodeKind::Header
                | NodeKind::Separator
                | NodeKind::Loader
                | NodeKind::Cell
                | NodeKind::Column
                | NodeKind::Placeholder => {
                    entries.push(Entry::from_node(node));
                }
            }
        }
        if entries
            .last()
            .is_some_and(|last| last.kind == NodeKind::Separator)
        {
            entries.pop();
        }
        entries
    }

    /// The node and its whole subtree as builder entries.
    fn to_entry(&self, node: &Node) -> Entry {
        let mut entry = Entry::from_node(node);
        entry.children = self.children(&node.key).map(|c| self.to_entry(c)).collect();
        entry
    }

    fn siblings_from<'a>(&'a self, first: Option<&'a Key>) -> impl Iterator<Item = &'a Node> {
        let mut next = first;
        std::iter::from_fn(move || {
            let node = self.get(next?)?;
            next = node.next_key.as_ref();
            Some(node)
        })
    }
}

/// Builds a [`Collection`], see [`Collection::build`].
#[derive(Debug, Default)]
pub struct CollectionBuilder {
    entries: Vec<Entry>,
}

#[derive(Debug)]
struct Entry {
    key: Key,
    kind: NodeKind,
    text_value: Arc<str>,
    aria_label: Option<Arc<str>>,
    is_disabled: bool,
    link: Option<ItemLink>,
    /// Set for items whose children are hidden (a collapsed tree item).
    has_child_nodes: bool,
    col_span: Option<usize>,
    children: Vec<Entry>,
}

impl Entry {
    fn new(key: Key, kind: NodeKind, text_value: Arc<str>) -> Self {
        Self {
            key,
            kind,
            text_value,
            aria_label: None,
            is_disabled: false,
            link: None,
            has_child_nodes: false,
            col_span: None,
            children: Vec::new(),
        }
    }

    fn from_node(node: &Node) -> Self {
        Self {
            key: node.key.clone(),
            kind: node.kind,
            text_value: node.text_value.clone(),
            aria_label: node.aria_label.clone(),
            is_disabled: node.is_disabled,
            link: node.link.clone(),
            has_child_nodes: node.has_child_nodes,
            col_span: node.col_span,
            children: Vec::new(),
        }
    }
}

impl CollectionBuilder {
    /// Add an item. `text_value` is used for type-ahead, filtering and accessibility.
    pub fn item(
        &mut self,
        key: impl Into<Key>,
        text_value: impl Into<Arc<str>>,
    ) -> ItemBuilder<'_> {
        self.entries
            .push(Entry::new(key.into(), NodeKind::Item, text_value.into()));
        ItemBuilder {
            entry: self.entries.last_mut().expect("just pushed"),
        }
    }

    /// Add a section, filled by `contents`.
    pub fn section(
        &mut self,
        key: impl Into<Key>,
        contents: impl FnOnce(&mut CollectionBuilder),
    ) -> SectionBuilder<'_> {
        let mut inner = CollectionBuilder::default();
        contents(&mut inner);
        let mut entry = Entry::new(key.into(), NodeKind::Section, Arc::from(""));
        entry.children = inner.entries;
        self.entries.push(entry);
        SectionBuilder {
            entry: self.entries.last_mut().expect("just pushed"),
        }
    }

    /// Add a grid row (an item) with cells. Cell keys are generated: [`Key::cell`]`(row, i)`.
    pub fn row(
        &mut self,
        key: impl Into<Key>,
        text_value: impl Into<Arc<str>>,
        cells: impl FnOnce(&mut RowBuilder),
    ) -> ItemBuilder<'_> {
        let key = key.into();
        let mut row = RowBuilder {
            row: key.clone(),
            cells: Vec::new(),
        };
        cells(&mut row);
        let mut entry = Entry::new(key, NodeKind::Item, text_value.into());
        entry.children = row.cells;
        self.entries.push(entry);
        ItemBuilder {
            entry: self.entries.last_mut().expect("just pushed"),
        }
    }

    /// Add a section heading. Belongs at the start of a section.
    pub fn header(&mut self, key: impl Into<Key>, text: impl Into<Arc<str>>) {
        self.entries
            .push(Entry::new(key.into(), NodeKind::Header, text.into()));
    }

    pub fn separator(&mut self, key: impl Into<Key>) {
        self.entries
            .push(Entry::new(key.into(), NodeKind::Separator, Arc::from("")));
    }

    pub fn loader(&mut self, key: impl Into<Key>) {
        self.entries
            .push(Entry::new(key.into(), NodeKind::Loader, Arc::from("")));
    }

    /// Add a table header row: column headers and placeholders (see `hooks::table`).
    pub(crate) fn header_row(&mut self, key: Key, cells: Vec<HeaderCell>) {
        let mut entry = Entry::new(key, NodeKind::HeaderRow, Arc::from(""));
        entry.children = cells
            .into_iter()
            .map(|cell| {
                let mut entry = Entry::new(cell.key, cell.kind, cell.text_value);
                entry.col_span = cell.col_span;
                entry
            })
            .collect();
        self.entries.push(entry);
    }

    /// Add everything `other` built, after this builder's nodes.
    pub(crate) fn append(&mut self, other: CollectionBuilder) {
        self.entries.extend(other.entries);
    }

    pub(crate) fn build(self) -> Collection {
        self.finish()
    }

    fn finish(self) -> Collection {
        let mut collection = Collection::default();
        let (first, last) = link(&mut collection, self.entries, None, 0);
        collection.first_key = first;
        collection.last_key = last;
        let order: HashMap<Key, usize> = collection
            .nodes_in_order_all()
            .enumerate()
            .map(|(position, key)| (key, position))
            .collect();
        collection.order = order;
        collection
    }
}

impl Collection {
    /// All keys in document order, including items' children (trees).
    fn nodes_in_order_all(&self) -> impl Iterator<Item = Key> + '_ {
        let mut stack: Vec<&Key> = Vec::new();
        let mut next = self.first_key.as_ref();
        std::iter::from_fn(move || {
            loop {
                if let Some(key) = next {
                    let node = self.get(key)?;
                    next = node.first_child_key.as_ref();
                    if let Some(sibling) = &node.next_key {
                        stack.push(sibling);
                    }
                    return Some(node.key.clone());
                }
                next = Some(stack.pop()?);
            }
        })
    }
}

/// Insert `entries` as siblings below `parent`, linking them. Returns the first and last key.
fn link(
    collection: &mut Collection,
    entries: Vec<Entry>,
    parent: Option<&Key>,
    level: usize,
) -> (Option<Key>, Option<Key>) {
    let mut keys: Vec<Key> = Vec::with_capacity(entries.len());
    let mut nodes: Vec<(Node, Vec<Entry>)> = Vec::with_capacity(entries.len());
    for entry in entries {
        if collection.nodes.contains_key(&entry.key) || keys.contains(&entry.key) {
            crate::utils::dev_warn!(
                "Duplicate key {:?} in collection; ignoring all but its first occurrence.",
                entry.key
            );
            continue;
        }
        keys.push(entry.key.clone());
        let node = Node {
            key: entry.key,
            kind: entry.kind,
            text_value: entry.text_value,
            aria_label: entry.aria_label,
            level,
            index: nodes.len(),
            parent_key: parent.cloned(),
            prev_key: None,
            next_key: None,
            first_child_key: None,
            last_child_key: None,
            has_child_nodes: entry.has_child_nodes || !entry.children.is_empty(),
            is_disabled: entry.is_disabled,
            link: entry.link,
            col_index: None,
            col_span: entry.col_span,
        };
        nodes.push((node, entry.children));
    }

    // In rows with cells spanning columns, and in table header rows, cells know their column
    // (react-aria's `colIndex`).
    let positioned = |kind: NodeKind| {
        matches!(
            kind,
            NodeKind::Cell | NodeKind::Column | NodeKind::Placeholder
        )
    };
    if nodes.iter().any(|(node, _)| {
        positioned(node.kind) && (node.col_span.is_some() || node.kind != NodeKind::Cell)
    }) {
        let mut column = 0;
        for (node, _) in &mut nodes {
            if positioned(node.kind) {
                node.col_index = Some(column);
                column += node.col_span.unwrap_or(1);
            }
        }
    }

    for (i, (mut node, children)) in nodes.into_iter().enumerate() {
        node.prev_key = i.checked_sub(1).map(|p| keys[p].clone());
        node.next_key = keys.get(i + 1).cloned();
        if node.kind == NodeKind::Item {
            collection.item_count += 1;
        }
        // Items' children are a tree level deeper; a section's contents are not.
        let child_level = if node.kind == NodeKind::Item {
            level + 1
        } else {
            level
        };
        let key = node.key.clone();
        collection.nodes.insert(key.clone(), node);
        let (first_child, last_child) = link(collection, children, Some(&key), child_level);
        let node = collection.nodes.get_mut(&key).expect("just inserted");
        node.first_child_key = first_child;
        node.last_child_key = last_child;
    }

    (keys.first().cloned(), keys.last().cloned())
}

/// A cell of a table header row, see `CollectionBuilder::header_row`.
#[derive(Debug)]
pub(crate) struct HeaderCell {
    pub key: Key,
    /// `Column` or `Placeholder`.
    pub kind: NodeKind,
    pub text_value: Arc<str>,
    pub col_span: Option<usize>,
}

/// Adds the cells of a row, see [`CollectionBuilder::row`].
#[derive(Debug)]
pub struct RowBuilder {
    row: Key,
    cells: Vec<Entry>,
}

impl RowBuilder {
    /// Add a cell.
    pub fn cell(&mut self, text_value: impl Into<Arc<str>>) -> CellBuilder<'_> {
        let key = Key::cell(&self.row, self.cells.len());
        self.cells
            .push(Entry::new(key, NodeKind::Cell, text_value.into()));
        CellBuilder {
            entry: self.cells.last_mut().expect("just pushed"),
        }
    }
}

/// Configures a cell added with [`RowBuilder::cell`].
pub struct CellBuilder<'a> {
    entry: &'a mut Entry,
}

#[allow(clippy::return_self_not_must_use)]
impl CellBuilder<'_> {
    /// Let the cell span `columns` columns.
    pub fn col_span(self, columns: usize) -> Self {
        self.entry.col_span = Some(columns);
        self
    }

    pub fn aria_label(self, label: impl Into<Arc<str>>) -> Self {
        self.entry.aria_label = Some(label.into());
        self
    }
}

/// Configures an item added with [`CollectionBuilder::item`].
pub struct ItemBuilder<'a> {
    entry: &'a mut Entry,
}

// The builder writes through `&mut`; using the returned value for chaining is optional, so
// `b.item(..).disabled(true);` is fine.
#[allow(clippy::return_self_not_must_use)]
impl ItemBuilder<'_> {
    /// Disable the item.
    pub fn disabled(self, disabled: bool) -> Self {
        self.entry.is_disabled = disabled;
        self
    }

    /// An accessible name for the item.
    pub fn aria_label(self, label: impl Into<Arc<str>>) -> Self {
        self.entry.aria_label = Some(label.into());
        self
    }

    /// Make the item a link.
    pub fn link(self, link: ItemLink) -> Self {
        self.entry.link = Some(link);
        self
    }

    /// Add child items (for trees).
    pub fn children(self, children: impl FnOnce(&mut CollectionBuilder)) -> Self {
        let mut inner = CollectionBuilder::default();
        children(&mut inner);
        self.entry.children = inner.entries;
        self
    }
}

/// Configures a section added with [`CollectionBuilder::section`].
pub struct SectionBuilder<'a> {
    entry: &'a mut Entry,
}

// The builder writes through `&mut`; using the returned value for chaining is optional, so
// `b.item(..).disabled(true);` is fine.
#[allow(clippy::return_self_not_must_use)]
impl SectionBuilder<'_> {
    /// An accessible name for a section without a visible header.
    pub fn aria_label(self, label: impl Into<Arc<str>>) -> Self {
        self.entry.aria_label = Some(label.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn keys<'a>(nodes: impl Iterator<Item = &'a Node>) -> Vec<String> {
        nodes.map(|n| n.key.to_string()).collect()
    }

    fn sectioned() -> Collection {
        Collection::build(|b| {
            b.item("a", "Apple");
            b.section("s1", |s| {
                s.header("h1", "Fruit");
                s.item("b", "Banana");
                s.item("c", "Cherry").disabled(true);
            });
            b.separator("sep");
            b.section("s2", |s| {
                s.item("d", "Durian");
            });
            b.item("e", "Elderberry");
        })
    }

    #[test]
    fn counts_only_items() {
        assert_that!(sectioned().size()).is_equal_to(5);
        assert_that!(Collection::default().is_empty()).is_true();
    }

    #[test]
    fn walks_document_order_through_sections() {
        let c = sectioned();
        assert_that!(keys(c.nodes_in_order())).is_equal_to(
            vec!["a", "s1", "h1", "b", "c", "sep", "s2", "d", "e"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>(),
        );
        assert_that!(keys(c.items())).is_equal_to(
            vec!["a", "b", "c", "d", "e"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn key_before_steps_into_previous_sections_and_out_to_parents() {
        let c = sectioned();
        // From the first child of a section, "before" is the section itself.
        assert_that!(c.key_before(&Key::from("h1"))).is_equal_to(Some(&Key::from("s1")));
        // Before a section comes the last node of the previous section.
        assert_that!(c.key_before(&Key::from("s2"))).is_equal_to(Some(&Key::from("sep")));
        assert_that!(c.key_before(&Key::from("e"))).is_equal_to(Some(&Key::from("d")));
        assert_that!(c.key_before(&Key::from("a"))).is_none();
    }

    #[test]
    fn first_and_last_keys() {
        let c = sectioned();
        assert_that!(c.first_key()).is_equal_to(Some(&Key::from("a")));
        assert_that!(c.last_key()).is_equal_to(Some(&Key::from("e")));
        let ends_in_section = Collection::build(|b| {
            let _ = b
                .section("s", |s| {
                    s.item("x", "X");
                })
                .aria_label("S");
        });
        assert_that!(ends_in_section.last_key()).is_equal_to(Some(&Key::from("x")));
    }

    #[test]
    fn node_metadata() {
        let c = sectioned();
        let cherry = c.get(&Key::from("c")).unwrap();
        assert_that!(cherry.is_disabled).is_true();
        assert_that!(cherry.parent_key.clone()).is_equal_to(Some(Key::from("s1")));
        assert_that!(cherry.index).is_equal_to(2);
        assert_that!(cherry.level).is_equal_to(0);
        assert_that!(&*cherry.text_value).is_equal_to("Cherry");
        assert_that!(c.get(&Key::from("s1")).unwrap().has_child_nodes).is_true();
    }

    #[test]
    fn tree_children_are_not_traversed_but_ordered() {
        let c = Collection::build(|b| {
            b.item("root", "Root").children(|c| {
                c.item("child", "Child");
            });
            b.item("next", "Next");
        });
        assert_that!(keys(c.items())).is_equal_to(vec!["root".to_owned(), "next".to_owned()]);
        assert_that!(c.size()).is_equal_to(3);
        let child = c.get(&Key::from("child")).unwrap();
        assert_that!(child.level).is_equal_to(1);
        assert_that!(keys(c.children(&Key::from("root")))).is_equal_to(vec!["child".to_owned()]);
        assert_that!(c.compare_order(&Key::from("child"), &Key::from("next")))
            .is_equal_to(Some(Ordering::Less));
    }

    #[test]
    fn item_keys_between_spans_sections_in_both_directions() {
        let c = sectioned();
        let expected: Vec<Key> = ["a", "b", "c"].into_iter().map(Key::from).collect();
        assert_that!(c.item_keys_between(&Key::from("a"), &Key::from("c")))
            .is_equal_to(expected.clone());
        assert_that!(c.item_keys_between(&Key::from("c"), &Key::from("a"))).is_equal_to(expected);
        assert_that!(c.item_keys_between(&Key::from("a"), &Key::from("missing"))).is_empty();
    }

    #[test]
    fn filter_drops_empty_sections_and_trailing_separators() {
        let c = sectioned();
        let filtered = c.filter(|text, _| text.contains('e'));
        // Apple, Cherry, Elderberry match. Section s2 (Durian) disappears; s1 keeps its header.
        assert_that!(keys(filtered.nodes_in_order())).is_equal_to(
            vec!["a", "s1", "h1", "c", "sep", "e"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>(),
        );
        assert_that!(filtered.size()).is_equal_to(3);

        let only_first = c.filter(|text, _| text == "Apple");
        assert_that!(keys(only_first.nodes_in_order())).is_equal_to(vec!["a".to_owned()]);
    }

    #[test]
    fn filter_keeps_children_of_kept_items() {
        let c = Collection::build(|b| {
            b.item("root", "Root").children(|c| {
                c.item("child", "Child");
            });
        });
        let filtered = c.filter(|text, _| text == "Root");
        assert_that!(filtered.contains_key(&Key::from("child"))).is_true();
    }

    #[test]
    fn duplicate_keys_keep_the_first_occurrence() {
        let c = Collection::build(|b| {
            b.item("a", "First");
            b.item("a", "Second");
            b.item("b", "B");
        });
        assert_that!(c.size()).is_equal_to(2);
        assert_that!(&*c.get(&Key::from("a")).unwrap().text_value).is_equal_to("First");
        assert_that!(c.key_after(&Key::from("a"))).is_equal_to(Some(&Key::from("b")));
    }

    #[test]
    fn equal_builds_are_equal() {
        assert_that!(sectioned()).is_equal_to(sectioned());
    }

    #[test]
    fn tree_view_shows_expanded_children_only() {
        let tree = Collection::build(|b| {
            b.item("docs", "Documents").children(|c| {
                c.item("cv", "CV");
                c.item("taxes", "Taxes").children(|c| {
                    c.item("2024", "2024");
                });
            });
            b.item("photos", "Photos").children(|c| {
                c.item("cat", "Cat");
            });
        });

        let view = tree.with_expanded(&std::collections::HashSet::from([Key::from("docs")]));
        let mut order = Vec::new();
        let mut key = view.first_key().cloned();
        while let Some(k) = key {
            key = view.key_after(&k).cloned();
            order.push(k.to_string());
        }
        assert_that!(order)
            .is_equal_to(["docs", "cv", "taxes", "photos"].map(String::from).to_vec());
        // Collapsed items still have (hidden) children.
        assert_that!(view.get(&Key::from("taxes")).map(|n| n.has_child_nodes))
            .is_equal_to(Some(true));
        assert_that!(view.get(&Key::from("photos")).map(|n| n.has_child_nodes))
            .is_equal_to(Some(true));
        assert_that!(view.get(&Key::from("cv")).map(|n| n.level)).is_equal_to(Some(1));
        assert_that!(
            view.key_before(&Key::from("photos"))
                .map(ToString::to_string)
        )
        .is_equal_to(Some("taxes".to_owned()));
        assert_that!(view.contains_key(&Key::from("2024"))).is_false();
    }
}
