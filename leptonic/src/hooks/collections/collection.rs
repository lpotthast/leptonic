// Upstream: react-aria/src/collections/BaseCollection.ts @ 99e6102368
// Upstream: react-aria/src/collections/CollectionBuilder.tsx @ 99e6102368
// Upstream: react-stately/src/list/ListCollection.ts @ 99e6102368
// Upstream: react-aria/test/collections/CollectionBuilder.test.js @ 99e6102368
// Upstream: react-aria-components/test/ListBox.test.js @ 99e6102368
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, hash_map},
    sync::{Arc, LazyLock},
};

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
///
/// The nodes are stored in document order with links by position, so traversals don't hash;
/// looking a node up by its key does.
#[derive(Debug, Clone, Default)]
pub struct Collection {
    /// Every node, in document order: a node before its children, they before its next sibling.
    nodes: Vec<Node>,
    /// The links of `nodes` (same positions).
    links: Vec<Links>,
    /// The position of every key. Std's randomly seeded hasher on purpose: keys are often
    /// user-controlled (record ids and names, file names), and collections are built on the
    /// server too, so a fixed-seed hasher would let crafted keys collide.
    positions: HashMap<Key, u32>,
    /// The first and last top-level node.
    first: Option<u32>,
    last: Option<u32>,
    item_count: usize,
    /// A tree's visible items (see [`Collection::with_expanded`]): navigation also visits items'
    /// children.
    is_tree_view: bool,
}

/// A node's neighbors, by position.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Links {
    parent: Option<u32>,
    prev: Option<u32>,
    next: Option<u32>,
    first_child: Option<u32>,
    last_child: Option<u32>,
}

/// Equal when they hold the same nodes in the same structure (the key index follows from them).
impl PartialEq for Collection {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first
            && self.last == other.last
            && self.item_count == other.item_count
            && self.is_tree_view == other.is_tree_view
            && self.links == other.links
            && self.nodes == other.nodes
    }
}

/// A position as a vector index.
fn at(position: u32) -> usize {
    position as usize
}

