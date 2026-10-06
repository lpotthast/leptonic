// Upstream: react-aria/src/overlays/calculatePosition.ts @ 99e6102368
//! Where to place an overlay relative to its target: the pure computation
//! ([`calculate_position_internal`]) and the DOM measurements feeding it ([`calculate_position`]).
// Positioning runs in the browser only; on the server the computation is unused.
#![cfg_attr(feature = "ssr", allow(dead_code))]

use crate::utils::{locale::WritingDirection, point::Point};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `Placement` is an enum of the 22 placements instead of strings like `'bottom start'`;
//   flipped/resolved sides are `PlacementAxis`.
// - Sides and axes are enums (`PlacementAxis`, `Axis`) instead of string-keyed object lookups.
// - `max_height` is an `Option`: `None` is unset, `Some(0.0)` a max height of 0 (upstream ignores a
//   falsy `maxHeight`).
//
// ## OMITTED FEATURES
// - The unused `scrollSize` input of `calculatePositionInternal`.
//
// =============================================================================

/// Where an overlay is placed relative to its target: the side of the target it opens on, then
/// how it aligns along that side. `Start`/`End` follow the writing direction (start is left in
/// left-to-right text).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Placement {
    /// Below the target, centered.
    #[default]
    Bottom,
    /// Below the target, left edges aligned.
    BottomLeft,
    /// Below the target, right edges aligned.
    BottomRight,
    /// Below the target, start edges aligned.
    BottomStart,
    /// Below the target, end edges aligned.
    BottomEnd,
    /// Above the target, centered.
    Top,
    /// Above the target, left edges aligned.
    TopLeft,
    /// Above the target, right edges aligned.
    TopRight,
    /// Above the target, start edges aligned.
    TopStart,
    /// Above the target, end edges aligned.
    TopEnd,
    /// Left of the target, vertically centered.
    Left,
    /// Left of the target, top edges aligned.
    LeftTop,
    /// Left of the target, bottom edges aligned.
    LeftBottom,
    /// Before the target (left in left-to-right text), vertically centered.
    Start,
    /// Before the target, top edges aligned.
    StartTop,
    /// Before the target, bottom edges aligned.
    StartBottom,
    /// Right of the target, vertically centered.
    Right,
    /// Right of the target, top edges aligned.
    RightTop,
    /// Right of the target, bottom edges aligned.
    RightBottom,
    /// After the target (right in left-to-right text), vertically centered.
    End,
    /// After the target, top edges aligned.
    EndTop,
    /// After the target, bottom edges aligned.
    EndBottom,
}

/// The side of the target an overlay was placed on, after flipping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlacementAxis {
    Top,
    Bottom,
    Left,
    Right,
}

impl PlacementAxis {
    /// `"top"`, `"bottom"`, `"left"` or `"right"`, e.g. for a `data-placement` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    fn axis(self) -> Axis {
        match self {
            Self::Top | Self::Bottom => Axis::Top,
            Self::Left | Self::Right => Axis::Left,
        }
    }

    fn flipped(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    /// Whether this side is the start of its axis (top or left).
    fn is_axis_start(self) -> bool {
        matches!(self, Self::Top | Self::Left)
    }
}

/// A position coordinate: the vertical one (`top`, sizes are heights) or the horizontal one
/// (`left`, sizes are widths).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Axis {
    Top,
    Left,
}

impl Axis {
    fn cross(self) -> Self {
        match self {
            Self::Top => Self::Left,
            Self::Left => Self::Top,
        }
    }
}

/// How the overlay aligns along the target's side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CrossPlacement {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

impl CrossPlacement {
    /// Whether this aligns the overlay's start (top or left) with the target's on `axis`.
    fn is_start_of(self, axis: Axis) -> bool {
        matches!(
            (self, axis),
            (Self::Top, Axis::Top) | (Self::Left, Axis::Left)
        )
    }
}

/// A placement with its sides resolved against the writing direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ParsedPlacement {
    pub placement: PlacementAxis,
    pub cross_placement: CrossPlacement,
    pub axis: Axis,
    pub cross_axis: Axis,
}

impl ParsedPlacement {
    fn new(placement: PlacementAxis, cross_placement: CrossPlacement) -> Self {
        let axis = placement.axis();
        Self {
            placement,
            cross_placement,
            axis,
            cross_axis: axis.cross(),
        }
    }

    fn flipped(self) -> Self {
        Self::new(self.placement.flipped(), self.cross_placement)
    }
}

impl Placement {
    /// The side and alignment, with start and end resolved for `direction` (react-aria:
    /// `translateRTL` + `parsePlacement`).
    pub(crate) fn parse(self, direction: WritingDirection) -> ParsedPlacement {
        use CrossPlacement as C;
        use PlacementAxis as P;
        let rtl = direction == WritingDirection::Rtl;
        let (start_side, end_side) = if rtl {
            (P::Right, P::Left)
        } else {
            (P::Left, P::Right)
        };
        let (start_cross, end_cross) = if rtl {
            (C::Right, C::Left)
        } else {
            (C::Left, C::Right)
        };
        let (placement, cross) = match self {
            Self::Bottom => (P::Bottom, C::Center),
            Self::BottomLeft => (P::Bottom, C::Left),
            Self::BottomRight => (P::Bottom, C::Right),
            Self::BottomStart => (P::Bottom, start_cross),
            Self::BottomEnd => (P::Bottom, end_cross),
            Self::Top => (P::Top, C::Center),
            Self::TopLeft => (P::Top, C::Left),
            Self::TopRight => (P::Top, C::Right),
            Self::TopStart => (P::Top, start_cross),
            Self::TopEnd => (P::Top, end_cross),
            Self::Left => (P::Left, C::Center),
            Self::LeftTop => (P::Left, C::Top),
            Self::LeftBottom => (P::Left, C::Bottom),
            Self::Start => (start_side, C::Center),
            Self::StartTop => (start_side, C::Top),
            Self::StartBottom => (start_side, C::Bottom),
            Self::Right => (P::Right, C::Center),
            Self::RightTop => (P::Right, C::Top),
            Self::RightBottom => (P::Right, C::Bottom),
            Self::End => (end_side, C::Center),
            Self::EndTop => (end_side, C::Top),
            Self::EndBottom => (end_side, C::Bottom),
        };
        ParsedPlacement::new(placement, cross)
    }
}

