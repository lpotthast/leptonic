use super::use_overlay_position::{PhysicalPlacementX, PlacementY};

/// An axis-aligned rectangle described by its top-left corner and dimensions.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Rect {
    pub top: f64,
    pub left: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn right(&self) -> f64 {
        self.left + self.width
    }

    pub fn bottom(&self) -> f64 {
        self.top + self.height
    }
}

/// Inputs to the pure positioning function.
pub(crate) struct CalculatePositionInput {
    /// Bounding rect of the trigger element.
    pub target: Rect,
    /// Size of the overlay (only `width` and `height` are used).
    pub overlay: Rect,
    /// Boundary rect (viewport). For `position: fixed`, this is `{top: 0, left: 0, width, height}`.
    pub boundary: Rect,
    /// Resolved physical horizontal placement.
    pub placement_x: PhysicalPlacementX,
    /// Vertical placement.
    pub placement_y: PlacementY,
    /// Offset along the main axis (pushes overlay away from the target). Default: 0.
    pub offset: f64,
    /// Offset along the cross axis. Default: 0.
    pub cross_offset: f64,
    /// Minimum padding between the overlay and the boundary edge. Default: 12.
    pub container_padding: f64,
    /// Whether the overlay should flip to the opposite side when insufficient space. Default: true.
    pub should_flip: bool,
    /// Optional user-specified max height override.
    pub max_height: Option<f64>,
}

/// Result of the positioning calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PositionResult {
    /// Top position in pixels.
    pub top: f64,
    /// Left position in pixels.
    pub left: f64,
    /// Computed maximum height for the overlay.
    pub max_height: f64,
    /// Resolved horizontal placement (may differ from input due to flipping).
    pub placement_x: PhysicalPlacementX,
    /// Resolved vertical placement (may differ from input due to flipping).
    pub placement_y: PlacementY,
}

impl Default for PositionResult {
    fn default() -> Self {
        Self {
            top: 0.0,
            left: 0.0,
            max_height: f64::MAX,
            placement_x: PhysicalPlacementX::Center,
            placement_y: PlacementY::Below,
        }
    }
}

/// Determines whether the Y axis is the main axis for the given placement combination.
///
/// - `Above`/`Below` → Y is main axis (overlay placed above/below the target).
/// - `OuterLeft`/`OuterRight` with non-outer Y → X is main axis.
/// - Otherwise (inner alignment on both axes) → no main axis.
fn y_is_main_axis(placement_x: PhysicalPlacementX, placement_y: PlacementY) -> Option<bool> {
    match placement_y {
        PlacementY::Above | PlacementY::Below => Some(true),
        _ => match placement_x {
            PhysicalPlacementX::OuterLeft | PhysicalPlacementX::OuterRight => Some(false),
            _ => None,
        },
    }
}

/// Compute the initial top position based on vertical placement.
fn compute_top(target: &Rect, overlay_height: f64, placement_y: PlacementY) -> f64 {
    match placement_y {
        PlacementY::Above => target.top - overlay_height,
        PlacementY::Top => target.top,
        PlacementY::Center => {
            target.top + (target.height / 2.0) - (overlay_height / 2.0)
        }
        PlacementY::Bottom => target.bottom() - overlay_height,
        PlacementY::Below => target.bottom(),
    }
}

/// Compute the initial left position based on horizontal placement.
fn compute_left(target: &Rect, overlay_width: f64, placement_x: PhysicalPlacementX) -> f64 {
    match placement_x {
        PhysicalPlacementX::OuterLeft => target.left - overlay_width,
        PhysicalPlacementX::Left => target.left,
        PhysicalPlacementX::Center => {
            target.left + (target.width / 2.0) - (overlay_width / 2.0)
        }
        PhysicalPlacementX::Right => target.right() - overlay_width,
        PhysicalPlacementX::OuterRight => target.right(),
    }
}

/// Apply main-axis offset (pushes overlay away from the target).
fn apply_offset(
    top: &mut f64,
    left: &mut f64,
    offset: f64,
    placement_x: PhysicalPlacementX,
    placement_y: PlacementY,
    y_main: Option<bool>,
) {
    match y_main {
        Some(true) => {
            // Y is main axis: offset pushes along Y
            match placement_y {
                PlacementY::Above => *top -= offset,
                PlacementY::Below => *top += offset,
                _ => {}
            }
        }
        Some(false) => {
            // X is main axis: offset pushes along X
            match placement_x {
                PhysicalPlacementX::OuterLeft => *left -= offset,
                PhysicalPlacementX::OuterRight => *left += offset,
                _ => {}
            }
        }
        None => {
            // No main axis (inner alignment), no offset to apply
        }
    }
}