impl Collection {
    /// Build a collection.
    ///
    /// ```
    /// use leptonic::hooks::collections::Collection;
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
        builder.build()
    }

    /// The number of items (sections, headers and separators don't count).
    pub fn size(&self) -> usize {
        self.item_count
    }

    pub fn is_empty(&self) -> bool {
        self.item_count == 0
    }

    pub fn get(&self, key: &Key) -> Option<&Node> {
        self.position(key).map(|position| &self.nodes[at(position)])
    }

    pub fn contains_key(&self, key: &Key) -> bool {
        self.positions.contains_key(key)
    }

    /// The keys of all nodes (sections, headers, items, cells, ...), in document order (items'
    /// children included).
    pub fn keys(&self) -> impl Iterator<Item = &Key> {
        self.nodes.iter().map(|node| &node.key)
    }

    /// The first top-level node.
    pub fn first_key(&self) -> Option<&Key> {
        self.key_at(self.first)
    }

    /// The last node in document order: as [`key_before`](Self::key_before), this descends into
    /// sections (and other non-item nodes), but not into items' children (a grid row's cells,
    /// child rows), except in a tree view.
    pub fn last_key(&self) -> Option<&Key> {
        self.key_at(Some(self.deepest_last(self.last?)))
    }

    /// The next node in document order. Descends into sections (and other non-item nodes), but
    /// not into items' children, except in a tree view ([`Collection::with_expanded`]).
    pub fn key_after(&self, key: &Key) -> Option<&Key> {
        self.key_at(self.after(self.position(key)?))
    }

    /// The previous node in document order. From the first node of a section, this is the
    /// section itself.
    pub fn key_before(&self, key: &Key) -> Option<&Key> {
        let links = self.links(self.position(key)?);
        match links.prev {
            Some(prev) => self.key_at(Some(self.deepest_last(prev))),
            None => self.key_at(links.parent),
        }
    }

    /// The node after `key` among its siblings.
    pub fn next_sibling(&self, key: &Key) -> Option<&Node> {
        self.node_at(self.links(self.position(key)?).next)
    }

    /// The node before `key` among its siblings.
    pub fn previous_sibling(&self, key: &Key) -> Option<&Node> {
        self.node_at(self.links(self.position(key)?).prev)
    }

    /// The first child of `key` (a section's first node, a tree item's first child or a row's
    /// first cell).
    pub fn first_child(&self, key: &Key) -> Option<&Node> {
        self.node_at(self.links(self.position(key)?).first_child)
    }

    /// The last child of `key`.
    pub fn last_child(&self, key: &Key) -> Option<&Node> {
        self.node_at(self.links(self.position(key)?).last_child)
    }

    /// Every node (sections, headers, items, cells, ...), in storage order.
    pub(crate) fn all_nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter()
    }

    /// The top-level nodes.
    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        self.siblings(self.first)
    }

    /// The children of `parent`: a section's contents or a tree item's child items.
    pub fn children(&self, parent: &Key) -> impl Iterator<Item = &Node> {
        self.siblings(
            self.position(parent)
                .and_then(|parent| self.links(parent).first_child),
        )
    }

    /// `key`'s node and the siblings after it.
    pub fn siblings_from(&self, key: &Key) -> impl Iterator<Item = &Node> {
        self.siblings(self.position(key))
    }

    /// `key`'s node and every node after it in document order (all levels, to the end).
    pub(crate) fn nodes_from(&self, key: &Key) -> &[Node] {
        self.position(key)
            .map_or(&[], |position| &self.nodes[at(position)..])
    }

    /// The cells of a grid row (a tree table's row also has child rows: not these).
    pub fn cells(&self, row: &Key) -> impl Iterator<Item = &Node> {
        self.children(row)
            .filter(|node| node.kind == NodeKind::Cell)
    }

    /// All nodes in document order, as visited by [`key_after`](Self::key_after).
    pub fn nodes_in_order(&self) -> impl Iterator<Item = &Node> {
        let mut next = self.first;
        std::iter::from_fn(move || {
            let position = next?;
            next = self.after(position);
            Some(&self.nodes[at(position)])
        })
    }

    /// All items in document order (sections are traversed, items' children are not).
    pub fn items(&self) -> impl Iterator<Item = &Node> {
        self.nodes_in_order().filter(|node| node.is_item())
    }

    /// Compares the document order of two nodes (including nested tree items). `None` if either
    /// key is not part of the collection.
    pub fn compare_order(&self, a: &Key, b: &Key) -> Option<Ordering> {
        Some(self.position(a)?.cmp(&self.position(b)?))
    }

    /// `keys` in collection order; keys missing from the collection come last (sorted by key).
    pub fn sorted_keys(&self, keys: impl IntoIterator<Item = Key>) -> Vec<Key> {
        let mut keys: Vec<(Option<u32>, Key)> = keys
            .into_iter()
            .map(|key| (self.position(&key), key))
            .collect();
        keys.sort_by(
            |(a_position, a), (b_position, b)| match (a_position, b_position) {
                (Some(a), Some(b)) => a.cmp(b),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => a.cmp(b),
            },
        );
        keys.into_iter().map(|(_, key)| key).collect()
    }

    /// The keys of all items between `from` and `to` (inclusive, in document order, either
    /// direction), e.g. for range selection: the items reachable by
    /// [`key_after`](Self::key_after) (in a tree view, the shown child items too).
    pub fn item_keys_between(&self, from: &Key, to: &Key) -> Vec<Key> {
        let (Some(from), Some(to)) = (self.position(from), self.position(to)) else {
            return Vec::new();
        };
        let (start, end) = if from > to { (to, from) } else { (from, to) };
        let mut keys = Vec::new();
        let mut current = Some(start);
        while let Some(position) = current {
            let node = &self.nodes[at(position)];
            if node.is_item() {
                keys.push(node.key.clone());
            }
            if position == end {
                break;
            }
            current = self.after(position);
        }
        keys
    }

    /// The visible part of a tree: items' children only where the item is in `expanded`
    /// (sections always show their contents). Navigation (`key_after`, `key_before`) walks the
    /// visible items in order, children included. Collapsed items keep `has_child_nodes`.
    #[must_use]
    pub fn with_expanded(&self, expanded: &HashSet<Key>) -> Self {
        let mut collection = CollectionBuilder {
            entries: self.expanded_entries(self.first, expanded),
            table_columns: None,
        }
        .build();
        collection.is_tree_view = true;
        collection
    }

    fn expanded_entries(&self, first: Option<u32>, expanded: &HashSet<Key>) -> Vec<Entry> {
        self.positioned_siblings(first)
            .map(|(position, node)| {
                let mut entry = Entry::from_node(node);
                let first_child = self.links(position).first_child;
                if node.kind != NodeKind::Item || expanded.contains(&node.key) {
                    entry.children = self.expanded_entries(first_child, expanded);
                } else {
                    // A collapsed item hides its child items, not its cells (a tree table's row).
                    entry.children = self
                        .siblings(first_child)
                        .filter(|child| child.kind == NodeKind::Cell)
                        .map(Entry::from_node)
                        .collect();
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
            entries: self.filtered_entries(self.first, &keep),
            table_columns: None,
        }
        .build()
    }

    fn filtered_entries(
        &self,
        first: Option<u32>,
        keep: &impl Fn(&str, &Node) -> bool,
    ) -> Vec<Entry> {
        let mut entries = Vec::new();
        // The kind of the previous sibling if it was kept.
        let mut kept_previous: Option<NodeKind> = None;
        for (position, node) in self.positioned_siblings(first) {
            let kept_before = entries.len();
            match node.kind {
                NodeKind::Item => {
                    if keep(&node.text_value, node) {
                        entries.push(self.to_entry(position));
                    }
                }
                NodeKind::Section => {
                    let children = self.filtered_entries(self.links(position).first_child, keep);
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
                NodeKind::HeaderRow => entries.push(self.to_entry(position)),
                // A separator stays only after a kept sibling that isn't a separator, so no
                // separator leads or doubles (react-aria-components' `SeparatorNode.filter`).
                NodeKind::Separator => {
                    if kept_previous.is_some_and(|kind| kind != NodeKind::Separator) {
                        entries.push(Entry::from_node(node));
                    }
                }
                NodeKind::Header
                | NodeKind::Loader
                | NodeKind::Cell
                | NodeKind::Column
                | NodeKind::Placeholder => {
                    entries.push(Entry::from_node(node));
                }
            }
            kept_previous = (entries.len() > kept_before).then_some(node.kind);
        }
        if entries
            .last()
            .is_some_and(|last| last.kind == NodeKind::Separator)
        {
            entries.pop();
        }
        entries
    }

    /// The node at `position` and its whole subtree as builder entries.
    fn to_entry(&self, position: u32) -> Entry {
        let mut entry = Entry::from_node(&self.nodes[at(position)]);
        entry.children = self
            .positioned_siblings(self.links(position).first_child)
            .map(|(child, _)| self.to_entry(child))
            .collect();
        entry
    }

    fn position(&self, key: &Key) -> Option<u32> {
        self.positions.get(key).copied()
    }

    fn links(&self, position: u32) -> Links {
        self.links[at(position)]
    }

    fn node_at(&self, position: Option<u32>) -> Option<&Node> {
        position.map(|position| &self.nodes[at(position)])
    }

    fn key_at(&self, position: Option<u32>) -> Option<&Key> {
        self.node_at(position).map(|node| &node.key)
    }

    /// Whether traversal descends into the node at `position`: sections and other non-items do,
    /// items only in a tree view.
    fn descends(&self, position: u32) -> bool {
        self.nodes[at(position)].kind != NodeKind::Item || self.is_tree_view
    }

    /// The position after `position` in document order (see [`key_after`](Self::key_after)).
    fn after(&self, mut position: u32) -> Option<u32> {
        if self.descends(position)
            && let Some(first_child) = self.links(position).first_child
        {
            return Some(first_child);
        }
        loop {
            let links = self.links(position);
            if let Some(next) = links.next {
                return Some(next);
            }
            position = links.parent?;
        }
    }

    /// The last node in document order within the subtree at `position` (descending as
    /// traversal does).
    fn deepest_last(&self, mut position: u32) -> u32 {
        while self.descends(position)
            && let Some(last_child) = self.links(position).last_child
        {
            position = last_child;
        }
        position
    }

    fn siblings(&self, first: Option<u32>) -> impl Iterator<Item = &Node> {
        self.positioned_siblings(first).map(|(_, node)| node)
    }

    fn positioned_siblings(&self, first: Option<u32>) -> impl Iterator<Item = (u32, &Node)> {
        let mut next = first;
        std::iter::from_fn(move || {
            let position = next?;
            next = self.links(position).next;
            Some((position, &self.nodes[at(position)]))
        })
    }
}

/// The empty text, shared: cloning it allocates nothing (an `Arc<str>` made from `""` would: an
/// `ArcInner` even for an empty slice), which counts in collections rebuilt from many items
/// without texts.
pub(crate) fn empty_text() -> Arc<str> {
    static EMPTY: LazyLock<Arc<str>> = LazyLock::new(|| Arc::from(""));
    Arc::clone(&EMPTY)
}

/// A node's text: empty texts share [`empty_text`].
pub(crate) fn text(value: impl Into<Arc<str>> + AsRef<str>) -> Arc<str> {
    if value.as_ref().is_empty() {
        empty_text()
    } else {
        value.into()
    }
}

/// Builds a [`Collection`], see [`Collection::build`].
#[derive(Debug, Default)]
pub struct CollectionBuilder {
    entries: Vec<Entry>,
    /// A table's data columns: the cells of rows (child rows too) are keyed by the column they
    /// start in ([`Key::cell`]`(row, column)`) instead of their position.
    pub(crate) table_columns: Option<Arc<[Key]>>,
}

#[derive(Debug)]
struct Entry {
    key: Key,
    kind: NodeKind,
    text_value: Arc<str>,
    aria_label: Option<Arc<str>>,
    is_disabled: bool,
    disabled_behavior: Option<super::DisabledBehavior>,
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
            disabled_behavior: None,
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
            disabled_behavior: node.disabled_behavior,
            link: node.link.clone(),
            has_child_nodes: node.has_child_nodes,
            col_span: node.col_span,
            children: Vec::new(),
        }
    }
}