/// A rectangle: its top-left corner and size.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub top: f64,
    pub left: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    fn at(&self, axis: Axis) -> f64 {
        match axis {
            Axis::Top => self.top,
            Axis::Left => self.left,
        }
    }

    fn size(&self, axis: Axis) -> f64 {
        match axis {
            Axis::Top => self.height,
            Axis::Left => self.width,
        }
    }
}

/// The size, origin and scroll position of a boundary or container element.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct Dimensions {
    pub width: f64,
    pub height: f64,
    pub total_width: f64,
    pub total_height: f64,
    pub top: f64,
    pub left: f64,
    pub scroll_top: f64,
    pub scroll_left: f64,
}

impl Dimensions {
    fn at(&self, axis: Axis) -> f64 {
        match axis {
            Axis::Top => self.top,
            Axis::Left => self.left,
        }
    }

    fn size(&self, axis: Axis) -> f64 {
        match axis {
            Axis::Top => self.height,
            Axis::Left => self.width,
        }
    }

    fn total_size(&self, axis: Axis) -> f64 {
        match axis {
            Axis::Top => self.total_height,
            Axis::Left => self.total_width,
        }
    }

    fn scroll(&self, axis: Axis) -> f64 {
        match axis {
            Axis::Top => self.scroll_top,
            Axis::Left => self.scroll_left,
        }
    }
}

/// The overlay's margins.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct Margins {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

impl Margins {
    /// The margin at the start of `axis` (top or left).
    fn start(&self, axis: Axis) -> f64 {
        match axis {
            Axis::Top => self.top,
            Axis::Left => self.left,
        }
    }

    /// The margins at both ends of `axis`.
    fn both(&self, axis: Axis) -> f64 {
        match axis {
            Axis::Top => self.top + self.bottom,
            Axis::Left => self.left + self.right,
        }
    }
}

/// The overlay's CSS position: two of `top`, `left`, `bottom`, `right`, in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Position {
    pub top: Option<f64>,
    pub left: Option<f64>,
    pub bottom: Option<f64>,
    pub right: Option<f64>,
}

impl Position {
    fn at(&self, axis: Axis) -> f64 {
        match axis {
            Axis::Top => self.top,
            Axis::Left => self.left,
        }
        .unwrap_or(0.0)
    }

    fn set_at(&mut self, axis: Axis, value: f64) {
        match axis {
            Axis::Top => self.top = Some(value),
            Axis::Left => self.left = Some(value),
        }
    }

    fn set_side(&mut self, side: PlacementAxis, value: f64) {
        match side {
            PlacementAxis::Top => self.top = Some(value),
            PlacementAxis::Bottom => self.bottom = Some(value),
            PlacementAxis::Left => self.left = Some(value),
            PlacementAxis::Right => self.right = Some(value),
        }
    }
}

/// The parts of the visual viewport the computation needs.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct VisualViewportMetrics {
    pub offset_top: f64,
    pub height: f64,
}

/// Where to put the overlay.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PositionResult {
    pub position: Position,
    /// The arrow's position along the overlay's left edge (for overlays above or below).
    pub arrow_offset_left: Option<f64>,
    /// The arrow's position along the overlay's top edge (for overlays left or right).
    pub arrow_offset_top: Option<f64>,
    /// The point of the overlay closest to the target, in the overlay's coordinates (for
    /// animations growing out of the target).
    pub trigger_anchor_point: Point,
    pub max_height: f64,
    /// The side the overlay was placed on, after flipping.
    pub placement: PlacementAxis,
}

/// The inputs of [`calculate_position_internal`], measured by [`calculate_position`].
#[derive(Debug, Clone, Copy)]
pub(crate) struct PositionInput {
    pub placement: ParsedPlacement,
    /// The target, in the container's coordinates.
    pub child_offset: Rect,
    /// The overlay's size (including its margins).
    pub overlay_size: Rect,
    pub margins: Margins,
    /// The minimum distance between the overlay and the boundary.
    pub padding: f64,
    pub flip: bool,
    pub boundary: Dimensions,
    pub container: Dimensions,
    /// The boundary's offset in the container's coordinates.
    pub container_offset_with_boundary: Rect,
    pub offset: f64,
    pub cross_offset: f64,
    pub is_container_positioned: bool,
    pub user_max_height: Option<f64>,
    pub arrow_size: f64,
    pub arrow_boundary_offset: f64,
    pub is_container_descendant_of_boundary: bool,
    pub visual_viewport: Option<VisualViewportMetrics>,
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

/// How far the overlay must move along `axis` to stay within the boundary.
fn get_delta(
    axis: Axis,
    offset: f64,
    size: f64,
    boundary: &Dimensions,
    container: &Dimensions,
    padding: f64,
    container_offset_with_boundary: &Rect,
) -> f64 {
    let container_scroll = container.scroll(axis);
    let boundary_size = boundary.size(axis);
    let boundary_start_edge =
        container_offset_with_boundary.at(axis) + boundary.scroll(axis) + padding;
    let boundary_end_edge =
        container_offset_with_boundary.at(axis) + boundary.scroll(axis) + boundary_size - padding;
    let start_edge_offset =
        offset - container_scroll + boundary.scroll(axis) + container_offset_with_boundary.at(axis)
            - boundary.at(axis);
    let end_edge_offset = start_edge_offset + size;

    if start_edge_offset < boundary_start_edge {
        boundary_start_edge - start_edge_offset
    } else if end_edge_offset > boundary_end_edge {
        (boundary_end_edge - end_edge_offset).max(boundary_start_edge - start_edge_offset)
    } else {
        0.0
    }
}

#[allow(clippy::too_many_arguments)]
fn compute_position(
    child_offset: &Rect,
    overlay_size: &Rect,
    info: ParsedPlacement,
    offset: f64,
    cross_offset: f64,
    is_container_positioned: bool,
    arrow_size: f64,
    arrow_boundary_offset: f64,
    container: &Dimensions,
) -> Position {
    let ParsedPlacement {
        placement,
        cross_placement,
        axis,
        cross_axis,
    } = info;
    let mut position = Position::default();

    let mut cross = child_offset.at(cross_axis);
    if cross_placement == CrossPlacement::Center {
        // The overlay's center at the target's center.
        cross += (child_offset.size(cross_axis) - overlay_size.size(cross_axis)) / 2.0;
    } else if !cross_placement.is_start_of(cross_axis) {
        // The overlay's end at the target's end.
        cross += child_offset.size(cross_axis) - overlay_size.size(cross_axis);
    }
    cross += cross_offset;

    // The overlay always overlaps the target on the cross axis (by at least the arrow).
    let min_position = child_offset.at(cross_axis) - overlay_size.size(cross_axis)
        + arrow_size
        + arrow_boundary_offset;
    let max_position = child_offset.at(cross_axis) + child_offset.size(cross_axis)
        - arrow_size
        - arrow_boundary_offset;
    position.set_at(cross_axis, clamp(cross, min_position, max_position));

    // Whole pixels. A top or left overlay is anchored with `bottom`/`right`, so it stays at the
    // target when its size changes.
    if placement.is_axis_start() {
        let container_size = if is_container_positioned {
            container.size(axis)
        } else {
            container.total_size(axis)
        };
        position.set_side(
            placement.flipped(),
            (container_size - child_offset.at(axis) + offset).floor(),
        );
    } else {
        position.set_at(
            axis,
            (child_offset.at(axis) + child_offset.size(axis) + offset).floor(),
        );
    }
    position
}

/// Which way the overlay's height grows when its content grows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeightGrowthDirection {
    Top,
    Bottom,
}

