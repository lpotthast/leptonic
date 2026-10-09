// Upstream: react-stately/src/virtualizer/ScrollAnchor.ts @ 99e6102368
// Upstream: react-stately/test/virtualizer/ScrollAnchor.test.ts @ 99e6102368
use std::sync::Arc;

use super::LayoutInfo;
use crate::hooks::collections::{Key, Rect, RectCorner, Size};

/// The edge of the content the viewport stays anchored to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollAnchorEdge {
    Start,
    End,
}

/// The scroll axis anchoring works along.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollAnchorAxis {
    X,
    Y,
}

impl ScrollAnchorAxis {
    fn of_rect(self, rect: &Rect) -> f64 {
        match self {
            Self::X => rect.x,
            Self::Y => rect.y,
        }
    }

    fn of_point(self, point: crate::utils::point::Point) -> f64 {
        match self {
            Self::X => point.x,
            Self::Y => point.y,
        }
    }

    /// The rectangle's extent along the axis.
    fn dimension_of_rect(self, rect: &Rect) -> f64 {
        match self {
            Self::X => rect.width,
            Self::Y => rect.height,
        }
    }

    fn dimension_of_size(self, size: Size) -> f64 {
        match self {
            Self::X => size.width,
            Self::Y => size.height,
        }
    }

    fn with(self, rect: &Rect, value: f64) -> Rect {
        match self {
            Self::X => Rect::new(value, rect.y, rect.width, rect.height),
            Self::Y => Rect::new(rect.x, value, rect.width, rect.height),
        }
    }
}

/// An item the viewport keeps in place: its key, which corner, and that corner's offset from
/// the viewport's start.
#[derive(Debug, Clone, PartialEq)]
pub struct ScrollAnchor {
    pub key: Key,
    pub corner: RectCorner,
    pub offset: f64,
}

/// How a layout anchors the viewport (react-stately's `ScrollAnchorInfo`).
#[derive(Clone)]
pub struct ScrollAnchorInfo {
    /// The edge of the content the viewport stays anchored to.
    pub edge: ScrollAnchorEdge,
    /// The axis `edge` refers to: `Y` for vertical lists.
    pub axis: ScrollAnchorAxis,
    /// Within this distance (px) from `edge`, the viewport is "following" it.
    pub threshold: f64,
    /// Excludes layout infos (e.g. loaders) from being the anchor. Default: all can be.
    pub is_anchorable: Option<Arc<dyn Fn(&LayoutInfo) -> bool + Send + Sync>>,
}

impl std::fmt::Debug for ScrollAnchorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScrollAnchorInfo")
            .field("edge", &self.edge)
            .field("axis", &self.axis)
            .field("threshold", &self.threshold)
            .finish_non_exhaustive()
    }
}

/// An item must overlap the viewport by this much (along the axis) to be the anchor: an item
/// that only slivers into it (essentially scrolled out of view) must not win over a visible one.
const MIN_ANCHOR_OVERLAP: f64 = 4.0;

/// The viewport coordinate (along `axis`) that keeps `anchor` at its offset, if it moved.
pub fn compute_scroll_anchor_target(
    anchor: &ScrollAnchor,
    axis: ScrollAnchorAxis,
    layout_info: impl Fn(&Key) -> Option<LayoutInfo>,
    visible_rect: &Rect,
    content_size: Size,
) -> Option<f64> {
    let final_info = layout_info(&anchor.key)?;
    let adjustment = axis.of_point(final_info.rect.corner(anchor.corner))
        - axis.of_rect(visible_rect)
        - anchor.offset;
    if adjustment == 0.0 {
        return None;
    }
    let target = axis.of_rect(visible_rect) + adjustment;
    let max =
        (axis.dimension_of_size(content_size) - axis.dimension_of_rect(visible_rect)).max(0.0);
    let clamped = target.min(max).max(0.0);
    (clamped != axis.of_rect(visible_rect)).then_some(clamped)
}