impl CollectionBuilder {
    /// Reserve room for at least `additional` more nodes at this level, e.g. the number of items
    /// a loop is about to add: the builder then doesn't grow while adding them.
    pub fn reserve(&mut self, additional: usize) {
        self.entries.reserve(additional);
    }

    /// Add an item. `text_value` is used for type-ahead, filtering and accessibility (`""` for
    /// none: it allocates nothing).
    pub fn item(
        &mut self,
        key: impl Into<Key>,
        text_value: impl Into<Arc<str>> + AsRef<str>,
    ) -> ItemBuilder<'_> {
        self.entries
            .push(Entry::new(key.into(), NodeKind::Item, text(text_value)));
        ItemBuilder {
            entry: self.entries.last_mut().expect("just pushed"),
            table_columns: self.table_columns.as_ref(),
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
        let mut entry = Entry::new(key.into(), NodeKind::Section, empty_text());
        entry.children = inner.entries;
        self.entries.push(entry);
        SectionBuilder {
            entry: self.entries.last_mut().expect("just pushed"),
        }
    }

    /// Add a grid row (an item) with cells. Cell keys are generated from their position:
    /// [`Key::cell`]`(row, i)` (in a table: from their column, see `TableBuilder::row`).
    pub fn row(
        &mut self,
        key: impl Into<Key>,
        text_value: impl Into<Arc<str>> + AsRef<str>,
        cells: impl FnOnce(&mut RowBuilder),
    ) -> ItemBuilder<'_> {
        let key = key.into();
        let mut row = RowBuilder {
            row: key.clone(),
            cells: Vec::new(),
            columns: self.table_columns.clone(),
            last_start: 0,
        };
        cells(&mut row);
        let mut entry = Entry::new(key, NodeKind::Item, text(text_value));
        entry.children = row.cells;
        self.entries.push(entry);
        ItemBuilder {
            entry: self.entries.last_mut().expect("just pushed"),
            table_columns: self.table_columns.as_ref(),
        }
    }

    /// Add a section heading. Belongs at the start of a section.
    pub fn header(&mut self, key: impl Into<Key>, text_value: impl Into<Arc<str>> + AsRef<str>) {
        self.entries
            .push(Entry::new(key.into(), NodeKind::Header, text(text_value)));
    }

    pub fn separator(&mut self, key: impl Into<Key>) {
        self.entries
            .push(Entry::new(key.into(), NodeKind::Separator, empty_text()));
    }

    pub fn loader(&mut self, key: impl Into<Key>) {
        self.entries
            .push(Entry::new(key.into(), NodeKind::Loader, empty_text()));
    }

    /// Add a table header row: column headers and placeholders (see `hooks::table`).
    pub(crate) fn header_row(&mut self, key: Key, cells: Vec<HeaderCell>) {
        let mut entry = Entry::new(key, NodeKind::HeaderRow, empty_text());
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

    /// A builder holding the rows of `collection` (its top-level nodes other than table header
    /// rows, with their subtrees), every row with cells (child rows too) starting with an empty
    /// cell in `column` ([`Key::cell`]`(row, column)`); the other cells keep their keys. Adds a
    /// table's selection checkbox cells to rows built without them.
    pub(crate) fn rows_with_leading_cell(collection: &Collection, column: &Key) -> Self {
        // A row's cells come before its child rows.
        fn row_entry(collection: &Collection, node: &Node, column: &Key) -> Entry {
            let mut entry = Entry::from_node(node);
            let mut children = collection.children(&node.key).peekable();
            if children
                .peek()
                .is_some_and(|child| child.kind == NodeKind::Cell)
            {
                entry.children.push(Entry::new(
                    Key::cell(&node.key, column.clone()),
                    NodeKind::Cell,
                    empty_text(),
                ));
            }
            for child in children {
                entry.children.push(if child.kind == NodeKind::Cell {
                    // Cells have no children to rebuild.
                    Entry::from_node(child)
                } else {
                    row_entry(collection, child, column)
                });
            }
            entry
        }
        Self {
            entries: collection
                .iter()
                .filter(|node| node.kind != NodeKind::HeaderRow)
                .map(|node| row_entry(collection, node, column))
                .collect(),
            table_columns: None,
        }
    }

    /// Add everything `other` built, after this builder's nodes.
    pub(crate) fn append(&mut self, other: CollectionBuilder) {
        self.entries.extend(other.entries);
    }

    pub(crate) fn build(self) -> Collection {
        fn count(entries: &[Entry]) -> usize {
            entries.len()
                + entries
                    .iter()
                    .map(|entry| count(&entry.children))
                    .sum::<usize>()
        }
        let capacity = count(&self.entries);
        let mut collection = Collection {
            nodes: Vec::with_capacity(capacity),
            links: Vec::with_capacity(capacity),
            positions: HashMap::with_capacity(capacity),
            ..Collection::default()
        };
        let (first, last) = link(&mut collection, self.entries, None, 0);
        collection.first = first;
        collection.last = last;
        collection
    }
}