/// Apply cross-axis offset.
fn apply_cross_offset(
    top: &mut f64,
    left: &mut f64,
    cross_offset: f64,
    y_main: Option<bool>,
) {
    match y_main {
        Some(true) => {
            // Y is main axis → cross axis is X
            *left += cross_offset;
        }
        Some(false) => {
            // X is main axis → cross axis is Y
            *top += cross_offset;
        }
        None => {
            // No main axis, apply cross_offset to X by convention
            *left += cross_offset;
        }
    }
}

/// Get available space in the given direction.
fn get_available_space_y(
    target: &Rect,
    boundary: &Rect,
    container_padding: f64,
    placement_y: PlacementY,
) -> f64 {
    match placement_y {
        PlacementY::Above => target.top - boundary.top - container_padding,
        PlacementY::Below => boundary.bottom() - target.bottom() - container_padding,
        _ => f64::MAX,
    }
}

fn get_available_space_x(
    target: &Rect,
    boundary: &Rect,
    container_padding: f64,
    placement_x: PhysicalPlacementX,
) -> f64 {
    match placement_x {
        PhysicalPlacementX::OuterLeft => target.left - boundary.left - container_padding,
        PhysicalPlacementX::OuterRight => boundary.right() - target.right() - container_padding,
        _ => f64::MAX,
    }
}

fn flip_y(placement_y: PlacementY) -> PlacementY {
    match placement_y {
        PlacementY::Above => PlacementY::Below,
        PlacementY::Below => PlacementY::Above,
        other => other,
    }
}

fn flip_x(placement_x: PhysicalPlacementX) -> PhysicalPlacementX {
    match placement_x {
        PhysicalPlacementX::OuterLeft => PhysicalPlacementX::OuterRight,
        PhysicalPlacementX::OuterRight => PhysicalPlacementX::OuterLeft,
        other => other,
    }
}

/// Compute a boundary-nudging delta for a single axis.
/// Keeps the overlay within `[boundary_start + padding, boundary_end - padding]`.
fn get_delta(
    start_edge: f64,
    end_edge: f64,
    boundary_start: f64,
    boundary_end: f64,
    padding: f64,
) -> f64 {
    let min = boundary_start + padding;
    let max = boundary_end - padding;
    if start_edge < min {
        min - start_edge
    } else if end_edge > max {
        // Nudge left/up, but don't push the start edge past the minimum.
        (max - end_edge).max(min - start_edge)
    } else {
        0.0
    }
}

/// Compute max height based on placement.
fn compute_max_height(
    top: f64,
    overlay_height: f64,
    boundary: &Rect,
    container_padding: f64,
    placement_y: PlacementY,
) -> f64 {
    match placement_y {
        // Overlay grows upward: space from overlay bottom to boundary top.
        PlacementY::Above => (top + overlay_height) - boundary.top - container_padding,
        // Overlay grows downward: space from overlay top to boundary bottom.
        _ => boundary.bottom() - top - container_padding,
    }
}

/// Apply flip logic, returning the (possibly flipped) placements.
fn apply_flip(
    target: &Rect,
    overlay: &Rect,
    boundary: &Rect,
    placement_x: PhysicalPlacementX,
    placement_y: PlacementY,
    container_padding: f64,
) -> (PhysicalPlacementX, PlacementY) {
    let mut px = placement_x;
    let mut py = placement_y;

    if matches!(py, PlacementY::Above | PlacementY::Below) {
        let available = get_available_space_y(target, boundary, container_padding, py);
        if overlay.height > available {
            let flipped = flip_y(py);
            let available_flipped =
                get_available_space_y(target, boundary, container_padding, flipped);
            if available_flipped > available {
                py = flipped;
            }
        }
    }

    if matches!(
        px,
        PhysicalPlacementX::OuterLeft | PhysicalPlacementX::OuterRight
    ) {
        let available = get_available_space_x(target, boundary, container_padding, px);
        if overlay.width > available {
            let flipped = flip_x(px);
            let available_flipped =
                get_available_space_x(target, boundary, container_padding, flipped);
            if available_flipped > available {
                px = flipped;
            }
        }
    }

    (px, py)
}

