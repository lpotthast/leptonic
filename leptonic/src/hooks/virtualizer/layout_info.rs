// Upstream: react-stately/src/virtualizer/LayoutInfo.ts @ 99e6102368
use std::sync::Arc;

use crate::hooks::collections::{Key, NodeKind, Rect};

/// Where an element of a virtualized collection goes, and how it is shown: produced by a
/// [`Layout`](super::Layout) for each collection node it lays out.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutInfo {
    /// What the element represents (the collection node's kind).
    pub kind: NodeKind,
    /// The collection node's key.
    pub key: Key,
    /// The key of the parent layout info (e.g. the section of an item).
    pub parent_key: Option<Key>,
    /// The element's size and position.
    pub rect: Rect,
    /// Whether the size is estimated: the element is measured once rendered.
    pub estimated_size: bool,
    /// Whether the element sticks to the viewport while scrolling.
    pub is_sticky: bool,
    pub opacity: f64,
    /// A CSS transform for the element.
    pub transform: Option<Arc<str>>,
    pub z_index: i32,
    /// Whether the element's contents may overflow it.
    pub allow_overflow: bool,
}

impl LayoutInfo {
    /// A layout info of `kind` for `key` at `rect`, otherwise with defaults.
    pub fn new(kind: NodeKind, key: Key, rect: Rect) -> Self {
        Self {
            kind,
            key,
            parent_key: None,
            rect,
            estimated_size: false,
            is_sticky: false,
            opacity: 1.0,
            transform: None,
            z_index: 0,
            allow_overflow: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    // Upstream: react-stately/test/virtualizer/LayoutInfo.test.tsx
    #[test]
    fn copies_all_fields() {
        let mut info = LayoutInfo::new(
            NodeKind::Item,
            Key::from("a"),
            Rect::new(1.0, 2.0, 3.0, 4.0),
        );
        info.estimated_size = true;
        info.is_sticky = true;
        info.opacity = 0.5;
        info.transform = Some("rotate(45deg)".into());
        info.z_index = 3;
        info.allow_overflow = true;
        info.parent_key = Some(Key::from("p"));
        let copy = info.clone();
        assert_that!(copy).is_equal_to(info);
    }
}
