// Upstream: react-stately/src/virtualizer/Virtualizer.ts @ 99e6102368
// Upstream: react-stately/src/virtualizer/utils.ts @ 99e6102368
// Upstream: react-stately/src/virtualizer/ReusableView.ts @ 99e6102368
use std::{collections::HashSet, sync::Arc};

use super::{
    InvalidationContext, Layout, LayoutInfo, OverscanManager, ScrollAnchorTracker,
    VirtualizerContext,
};
use crate::{
    hooks::collections::{Collection, Key, Rect, Size},
    utils::point::Point,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - No reusable views: `render` returns the visible layout infos; the caller renders them keyed
//   (Leptos' keyed `For` keeps each key's elements). The delegate's `setVisibleRect` is
//   `RenderResult::scroll_to`, its `invalidate` the caller's next `render` with a new
//   invalidation generation.
// - The layout gets the virtualizer's state as a `VirtualizerContext` argument (react-stately:
//   a `virtualizer` back reference).
//
// - The views stay in layout order while scrolling (react-stately keeps their order and puts new
//   ones last until the scrolling ends, sparing DOM moves): the renderer mounts each new row
//   before its successor and leaves mounted rows in place, so a scroll needs no reordering pass.
//
// ## ADDITIONS
// - Anchoring that switches on (the layout's scroll anchor info appearing, e.g. a log's "follow"
//   turned on) starts over like the first anchored layout: it snaps to the edge and keeps it while
//   sizes settle (react-stately resets the anchor tracker only for a new layout).
//
// =============================================================================

/// What [`Virtualizer::render`] renders from (react-stately's `VirtualizerRenderOptions`).
#[derive(Debug, Clone)]
pub struct RenderInput<O> {
    pub collection: Arc<Collection>,
    /// Keys that stay rendered while not visible (e.g. the focused item).
    pub persisted_keys: HashSet<Key>,
    pub visible_rect: Rect,
    /// The scroll view's size.
    pub size: Size,
    /// What changed, with its generation: a new generation is a new invalidation.
    pub invalidation: InvalidationContext<O>,
    pub invalidation_generation: u64,
    pub is_scrolling: bool,
    /// The current time in milliseconds (for the overscan's scroll velocity).
    pub now: f64,
}

/// The result of [`Virtualizer::render`].
#[derive(Debug, Clone, PartialEq)]
pub struct RenderResult {
    /// The layout infos to render, parents before children.
    pub visible: Vec<LayoutInfo>,
    /// Scroll the view here (e.g. to keep an anchor in place), then render again.
    pub scroll_to: Option<Rect>,
}

/// Lays out a collection with a [`Layout`] and finds what is visible (react-stately's
/// `Virtualizer`).
pub struct Virtualizer<L: Layout> {
    pub layout: L,
    collection: Arc<Collection>,
    content_size: Size,
    visible_rect: Rect,
    size: Size,
    persisted_keys: HashSet<Key>,
    visible: Vec<LayoutInfo>,
    is_scrolling: bool,
    invalidation: InvalidationContext<L::Options>,
    /// The generation of `invalidation`; `None` before the first render (which always takes its
    /// invalidation, e.g. the first layout options).
    invalidation_generation: Option<u64>,
    overscan: OverscanManager,
    scroll_anchor: ScrollAnchorTracker,
    /// Whether the last layout pass was anchored.
    was_anchored: bool,
}

impl<L: Layout> std::fmt::Debug for Virtualizer<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Virtualizer")
            .field("content_size", &self.content_size)
            .field("visible_rect", &self.visible_rect)
            .field("visible", &self.visible.len())
            .finish_non_exhaustive()
    }
}

/// The virtualizer's state for the layout, borrowing its fields (not the layout itself).
macro_rules! context {
    ($self:ident) => {
        VirtualizerContext {
            collection: &$self.collection,
            persisted_keys: &$self.persisted_keys,
            size: $self.size,
            visible_rect: $self.visible_rect,
            content_size: $self.content_size,
        }
    };
}

impl<L: Layout> Virtualizer<L> {
    pub fn new(layout: L) -> Self {
        Self {
            layout,
            collection: Arc::new(Collection::default()),
            content_size: Size::default(),
            visible_rect: Rect::default(),
            size: Size::default(),
            persisted_keys: HashSet::new(),
            visible: Vec::new(),
            is_scrolling: false,
            invalidation: InvalidationContext::default(),
            invalidation_generation: None,
            overscan: OverscanManager::default(),
            scroll_anchor: ScrollAnchorTracker::default(),
            was_anchored: false,
        }
    }

    /// The size of the scrollable content.
    pub fn content_size(&self) -> Size {
        self.content_size
    }