/// Apply boundary nudging to keep the overlay within the boundary.
fn apply_boundary_nudging(
    top: &mut f64,
    left: &mut f64,
    overlay_width: f64,
    overlay_height: f64,
    boundary: &Rect,
    container_padding: f64,
    y_main: Option<bool>,
) {
    match y_main {
        Some(true) => {
            *left += get_delta(
                *left, *left + overlay_width,
                boundary.left, boundary.right(), container_padding,
            );
        }
        Some(false) => {
            *top += get_delta(
                *top, *top + overlay_height,
                boundary.top, boundary.bottom(), container_padding,
            );
        }
        None => {
            *left += get_delta(
                *left, *left + overlay_width,
                boundary.left, boundary.right(), container_padding,
            );
            *top += get_delta(
                *top, *top + overlay_height,
                boundary.top, boundary.bottom(), container_padding,
            );
        }
    }
}

/// Pure positioning function. Given target/overlay geometry, boundary, and placement
/// parameters, computes the final overlay position, resolved placements, and max height.
///
/// This mirrors react-aria's `calculatePosition` but uses `position: fixed` relative to the
/// viewport (intentional deviation — Leptos `<Portal>` appends to `<body>`).
pub(crate) fn calculate_position(input: &CalculatePositionInput) -> PositionResult {
    let mut placement_x = input.placement_x;
    let mut placement_y = input.placement_y;

    let y_main = y_is_main_axis(placement_x, placement_y);

    if input.should_flip {
        (placement_x, placement_y) = apply_flip(
            &input.target, &input.overlay, &input.boundary,
            placement_x, placement_y, input.container_padding,
        );
    }

    let mut top = compute_top(&input.target, input.overlay.height, placement_y);
    let mut left = compute_left(&input.target, input.overlay.width, placement_x);

    apply_offset(&mut top, &mut left, input.offset, placement_x, placement_y, y_main);
    apply_cross_offset(&mut top, &mut left, input.cross_offset, y_main);
    apply_boundary_nudging(
        &mut top, &mut left,
        input.overlay.width, input.overlay.height,
        &input.boundary, input.container_padding, y_main,
    );

    let mut max_height = compute_max_height(
        top, input.overlay.height, &input.boundary, input.container_padding, placement_y,
    );
    if let Some(user_mh) = input.max_height {
        max_height = max_height.min(user_mh);
    }
    max_height = max_height.max(0.0);

    // Two-pass: if overlay is taller than max_height, recompute position with constrained height.
    if input.overlay.height > max_height {
        top = compute_top(&input.target, max_height, placement_y);
        apply_offset(&mut top, &mut left, input.offset, placement_x, placement_y, y_main);
        if y_main == Some(false) {
            top += input.cross_offset;
        }
        if y_main == Some(false) {
            top += get_delta(
                top, top + max_height,
                input.boundary.top, input.boundary.bottom(), input.container_padding,
            );
        }
    }

    PositionResult { top, left, max_height, placement_x, placement_y }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn default_input() -> CalculatePositionInput {
        CalculatePositionInput {
            target: Rect {
                top: 200.0,
                left: 300.0,
                width: 100.0,
                height: 50.0,
            },
            overlay: Rect {
                top: 0.0,
                left: 0.0,
                width: 150.0,
                height: 100.0,
            },
            boundary: Rect {
                top: 0.0,
                left: 0.0,
                width: 1024.0,
                height: 768.0,
            },
            placement_x: PhysicalPlacementX::Center,
            placement_y: PlacementY::Below,
            offset: 0.0,
            cross_offset: 0.0,
            container_padding: 0.0,
            should_flip: true,
            max_height: None,
        }
    }

    // =========================================================================
    // Basic placement tests
    // =========================================================================

    #[test]
    fn below_center() {
        let result = calculate_position(&default_input());
        // Below: top = target.bottom() = 200 + 50 = 250
        assert_that(result.top).is_close_to(250.0, 0.01);
        // Center: left = target.left + target.width/2 - overlay.width/2 = 300 + 50 - 75 = 275
        assert_that(result.left).is_close_to(275.0, 0.01);
        assert_that(result.placement_y).is_equal_to(PlacementY::Below);
        assert_that(result.placement_x).is_equal_to(PhysicalPlacementX::Center);
    }

    #[test]
    fn above_center() {
        let mut input = default_input();
        input.placement_y = PlacementY::Above;
        let result = calculate_position(&input);
        // Above: top = target.top - overlay.height = 200 - 100 = 100
        assert_that(result.top).is_close_to(100.0, 0.01);
        assert_that(result.left).is_close_to(275.0, 0.01);
    }

    #[test]
    fn below_left() {
        let mut input = default_input();
        input.placement_x = PhysicalPlacementX::Left;
        let result = calculate_position(&input);
        // Below: top = 250
        assert_that(result.top).is_close_to(250.0, 0.01);
        // Left: left = target.left = 300
        assert_that(result.left).is_close_to(300.0, 0.01);
    }

    #[test]
    fn below_right() {
        let mut input = default_input();
        input.placement_x = PhysicalPlacementX::Right;
        let result = calculate_position(&input);
        // Right: left = target.right() - overlay.width = 400 - 150 = 250
        assert_that(result.left).is_close_to(250.0, 0.01);
    }

    #[test]
    fn center_outer_right() {
        let mut input = default_input();
        input.placement_x = PhysicalPlacementX::OuterRight;
        input.placement_y = PlacementY::Center;
        let result = calculate_position(&input);
        // OuterRight: left = target.right() = 400
        assert_that(result.left).is_close_to(400.0, 0.01);
        // Center Y: top = target.top + target.height/2 - overlay.height/2 = 200 + 25 - 50 = 175
        assert_that(result.top).is_close_to(175.0, 0.01);
    }

    #[test]
    fn center_outer_left() {
        let mut input = default_input();
        input.placement_x = PhysicalPlacementX::OuterLeft;
        input.placement_y = PlacementY::Center;
        let result = calculate_position(&input);
        // OuterLeft: left = target.left - overlay.width = 300 - 150 = 150
        assert_that(result.left).is_close_to(150.0, 0.01);
        assert_that(result.top).is_close_to(175.0, 0.01);
    }

    #[test]
    fn top_center() {
        let mut input = default_input();
        input.placement_y = PlacementY::Top;
        let result = calculate_position(&input);
        // Top: top = target.top = 200
        assert_that(result.top).is_close_to(200.0, 0.01);
    }

    #[test]
    fn bottom_center() {
        let mut input = default_input();
        input.placement_y = PlacementY::Bottom;
        let result = calculate_position(&input);
        // Bottom: top = target.bottom() - overlay.height = 250 - 100 = 150
        assert_that(result.top).is_close_to(150.0, 0.01);
    }

    // =========================================================================
    // Offset tests
    // =========================================================================

    #[test]
    fn offset_below() {
        let mut input = default_input();
        input.offset = 8.0;
        let result = calculate_position(&input);
        // Below with offset: top = 250 + 8 = 258
        assert_that(result.top).is_close_to(258.0, 0.01);
        // Left unchanged
        assert_that(result.left).is_close_to(275.0, 0.01);
    }

    #[test]
    fn offset_above() {
        let mut input = default_input();
        input.placement_y = PlacementY::Above;
        input.offset = 8.0;
        let result = calculate_position(&input);
        // Above with offset: top = 100 - 8 = 92
        assert_that(result.top).is_close_to(92.0, 0.01);
    }

    #[test]
    fn offset_outer_right() {
        let mut input = default_input();
        input.placement_x = PhysicalPlacementX::OuterRight;
        input.placement_y = PlacementY::Center;
        input.offset = 10.0;
        let result = calculate_position(&input);
        // OuterRight with offset: left = 400 + 10 = 410
        assert_that(result.left).is_close_to(410.0, 0.01);
    }

    #[test]
    fn offset_outer_left() {
        let mut input = default_input();
        input.placement_x = PhysicalPlacementX::OuterLeft;
        input.placement_y = PlacementY::Center;
        input.offset = 10.0;
        let result = calculate_position(&input);
        // OuterLeft with offset: left = 150 - 10 = 140
        assert_that(result.left).is_close_to(140.0, 0.01);
    }

    // =========================================================================
    // Cross offset tests
    // =========================================================================

    #[test]
    fn cross_offset_when_y_is_main() {
        let mut input = default_input();
        input.cross_offset = 20.0;
        let result = calculate_position(&input);
        // Y main → cross axis is X → left += 20 → 275 + 20 = 295
        assert_that(result.left).is_close_to(295.0, 0.01);
        // top unchanged
        assert_that(result.top).is_close_to(250.0, 0.01);
    }

    #[test]
    fn cross_offset_when_x_is_main() {
        let mut input = default_input();
        input.placement_x = PhysicalPlacementX::OuterRight;
        input.placement_y = PlacementY::Center;
        input.cross_offset = 15.0;
        let result = calculate_position(&input);
        // X main → cross axis is Y → top += 15 → 175 + 15 = 190
        assert_that(result.top).is_close_to(190.0, 0.01);
    }

    // =========================================================================
    // Flip tests
    // =========================================================================

    #[test]
    fn flip_below_to_above_when_insufficient_space() {
        let mut input = default_input();
        // Target near bottom edge: 700px from top, boundary is 768px.
        // Space below = 768 - (700 + 50) = 18, overlay height = 100 → not enough.
        // Space above = 700 = plenty.
        input.target.top = 700.0;
        let result = calculate_position(&input);
        assert_that(result.placement_y).is_equal_to(PlacementY::Above);
        // Above: top = 700 - 100 = 600
        assert_that(result.top).is_close_to(600.0, 0.01);
    }

    #[test]
    fn flip_above_to_below_when_insufficient_space() {
        let mut input = default_input();
        input.placement_y = PlacementY::Above;
        // Target near top: 50px from top. Space above = 50, overlay = 100.
        // Space below = 768 - (50 + 50) = 668.
        input.target.top = 50.0;
        let result = calculate_position(&input);
        assert_that(result.placement_y).is_equal_to(PlacementY::Below);
    }

    #[test]
    fn flip_outer_right_to_outer_left() {
        let mut input = default_input();
        input.placement_x = PhysicalPlacementX::OuterRight;
        input.placement_y = PlacementY::Center;
        // Target near right edge: 900. Right = 1000. Space right = 1024 - 1000 = 24. Overlay = 150.
        // Space left = 900.
        input.target.left = 900.0;
        let result = calculate_position(&input);
        assert_that(result.placement_x).is_equal_to(PhysicalPlacementX::OuterLeft);
    }

    #[test]
    fn flip_outer_left_to_outer_right() {
        let mut input = default_input();
        input.placement_x = PhysicalPlacementX::OuterLeft;
        input.placement_y = PlacementY::Center;
        // Target near left edge: 50. Space left = 50, overlay width = 150.
        // Space right = 1024 - 150 = 874.
        input.target.left = 50.0;
        let result = calculate_position(&input);
        assert_that(result.placement_x).is_equal_to(PhysicalPlacementX::OuterRight);
    }

    #[test]
    fn no_flip_when_should_flip_is_false() {
        let mut input = default_input();
        input.should_flip = false;
        // Target near bottom, would normally flip.
        input.target.top = 700.0;
        let result = calculate_position(&input);
        assert_that(result.placement_y).is_equal_to(PlacementY::Below);
    }

    #[test]
    fn no_flip_when_both_sides_insufficient_but_original_is_better() {
        let mut input = default_input();
        input.placement_y = PlacementY::Below;
        // Target in the middle but overlay is huge
        input.overlay.height = 500.0;
        // Space below = 768 - 250 = 518, space above = 200. Below is better, don't flip.
        let result = calculate_position(&input);
        assert_that(result.placement_y).is_equal_to(PlacementY::Below);
    }

    // =========================================================================
    // Boundary nudging tests
    // =========================================================================

    #[test]
    fn nudge_right_when_overlay_extends_past_left_boundary() {
        let mut input = default_input();
        input.container_padding = 12.0;
        // Target near left edge, center placement pushes overlay past left boundary.
        // target.left=20, target.width=100 → center of target = 70
        // left = 70 - 75 = -5, which is < 0 + 12 = 12 → nudge right
        input.target.left = 20.0;
        let result = calculate_position(&input);
        // Should be nudged to at least container_padding (12)
        assert_that(result.left).is_greater_or_equal_to(12.0);
    }

    #[test]
    fn nudge_left_when_overlay_extends_past_right_boundary() {
        let mut input = default_input();
        input.container_padding = 12.0;
        // Target near right edge, center placement pushes overlay past right boundary.
        // target.left=950, center = 1000 - 75 = 925. right edge = 925 + 150 = 1075 > 1024 - 12 = 1012
        input.target.left = 950.0;
        let result = calculate_position(&input);
        // Right edge of overlay should be <= boundary.right - padding
        let right_edge = result.left + 150.0;
        assert_that(right_edge).is_less_or_equal_to(1012.0);
    }

    #[test]
    fn nudge_y_when_x_is_main_axis() {
        let mut input = default_input();
        input.placement_x = PhysicalPlacementX::OuterRight;
        input.placement_y = PlacementY::Center;
        input.container_padding = 12.0;
        // Target near top. Center Y = 25 - 50 = -25 → nudge down.
        input.target.top = 25.0;
        input.target.height = 50.0;
        let result = calculate_position(&input);
        assert_that(result.top).is_greater_or_equal_to(12.0);
    }

    // =========================================================================
    // Max height tests
    // =========================================================================

    #[test]
    fn max_height_below() {
        let mut input = default_input();
        input.container_padding = 12.0;
        let result = calculate_position(&input);
        // Below: max_height = boundary.bottom - top - padding = 768 - 250 - 12 = 506
        assert_that(result.max_height).is_close_to(506.0, 0.01);
    }

    #[test]
    fn max_height_above() {
        let mut input = default_input();
        input.placement_y = PlacementY::Above;
        input.container_padding = 12.0;
        let result = calculate_position(&input);
        // Above: max_height = (top + overlay.height) - boundary.top - padding
        //       = (100 + 100) - 0 - 12 = 188
        assert_that(result.max_height).is_close_to(188.0, 0.01);
    }

    #[test]
    fn user_max_height_override() {
        let mut input = default_input();
        input.max_height = Some(200.0);
        let result = calculate_position(&input);
        // Computed max_height = 768 - 250 - 0 = 518. min(518, 200) = 200
        assert_that(result.max_height).is_close_to(200.0, 0.01);
    }

    #[test]
    fn user_max_height_does_not_increase_computed() {
        let mut input = default_input();
        input.container_padding = 12.0;
        input.max_height = Some(9999.0);
        let result = calculate_position(&input);
        // Computed = 506, user = 9999, result = 506
        assert_that(result.max_height).is_close_to(506.0, 0.01);
    }

    // =========================================================================
    // Two-pass recomputation test
    // =========================================================================

    #[test]
    fn two_pass_adjusts_position_after_height_constraint() {
        let mut input = default_input();
        input.placement_y = PlacementY::Above;
        input.container_padding = 12.0;
        // Disable flipping so the two-pass recomputation is exercised.
        input.should_flip = false;
        // target.top = 80, overlay.height = 100
        // Above: top = 80 - 100 = -20. max_height = (-20 + 100) - 0 - 12 = 68.
        // overlay.height (100) > max_height (68) → constrain to 68.
        // Recompute: top = 80 - 68 = 12.
        input.target.top = 80.0;
        let result = calculate_position(&input);
        assert_that(result.max_height).is_close_to(68.0, 0.01);
        assert_that(result.top).is_close_to(12.0, 0.01);
    }

    // =========================================================================
    // Edge case: overlay larger than viewport
    // =========================================================================

    #[test]
    fn overlay_larger_than_viewport() {
        let mut input = default_input();
        input.overlay.width = 2000.0;
        input.overlay.height = 2000.0;
        input.container_padding = 12.0;
        let result = calculate_position(&input);
        // Should still produce valid (non-NaN) results, nudged to container_padding
        assert_that(result.left).is_close_to(12.0, 0.01);
        assert_that(result.max_height).is_greater_or_equal_to(0.0);
    }

    // =========================================================================
    // All 25 placement combinations produce valid coordinates
    // =========================================================================

    #[test]
    fn all_placement_combinations_produce_valid_results() {
        let placements_x = [
            PhysicalPlacementX::OuterLeft,
            PhysicalPlacementX::Left,
            PhysicalPlacementX::Center,
            PhysicalPlacementX::Right,
            PhysicalPlacementX::OuterRight,
        ];
        let placements_y = [
            PlacementY::Above,
            PlacementY::Top,
            PlacementY::Center,
            PlacementY::Bottom,
            PlacementY::Below,
        ];

        for &px in &placements_x {
            for &py in &placements_y {
                let mut input = default_input();
                input.placement_x = px;
                input.placement_y = py;
                input.container_padding = 12.0;
                let result = calculate_position(&input);
                assert!(
                    result.top.is_finite(),
                    "top is not finite for {px:?}/{py:?}"
                );
                assert!(
                    result.left.is_finite(),
                    "left is not finite for {px:?}/{py:?}"
                );
                assert!(
                    result.max_height.is_finite() && result.max_height >= 0.0,
                    "max_height invalid for {px:?}/{py:?}: {}",
                    result.max_height
                );
            }
        }
    }
}
