use std::collections::{BTreeMap, HashSet};

use super::Key;

/// A generic interface to access a readonly sequential
/// collection of unique `Key`ed items.
trait Collection<T> {
    /// The number of items in the collection.
    fn size(&self) -> usize;

    /// Iterate over all keys in the collection.
    fn keys(&self) -> impl Iterator<Item = Key>;

    /// Get an item by its key.
    fn get_item(&self, key: Key) -> Option<T>;

    /// Get an item by the index of its key.
    fn at(&self, idx: usize) -> Option<T>;

    /// Get the key that comes before the given key in the collection.
    fn get_key_before(&self, key: Key) -> Option<T>;

    /// Get the key that comes after the given key in the collection.
    fn get_key_after(&self, key: Key) -> Option<T>;

    /// Get the first key in the collection.
    fn get_first_key(&self) -> Option<T>;

    /// Get the last key in the collection.
    fn get_last_key(&self) -> Option<T>;

    /// Iterate over the child items of the given key.
    fn get_children(&self, key: Key) -> impl Iterator<Item = T>;

    /// Returns a string representation of the item's contents.
    fn get_text_value(&self, key: Key) -> String;
}

struct Node<T> {
    /// A unique key for the node. TODO: This must be synchronized with the BTreeMap key...
    key: Key,

    /// The object value the node was created from.
    value: T, // TODO: Use `Option<T>` instead?

    /// The level of depth this node is at in the hierarchy.
    level: usize,

    /// Whether this item has children, even if not loaded yet. // TODO: Remove this?
    has_child_nodes: bool,

    // The rendered contents of this node (e.g. JSX). TODO: Remove this?
    //rendered: ReactNode,
    /// A string value for this node, used for features like typeahead. TODO:  Use `Oco` instead?
    text_value: String,

    /// An accessibility label for this node. TODO: Use `Option<String>` instead? Use `Oco` instead?
    aria_label: String,

    /// The index of this node within its parent. TODO: This is hard to maintain!!
    index: usize,

    // A function that should be called to wrap the rendered node.
    //wrapper?: (element: ReactElement) => ReactElement,
    /** The key of the parent node. */
    parent_key: Option<Key>,

    /// The key of the node before this node.
    prev_key: Option<Key>,

    /// The key of the node after this node.
    next_key: Option<Key>,
    // Additional properties specific to a particular node type.
    // props?: any,

    // @private
    //shouldInvalidate?: (context: unknown) => boolean
}

type Nodes<T> = BTreeMap<Key, NodeTreeElem<T>>;

enum NodeTreeElem<T> {
    Section { node: Node<T>, children: Nodes<T> },
    Item { node: Node<T> },
}

pub(crate) struct NodeTree<T> {
    pub(crate) root: Nodes<T>,

    // TODO: How should we track expanded keys?...
    pub(crate) expanded_keys: HashSet<Key>,
}

impl<T> NodeTree<T> {
    pub(crate) fn new() -> Self {
        Self {
            root: BTreeMap::new(),
            expanded_keys: HashSet::new(),
        }
    }
}
