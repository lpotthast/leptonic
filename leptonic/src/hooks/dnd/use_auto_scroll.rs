//! Auto-scroll hook for drag-and-drop operations.
//!
//! Scrolls a container element when the pointer is near its edges during
//! a drag operation.
//!
//! Based on react-aria's auto-scroll logic from
//! `@react-aria/dnd/src/useDroppableCollection.ts`.

use leptos::prelude::*;

//
// ## RUST-NATIVE DESIGN
//
// - Uses a `setInterval` timer with stored velocity for scrolling. The
//   interval fires at ~60fps (16ms). Cleanup via `StoredValue` holding the
//   interval handle.
//
// ## PLATFORM CONSIDERATIONS
//
// - Only needed on WebKit macOS where native auto-scroll during drag is
//   not supported. Other browsers handle this natively.
//

/// Distance in pixels from the container edge that triggers auto-scroll.
const EDGE_SIZE: f64 = 20.0;

/// Maximum scroll speed in pixels per tick.
const MAX_SPEED: f64 = 15.0;

/// Interval in milliseconds for the scroll timer (~60fps).
#[cfg(not(feature = "ssr"))]
const SCROLL_INTERVAL_MS: i32 = 16;

/// Input for the [`use_auto_scroll`] hook.
pub struct UseAutoScrollInput {
    /// The scrollable container element.
    pub container: Signal<Option<web_sys::Element>>,
}

/// Return value of the [`use_auto_scroll`] hook.
pub struct UseAutoScrollReturn {
    /// Call with pointer coordinates during drag move to trigger auto-scroll.
    pub move_to: Callback<(f64, f64)>,
    /// Call when drag exits or ends to stop auto-scroll.
    pub stop: Callback<()>,
}

/// Provides auto-scroll behavior for a container during drag operations.
///
/// When the pointer is within `EDGE_SIZE` pixels of the container's edges,
/// the container scrolls in that direction. The scroll speed is proportional
/// to how close the pointer is to the edge.
///
/// # Example
///
/// ```ignore
/// let auto_scroll = use_auto_scroll(UseAutoScrollInput {
///     container: container_signal,
/// });
///
/// // In dragover handler:
/// auto_scroll.move_to.run((e.client_x(), e.client_y()));
///
/// // In dragleave/drop handler:
/// auto_scroll.stop.run(());
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_auto_scroll(input: UseAutoScrollInput) -> UseAutoScrollReturn {
    let container = input.container;

    // Store the current scroll velocity.
    let velocity_x: StoredValue<f64> = StoredValue::new(0.0);
    let velocity_y: StoredValue<f64> = StoredValue::new(0.0);

    // Store the interval handle and its Closure for cancellation.
    #[cfg(not(feature = "ssr"))]
    let interval_state: StoredValue<
        Option<(i32, wasm_bindgen::closure::Closure<dyn FnMut()>)>,
        LocalStorage,
    > = StoredValue::new_local(None);

    let stop_scrolling = move || {
        velocity_x.set_value(0.0);
        velocity_y.set_value(0.0);
        #[cfg(not(feature = "ssr"))]
        interval_state.update_value(|state| {
            if let Some((id, _closure)) = state.take() {
                if let Some(window) = web_sys::window() {
                    window.clear_interval_with_handle(id);
                }
            }
        });
    };

    let start_scrolling = move || {
        #[cfg(not(feature = "ssr"))]
        {
            // Don't start if already running.
            let already_running = interval_state.with_value(Option::is_some);
            if already_running {
                return;
            }

            let container_for_scroll = container;
            let closure: wasm_bindgen::closure::Closure<dyn FnMut()> =
                wasm_bindgen::closure::Closure::new(move || {
                    let vx = velocity_x.get_value();
                    let vy = velocity_y.get_value();

                    if let Some(el) = container_for_scroll.get_untracked() {
                        let new_left = el.scroll_left() + vx;
                        let new_top = el.scroll_top() + vy;
                        el.set_scroll_left(new_left);
                        el.set_scroll_top(new_top);
                    }
                });

            if let Some(window) = web_sys::window() {
                use wasm_bindgen::JsCast;
                if let Ok(id) = window.set_interval_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    SCROLL_INTERVAL_MS,
                ) {
                    interval_state.set_value(Some((id, closure)));
                }
            }
        }
    };

    let move_to = Callback::new(move |(x, y): (f64, f64)| {
        let Some(el) = container.get_untracked() else {
            return;
        };

        let rect = el.get_bounding_client_rect();

        // Compute velocity based on proximity to edges.
        let mut vx = 0.0;
        let mut vy = 0.0;

        // Top edge
        let top_dist = y - rect.top();
        if (0.0..EDGE_SIZE).contains(&top_dist) {
            vy = -(1.0 - top_dist / EDGE_SIZE) * MAX_SPEED;
        }

        // Bottom edge
        let bottom_dist = rect.bottom() - y;
        if (0.0..EDGE_SIZE).contains(&bottom_dist) {
            vy = (1.0 - bottom_dist / EDGE_SIZE) * MAX_SPEED;
        }

        // Left edge
        let left_dist = x - rect.left();
        if (0.0..EDGE_SIZE).contains(&left_dist) {
            vx = -(1.0 - left_dist / EDGE_SIZE) * MAX_SPEED;
        }

        // Right edge
        let right_dist = rect.right() - x;
        if (0.0..EDGE_SIZE).contains(&right_dist) {
            vx = (1.0 - right_dist / EDGE_SIZE) * MAX_SPEED;
        }

        velocity_x.set_value(vx);
        velocity_y.set_value(vy);

        #[allow(clippy::float_cmp)]
        if vx != 0.0 || vy != 0.0 {
            start_scrolling();
        } else {
            stop_scrolling();
        }
    });

    let stop = Callback::new(move |()| {
        stop_scrolling();
    });

    // Cleanup on unmount.
    on_cleanup(move || {
        stop_scrolling();
    });

    UseAutoScrollReturn { move_to, stop }
}