    /// The currently visible rectangle.
    pub fn visible_rect(&self) -> Rect {
        self.visible_rect
    }

    pub fn collection(&self) -> &Arc<Collection> {
        &self.collection
    }

    /// The layout info of `key` (laying out more of the collection if needed).
    pub fn layout_info(&mut self, key: &Key) -> Option<LayoutInfo> {
        let ctx = context!(self);
        self.layout.layout_info(&ctx, key)
    }

    /// The key of the item at `point` (content coordinates).
    pub fn key_at_point(&mut self, point: Point) -> Option<Key> {
        let rect = Rect::new(point.x, point.y, 1.0, 1.0);
        let ctx = context!(self);
        // Persisted keys may come along: find one that intersects.
        self.layout
            .visible_layout_infos(&ctx, rect)
            .into_iter()
            .find(|info| info.rect.intersects(&rect))
            .map(|info| info.key)
    }

    /// Sets the measured size of `key`; whether it changed the layout (then render again with
    /// `item_size_changed` in a new invalidation).
    pub fn update_item_size(&mut self, key: &Key, size: Size) -> bool {
        let ctx = context!(self);
        self.layout.update_item_size(&ctx, key, size)
    }

    fn visible_layout_infos(&mut self) -> Vec<LayoutInfo> {
        let rect = self.overscan.overscanned_rect();
        let ctx = context!(self);
        self.layout.visible_layout_infos(&ctx, rect)
    }

    /// Lays out the collection again; a scroll target if the viewport has to move first.
    fn relayout(&mut self, invalidation: &InvalidationContext<L::Options>) -> Option<Rect> {
        let anchor_info = self
            .layout
            .scroll_anchor_info(invalidation.layout_options.as_ref());
        if anchor_info.is_some() && !self.was_anchored {
            self.scroll_anchor.reset();
        }
        self.was_anchored = anchor_info.is_some();
        // The anchor from the positions before the layout (none on the first render).
        let anchor = anchor_info.as_ref().and_then(|anchor_info| {
            let previous = std::mem::take(&mut self.visible);
            let pre_layout: Vec<LayoutInfo> = previous
                .iter()
                .map(|view| self.layout_info(&view.key).unwrap_or_else(|| view.clone()))
                .collect();
            self.visible = previous;
            self.scroll_anchor.capture_before_layout(
                Some(anchor_info),
                pre_layout.iter().map(|info| (&info.key, info)),
                &self.visible_rect,
            )
        });

        let previous_content_size = self.content_size;
        let previous_visible_rect = self.visible_rect;
        let ctx = context!(self);
        self.layout.update(&ctx, invalidation);
        let size = self.layout.content_size();
        self.content_size = Size::new(size.width, size.height);

        let post_layout = if anchor_info.is_some() {
            self.visible_layout_infos()
        } else {
            Vec::new()
        };
        let collection = Arc::clone(&self.collection);
        let persisted_keys = self.persisted_keys.clone();
        let (layout, content_size, visible_rect) =
            (&mut self.layout, self.content_size, self.visible_rect);
        let layout = std::cell::RefCell::new(layout);
        let target = self
            .scroll_anchor
            .resolve_after_layout(super::ResolveAfterLayout {
                anchor_info: anchor_info.as_ref(),
                anchor: anchor.as_ref(),
                post_layout_infos: &post_layout,
                previous_visible_rect,
                previous_content_size,
                content_size,
                item_size_changed: invalidation.item_size_changed,
                is_scrolling: self.is_scrolling,
                layout_info: |key: &Key| {
                    let ctx = VirtualizerContext {
                        collection: &collection,
                        persisted_keys: &persisted_keys,
                        size: self.size,
                        visible_rect,
                        content_size,
                    };
                    layout.borrow_mut().layout_info(&ctx, key)
                },
            });
        if target.is_some() {
            // A new render follows at the target: positioning the views against the old visible
            // rectangle now would flash.
            return target;
        }

        // Keep the scroll position within the content (at the top after the content changed).
        let visible = self.visible_rect;
        let max_x = (self.content_size.width - visible.width).max(0.0);
        let max_y = (self.content_size.height - visible.height).max(0.0);
        let x = if invalidation.content_changed {
            0.0
        } else {
            visible.x
        }
        .min(max_x)
        .max(0.0);
        let y = if invalidation.content_changed {
            0.0
        } else {
            visible.y
        }
        .min(max_y)
        .max(0.0);
        if x != visible.x || y != visible.y {
            return Some(Rect::new(x, y, visible.width, visible.height));
        }
        self.update_subviews();
        None
    }