#[allow(clippy::too_many_arguments)]
fn get_max_height(
    position: &Position,
    boundary: &Dimensions,
    container_offset_with_boundary: &Rect,
    margins: &Margins,
    padding: f64,
    overlay_height: f64,
    height_growth_direction: HeightGrowthDirection,
    container: &Dimensions,
    is_container_descendant_of_boundary: bool,
    visual_viewport: Option<VisualViewportMetrics>,
) -> f64 {
    // The overlay's true top in the container (it may be positioned by `bottom`).
    let overlay_top = position.top.unwrap_or_else(|| {
        container.total_height - position.bottom.unwrap_or(0.0) - overlay_height
    }) - container.scroll_top;
    let boundary_to_container_offset = if is_container_descendant_of_boundary {
        container_offset_with_boundary.top
    } else {
        0.0
    };
    // The most restrictive of the boundary and the visual viewport.
    let bounding_top = (boundary.top + boundary_to_container_offset).max(
        visual_viewport.map_or(boundary.top, |vv| vv.offset_top) + boundary_to_container_offset,
    );
    let bounding_bottom = (boundary.top + boundary.height + boundary_to_container_offset).min(
        visual_viewport.map_or(0.0, |vv| vv.offset_top)
            + visual_viewport.map_or(0.0, |vv| vv.height),
    );
    let reserved = margins.top + margins.bottom + padding;
    match height_growth_direction {
        // From the overlay's top to the bottom of the boundary.
        HeightGrowthDirection::Bottom => (bounding_bottom - overlay_top - reserved).max(0.0),
        // From the overlay's bottom to the top of the boundary.
        HeightGrowthDirection::Top => {
            (overlay_top + overlay_height - bounding_top - reserved).max(0.0)
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn get_available_space(
    boundary: &Dimensions,
    container_offset_with_boundary: &Rect,
    child_offset: &Rect,
    margins: &Margins,
    padding: f64,
    info: ParsedPlacement,
    container: &Dimensions,
    is_container_descendant_of_boundary: bool,
) -> f64 {
    let ParsedPlacement {
        placement, axis, ..
    } = info;
    let boundary_offset = if is_container_descendant_of_boundary {
        container_offset_with_boundary.at(axis)
    } else {
        0.0
    };
    if placement.is_axis_start() {
        return (child_offset.at(axis)
            - container.scroll(axis)
            - (boundary.at(axis) + boundary_offset)
            - margins.both(axis)
            - padding)
            .max(0.0);
    }
    (boundary.size(axis) + boundary.at(axis) + boundary_offset
        - child_offset.at(axis)
        - child_offset.size(axis)
        + container.scroll(axis)
        - margins.both(axis)
        - padding)
        .max(0.0)
}

/// Where to place the overlay, from measured geometry (react-aria: `calculatePositionInternal`).
#[allow(clippy::too_many_lines)]
pub(crate) fn calculate_position_internal(input: &PositionInput) -> PositionResult {
    let PositionInput {
        placement: mut placement_info,
        child_offset,
        mut overlay_size,
        margins,
        padding,
        flip,
        boundary,
        container,
        container_offset_with_boundary,
        offset,
        cross_offset,
        is_container_positioned,
        user_max_height,
        arrow_size,
        arrow_boundary_offset,
        is_container_descendant_of_boundary,
        visual_viewport,
    } = *input;
    let compute = |info: ParsedPlacement, overlay_size: &Rect| {
        compute_position(
            &child_offset,
            overlay_size,
            info,
            offset,
            cross_offset,
            is_container_positioned,
            arrow_size,
            arrow_boundary_offset,
            &container,
        )
    };
    let available_space = |info: ParsedPlacement| {
        get_available_space(
            &boundary,
            &container_offset_with_boundary,
            &child_offset,
            &margins,
            padding + offset,
            info,
            &container,
            is_container_descendant_of_boundary,
        )
    };

    let mut position = compute(placement_info, &overlay_size);
    let space = available_space(placement_info);

    // Flip when the overlay doesn't fit and the other side has more room.
    if flip && overlay_size.size(placement_info.axis) > space {
        let flipped_info = placement_info.flipped();
        let flipped_position = compute(flipped_info, &overlay_size);
        let flipped_space = available_space(flipped_info);
        if flipped_space > space {
            placement_info = flipped_info;
            position = flipped_position;
        }
    }

    let height_growth_direction = if placement_info.axis == Axis::Top {
        if placement_info.placement == PlacementAxis::Top {
            HeightGrowthDirection::Top
        } else {
            HeightGrowthDirection::Bottom
        }
    } else if placement_info.cross_axis == Axis::Top
        && placement_info.cross_placement == CrossPlacement::Bottom
    {
        HeightGrowthDirection::Top
    } else {
        HeightGrowthDirection::Bottom
    };

    let cross_axis = placement_info.cross_axis;
    let delta = |position: &Position, overlay_size: &Rect| {
        get_delta(
            cross_axis,
            position.at(cross_axis),
            overlay_size.size(cross_axis),
            &boundary,
            &container,
            padding,
            &container_offset_with_boundary,
        )
    };
    let d = delta(&position, &overlay_size);
    position.set_at(cross_axis, position.at(cross_axis) + d);

    let mut max_height = get_max_height(
        &position,
        &boundary,
        &container_offset_with_boundary,
        &margins,
        padding,
        overlay_size.height,
        height_growth_direction,
        &container,
        is_container_descendant_of_boundary,
        visual_viewport,
    );
    // A user max height only restricts (and 0 means none, as react-aria's falsy check).
    if let Some(user_max_height) = user_max_height
        && user_max_height < max_height
    {
        max_height = user_max_height;
    }

    overlay_size.height = overlay_size.height.min(max_height);

    position = compute(placement_info, &overlay_size);
    let d = delta(&position, &overlay_size);
    position.set_at(cross_axis, position.at(cross_axis) + d);

    // The arrow: preferably at the target's center, within the target and the overlay. Values are
    // in the overlay's coordinates (after its margin).
    let margin_start = margins.start(cross_axis);
    let mut origin = child_offset.at(cross_axis) - position.at(cross_axis) - margin_start;
    let preferred_arrow_position = origin + 0.5 * child_offset.size(cross_axis);
    let arrow_min_position = arrow_size / 2.0 + arrow_boundary_offset;
    let arrow_max_position = overlay_size.size(cross_axis)
        - margins.both(cross_axis)
        - arrow_size / 2.0
        - arrow_boundary_offset;
    let arrow_overlapping_child_min_edge =
        child_offset.at(cross_axis) + arrow_size / 2.0 - (position.at(cross_axis) + margin_start);
    let arrow_overlapping_child_max_edge = child_offset.at(cross_axis)
        + child_offset.size(cross_axis)
        - arrow_size / 2.0
        - (position.at(cross_axis) + margin_start);
    let arrow_position_overlapping_child = clamp(
        preferred_arrow_position,
        arrow_overlapping_child_min_edge,
        arrow_overlapping_child_max_edge,
    );
    let arrow_position = clamp(
        arrow_position_overlapping_child,
        arrow_min_position,
        arrow_max_position,
    );

    // The animation origin: the arrow if there is one, else the target's edge.
    let ParsedPlacement {
        placement,
        cross_placement,
        axis,
        ..
    } = placement_info;
    if arrow_size != 0.0 {
        origin = arrow_position;
    } else if cross_placement == CrossPlacement::Right {
        origin += child_offset.size(cross_axis);
    } else if cross_placement == CrossPlacement::Center {
        origin += child_offset.size(cross_axis) / 2.0;
    }
    let cross_origin = if placement.is_axis_start() {
        overlay_size.size(axis)
    } else {
        0.0
    };
    let trigger_anchor_point = match axis {
        Axis::Top => Point::new(origin, cross_origin),
        Axis::Left => Point::new(cross_origin, origin),
    };

    PositionResult {
        position,
        arrow_offset_left: (cross_axis == Axis::Left).then_some(arrow_position),
        arrow_offset_top: (cross_axis == Axis::Top).then_some(arrow_position),
        trigger_anchor_point,
        max_height,
        placement,
    }
}

/// Measures the target, the overlay and its surroundings in the DOM (react-aria:
/// `calculatePosition`).
#[cfg(not(feature = "ssr"))]
pub(crate) mod dom {
    use wasm_bindgen::JsCast;

    use super::{
        Dimensions, Margins, ParsedPlacement, PositionInput, PositionResult, Rect,
        VisualViewportMetrics, calculate_position_internal,
    };

    /// The inputs of [`calculate_position`](self::calculate_position).
    pub(crate) struct PositionOptions<'a> {
        pub placement: ParsedPlacement,
        pub target: &'a web_sys::Element,
        pub overlay: &'a web_sys::Element,
        pub boundary: &'a web_sys::Element,
        pub padding: f64,
        pub should_flip: bool,
        pub offset: f64,
        pub cross_offset: f64,
        pub max_height: Option<f64>,
        pub arrow_size: f64,
        pub arrow_boundary_offset: f64,
        /// Replaces the target's bounding rectangle (e.g. a point for context menus).
        pub target_rect: Option<Rect>,
    }

    fn window() -> Option<web_sys::Window> {
        leptos_use::use_window().as_ref().cloned()
    }

    fn document_element() -> Option<web_sys::Element> {
        leptos_use::use_document()
            .as_ref()
            .and_then(web_sys::Document::document_element)
    }

    fn computed_style(element: &web_sys::Element) -> Option<web_sys::CssStyleDeclaration> {
        window()?.get_computed_style(element).ok().flatten()
    }

    fn style_value(style: Option<&web_sys::CssStyleDeclaration>, property: &str) -> String {
        style
            .and_then(|style| style.get_property_value(property).ok())
            .unwrap_or_default()
    }

    /// `parseInt(value, 10) || 0`: the leading integer of a CSS length like `"12.5px"`.
    fn parse_int(value: &str) -> f64 {
        let value = value.trim();
        let end = value
            .char_indices()
            .find(|&(i, c)| !(c.is_ascii_digit() || (i == 0 && (c == '-' || c == '+'))))
            .map_or(value.len(), |(i, _)| i);
        value[..end].parse::<i32>().map_or(0.0, f64::from)
    }

    fn is_body_or_html(element: &web_sys::Element) -> bool {
        matches!(element.tag_name().as_str(), "BODY" | "HTML")
    }

    fn visual_viewport_metrics(
        visual_viewport: Option<&web_sys::VisualViewport>,
    ) -> Option<VisualViewportMetrics> {
        visual_viewport.map(|vv| VisualViewportMetrics {
            offset_top: vv.offset_top(),
            height: vv.height(),
        })
    }

    fn get_container_dimensions(
        container: &web_sys::Element,
        visual_viewport: Option<&web_sys::VisualViewport>,
    ) -> Dimensions {
        let mut dimensions = Dimensions::default();
        let is_pinch_zoomed_in = visual_viewport.is_some_and(|vv| vv.scale() > 1.0);

        // The viewport (the initial containing block) for `html`/`body`: the visual viewport.
        if is_body_or_html(container) {
            let root = document_element();
            let total_width = root.as_ref().map_or(0.0, |r| f64::from(r.client_width()));
            let total_height = root.as_ref().map_or(0.0, |r| f64::from(r.client_height()));
            dimensions.total_width = total_width;
            dimensions.total_height = total_height;
            dimensions.width = visual_viewport.map_or(total_width, web_sys::VisualViewport::width);
            dimensions.height =
                visual_viewport.map_or(total_height, web_sys::VisualViewport::height);
            let root_scroll_top = root.as_ref().map_or(0.0, web_sys::Element::scroll_top);
            let root_scroll_left = root.as_ref().map_or(0.0, web_sys::Element::scroll_left);
            dimensions.scroll_top = if root_scroll_top == 0.0 {
                container.scroll_top()
            } else {
                root_scroll_top
            };
            dimensions.scroll_left = if root_scroll_left == 0.0 {
                container.scroll_left()
            } else {
                root_scroll_left
            };
            // The visual viewport's top-left relative to the layout viewport (page offsets: WebKit
            // misreports `offsetTop`/`offsetLeft` during pans).
            if let Some(vv) = visual_viewport {
                dimensions.top = (vv.page_top() - dimensions.scroll_top).max(0.0);
                dimensions.left = (vv.page_left() - dimensions.scroll_left).max(0.0);
            }
        } else {
            let offset = get_offset(container, false, None);
            dimensions.width = offset.width;
            dimensions.height = offset.height;
            dimensions.top = offset.top;
            dimensions.left = offset.left;
            dimensions.scroll_top = container.scroll_top();
            dimensions.scroll_left = container.scroll_left();
            dimensions.total_width = offset.width;
            dimensions.total_height = offset.height;
        }

        // Safari reports a scroll offset for the non-scrolling body/html while pinch zoomed in.
        if crate::utils::platform::browser::is_webkit()
            && is_body_or_html(container)
            && is_pinch_zoomed_in
        {
            dimensions.scroll_top = 0.0;
            dimensions.scroll_left = 0.0;
            dimensions.top = visual_viewport.map_or(0.0, web_sys::VisualViewport::page_top);
            dimensions.left = visual_viewport.map_or(0.0, web_sys::VisualViewport::page_left);
        }

        dimensions
    }

    fn get_margins(element: &web_sys::Element) -> Margins {
        let style = computed_style(element);
        Margins {
            top: parse_int(&style_value(style.as_ref(), "margin-top")),
            bottom: parse_int(&style_value(style.as_ref(), "margin-bottom")),
            left: parse_int(&style_value(style.as_ref(), "margin-left")),
            right: parse_int(&style_value(style.as_ref(), "margin-right")),
        }
    }

    /// The element's bounding rectangle; with `ignore_scale`, its layout size (unaffected by scale
    /// transforms).
    pub(crate) fn get_rect(element: &web_sys::Element, ignore_scale: bool) -> Rect {
        let rect = element.get_bounding_client_rect();
        let mut offset = Rect {
            top: rect.top(),
            left: rect.left(),
            width: rect.width(),
            height: rect.height(),
        };
        if ignore_scale && let Some(html) = element.dyn_ref::<web_sys::HtmlElement>() {
            offset.width = f64::from(html.offset_width());
            offset.height = f64::from(html.offset_height());
        }
        offset
    }

    /// The element's rectangle in document coordinates.
    fn get_offset(
        element: &web_sys::Element,
        ignore_scale: bool,
        override_rect: Option<Rect>,
    ) -> Rect {
        let rect = override_rect.unwrap_or_else(|| get_rect(element, ignore_scale));
        let root = document_element();
        let (scroll_top, scroll_left, client_top, client_left) =
            root.as_ref().map_or((0.0, 0.0, 0.0, 0.0), |root| {
                (
                    root.scroll_top(),
                    root.scroll_left(),
                    f64::from(root.client_top()),
                    f64::from(root.client_left()),
                )
            });
        Rect {
            top: rect.top + scroll_top - client_top,
            left: rect.left + scroll_left - client_left,
            width: rect.width,
            height: rect.height,
        }
    }

    /// The element's rectangle relative to `parent`'s padding box.
    fn get_position(
        element: &web_sys::Element,
        parent: &web_sys::Element,
        ignore_scale: bool,
        override_rect: Option<Rect>,
    ) -> Rect {
        let style = computed_style(element);
        let mut offset = if style_value(style.as_ref(), "position") == "fixed" {
            override_rect.unwrap_or_else(|| get_rect(element, ignore_scale))
        } else {
            let mut offset = get_offset(element, ignore_scale, override_rect);
            let mut parent_offset = get_offset(parent, ignore_scale, None);
            let parent_style = computed_style(parent);
            parent_offset.top += parse_int(&style_value(parent_style.as_ref(), "border-top-width"))
                - parent.scroll_top();
            parent_offset.left +=
                parse_int(&style_value(parent_style.as_ref(), "border-left-width"))
                    - parent.scroll_left();
            offset.top -= parent_offset.top;
            offset.left -= parent_offset.left;
            offset
        };
        offset.top -= parse_int(&style_value(style.as_ref(), "margin-top"));
        offset.left -= parse_int(&style_value(style.as_ref(), "margin-left"));
        offset
    }

    /// Whether the element is the containing block of positioned descendants.
    fn is_containing_block(element: &web_sys::Element) -> bool {
        let style = computed_style(element);
        let style = style.as_ref();
        let none_or_empty = |property: &str| {
            let value = style_value(style, property);
            value.is_empty() || value == "none"
        };
        let will_change = style_value(style, "will-change");
        !none_or_empty("transform")
            || will_change.contains("transform")
            || will_change.contains("perspective")
            || !none_or_empty("filter")
            || style_value(style, "contain") == "paint"
            || !none_or_empty("backdrop-filter")
            || !none_or_empty("-webkit-backdrop-filter")
    }

    /// The element the overlay is positioned relative to.
    fn get_containing_block(overlay: &web_sys::HtmlElement) -> Option<web_sys::Element> {
        let root = document_element();
        let body: Option<web_sys::Element> = leptos_use::use_document()
            .as_ref()
            .and_then(web_sys::Document::body)
            .map(Into::into);
        let mut offset_parent = overlay.offset_parent();

        // `offsetParent` stops at the body, even when the body is no containing block.
        if let Some(parent) = &offset_parent
            && body.as_ref() == Some(parent)
            && style_value(computed_style(parent).as_ref(), "position") == "static"
            && !is_containing_block(parent)
        {
            offset_parent.clone_from(&root);
        }

        // No offset parent (e.g. `position: fixed`): walk up to the nearest containing block.
        if offset_parent.is_none() {
            offset_parent = overlay.parent_element();
            while let Some(parent) = &offset_parent {
                if is_containing_block(parent) {
                    break;
                }
                offset_parent = parent.parent_element();
            }
        }

        offset_parent.or(root)
    }

    /// Where to place the overlay.
    pub(crate) fn calculate_position(options: &PositionOptions<'_>) -> Option<PositionResult> {
        let window = window()?;
        let root = document_element()?;
        let visual_viewport = window.visual_viewport();
        let container = options
            .overlay
            .dyn_ref::<web_sys::HtmlElement>()
            .and_then(get_containing_block)
            .unwrap_or_else(|| root.clone());
        let is_viewport_container = container == root;
        let container_position = style_value(computed_style(&container).as_ref(), "position");
        let is_container_positioned =
            !container_position.is_empty() && container_position != "static";
        let mut child_offset = if is_viewport_container {
            get_offset(options.target, false, options.target_rect)
        } else {
            get_position(options.target, &container, false, options.target_rect)
        };
        if !is_viewport_container {
            let target_style = computed_style(options.target);
            child_offset.top += parse_int(&style_value(target_style.as_ref(), "margin-top"));
            child_offset.left += parse_int(&style_value(target_style.as_ref(), "margin-left"));
        }

        let mut overlay_size = get_offset(options.overlay, true, None);
        let margins = get_margins(options.overlay);
        overlay_size.width += margins.left + margins.right;
        overlay_size.height += margins.top + margins.bottom;

        let boundary = get_container_dimensions(options.boundary, visual_viewport.as_ref());
        let container_dimensions = get_container_dimensions(&container, visual_viewport.as_ref());

        // The boundary's coordinates in the container's coordinate system.
        let container_offset_with_boundary =
            match (is_body_or_html(options.boundary), is_viewport_container) {
                (true, false) => {
                    // `boundary` is in viewport coordinates here.
                    let container_rect = get_rect(&container, false);
                    Rect {
                        top: -(container_rect.top - boundary.top),
                        left: -(container_rect.left - boundary.left),
                        width: 0.0,
                        height: 0.0,
                    }
                }
                (true, true) => Rect::default(),
                (false, _) => get_position(options.boundary, &container, false, None),
            };

        let is_container_descendant_of_boundary =
            crate::utils::shadow_dom::node_contains(options.boundary, &container);
        Some(calculate_position_internal(&PositionInput {
            placement: options.placement,
            child_offset,
            overlay_size,
            margins,
            padding: options.padding,
            flip: options.should_flip,
            boundary,
            container: container_dimensions,
            container_offset_with_boundary,
            offset: options.offset,
            cross_offset: options.cross_offset,
            is_container_positioned,
            user_max_height: options.max_height,
            arrow_size: options.arrow_size,
            arrow_boundary_offset: options.arrow_boundary_offset,
            is_container_descendant_of_boundary,
            visual_viewport: visual_viewport_metrics(visual_viewport.as_ref()),
        }))
    }

    #[cfg(test)]
    mod tests {
        use assertr::prelude::*;

        use super::parse_int;

        #[test]
        fn parse_int_reads_the_leading_integer() {
            assert_that!(parse_int("12.5px")).is_equal_to(12.0);
            assert_that!(parse_int("-3px")).is_equal_to(-3.0);
            assert_that!(parse_int("auto")).is_equal_to(0.0);
            assert_that!(parse_int("")).is_equal_to(0.0);
        }
    }
}

// Upstream: react-aria/test/overlays/calculatePosition.test.ts @ 99e6102368
#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    /// The upstream test setup: a positioned 600x600 container at the origin, a separate
    /// boundary element of 600 x (600 - `provider_offset`), a 200x200 overlay without margins, a
    /// 600px high visual viewport and 50px padding.
    #[allow(clippy::too_many_arguments)]
    fn calculate(
        placement: Placement,
        target: Rect,
        offset: f64,
        cross_offset: f64,
        flip: bool,
        provider_offset: f64,
        arrow_size: f64,
        arrow_boundary_offset: f64,
    ) -> PositionResult {
        let container = Dimensions {
            width: 600.0,
            height: 600.0,
            total_width: 600.0,
            total_height: 600.0,
            ..Dimensions::default()
        };
        let boundary_height = 600.0 - provider_offset;
        let boundary = Dimensions {
            width: 600.0,
            height: boundary_height,
            total_width: 600.0,
            total_height: boundary_height,
            ..Dimensions::default()
        };
        calculate_position_internal(&PositionInput {
            placement: placement.parse(WritingDirection::Ltr),
            child_offset: target,
            overlay_size: Rect {
                top: 0.0,
                left: 0.0,
                width: 200.0,
                height: 200.0,
            },
            margins: Margins::default(),
            padding: 50.0,
            flip,
            boundary,
            container,
            container_offset_with_boundary: Rect::default(),
            offset,
            cross_offset,
            is_container_positioned: true,
            user_max_height: None,
            arrow_size,
            arrow_boundary_offset,
            is_container_descendant_of_boundary: false,
            visual_viewport: Some(VisualViewportMetrics {
                offset_top: 0.0,
                height: 600.0,
            }),
        })
    }

    fn target(left: f64, top: f64) -> Rect {
        Rect {
            top,
            left,
            width: 100.0,
            height: 100.0,
        }
    }

    /// An expected result in the upstream table's form: overlay `x`, `y` (top-left), arrow left,
    /// arrow top, max height.
    #[derive(Debug, Clone, Copy)]
    struct Expected(f64, f64, Option<f64>, Option<f64>, f64);

    /// Checks `result` against `expected` as the upstream test does (converting top/left positions
    /// into bottom/right ones for overlays above or left of the target).
    fn check(
        placement: Placement,
        result: &PositionResult,
        expected: Expected,
        flip: bool,
        provider_offset: f64,
        context: &str,
    ) {
        let Expected(x, y, arrow_left, arrow_top, max_height) = expected;
        let parsed = placement.parse(WritingDirection::Ltr);
        let placement_axis = parsed.placement;
        let cross = parsed.cross_placement;
        let mut position = Position::default();
        match (placement_axis, flip) {
            (PlacementAxis::Left, false) | (PlacementAxis::Right, true) => {
                position.right = Some(600.0 - (x + 200.0));
                position.top = Some(y);
            }
            (PlacementAxis::Top, _) => {
                position.left = Some(x);
                position.bottom = Some(600.0 - (y + 200.0));
            }
            (PlacementAxis::Right | PlacementAxis::Left | PlacementAxis::Bottom, _) => {
                position.left = Some(x);
                position.top = Some(y);
            }
        }
        let calculated_placement = if flip {
            placement_axis.flipped()
        } else {
            placement_axis
        };
        let max_height = max_height
            - if placement_axis != PlacementAxis::Top && cross != CrossPlacement::Bottom {
                provider_offset
            } else {
                0.0
            };
        let anchor = Point::new(
            arrow_left.unwrap_or(if calculated_placement == PlacementAxis::Left {
                200.0
            } else {
                0.0
            }),
            arrow_top.unwrap_or(if calculated_placement == PlacementAxis::Top {
                200.0_f64.min(max_height)
            } else {
                0.0
            }),
        );
        let expected = PositionResult {
            position,
            arrow_offset_left: arrow_left,
            arrow_offset_top: arrow_top,
            trigger_anchor_point: anchor,
            max_height,
            placement: calculated_placement,
        };
        assert_that!(*result)
            .with_detail_message(format!("{placement:?}: {context}"))
            .is_equal_to(expected);
    }

    struct Case {
        placement: Placement,
        no_offset: Expected,
        offset_before: Expected,
        offset_after: Expected,
        cross_axis_offset_positive: Expected,
        cross_axis_offset_negative: Expected,
        main_axis_offset: Expected,
        arrow_boundary_offset: Expected,
    }

    #[allow(clippy::too_many_lines)]
    fn cases() -> Vec<Case> {
        let e = |x, y, al: Option<f64>, at: Option<f64>, mh| Expected(x, y, al, at, mh);
        vec![
            Case {
                placement: Placement::Left,
                no_offset: e(50.0, 200.0, None, Some(100.0), 350.0),
                offset_before: e(-200.0, 50.0, None, Some(4.0), 500.0),
                offset_after: e(300.0, 350.0, None, Some(196.0), 200.0),
                cross_axis_offset_positive: e(50.0, 210.0, None, Some(90.0), 340.0),
                cross_axis_offset_negative: e(50.0, 190.0, None, Some(110.0), 360.0),
                main_axis_offset: e(40.0, 200.0, None, Some(100.0), 350.0),
                arrow_boundary_offset: e(50.0, 322.0, None, Some(24.0), 228.0),
            },
            Case {
                placement: Placement::LeftTop,
                no_offset: e(50.0, 250.0, None, Some(50.0), 300.0),
                offset_before: e(-200.0, 50.0, None, Some(4.0), 500.0),
                offset_after: e(300.0, 350.0, None, Some(196.0), 200.0),
                cross_axis_offset_positive: e(50.0, 260.0, None, Some(40.0), 290.0),
                cross_axis_offset_negative: e(50.0, 240.0, None, Some(60.0), 310.0),
                main_axis_offset: e(40.0, 250.0, None, Some(50.0), 300.0),
                arrow_boundary_offset: e(50.0, 322.0, None, Some(24.0), 228.0),
            },
            Case {
                placement: Placement::LeftBottom,
                no_offset: e(50.0, 150.0, None, Some(150.0), 300.0),
                offset_before: e(-200.0, 50.0, None, Some(4.0), 200.0),
                offset_after: e(300.0, 350.0, None, Some(196.0), 500.0),
                cross_axis_offset_positive: e(50.0, 160.0, None, Some(140.0), 310.0),
                cross_axis_offset_negative: e(50.0, 140.0, None, Some(160.0), 290.0),
                main_axis_offset: e(40.0, 150.0, None, Some(150.0), 300.0),
                arrow_boundary_offset: e(50.0, 322.0, None, Some(24.0), 472.0),
            },
            Case {
                placement: Placement::Top,
                no_offset: e(200.0, 50.0, Some(100.0), None, 200.0),
                offset_before: e(50.0, -200.0, Some(4.0), None, 0.0),
                offset_after: e(350.0, 300.0, Some(196.0), None, 450.0),
                cross_axis_offset_positive: e(210.0, 50.0, Some(90.0), None, 200.0),
                cross_axis_offset_negative: e(190.0, 50.0, Some(110.0), None, 200.0),
                main_axis_offset: e(200.0, 40.0, Some(100.0), None, 190.0),
                arrow_boundary_offset: e(322.0, 50.0, Some(24.0), None, 200.0),
            },
            Case {
                placement: Placement::TopLeft,
                no_offset: e(250.0, 50.0, Some(50.0), None, 200.0),
                offset_before: e(50.0, -200.0, Some(4.0), None, 0.0),
                offset_after: e(350.0, 300.0, Some(196.0), None, 450.0),
                cross_axis_offset_positive: e(260.0, 50.0, Some(40.0), None, 200.0),
                cross_axis_offset_negative: e(240.0, 50.0, Some(60.0), None, 200.0),
                main_axis_offset: e(250.0, 40.0, Some(50.0), None, 190.0),
                arrow_boundary_offset: e(322.0, 50.0, Some(24.0), None, 200.0),
            },
            Case {
                placement: Placement::TopRight,
                no_offset: e(150.0, 50.0, Some(150.0), None, 200.0),
                offset_before: e(50.0, -200.0, Some(4.0), None, 0.0),
                offset_after: e(350.0, 300.0, Some(196.0), None, 450.0),
                cross_axis_offset_positive: e(160.0, 50.0, Some(140.0), None, 200.0),
                cross_axis_offset_negative: e(140.0, 50.0, Some(160.0), None, 200.0),
                main_axis_offset: e(150.0, 40.0, Some(150.0), None, 190.0),
                arrow_boundary_offset: e(322.0, 50.0, Some(24.0), None, 200.0),
            },
            Case {
                placement: Placement::Bottom,
                no_offset: e(200.0, 350.0, Some(100.0), None, 200.0),
                offset_before: e(50.0, 100.0, Some(4.0), None, 450.0),
                offset_after: e(350.0, 600.0, Some(196.0), None, 0.0),
                cross_axis_offset_positive: e(210.0, 350.0, Some(90.0), None, 200.0),
                cross_axis_offset_negative: e(190.0, 350.0, Some(110.0), None, 200.0),
                main_axis_offset: e(200.0, 360.0, Some(100.0), None, 190.0),
                arrow_boundary_offset: e(322.0, 350.0, Some(24.0), None, 200.0),
            },
            Case {
                placement: Placement::BottomLeft,
                no_offset: e(250.0, 350.0, Some(50.0), None, 200.0),
                offset_before: e(50.0, 100.0, Some(4.0), None, 450.0),
                offset_after: e(350.0, 600.0, Some(196.0), None, 0.0),
                cross_axis_offset_positive: e(260.0, 350.0, Some(40.0), None, 200.0),
                cross_axis_offset_negative: e(240.0, 350.0, Some(60.0), None, 200.0),
                main_axis_offset: e(250.0, 360.0, Some(50.0), None, 190.0),
                arrow_boundary_offset: e(322.0, 350.0, Some(24.0), None, 200.0),
            },
            Case {
                placement: Placement::BottomRight,
                no_offset: e(150.0, 350.0, Some(150.0), None, 200.0),
                offset_before: e(50.0, 100.0, Some(4.0), None, 450.0),
                offset_after: e(350.0, 600.0, Some(196.0), None, 0.0),
                cross_axis_offset_positive: e(160.0, 350.0, Some(140.0), None, 200.0),
                cross_axis_offset_negative: e(140.0, 350.0, Some(160.0), None, 200.0),
                main_axis_offset: e(150.0, 360.0, Some(150.0), None, 190.0),
                arrow_boundary_offset: e(322.0, 350.0, Some(24.0), None, 200.0),
            },
            Case {
                placement: Placement::Right,
                no_offset: e(350.0, 200.0, None, Some(100.0), 350.0),
                offset_before: e(100.0, 50.0, None, Some(4.0), 500.0),
                offset_after: e(600.0, 350.0, None, Some(196.0), 200.0),
                cross_axis_offset_positive: e(350.0, 210.0, None, Some(90.0), 340.0),
                cross_axis_offset_negative: e(350.0, 190.0, None, Some(110.0), 360.0),
                main_axis_offset: e(360.0, 200.0, None, Some(100.0), 350.0),
                arrow_boundary_offset: e(350.0, 322.0, None, Some(24.0), 228.0),
            },
            Case {
                placement: Placement::RightTop,
                no_offset: e(350.0, 250.0, None, Some(50.0), 300.0),
                offset_before: e(100.0, 50.0, None, Some(4.0), 500.0),
                offset_after: e(600.0, 350.0, None, Some(196.0), 200.0),
                cross_axis_offset_positive: e(350.0, 260.0, None, Some(40.0), 290.0),
                cross_axis_offset_negative: e(350.0, 240.0, None, Some(60.0), 310.0),
                main_axis_offset: e(360.0, 250.0, None, Some(50.0), 300.0),
                arrow_boundary_offset: e(350.0, 322.0, None, Some(24.0), 228.0),
            },
            Case {
                placement: Placement::RightBottom,
                no_offset: e(350.0, 150.0, None, Some(150.0), 300.0),
                offset_before: e(100.0, 50.0, None, Some(4.0), 200.0),
                offset_after: e(600.0, 350.0, None, Some(196.0), 500.0),
                cross_axis_offset_positive: e(350.0, 160.0, None, Some(140.0), 310.0),
                cross_axis_offset_negative: e(350.0, 140.0, None, Some(160.0), 290.0),
                main_axis_offset: e(360.0, 150.0, None, Some(150.0), 300.0),
                arrow_boundary_offset: e(350.0, 322.0, None, Some(24.0), 472.0),
            },
        ]
    }

    #[test]
    fn placements() {
        for case in cases() {
            let p = case.placement;
            let run = |target: Rect, offset, cross, provider, abo| {
                calculate(p, target, offset, cross, false, provider, 8.0, abo)
            };
            check(
                p,
                &run(target(250.0, 250.0), 0.0, 0.0, 0.0, 0.0),
                case.no_offset,
                false,
                0.0,
                "no viewport offset",
            );
            check(
                p,
                &run(target(250.0, 250.0), 0.0, 0.0, 50.0, 0.0),
                case.no_offset,
                false,
                50.0,
                "provider offset",
            );
            check(
                p,
                &run(target(0.0, 0.0), 0.0, 0.0, 0.0, 0.0),
                case.offset_before,
                false,
                0.0,
                "viewport offset before",
            );
            check(
                p,
                &run(target(500.0, 500.0), 0.0, 0.0, 0.0, 0.0),
                case.offset_after,
                false,
                0.0,
                "viewport offset after",
            );
            check(
                p,
                &run(target(250.0, 250.0), 10.0, 0.0, 0.0, 0.0),
                case.main_axis_offset,
                false,
                0.0,
                "main axis offset",
            );
            check(
                p,
                &run(target(250.0, 250.0), 0.0, 10.0, 0.0, 0.0),
                case.cross_axis_offset_positive,
                false,
                0.0,
                "cross axis offset positive",
            );
            check(
                p,
                &run(target(250.0, 250.0), 0.0, -10.0, 0.0, 0.0),
                case.cross_axis_offset_negative,
                false,
                0.0,
                "cross axis offset negative",
            );
            check(
                p,
                &run(target(250.0, 250.0), 0.0, 1000.0, 0.0, 20.0),
                case.arrow_boundary_offset,
                false,
                0.0,
                "minimum overlay arrow offset",
            );
        }
    }

    #[test]
    fn flips_from_left_to_right() {
        let result = calculate(
            Placement::Left,
            target(0.0, 0.0),
            0.0,
            0.0,
            true,
            0.0,
            8.0,
            0.0,
        );
        check(
            Placement::Left,
            &result,
            Expected(100.0, 50.0, None, Some(4.0), 500.0),
            true,
            0.0,
            "flip",
        );

        let result = calculate(
            Placement::Left,
            target(0.0, 250.0),
            10.0,
            0.0,
            true,
            0.0,
            8.0,
            0.0,
        );
        check(
            Placement::Left,
            &result,
            Expected(110.0, 200.0, None, Some(100.0), 350.0),
            true,
            0.0,
            "flip with positive offset",
        );
    }

    #[test]
    fn overlay_smaller_than_target_aligns_in_center() {
        let big_target = Rect {
            top: 250.0,
            left: 250.0,
            width: 300.0,
            height: 300.0,
        };
        let result = calculate(Placement::Right, big_target, 0.0, 0.0, false, 0.0, 8.0, 0.0);
        check(
            Placement::Right,
            &result,
            Expected(550.0, 300.0, None, Some(100.0), 250.0),
            false,
            0.0,
            "overlay smaller than target",
        );
    }

    #[test]
    fn start_and_end_follow_the_writing_direction() {
        let ltr = Placement::Start.parse(WritingDirection::Ltr);
        let rtl = Placement::Start.parse(WritingDirection::Rtl);
        assert_that!(ltr.placement).is_equal_to(PlacementAxis::Left);
        assert_that!(rtl.placement).is_equal_to(PlacementAxis::Right);
        let rtl = Placement::BottomEnd.parse(WritingDirection::Rtl);
        assert_that!(rtl.cross_placement).is_equal_to(CrossPlacement::Left);
    }
}