/// The item to anchor to: the one nearest the viewport's start when anchoring to the end, nearest
/// its end when anchoring to the start.
pub fn capture_scroll_anchor<'a>(
    edge: ScrollAnchorEdge,
    axis: ScrollAnchorAxis,
    visible_rect: &Rect,
    visible_layout_infos: impl IntoIterator<Item = (&'a Key, &'a LayoutInfo)>,
    is_anchorable: Option<&(dyn Fn(&LayoutInfo) -> bool + Send + Sync)>,
) -> Option<ScrollAnchor> {
    let mut best: Option<ScrollAnchor> = None;
    for (key, layout_info) in visible_layout_infos {
        if is_anchorable.is_some_and(|is_anchorable| !is_anchorable(layout_info)) {
            continue;
        }
        let overlap = axis.dimension_of_rect(&layout_info.rect.intersection(visible_rect));
        if layout_info.rect.area() > 0.0 && overlap >= MIN_ANCHOR_OVERLAP {
            let corner = layout_info
                .rect
                .corner_in_rect(visible_rect)
                .unwrap_or(RectCorner::TopLeft);
            let offset =
                axis.of_point(layout_info.rect.corner(corner)) - axis.of_rect(visible_rect);
            let is_better = best.as_ref().is_none_or(|best| match edge {
                ScrollAnchorEdge::End => offset < best.offset,
                ScrollAnchorEdge::Start => offset > best.offset,
            });
            if is_better {
                best = Some(ScrollAnchor {
                    key: key.clone(),
                    corner,
                    offset,
                });
            }
        }
    }
    best
}

/// The viewport coordinate (along `axis`) that pins the viewport to `edge` of the content.
pub fn edge_snap_target(
    edge: ScrollAnchorEdge,
    axis: ScrollAnchorAxis,
    content_size: Size,
    previous_visible_rect: &Rect,
) -> f64 {
    match edge {
        ScrollAnchorEdge::Start => 0.0,
        ScrollAnchorEdge::End => (axis.dimension_of_size(content_size)
            - axis.dimension_of_rect(previous_visible_rect))
        .max(0.0),
    }
}

/// Whether the viewport is within `threshold` px of `edge` of the content.
pub fn is_near_edge(
    visible_rect: &Rect,
    content_size: Size,
    edge: ScrollAnchorEdge,
    axis: ScrollAnchorAxis,
    threshold: f64,
) -> bool {
    match edge {
        ScrollAnchorEdge::Start => axis.of_rect(visible_rect) <= threshold,
        ScrollAnchorEdge::End => {
            let distance_from_end = axis.dimension_of_size(content_size)
                - (axis.of_rect(visible_rect) + axis.dimension_of_rect(visible_rect));
            distance_from_end <= threshold
        }
    }
}

/// The new viewport after the content changed: the anchor where it was, else (if the user was
/// near the edge and isn't scrolling away) the edge.
#[allow(clippy::too_many_arguments)]
pub fn resolve_scroll_adjustment(
    edge: ScrollAnchorEdge,
    axis: ScrollAnchorAxis,
    anchor: Option<&ScrollAnchor>,
    was_near_anchor_edge: bool,
    is_scrolling: bool,
    item_size_changed: bool,
    content_size_delta: f64,
    layout_info: impl Fn(&Key) -> Option<LayoutInfo>,
    previous_visible_rect: &Rect,
    content_size: Size,
) -> Option<Rect> {
    if let Some(anchor) = anchor
        && let Some(target) = compute_scroll_anchor_target(
            anchor,
            axis,
            layout_info,
            previous_visible_rect,
            content_size,
        )
    {
        return Some(axis.with(previous_visible_rect, target));
    }
    if was_near_anchor_edge && !is_scrolling && (!item_size_changed || content_size_delta > 0.0) {
        let target = axis.with(
            previous_visible_rect,
            edge_snap_target(edge, axis, content_size, previous_visible_rect),
        );
        return (target != *previous_visible_rect).then_some(target);
    }
    None
}

