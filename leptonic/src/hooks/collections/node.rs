// Upstream: react-aria/src/collections/BaseCollection.ts @ 99e6102368
use std::sync::Arc;

use super::Key;
use crate::utils::{Modifiers, dom_ext::ElementExt, open_link::open_link};

/// What a [`Node`] represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeKind {
    /// A selectable, focusable item (an option, a menu item, a row, a tree item, ...).
    Item,
    /// A group of nodes, e.g. a listbox section.
    Section,
    /// The heading of a section.
    Header,
    /// A visual separator between nodes.
    Separator,
    /// A placeholder shown while more items load.
    Loader,
    /// A cell of a grid row (the row is an `Item`).
    Cell,
    /// A row of table column headers (before the body rows).
    HeaderRow,
    /// A table column header, in a `HeaderRow`.
    Column,
    /// An empty header cell, filling a `HeaderRow` where a column has no column group above it.
    Placeholder,
}

/// A link target for items that navigate when activated ("items as links").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemLink {
    pub href: Arc<str>,
    pub target: Option<Arc<str>>,
    pub rel: Option<Arc<str>>,
}

impl ItemLink {
    pub fn new(href: impl Into<Arc<str>>) -> Self {
        Self {
            href: href.into(),
            target: None,
            rel: None,
        }
    }

    /// Opens the link as if `element` (the item) was a link that was clicked with `modifiers`. An
    /// `<a>` item is clicked itself; for other elements, a temporary `<a>` is clicked, and focus
    /// returns to the item.
    ///
    /// Runs in a microtask: the item's own click handling (e.g. a press ending in a `click`) may
    /// still be running, and a nested `click` dispatch can't re-enter its listener.
    pub(crate) fn open(&self, element: &web_sys::Element, modifiers: Modifiers) {
        let link = self.clone();
        let element = send_wrapper::SendWrapper::new(element.clone());
        leptos::prelude::queue_microtask(move || link.open_now(&element, modifiers));
    }

    fn open_now(&self, element: &web_sys::Element, modifiers: Modifiers) {
        if element.is_anchor_link() {
            open_link(element, modifiers);
            return;
        }
        let Some(document) = element.owner_document() else {
            return;
        };
        let Ok(link) = document.create_element("a") else {
            return;
        };
        let _ = link.set_attribute("href", &self.href);
        if let Some(target) = &self.target {
            let _ = link.set_attribute("target", target);
        }
        if let Some(rel) = &self.rel {
            let _ = link.set_attribute("rel", rel);
        }
        if element.append_child(&link).is_ok() {
            open_link(&link, modifiers);
            let _ = element.remove_child(&link);
            // Opening focused the temporary link, which is gone now.
            crate::utils::focus::focus_element(element, true);
        }
    }
}

/// One entry of a [`Collection`](super::Collection): an item, a section, a header, ...
///
/// Nodes are created by [`CollectionBuilder`](super::CollectionBuilder) and are immutable.
/// Besides their own data, they link to their neighbors, parent and children, so that
/// collections can be traversed in document order without allocations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Node {
    pub key: Key,
    pub kind: NodeKind,
    /// A plain text representation, used for type-ahead, filtering and accessibility.
    pub text_value: Arc<str>,
    /// An accessible name, for nodes whose text value isn't a good label.
    pub aria_label: Option<Arc<str>>,
    /// The tree depth: `0` for top-level items and items in sections, `1` for children of a
    /// top-level item, and so on. Sections don't add a level.
    pub level: usize,
    /// The position among the parent's children (or among the top-level nodes).
    pub index: usize,
    pub parent_key: Option<Key>,
    pub prev_key: Option<Key>,
    pub next_key: Option<Key>,
    pub first_child_key: Option<Key>,
    pub last_child_key: Option<Key>,
    /// Whether this node has children (sections: their contents, items: tree children).
    pub has_child_nodes: bool,
    /// Whether the item itself is disabled (in addition to any `disabled_keys` of the state).
    pub is_disabled: bool,
    /// How the item behaves while disabled, overriding the collection's `disabled_behavior`
    /// (react-aria: an item's `disabledBehavior` prop). `None`: the collection's.
    pub disabled_behavior: Option<super::DisabledBehavior>,
    /// Where the item navigates to, for items that are links.
    pub link: Option<ItemLink>,
    /// Cells: the column, counting spanned columns (set when a row has cells spanning columns, and
    /// always for the cells of table header rows).
    pub col_index: Option<usize>,
    /// Cells (and table column groups): how many columns the cell spans.
    pub col_span: Option<usize>,
}

impl Node {
    pub fn is_item(&self) -> bool {
        self.kind == NodeKind::Item
    }
}
