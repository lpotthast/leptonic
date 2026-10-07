// Upstream: react-stately/src/virtualizer/OverscanManager.ts @ 99e6102368
use crate::{hooks::collections::Rect, utils::point::Point};

/// Extends the visible rectangle by a third in each direction, towards where the user scrolls,
/// so rows are rendered before they come into view.
#[derive(Debug, Clone, Default)]
pub struct OverscanManager {
    start_time: f64,
    velocity: Point,
    visible_rect: Rect,
}

impl OverscanManager {
    /// The new visible rectangle at `now` (milliseconds, e.g. `performance.now()`).
    pub fn set_visible_rect(&mut self, rect: Rect, now: f64) {
        let time = now - self.start_time;
        if time < 500.0 {
            if rect.x != self.visible_rect.x && time > 0.0 {
                self.velocity.x = (rect.x - self.visible_rect.x) / time;
            }
            if rect.y != self.visible_rect.y && time > 0.0 {
                self.velocity.y = (rect.y - self.visible_rect.y) / time;
            }
        }
        self.start_time = now;
        self.visible_rect = rect;
    }

    /// The visible rectangle, extended in the scroll direction.
    pub fn overscanned_rect(&self) -> Rect {
        let mut overscanned = self.visible_rect;
        let overscan_y = self.visible_rect.height / 3.0;
        overscanned.height += overscan_y;
        if self.velocity.y < 0.0 {
            overscanned.y -= overscan_y;
        }
        let overscan_x = self.visible_rect.width / 3.0;
        overscanned.width += overscan_x;
        if self.velocity.x < 0.0 {
            overscanned.x -= overscan_x;
        }
        overscanned
    }
}