/// Appends `entries` as siblings below the node at `parent`, in document order (a node, then its
/// subtree, then its next sibling), linking them. An entry whose key occurred before (at any
/// level) is left out with its subtree. Returns the first and last sibling's position.
fn link(
    collection: &mut Collection,
    entries: Vec<Entry>,
    parent: Option<u32>,
    level: usize,
) -> (Option<u32>, Option<u32>) {
    // In rows with cells spanning columns, and in table header rows, cells know their column
    // (react-aria's `colIndex`).
    let positioned = |kind: NodeKind| {
        matches!(
            kind,
            NodeKind::Cell | NodeKind::Column | NodeKind::Placeholder
        )
    };
    let has_columns = entries.iter().any(|entry| {
        positioned(entry.kind) && (entry.col_span.is_some() || entry.kind != NodeKind::Cell)
    });
    let mut column = 0;
    let parent_key = parent.map(|parent| collection.nodes[at(parent)].key.clone());
    let (mut first, mut last) = (None, None);
    let mut index = 0;

    for entry in entries {
        let position =
            u32::try_from(collection.nodes.len()).expect("a collection has fewer than 2^32 nodes");
        match collection.positions.entry(entry.key) {
            hash_map::Entry::Occupied(occupied) => {
                crate::utils::dev_warn!(
                    "Duplicate key {:?} in collection; ignoring all but its first occurrence.",
                    occupied.key()
                );
                continue;
            }
            hash_map::Entry::Vacant(vacant) => {
                let key = vacant.key().clone();
                vacant.insert(position);
                let col_index = (has_columns && positioned(entry.kind)).then(|| {
                    let col_index = column;
                    column += entry.col_span.unwrap_or(1);
                    col_index
                });
                if entry.kind == NodeKind::Item {
                    collection.item_count += 1;
                }
                collection.nodes.push(Node {
                    key,
                    kind: entry.kind,
                    text_value: entry.text_value,
                    aria_label: entry.aria_label,
                    level,
                    index,
                    parent_key: parent_key.clone(),
                    has_child_nodes: entry.has_child_nodes,
                    is_disabled: entry.is_disabled,
                    disabled_behavior: entry.disabled_behavior,
                    link: entry.link,
                    col_index,
                    col_span: entry.col_span,
                });
                collection.links.push(Links {
                    parent,
                    prev: last,
                    ..Links::default()
                });
            }
        }
        if let Some(last) = last {
            collection.links[at(last)].next = Some(position);
        }
        first.get_or_insert(position);
        last = Some(position);
        index += 1;

        if !entry.children.is_empty() {
            // Items' children are a tree level deeper; a section's contents are not.
            let child_level = if entry.kind == NodeKind::Item {
                level + 1
            } else {
                level
            };
            let (first_child, last_child) =
                link(collection, entry.children, Some(position), child_level);
            let links = &mut collection.links[at(position)];
            links.first_child = first_child;
            links.last_child = last_child;
            if first_child.is_some() {
                collection.nodes[at(position)].has_child_nodes = true;
            }
        }
    }
    (first, last)
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
    /// A table's data columns, which key the cells (see `CollectionBuilder::table_columns`).
    columns: Option<Arc<[Key]>>,
    /// The column the last cell starts in (table rows).
    last_start: usize,
}

