// Upstream: react-aria/src/dnd/useAutoScroll.ts @ 99e6102368
// Upstream: react-aria/test/dnd/useDroppableCollection.test.js @ 99e6102368
use leptos::prelude::*;
use send_wrapper::SendWrapper;

use crate::{
    CapturedElement,
    utils::{
        platform::{browser::is_webkit, device::is_ios},
        scroll::{get_scroll_parent, is_scrollable},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns a `Copy` handle with `move_to` and `stop` (react-aria: `move` and `stop`).
//
// =============================================================================

/// How close (in px) to the scroll container's edge a drag scrolls it.
const AUTOSCROLL_AREA_SIZE: f64 = 20.0;

/// Scrolling a drop target while a drag nears its edges.
#[derive(Debug, Clone, Copy)]
pub struct AutoScroll {
    scrollable: StoredValue<Option<(SendWrapper<web_sys::Element>, bool, bool)>>,
    state: StoredValue<(f64, f64)>,
    timer: StoredValue<Option<AnimationFrameRequestHandle>>,
}

impl AutoScroll {
    /// The drag is at `x`, `y` (relative to the drop target): scroll if near an edge.
    pub fn move_to(&self, x: f64, y: f64) {
        // Only Safari doesn't scroll during native drags itself.
        if !is_webkit() || is_ios() {
            return;
        }
        let Some((element, _, _)) = self.scrollable.get_value() else {
            return;
        };
        let rect = element.get_bounding_client_rect();
        let (left, top) = (AUTOSCROLL_AREA_SIZE, AUTOSCROLL_AREA_SIZE);
        let bottom = rect.height() - AUTOSCROLL_AREA_SIZE;
        let right = rect.width() - AUTOSCROLL_AREA_SIZE;
        if x < left || x > right || y < top || y > bottom {
            self.state.update_value(|(dx, dy)| {
                if x < left {
                    *dx = x - left;
                } else if x > right {
                    *dx = x - right;
                }
                if y < top {
                    *dy = y - top;
                } else if y > bottom {
                    *dy = y - bottom;
                }
            });
            if self.timer.with_value(Option::is_none) {
                self.schedule();
            }
        } else {
            self.stop();
        }
    }

    /// Stop scrolling.
    pub fn stop(&self) {
        if let Some(timer) = self.timer.try_update_value(Option::take).flatten() {
            timer.cancel();
        }
    }

    /// Scroll in the next animation frame (and every frame after it, until stopped).
    fn schedule(self) {
        let handle = request_animation_frame_with_handle(move || self.scroll()).ok();
        if let Some(Some(handle)) = self.timer.try_set_value(handle) {
            handle.cancel();
        }
    }

    fn scroll(self) {
        // The drop target may have been unmounted during the drag: stop.
        let (Some(scrollable), Some((dx, dy))) =
            (self.scrollable.try_get_value(), self.state.try_get_value())
        else {
            return;
        };
        if let Some((element, scroll_x, scroll_y)) = scrollable {
            if scroll_x {
                element.set_scroll_left(element.scroll_left() + dx);
            }
            if scroll_y {
                element.set_scroll_top(element.scroll_top() + dy);
            }
        }
        self.schedule();
    }
}

/// Auto scrolling for the drop target `element` (or its scroll parent) during native drags.
pub fn use_auto_scroll(element: CapturedElement) -> AutoScroll {
    let scrollable = StoredValue::new(None);
    Effect::new(move || {
        let Some(el) = element.get() else {
            return;
        };
        let target: web_sys::Element = if is_scrollable(&el, false) {
            (*el).clone()
        } else {
            get_scroll_parent(&el, false)
        };
        let (x, y) = leptos_use::use_window()
            .as_ref()
            .and_then(|w| w.get_computed_style(&target).ok().flatten())
            .map_or((true, true), |style| {
                let scrolls = |v: String| v.contains("auto") || v.contains("scroll");
                (
                    scrolls(style.get_property_value("overflow-x").unwrap_or_default()),
                    scrolls(style.get_property_value("overflow-y").unwrap_or_default()),
                )
            });
        scrollable.set_value(Some((SendWrapper::new(target), x, y)));
    });
    let auto_scroll = AutoScroll {
        scrollable,
        state: StoredValue::new((0.0, 0.0)),
        timer: StoredValue::new(None),
    };
    on_cleanup(move || auto_scroll.stop());
    auto_scroll
}