/// What [`ScrollAnchorTracker::resolve_after_layout`] decides from.
pub struct ResolveAfterLayout<'a, F: Fn(&Key) -> Option<LayoutInfo>> {
    pub anchor_info: Option<&'a ScrollAnchorInfo>,
    /// The anchor captured before this pass's layout update.
    pub anchor: Option<&'a ScrollAnchor>,
    /// The visible layout infos after the update.
    pub post_layout_infos: &'a [LayoutInfo],
    pub previous_visible_rect: Rect,
    pub previous_content_size: Size,
    pub content_size: Size,
    pub item_size_changed: bool,
    pub is_scrolling: bool,
    pub layout_info: F,
}

/// Keeps the viewport anchored to a layout's edge across layout passes (react-stately's
/// `ScrollAnchorTracker`).
#[derive(Debug, Clone, Default)]
pub struct ScrollAnchorTracker {
    has_snapped_to_edge: bool,
    had_estimated_visible_items: bool,
    was_near_anchor_edge: bool,
}

impl ScrollAnchorTracker {
    /// Forgets the tracked state (e.g. when the layout changes).
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Captures the anchor from the positions before the layout update.
    pub fn capture_before_layout<'a>(
        &self,
        anchor_info: Option<&ScrollAnchorInfo>,
        pre_layout_infos: impl IntoIterator<Item = (&'a Key, &'a LayoutInfo)>,
        visible_rect: &Rect,
    ) -> Option<ScrollAnchor> {
        let anchor_info = anchor_info?;
        capture_scroll_anchor(
            anchor_info.edge,
            anchor_info.axis,
            visible_rect,
            pre_layout_infos,
            anchor_info.is_anchorable.as_deref(),
        )
    }

    /// Decides after a layout update where the viewport goes, if anywhere.
    pub fn resolve_after_layout<F: Fn(&Key) -> Option<LayoutInfo>>(
        &mut self,
        options: ResolveAfterLayout<'_, F>,
    ) -> Option<Rect> {
        let ResolveAfterLayout {
            anchor_info,
            anchor,
            post_layout_infos,
            previous_visible_rect,
            previous_content_size,
            content_size,
            item_size_changed,
            is_scrolling,
            layout_info,
        } = options;
        let anchor_info = anchor_info?;

        // The previous pass's state, before the writes below.
        let was_settling_last_pass = self.had_estimated_visible_items;
        let was_near_anchor_edge_last_pass = self.was_near_anchor_edge;
        self.had_estimated_visible_items = post_layout_infos.iter().any(|info| info.estimated_size);

        // Mid-resize, "near the edge?" would look like a scroll that never happened: reuse the
        // answer from before the resizing started.
        if !was_settling_last_pass {
            self.was_near_anchor_edge = is_near_edge(
                &previous_visible_rect,
                previous_content_size,
                anchor_info.edge,
                anchor_info.axis,
                anchor_info.threshold,
            );
        }
        if previous_visible_rect.area() == 0.0 {
            return None;
        }
        let axis = anchor_info.axis;
        let content_size_delta =
            axis.dimension_of_size(content_size) - axis.dimension_of_size(previous_content_size);
        let is_first_anchored_layout = !self.has_snapped_to_edge;
        self.has_snapped_to_edge = true;

        // Only when the content changed (or on the first layout, which always snaps).
        if !(is_first_anchored_layout || content_size_delta != 0.0 || item_size_changed) {
            return None;
        }
        let was_near_anchor_edge = is_first_anchored_layout
            || (was_settling_last_pass && was_near_anchor_edge_last_pass)
            || is_near_edge(
                &previous_visible_rect,
                previous_content_size,
                anchor_info.edge,
                axis,
                anchor_info.threshold,
            );
        // A first layout always snaps: later passes of this cascade reuse that decision.
        if !was_settling_last_pass {
            self.was_near_anchor_edge = was_near_anchor_edge;
        }
        // While resizing, items above the anchor are still growing: following it would fall
        // short of the edge.
        let effective_anchor = if is_first_anchored_layout
            || (was_settling_last_pass && was_near_anchor_edge_last_pass)
        {
            None
        } else {
            anchor
        };
        resolve_scroll_adjustment(
            anchor_info.edge,
            axis,
            effective_anchor,
            was_near_anchor_edge,
            is_scrolling,
            item_size_changed,
            content_size_delta,
            layout_info,
            &previous_visible_rect,
            content_size,
        )
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::hooks::collections::NodeKind;

    fn item(key: &str, rect: Rect) -> LayoutInfo {
        LayoutInfo::new(NodeKind::Item, Key::from(key), rect)
    }

    fn pairs(infos: &[LayoutInfo]) -> Vec<(&Key, &LayoutInfo)> {
        infos.iter().map(|info| (&info.key, info)).collect()
    }

    fn anchor(offset: f64) -> ScrollAnchor {
        ScrollAnchor {
            key: Key::from("item"),
            corner: RectCorner::TopLeft,
            offset,
        }
    }

    use ScrollAnchorAxis::{X, Y};
    use ScrollAnchorEdge::{End, Start};

    #[test]
    fn capture_skips_slivers() {
        let visible = Rect::new(0.0, 1023.0, 400.0, 468.0);
        let infos = [
            item("substantially-visible", Rect::new(0.0, 1032.0, 400.0, 40.0)),
            item("sliver", Rect::new(0.0, 976.0, 400.0, 48.0)),
        ];
        let anchor = capture_scroll_anchor(End, Y, &visible, pairs(&infos), None);
        assert_that!(anchor.map(|anchor| anchor.key))
            .is_equal_to(Some(Key::from("substantially-visible")));

        let only = [item("only-candidate", Rect::new(0.0, 976.0, 400.0, 48.0))];
        assert_that!(capture_scroll_anchor(End, Y, &visible, pairs(&only), None)).is_none();

        let infos = [
            item("farther", Rect::new(0.0, 1080.0, 400.0, 48.0)),
            item("closer", Rect::new(0.0, 1032.0, 400.0, 40.0)),
        ];
        let anchor = capture_scroll_anchor(End, Y, &visible, pairs(&infos), None);
        assert_that!(anchor.map(|anchor| anchor.key)).is_equal_to(Some(Key::from("closer")));
    }

    #[test]
    fn compute_target() {
        let visible = Rect::new(0.0, 100.0, 400.0, 468.0);
        let content = Size::new(400.0, 2000.0);
        assert_that!(compute_scroll_anchor_target(
            &anchor(10.0),
            Y,
            |_| None,
            &visible,
            content
        ))
        .is_none();
        let unmoved = item("item", Rect::new(0.0, 110.0, 400.0, 40.0));
        assert_that!(compute_scroll_anchor_target(
            &anchor(10.0),
            Y,
            |_| Some(unmoved.clone()),
            &visible,
            content
        ))
        .is_none();
        // Content prepended above pushed the anchor down by 200px.
        let moved = item("item", Rect::new(0.0, 310.0, 400.0, 40.0));
        assert_that!(compute_scroll_anchor_target(
            &anchor(10.0),
            Y,
            |_| Some(moved.clone()),
            &visible,
            content
        ))
        .is_equal_to(Some(300.0));
        let up = item("item", Rect::new(0.0, 0.0, 400.0, 40.0));
        assert_that!(compute_scroll_anchor_target(
            &anchor(100.0),
            Y,
            |_| Some(up.clone()),
            &visible,
            content
        ))
        .is_equal_to(Some(0.0));
        let far = item("item", Rect::new(0.0, 5000.0, 400.0, 40.0));
        assert_that!(compute_scroll_anchor_target(
            &anchor(10.0),
            Y,
            |_| Some(far.clone()),
            &visible,
            Size::new(400.0, 600.0)
        ))
        .is_equal_to(Some(600.0 - 468.0));
        let horizontal = item("item", Rect::new(310.0, 0.0, 40.0, 400.0));
        assert_that!(compute_scroll_anchor_target(
            &anchor(10.0),
            X,
            |_| Some(horizontal.clone()),
            &Rect::new(100.0, 0.0, 468.0, 400.0),
            Size::new(2000.0, 400.0)
        ))
        .is_equal_to(Some(300.0));
    }

    #[test]
    fn edge_snap_and_near_edge() {
        let content = Size::new(400.0, 2000.0);
        let visible = Rect::new(0.0, 500.0, 400.0, 468.0);
        assert_that!(edge_snap_target(Start, Y, content, &visible)).is_equal_to(0.0);
        assert_that!(edge_snap_target(End, Y, content, &visible)).is_equal_to(2000.0 - 468.0);
        assert_that!(edge_snap_target(
            End,
            Y,
            Size::new(400.0, 200.0),
            &Rect::new(0.0, 0.0, 400.0, 468.0)
        ))
        .is_equal_to(0.0);

        assert_that!(is_near_edge(
            &Rect::new(0.0, 10.0, 400.0, 468.0),
            content,
            Start,
            Y,
            10.0
        ))
        .is_true();
        assert_that!(is_near_edge(
            &Rect::new(0.0, 11.0, 400.0, 468.0),
            content,
            Start,
            Y,
            10.0
        ))
        .is_false();
        assert_that!(is_near_edge(
            &Rect::new(0.0, 1532.0, 400.0, 468.0),
            content,
            End,
            Y,
            10.0
        ))
        .is_true();
        assert_that!(is_near_edge(
            &Rect::new(0.0, 1500.0, 400.0, 468.0),
            content,
            End,
            Y,
            10.0
        ))
        .is_false();
    }

    #[test]
    fn resolve_adjustment() {
        let visible = Rect::new(0.0, 500.0, 400.0, 468.0);
        let content = Size::new(400.0, 2000.0);
        let moved = item("item", Rect::new(0.0, 610.0, 400.0, 40.0));
        let result = resolve_scroll_adjustment(
            End,
            Y,
            Some(&anchor(10.0)),
            false,
            false,
            false,
            0.0,
            |_| Some(moved.clone()),
            &visible,
            content,
        );
        assert_that!(result.map(|rect| rect.y)).is_equal_to(Some(600.0));
        let snap = |near: bool, scrolling: bool, resized: bool, delta: f64, rect: Rect| {
            resolve_scroll_adjustment(
                End,
                Y,
                None,
                near,
                scrolling,
                resized,
                delta,
                |_| None,
                &rect,
                content,
            )
            .map(|rect| rect.y)
        };
        assert_that!(snap(true, false, false, 0.0, visible)).is_equal_to(Some(2000.0 - 468.0));
        assert_that!(snap(true, false, true, 50.0, visible)).is_equal_to(Some(2000.0 - 468.0));
        assert_that!(snap(
            true,
            false,
            false,
            0.0,
            Rect::new(0.0, 2000.0 - 468.0, 400.0, 468.0)
        ))
        .is_none();
        assert_that!(snap(false, false, false, 0.0, visible)).is_none();
        assert_that!(snap(true, true, false, 0.0, visible)).is_none();
        assert_that!(snap(true, false, true, 0.0, visible)).is_none();
    }

    fn info() -> ScrollAnchorInfo {
        ScrollAnchorInfo {
            edge: End,
            axis: Y,
            threshold: 50.0,
            is_anchorable: None,
        }
    }

    fn resolve(
        tracker: &mut ScrollAnchorTracker,
        anchor_info: Option<&ScrollAnchorInfo>,
        post: &[LayoutInfo],
        previous: Rect,
        previous_content: Size,
        content: Size,
        resized: bool,
    ) -> Option<f64> {
        tracker
            .resolve_after_layout(ResolveAfterLayout {
                anchor_info,
                anchor: None,
                post_layout_infos: post,
                previous_visible_rect: previous,
                previous_content_size: previous_content,
                content_size: content,
                item_size_changed: resized,
                is_scrolling: false,
                layout_info: |_| None,
            })
            .map(|rect| rect.y)
    }

    #[test]
    fn tracker() {
        let info = info();
        let content = Size::new(400.0, 2000.0);
        let mut tracker = ScrollAnchorTracker::default();
        assert_that!(tracker.capture_before_layout(None, [], &Rect::new(0.0, 500.0, 400.0, 468.0)))
            .is_none();
        let candidate = [item("item", Rect::new(0.0, 1032.0, 400.0, 40.0))];
        let captured = tracker.capture_before_layout(
            Some(&info),
            pairs(&candidate),
            &Rect::new(0.0, 1023.0, 400.0, 468.0),
        );
        assert_that!(captured.map(|anchor| anchor.key)).is_equal_to(Some(Key::from("item")));

        let visible = Rect::new(0.0, 500.0, 400.0, 468.0);
        assert_that!(resolve(
            &mut tracker,
            None,
            &[],
            visible,
            content,
            content,
            false
        ))
        .is_none();
        assert_that!(resolve(
            &mut tracker,
            Some(&info),
            &[],
            Rect::default(),
            content,
            content,
            false
        ))
        .is_none();

        // The first anchored layout always snaps, even far from the edge.
        let mut first = ScrollAnchorTracker::default();
        assert_that!(resolve(
            &mut first,
            Some(&info),
            &[],
            Rect::new(0.0, 0.0, 400.0, 468.0),
            content,
            content,
            false
        ))
        .is_equal_to(Some(2000.0 - 468.0));

        // Later passes skip when nothing relevant changed, recompute when the content grew.
        let at_edge = Rect::new(0.0, 2000.0 - 468.0, 400.0, 468.0);
        let mut later = ScrollAnchorTracker::default();
        resolve(
            &mut later,
            Some(&info),
            &[],
            at_edge,
            content,
            content,
            false,
        );
        assert_that!(resolve(
            &mut later,
            Some(&info),
            &[],
            at_edge,
            content,
            content,
            false
        ))
        .is_none();
        let grown = Size::new(400.0, 2200.0);
        assert_that!(resolve(
            &mut later,
            Some(&info),
            &[],
            at_edge,
            content,
            grown,
            false
        ))
        .is_equal_to(Some(2200.0 - 468.0));

        // `reset` makes the next pass a first one again.
        later.reset();
        assert_that!(resolve(
            &mut later,
            Some(&info),
            &[],
            Rect::new(0.0, 0.0, 400.0, 468.0),
            content,
            content,
            false
        ))
        .is_equal_to(Some(2000.0 - 468.0));
    }

    #[test]
    fn tracker_reuses_near_edge_while_settling() {
        let info = info();
        let content = Size::new(400.0, 2000.0);
        let near = Rect::new(0.0, 2000.0 - 468.0, 400.0, 468.0);
        let far = Rect::new(0.0, 0.0, 400.0, 468.0);
        let mut tracker = ScrollAnchorTracker::default();
        resolve(
            &mut tracker,
            Some(&info),
            &[],
            near,
            content,
            content,
            false,
        );

        let mut estimated = item("item", Rect::new(0.0, 0.0, 400.0, 40.0));
        estimated.estimated_size = true;
        let mid = Size::new(400.0, 2100.0);
        assert_that!(resolve(
            &mut tracker,
            Some(&info),
            &[estimated],
            near,
            content,
            mid,
            true
        ))
        .is_equal_to(Some(2100.0 - 468.0));

        let settled = item("item", Rect::new(0.0, 0.0, 400.0, 40.0));
        let end = Size::new(400.0, 2200.0);
        assert_that!(resolve(
            &mut tracker,
            Some(&info),
            &[settled],
            far,
            mid,
            end,
            true
        ))
        .is_equal_to(Some(2200.0 - 468.0));
    }
}