impl RowBuilder {
    /// Add a cell.
    pub fn cell(&mut self, text_value: impl Into<Arc<str>> + AsRef<str>) -> CellBuilder<'_> {
        let column = match &self.columns {
            // A table row: the column the cell starts in, after the cells before it and their
            // spans. Cells past the last column (a malformed row, warned about in debug builds)
            // get keys no column has.
            Some(columns) => {
                if let Some(last) = self.cells.last() {
                    self.last_start += last.col_span.unwrap_or(1);
                }
                columns
                    .get(self.last_start)
                    .cloned()
                    .unwrap_or_else(|| Key::generated("excess-cell", self.last_start))
            }
            // A grid row: the cell's position.
            None => Key::from(self.cells.len()),
        };
        let key = Key::cell(&self.row, column);
        self.cells
            .push(Entry::new(key, NodeKind::Cell, text(text_value)));
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

    pub fn aria_label(self, label: impl Into<Arc<str>> + AsRef<str>) -> Self {
        self.entry.aria_label = Some(text(label));
        self
    }
}

/// Configures an item added with [`CollectionBuilder::item`].
pub struct ItemBuilder<'a> {
    entry: &'a mut Entry,
    table_columns: Option<&'a Arc<[Key]>>,
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
    pub fn aria_label(self, label: impl Into<Arc<str>> + AsRef<str>) -> Self {
        self.entry.aria_label = Some(text(label));
        self
    }

    /// How the item behaves while disabled, overriding the collection's `disabled_behavior`:
    /// with [`DisabledBehavior::Selection`](super::DisabledBehavior::Selection), a disabled item
    /// can still be focused and have actions.
    pub fn disabled_behavior(self, behavior: super::DisabledBehavior) -> Self {
        self.entry.disabled_behavior = Some(behavior);
        self
    }

    /// Make the item a link.
    pub fn link(self, link: ItemLink) -> Self {
        self.entry.link = Some(link);
        self
    }

    /// Add child items (for trees), after the item's cells (a tree table's child rows).
    pub fn children(self, children: impl FnOnce(&mut CollectionBuilder)) -> Self {
        let mut inner = CollectionBuilder {
            entries: Vec::new(),
            table_columns: self.table_columns.cloned(),
        };
        children(&mut inner);
        self.entry.children.extend(inner.entries);
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
    pub fn aria_label(self, label: impl Into<Arc<str>> + AsRef<str>) -> Self {
        self.entry.aria_label = Some(text(label));
        self
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use leptos::prelude::{Get, RwSignal, Set, WithUntracked};

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
    fn nodes_without_texts_share_the_empty_text() {
        let collection = Collection::build(|b| {
            b.reserve(2);
            b.item("a", empty_text());
            b.section("s", |s| {
                s.item("b", "Banana");
            });
            b.separator("sep");
            b.loader("loader");
            b.row("r", "Row", |r| {
                r.cell("Cell");
            });
        });
        for key in ["a", "s", "sep", "loader"] {
            let node = collection.get(&Key::from(key)).expect("built");
            assert_that!(Arc::ptr_eq(&node.text_value, &empty_text())).is_true();
        }
        let leading_cells =
            CollectionBuilder::rows_with_leading_cell(&collection, &Key::from("check")).build();
        let checkbox_cell = leading_cells
            .get(&Key::cell(&Key::from("r"), "check"))
            .expect("built");
        // The other cells keep their keys.
        assert_that!(leading_cells.contains_key(&Key::cell(&Key::from("r"), 0))).is_true();
        assert_that!(Arc::ptr_eq(&checkbox_cell.text_value, &empty_text())).is_true();
        assert_that!(collection.size()).is_equal_to(3);
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
    fn a_node_moved_by_other_nodes_keeps_its_content() {
        let before = Collection::build(|b| {
            b.item("a", "A");
            b.item("b", "B");
        });
        // `b` moves: another index, other neighbors.
        let after = Collection::build(|b| {
            b.item("x", "X");
            b.item("y", "Y");
            b.item("b", "B");
        });
        let renamed = Collection::build(|b| {
            b.item("a", "A");
            b.item("b", "Bee");
        });
        let b = |c: &Collection| c.get(&Key::from("b")).cloned().unwrap();
        assert_that!(b(&before)).is_not_equal_to(b(&after));
        assert_that!(b(&before).same_content(&b(&after))).is_true();
        assert_that!(b(&before).same_content(&b(&renamed))).is_false();
    }

    // Upstream: CollectionBuilder.test.js "should throw when two items share a key" (here: the
    // first occurrence is kept).
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

        let c = Collection::build(|b| {
            b.item("a", "First");
            b.item("x", "X");
            b.item("a", "Again");
        });
        assert_that!(keys(c.items())).is_equal_to(vec!["a".to_owned(), "x".to_owned()]);
        assert_that!(c.last_key()).is_equal_to(Some(&Key::from("x")));
    }

    // Upstream: CollectionBuilder.test.js "should have correct firstKey, lastKey and should be
    // frozen after all items are deleted".
    #[test]
    fn an_emptied_collection_has_no_first_or_last_key() {
        crate::testing::with_owner(|| {
            let items = RwSignal::new(vec!["a", "b"]);
            let collection = super::super::use_collection(move |b| {
                for item in items.get() {
                    b.item(item, item);
                }
            });
            assert_that!(collection.with_untracked(|c| c.first_key().cloned()))
                .is_equal_to(Some(Key::from("a")));
            items.set(Vec::new());
            assert_that!(collection.with_untracked(|c| c.first_key().cloned())).is_none();
            assert_that!(collection.with_untracked(|c| c.last_key().cloned())).is_none();
            assert_that!(collection.with_untracked(|c| c.size())).is_equal_to(0);
        });
    }

    // Upstream: CollectionBuilder.test.js "should allow a new item to reuse the key of an item
    // removed in the same render".
    #[test]
    fn rebuilds_keep_the_new_order() {
        crate::testing::with_owner(|| {
            let items = RwSignal::new(vec!["a", "b"]);
            let collection = super::super::use_collection(move |b| {
                for item in items.get() {
                    b.item(item, item);
                }
            });
            items.set(vec!["a", "c", "b"]);
            assert_that!(collection.with_untracked(|c| keys(c.items()))).is_equal_to(vec![
                "a".to_owned(),
                "c".to_owned(),
                "b".to_owned(),
            ]);
            items.set(vec!["a"]);
            assert_that!(collection.with_untracked(|c| keys(c.items())))
                .is_equal_to(vec!["a".to_owned()]);
            assert_that!(collection.with_untracked(|c| c.key_after(&Key::from("a")).cloned()))
                .is_none();
        });
    }

    // Upstream: ListBox.test.js "should update collection when moving item to a different
    // section".
    #[test]
    fn an_item_moved_to_another_section_belongs_to_it() {
        crate::testing::with_owner(|| {
            let moved = RwSignal::new(false);
            let collection = super::super::use_collection(move |b| {
                let moved = moved.get();
                b.section("veggies", |s| {
                    s.item("lettuce", "Lettuce");
                    if moved {
                        s.item("ham", "Ham");
                    }
                });
                b.section("protein", |s| {
                    if !moved {
                        s.item("ham", "Ham");
                    }
                    s.item("tuna", "Tuna");
                });
            });
            moved.set(true);
            collection.with_untracked(|c| {
                assert_that!(keys(c.children(&Key::from("veggies"))))
                    .is_equal_to(vec!["lettuce".to_owned(), "ham".to_owned()]);
                assert_that!(keys(c.children(&Key::from("protein"))))
                    .is_equal_to(vec!["tuna".to_owned()]);
                assert_that!(c.get(&Key::from("ham")).and_then(|n| n.parent_key.clone()))
                    .is_equal_to(Some(Key::from("veggies")));
                assert_that!(keys(c.items())).is_equal_to(vec![
                    "lettuce".to_owned(),
                    "ham".to_owned(),
                    "tuna".to_owned(),
                ]);
            });
        });
    }

    #[test]
    fn filter_drops_leading_and_doubled_separators() {
        let c = Collection::build(|b| {
            b.separator("sep0");
            b.item("a", "Apple");
            b.separator("sep1");
            b.item("b", "Banana");
            b.separator("sep2");
            b.item("c", "Cherry");
            b.separator("sep3");
            b.item("d", "Durian");
        });
        // Banana is gone: the separator after it follows a removed item, the one before it stays
        // (as in react-aria-components' `SeparatorNode.filter`).
        let filtered = c.filter(|text, _| text != "Banana");
        assert_that!(keys(filtered.nodes_in_order()))
            .is_equal_to(["a", "sep1", "c", "sep3", "d"].map(String::from).to_vec());
        // Only the last item is left: no separator before it.
        let filtered = c.filter(|text, _| text == "Durian");
        assert_that!(keys(filtered.nodes_in_order())).is_equal_to(vec!["d".to_owned()]);
        // A separator right after another one is dropped.
        let doubled = Collection::build(|b| {
            b.item("a", "Apple");
            b.separator("sep1");
            b.separator("sep2");
            b.item("b", "Banana");
        });
        assert_that!(keys(doubled.filter(|_, _| true).nodes_in_order()))
            .is_equal_to(["a", "sep1", "b"].map(String::from).to_vec());
    }

    #[test]
    fn duplicate_keys_across_tree_levels_keep_the_first_in_document_order() {
        let c = Collection::build(|b| {
            b.item("a", "A").children(|c| {
                c.item("x", "Child X");
            });
            b.item("x", "Top-level X");
            b.item("b", "B");
        });
        assert_that!(&*c.get(&Key::from("x")).unwrap().text_value).is_equal_to("Child X");
        assert_that!(c.get(&Key::from("x")).unwrap().parent_key.clone())
            .is_equal_to(Some(Key::from("a")));
        assert_that!(keys(c.iter())).is_equal_to(["a", "b"].map(String::from).to_vec());
        assert_that!(c.key_after(&Key::from("a"))).is_equal_to(Some(&Key::from("b")));
        assert_that!(c.size()).is_equal_to(3);
    }

    #[test]
    fn builds_large_collections() {
        let c = Collection::build(|b| {
            for i in 0..20_000 {
                b.item(i.to_string(), format!("Item {i}"));
            }
        });
        assert_that!(c.size()).is_equal_to(20_000);
        assert_that!(c.last_key()).is_equal_to(Some(&Key::from("19999")));
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

        let view = tree.with_expanded(&HashSet::from([Key::from("docs")]));
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
