// Upstream: react-stately/src/virtualizer/Layout.ts @ 99e6102368
// Upstream: react-stately/src/virtualizer/types.ts @ 99e6102368
use std::{collections::HashSet, sync::Arc};

use super::{LayoutInfo, ScrollAnchorInfo};
use crate::hooks::collections::{Collection, Key, Rect, Size};

/// What changed since the last layout (react-stately's `InvalidationContext`).
// Independent change flags, as upstream.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq)]
pub struct InvalidationContext<O> {
    pub content_changed: bool,
    pub offset_changed: bool,
    pub size_changed: bool,
    pub width_changed: bool,
    pub height_changed: bool,
    pub item_size_changed: bool,
    pub layout_options_changed: bool,
    /// The layout options of this pass.
    pub layout_options: Option<O>,
}

// Not derived: that would require `O: Default`.
impl<O> Default for InvalidationContext<O> {
    fn default() -> Self {
        Self {
            content_changed: false,
            offset_changed: false,
            size_changed: false,
            width_changed: false,
            height_changed: false,
            item_size_changed: false,
            layout_options_changed: false,
            layout_options: None,
        }
    }
}

/// The virtualizer's state a [`Layout`] works with (react-stately's layouts reach it through
/// their `virtualizer` back reference).
#[derive(Debug, Clone, Copy)]
pub struct VirtualizerContext<'a> {
    pub collection: &'a Arc<Collection>,
    pub persisted_keys: &'a HashSet<Key>,
    /// The scroll view's size.
    pub size: Size,
    pub visible_rect: Rect,
    /// The content size of the previous layout pass.
    pub content_size: Size,
}

/// Computes where the elements of a virtualized collection go (react-stately's `Layout`): the
/// virtualizer renders the elements whose layout infos intersect the visible rectangle.
pub trait Layout: Send + Sync + 'static {
    /// The options of this layout (react-stately's `layoutOptions`).
    type Options: Clone + PartialEq + Send + Sync + 'static;

    /// The layout infos inside `rect` (and those of persisted keys).
    fn visible_layout_infos(&mut self, ctx: &VirtualizerContext<'_>, rect: Rect)
    -> Vec<LayoutInfo>;

    /// The layout info of `key`, laying out more of the collection if needed.
    fn layout_info(&mut self, ctx: &VirtualizerContext<'_>, key: &Key) -> Option<LayoutInfo>;

    /// The size of the whole content.
    fn content_size(&self) -> Size;

    /// Whether a change of the visible rectangle invalidates the layout. Default: when its size
    /// changes (layouts with sticky elements also invalidate when scrolling).
    fn should_invalidate(&self, new_rect: Rect, old_rect: Rect) -> bool {
        new_rect.width != old_rect.width || new_rect.height != old_rect.height
    }

    /// Whether new layout options invalidate the layout. Default: when they differ.
    fn should_invalidate_layout_options(&self, new: &Self::Options, old: &Self::Options) -> bool {
        new != old
    }

    /// Prepares the layout infos (called before the visible ones are read).
    fn update(
        &mut self,
        ctx: &VirtualizerContext<'_>,
        invalidation: &InvalidationContext<Self::Options>,
    );

    /// Sets the measured size of `key`; whether it changed the layout.
    fn update_item_size(&mut self, ctx: &VirtualizerContext<'_>, key: &Key, size: Size) -> bool {
        let _ = (ctx, key, size);
        false
    }

    /// Which edge the viewport stays anchored to while the content changes (react-stately's
    /// `UNSTABLE_getScrollAnchorInfo`). Default: none.
    fn scroll_anchor_info(&self, options: Option<&Self::Options>) -> Option<ScrollAnchorInfo> {
        let _ = options;
        None
    }
}