    fn update_subviews(&mut self) {
        // In layout order (react-stately keeps the views' order while scrolling, as reordering
        // DOM nodes is costly there; the renderer here never moves a mounted row anyway).
        self.visible = self.visible_layout_infos();
    }

    /// Lays out what changed and returns the views to render.
    pub fn render(&mut self, input: RenderInput<L::Options>) -> RenderResult {
        let RenderInput {
            collection,
            persisted_keys,
            visible_rect,
            size,
            invalidation,
            invalidation_generation,
            is_scrolling,
            now,
        } = input;
        let mut needs_layout = false;
        let mut needs_update = false;
        let mut offset_changed = false;
        let mut size_changed = false;
        let mut width_changed = false;
        let mut height_changed = false;
        let mut item_size_changed = false;
        let mut layout_options_changed = false;

        if !Arc::ptr_eq(&collection, &self.collection) {
            self.collection = collection;
            needs_layout = true;
        }
        if persisted_keys != self.persisted_keys {
            self.persisted_keys = persisted_keys;
            needs_update = true;
        }
        if self.visible_rect != visible_rect || self.size != size {
            self.overscan.set_visible_rect(visible_rect, now);
            // The scroll position with the scroll view's size (the visible rectangle may be
            // smaller while the window scrolls).
            let old_rect = Rect::new(
                self.visible_rect.x,
                self.visible_rect.y,
                self.size.width,
                self.size.height,
            );
            let new_rect = Rect::new(visible_rect.x, visible_rect.y, size.width, size.height);
            if self.layout.should_invalidate(new_rect, old_rect) {
                offset_changed = !visible_rect.point_equals(&self.visible_rect);
                size_changed = self.size != size;
                width_changed = self.size.width != size.width;
                height_changed = self.size.height != size.height;
                needs_layout = true;
            } else {
                needs_update = true;
            }
            self.visible_rect = visible_rect;
            self.size = size;
        }
        if Some(invalidation_generation) != self.invalidation_generation {
            size_changed |= invalidation.size_changed;
            width_changed |= invalidation.width_changed;
            height_changed |= invalidation.height_changed;
            offset_changed |= invalidation.offset_changed;
            item_size_changed |= invalidation.item_size_changed;
            layout_options_changed |= match (
                &invalidation.layout_options,
                &self.invalidation.layout_options,
            ) {
                (Some(new), Some(old)) => self.layout.should_invalidate_layout_options(new, old),
                _ => false,
            };
            needs_layout |=
                item_size_changed || size_changed || offset_changed || layout_options_changed;
            self.invalidation = invalidation;
            self.invalidation_generation = Some(invalidation_generation);
        }
        // Only for the anchor: the views' order doesn't depend on it (see `update_subviews`).
        self.is_scrolling = is_scrolling;

        let mut scroll_to = None;
        if needs_layout {
            let invalidation = InvalidationContext {
                content_changed: false,
                offset_changed,
                size_changed,
                width_changed,
                height_changed,
                item_size_changed,
                layout_options_changed,
                layout_options: self.invalidation.layout_options.clone(),
            };
            scroll_to = self.relayout(&invalidation);
        } else if needs_update {
            self.update_subviews();
        }
        RenderResult {
            visible: self.visible.clone(),
            scroll_to,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, sync::Arc};

    use assertr::prelude::*;

    use super::*;
    use crate::hooks::virtualizer::{EndAnchor, ItemSize, ListLayout, ListLayoutOptions};

    fn rows(count: usize) -> Arc<Collection> {
        Arc::new(Collection::build(|b| {
            for i in 0..count {
                b.item(format!("row-{i}"), format!("Row {i}"));
            }
        }))
    }

    fn input(
        collection: &Arc<Collection>,
        y: f64,
        generation: u64,
    ) -> RenderInput<ListLayoutOptions> {
        RenderInput {
            collection: Arc::clone(collection),
            persisted_keys: HashSet::new(),
            visible_rect: Rect::new(0.0, y, 400.0, 480.0),
            size: Size::new(400.0, 480.0),
            invalidation: InvalidationContext::default(),
            invalidation_generation: generation,
            is_scrolling: false,
            now: 0.0,
        }
    }

    fn keys(result: &RenderResult) -> Vec<String> {
        result
            .visible
            .iter()
            .map(|info| info.key.to_string())
            .collect()
    }

    #[test]
    fn renders_the_visible_rows_of_fixed_size() {
        let collection = rows(1000);
        let mut virtualizer = Virtualizer::new(ListLayout::new(ListLayoutOptions {
            row_size: ItemSize::Fixed(48.0),
            ..ListLayoutOptions::default()
        }));
        let result = virtualizer.render(input(&collection, 0.0, 0));
        assert_that!(virtualizer.content_size()).is_equal_to(Size::new(400.0, 48_000.0));
        // 480px plus a third of overscan: 640px, snapped to whole rows (672px: 14 rows), plus the
        // row touching its edge (rectangles touching count as intersecting, as upstream).
        assert_that!(keys(&result).first().cloned()).is_equal_to(Some("row-0".to_owned()));
        assert_that!(result.visible.len()).is_equal_to(15);

        // Scrolled down: row 99 touches the viewport's top edge, row 100 starts there.
        let result = virtualizer.render(input(&collection, 4800.0, 0));
        assert_that!(keys(&result).first().cloned()).is_equal_to(Some("row-99".to_owned()));
        assert_that!(result.visible[1].key.to_string()).is_equal_to("row-100".to_owned());
        assert_that!(result.visible[1].rect).is_equal_to(Rect::new(0.0, 4800.0, 400.0, 48.0));
    }

    #[test]
    fn the_first_render_applies_its_layout_options() {
        let collection = rows(10);
        let mut virtualizer = Virtualizer::new(ListLayout::new(ListLayoutOptions {
            row_size: ItemSize::Fixed(32.0),
            ..ListLayoutOptions::default()
        }));
        let mut first = input(&collection, 0.0, 0);
        first.invalidation.layout_options = Some(ListLayoutOptions {
            row_size: ItemSize::Fixed(48.0),
            ..ListLayoutOptions::default()
        });
        virtualizer.render(first);
        assert_that!(virtualizer.content_size().height).is_equal_to(480.0);
    }

    #[test]
    fn measures_estimated_rows() {
        let collection = rows(100);
        let mut virtualizer = Virtualizer::new(ListLayout::new(ListLayoutOptions {
            row_size: ItemSize::Estimated(20.0),
            ..ListLayoutOptions::default()
        }));
        let result = virtualizer.render(input(&collection, 0.0, 0));
        assert_that!(result.visible[0].estimated_size).is_true();
        assert_that!(virtualizer.content_size().height).is_equal_to(2000.0);

        // The first row is 50px high: the rows after it move, the content grows.
        assert_that!(virtualizer.update_item_size(&Key::from("row-0"), Size::new(400.0, 50.0)))
            .is_true();
        let mut next = input(&collection, 0.0, 1);
        next.invalidation.item_size_changed = true;
        let result = virtualizer.render(next);
        assert_that!(result.visible[0].estimated_size).is_false();
        assert_that!(result.visible[1].rect.y).is_equal_to(50.0);
        assert_that!(virtualizer.content_size().height).is_equal_to(2030.0);
    }

    #[test]
    fn keeps_persisted_keys_and_lays_out_far_keys() {
        let collection = rows(1000);
        let mut virtualizer = Virtualizer::new(ListLayout::new(ListLayoutOptions {
            row_size: ItemSize::Fixed(48.0),
            ..ListLayoutOptions::default()
        }));
        let mut first = input(&collection, 0.0, 0);
        first.persisted_keys = HashSet::from([Key::from("row-900")]);
        let result = virtualizer.render(first);
        assert_that!(keys(&result).contains(&"row-900".to_owned())).is_true();
        // A key outside the area laid out so far (e.g. End): laid out on demand.
        let info = virtualizer.layout_info(&Key::from("row-999"));
        assert_that!(info.map(|info| info.rect.y)).is_equal_to(Some(999.0 * 48.0));
    }

    #[test]
    fn stays_at_the_end_when_rows_are_appended() {
        let mut virtualizer = Virtualizer::new(ListLayout::new(ListLayoutOptions {
            row_size: ItemSize::Fixed(48.0),
            anchor_to_end: Some(EndAnchor::default()),
            ..ListLayoutOptions::default()
        }));
        // The first anchored layout snaps to the end.
        let collection = rows(100);
        let result = virtualizer.render(input(&collection, 0.0, 0));
        let end = 100.0 * 48.0 - 480.0;
        assert_that!(result.scroll_to.map(|rect| rect.y)).is_equal_to(Some(end));
        virtualizer.render(input(&collection, end, 0));

        // At the end, appended rows keep the viewport at the (new) end.
        let more = rows(110);
        let result = virtualizer.render(input(&more, end, 0));
        assert_that!(result.scroll_to.map(|rect| rect.y)).is_equal_to(Some(110.0 * 48.0 - 480.0));

        // Scrolled away from the end, appended rows don't move the viewport.
        virtualizer.render(input(&more, 1000.0, 0));
        let result = virtualizer.render(input(&rows(120), 1000.0, 0));
        assert_that!(result.scroll_to).is_none();
    }
}
