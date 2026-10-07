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

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn extends_downwards_and_to_the_right_at_rest() {
        let mut overscan = OverscanManager::default();
        overscan.set_visible_rect(Rect::new(0.0, 300.0, 90.0, 300.0), 1000.0);
        // A third of the size, after the visible rectangle.
        assert_that!(overscan.overscanned_rect()).is_equal_to(Rect::new(0.0, 300.0, 120.0, 400.0));
    }

    #[test]
    fn extends_towards_where_the_user_scrolls() {
        let mut overscan = OverscanManager::default();
        overscan.set_visible_rect(Rect::new(0.0, 600.0, 90.0, 300.0), 1000.0);
        // Scrolling up (and left) within 500ms: the overscan goes before the visible rectangle.
        overscan.set_visible_rect(Rect::new(0.0, 500.0, 90.0, 300.0), 1100.0);
        assert_that!(overscan.overscanned_rect()).is_equal_to(Rect::new(0.0, 400.0, 120.0, 400.0));
        overscan.set_visible_rect(Rect::new(-30.0, 500.0, 90.0, 300.0), 1200.0);
        assert_that!(overscan.overscanned_rect().x).is_equal_to(-60.0);

        // Scrolling down again: after it.
        overscan.set_visible_rect(Rect::new(-30.0, 550.0, 90.0, 300.0), 1300.0);
        assert_that!(overscan.overscanned_rect().y).is_equal_to(550.0);
    }

    #[test]
    fn keeps_the_direction_after_a_pause() {
        let mut overscan = OverscanManager::default();
        overscan.set_visible_rect(Rect::new(0.0, 600.0, 90.0, 300.0), 1000.0);
        overscan.set_visible_rect(Rect::new(0.0, 500.0, 90.0, 300.0), 1100.0);
        // More than 500ms later, the velocity isn't updated (as upstream): still upwards.
        overscan.set_visible_rect(Rect::new(0.0, 700.0, 90.0, 300.0), 2000.0);
        assert_that!(overscan.overscanned_rect().y).is_equal_to(600.0);
    }
}
