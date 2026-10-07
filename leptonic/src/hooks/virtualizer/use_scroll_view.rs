// Upstream: react-aria/src/virtualizer/ScrollView.tsx @ 99e6102368
// Upstream: react-aria/src/virtualizer/utils.ts @ 99e6102368
use leptos::prelude::*;

use crate::{
    hooks::collections::{Rect, Size},
    utils::{CapturedElement, i18n::use_direction, locale::WritingDirection, styles::Styles},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The scroll view is the element the caller captures (`element`; react-aria: a `ScrollView`
//   component or `useScrollView`'s ref); the hook returns the styles of it and of its content box.
// - Callbacks are `Callback`s of typed values. `scroll_to` is part of the hook (react-aria's
//   `Virtualizer` writes `scrollLeft`/`scrollTop` in its `onVisibleRectChange`).
// - `scroll_to` subtracts the view's offset in the window (window scrolling): the visible
//   rectangle includes it, the view's own scroll position doesn't (react-aria writes the
//   rectangle's position as it is, which overshoots while the page is scrolled past the view).
//   In right-to-left, it writes the negative `scrollLeft` the specification defines (react-aria
//   writes the positive offset).
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - `scroll_to` scrolls the view (after the next frame, once the content has its new size,
//   unless the user scrolled meanwhile) and the scroll event it causes doesn't count as the user
//   scrolling: items are measured in effects
//   after that event (react-aria: in layout effects before it), and anchoring skips while the
//   user scrolls.
// - Scrollbars appearing or disappearing after a size change (the content laid out for the new
//   size) are picked up by measuring again in the next frame (react-aria measures again
//   synchronously after `flushSync`); at most once per change, as react-aria.
//
// ## OMITTED FEATURES
// - Typekit's `tk.disconnect-observer`/`tk.connect-observer` events.
// - `onScroll`: listen to `scroll` on the element itself.
// - Right-to-left `scrollLeft` normalization (`getRTLOffsetType`): modern browsers report
//   negative offsets as specified; horizontal virtualization in right-to-left is untested.
//
// =============================================================================

/// The scroll direction of a scroll view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollDirection {
    Horizontal,
    Vertical,
    #[default]
    Both,
}

/// Input of [`use_scroll_view`].
pub struct UseScrollViewInput {
    /// The scroll view element (captured by the caller).
    pub element: CapturedElement,
    /// The size of the content (the virtualizer's).
    pub content_size: Signal<Size>,
    /// The visible part of the content changed.
    pub on_visible_rect_change: Callback<Rect>,
    /// The scroll view's size changed.
    pub on_size_change: Option<Callback<Size>>,
    pub on_scroll_start: Option<Callback<()>>,
    pub on_scroll_end: Option<Callback<()>>,
    /// The axes the view scrolls along. React-aria's default: `Both`.
    pub scroll_direction: Signal<ScrollDirection>,
    /// The visible area is also bounded by the window's viewport (the scroll view may grow with
    /// its content and scroll with the page). React-aria's default: `false`.
    pub allows_window_scrolling: Signal<bool>,
}

/// Return value of [`use_scroll_view`].
pub struct UseScrollViewReturn {
    /// Styles of the scroll view: no padding (the layout pads), the overflow.
    pub scroll_view_styles: Styles,
    /// Styles of the content box inside it: the content size, `position: relative`, no pointer
    /// events while scrolling.
    pub content_styles: Styles,
    pub is_scrolling: Signal<bool>,
    /// Scrolls the view so `rect`'s position is the visible rectangle's (e.g. where the
    /// virtualizer moved the viewport).
    pub scroll_to: Callback<crate::hooks::collections::Rect>,
}

#[derive(Debug, Clone, Copy, Default)]
struct ScrollState {
    scroll_position: (f64, f64),
    size: Size,
    /// The offset of the scroll view relative to the window viewport.
    viewport_offset: (f64, f64),
    viewport_size: Size,
    last_visible_rect: Rect,
}

/// A scrollable view of a virtualized collection's content (react-aria's `useScrollView`):
/// reports the visible rectangle as it scrolls and resizes.
#[allow(clippy::too_many_lines)]
pub fn use_scroll_view(input: UseScrollViewInput) -> UseScrollViewReturn {
    let UseScrollViewInput {
        element,
        content_size,
        on_visible_rect_change,
        on_size_change,
        on_scroll_start,
        on_scroll_end,
        scroll_direction,
        allows_window_scrolling,
    } = input;
    let state = StoredValue::new(ScrollState::default());
    let is_scrolling = RwSignal::new(false);
    // The position `scroll_to` scrolled to, until its scroll event arrived.
    let expected_position = StoredValue::new(None::<(f64, f64)>);
    // Counts the user's scrolls: a `scroll_to` requested before one is dropped (the user wins).
    let user_scrolls = StoredValue::new(0_u64);

    let update_visible_rect = move || {
        let Some(s) = state.try_get_value() else {
            return;
        };
        // Intersect the window viewport with the scroll view: a scroll view of unbounded height
        // still virtualizes while the page scrolls.
        let visible_rect = if allows_window_scrolling.get_untracked() {
            Rect::new(
                s.viewport_offset.0 + s.scroll_position.0,
                s.viewport_offset.1 + s.scroll_position.1,
                (s.size.width - s.viewport_offset.0)
                    .min(s.viewport_size.width)
                    .max(0.0),
                (s.size.height - s.viewport_offset.1)
                    .min(s.viewport_size.height)
                    .max(0.0),
            )
        } else {
            Rect::new(
                s.scroll_position.0,
                s.scroll_position.1,
                s.size.width,
                s.size.height,
            )
        };
        // Nothing to report while the area stays empty.
        if visible_rect.area() > 0.0 || s.last_visible_rect.area() > 0.0 {
            on_visible_rect_change.run(visible_rect);
            state.update_value(|s| s.last_visible_rect = visible_rect);
        }
    };

    #[cfg(not(feature = "ssr"))]
    {
        use std::{
            sync::{Arc, Mutex},
            time::Duration,
        };

        use leptos::ev;
        use send_wrapper::SendWrapper;
        use wasm_bindgen::JsCast;

        use crate::utils::{
            event_listeners::listen_to,
            shadow_dom::{get_event_target, node_contains},
        };

        let scroll_timeout = Arc::new(Mutex::new(None::<TimeoutHandle>));
        let scroll_end_time = StoredValue::new(0.0_f64);

        // Measures the view; whether its size changed.
        let measure = move || -> bool {
            // Disposed before a deferred call (the element capture belongs to the same owner).
            let Some(previous) = state.try_get_value() else {
                return false;
            };
            let Some(dom) = element.get_untracked() else {
                return false;
            };
            let Some(dom) = dom.dyn_ref::<web_sys::HtmlElement>() else {
                return false;
            };
            let Some(window) = leptos_use::use_window().as_ref().cloned() else {
                return false;
            };
            let viewport = Size::new(
                window
                    .inner_width()
                    .ok()
                    .and_then(|w| w.as_f64())
                    .unwrap_or(0.0),
                window
                    .inner_height()
                    .ok()
                    .and_then(|h| h.as_f64())
                    .unwrap_or(0.0),
            );
            let size = Size::new(
                f64::from(dom.client_width()),
                f64::from(dom.client_height()),
            );
            if previous.size == size && previous.viewport_size == viewport {
                return false;
            }
            state.update_value(|s| {
                s.size = size;
                s.viewport_size = viewport;
            });
            update_visible_rect();
            if let Some(on_size_change) = on_size_change {
                on_size_change.run(size);
            }
            true
        };
        let update_size = move || {
            // The new layout may show or hide scrollbars, which changes the client size again:
            // measure once more after it rendered.
            if measure() {
                request_animation_frame(move || {
                    measure();
                });
            }
        };
        // Switching window scrolling changes the visible rectangle.
        Effect::new(move |previous: Option<bool>| {
            let allows = allows_window_scrolling.get();
            if previous.is_some_and(|previous| previous != allows) {
                update_visible_rect();
            }
            allows
        });

        // Scrolls of the view and of its ancestors (a capturing document listener).
        let scroll_timeout_handle = Arc::clone(&scroll_timeout);
        let on_scroll = move |e: web_sys::Event| {
            let Some(view) = element.get_untracked() else {
                return;
            };
            let Some(target) =
                get_event_target(&e).and_then(|t| t.dyn_into::<web_sys::Node>().ok())
            else {
                return;
            };
            let view_node: &web_sys::Node = &view;
            if !node_contains(&target, view_node) {
                return;
            }
            if target == *view_node {
                // The view scrolled; out-of-bounds (rubber band) positions are clamped.
                let content = content_size.get_untracked();
                let size = state.with_value(|s| s.size);
                let left = view.scroll_left().abs();
                let top = view.scroll_top();
                state.update_value(|s| {
                    s.scroll_position = (
                        left.min(content.width - size.width).max(0.0),
                        top.min(content.height - size.height).max(0.0),
                    );
                });
                // Our own scroll (`scroll_to`): not the user scrolling.
                let expected = expected_position.get_value();
                if expected.is_some_and(|(x, y)| (x - left).abs() < 1.0 && (y - top).abs() < 1.0) {
                    expected_position.set_value(None);
                    update_visible_rect();
                    return;
                }
            } else {
                // An ancestor (or the window) scrolled: the view's offset in the viewport.
                let bounds = view.get_bounding_client_rect();
                let x = if bounds.x() < 0.0 { -bounds.x() } else { 0.0 };
                let y = if bounds.y() < 0.0 { -bounds.y() } else { 0.0 };
                if state.with_value(|s| s.viewport_offset == (x, y)) {
                    return;
                }
                state.update_value(|s| s.viewport_offset = (x, y));
            }
            if target == *view_node {
                user_scrolls.update_value(|count| *count += 1);
            }
            update_visible_rect();
            if !is_scrolling.get_untracked() {
                is_scrolling.set(true);
                if let Some(on_scroll_start) = on_scroll_start {
                    on_scroll_start.run(());
                }
            }
            // Reschedule the end only when it comes close (not on every event).
            let now = js_sys::Date::now();
            if scroll_end_time.get_value() <= now + 50.0 {
                scroll_end_time.set_value(now + 300.0);
                if let Ok(mut timeout) = scroll_timeout_handle.lock() {
                    if let Some(handle) = timeout.take() {
                        handle.clear();
                    }
                    *timeout = set_timeout_with_handle(
                        move || {
                            if is_scrolling.try_set(false).is_none()
                                && let Some(on_scroll_end) = on_scroll_end
                            {
                                let _ = on_scroll_end.try_run(());
                            }
                        },
                        Duration::from_millis(300),
                    )
                    .ok();
                }
            }
        };
        let listener =
            StoredValue::new(None::<SendWrapper<crate::utils::event_listeners::Listener>>);
        Effect::new(move |_| {
            if element.get().is_none() {
                return;
            }
            let Some(document) = leptos_use::use_document().as_ref().cloned() else {
                return;
            };
            listener.set_value(Some(SendWrapper::new(listen_to(
                &document,
                ev::scroll,
                true,
                on_scroll.clone(),
            ))));
            update_size();
        });

        // The window's size bounds the visible rectangle.
        let resize_listener =
            StoredValue::new(None::<SendWrapper<crate::utils::event_listeners::Listener>>);
        if let Some(window) = leptos_use::use_window().as_ref().cloned() {
            resize_listener.set_value(Some(SendWrapper::new(listen_to(
                &window,
                ev::resize,
                false,
                move |_: web_sys::UiEvent| update_size(),
            ))));
        }

        // The view's border box (not its content box: scrollbars appearing would loop).
        let _ = leptos_use::use_resize_observer_with_options(
            Signal::derive(move || element.get().map(|el| (*el).clone())),
            move |_, _| update_size(),
            leptos_use::UseResizeObserverOptions::default()
                .box_(web_sys::ResizeObserverBoxOptions::BorderBox),
        );

        // When the content size changes, scrollbars may appear or disappear.
        Effect::new(move |previous: Option<Size>| {
            let size = content_size.get();
            if previous.is_some_and(|previous| previous != size) {
                queue_microtask(update_size);
            }
            size
        });

        on_cleanup(move || {
            if let Ok(mut timeout) = scroll_timeout.lock()
                && let Some(handle) = timeout.take()
            {
                handle.clear();
            }
            listener.set_value(None);
            resize_listener.set_value(None);
        });
    }
    #[cfg(feature = "ssr")]
    let _ = (
        on_size_change,
        on_scroll_start,
        on_scroll_end,
        update_visible_rect,
    );

    // The overflow per axis: without horizontal overflow, no horizontal scrollbar flickers
    // while resizing.
    let overflow = move || -> (&'static str, &'static str) {
        let content = content_size.get();
        let size = state.try_get_value().map(|s| s.size).unwrap_or_default();
        match scroll_direction.get() {
            ScrollDirection::Horizontal => ("auto", "hidden"),
            ScrollDirection::Vertical => ("hidden", "auto"),
            ScrollDirection::Both if content.width == size.width => ("hidden", "auto"),
            ScrollDirection::Both => ("auto", "auto"),
        }
    };
    // The layout pads; padding would offset the absolute positions.
    let scroll_view_styles = Styles::new()
        .add_unchecked("padding", "0")
        .add_optional_unchecked("overflow-x", move || Some(overflow().0))
        .add_optional_unchecked("overflow-y", move || Some(overflow().1));
    let px = |value: f64| value.is_finite().then(|| format!("{value}px"));
    let content_styles = Styles::new()
        .add_unchecked("position", "relative")
        .add_optional_unchecked("pointer-events", move || {
            Some(if is_scrolling.get() { "none" } else { "auto" })
        })
        .add_optional_unchecked("width", move || px(content_size.get().width))
        .add_optional_unchecked("height", move || px(content_size.get().height));

    let direction = use_direction();
    let scroll_to = Callback::new(move |rect: crate::hooks::collections::Rect| {
        // Once the content box has its new size (else the browser clamps the position).
        let requested_after = user_scrolls.try_get_value();
        request_animation_frame(move || {
            if user_scrolls.try_get_value() != requested_after {
                return;
            }
            // Disposed meanwhile (the element capture belongs to the same owner).
            let Some(offset) = state.try_get_value().map(|s| s.viewport_offset) else {
                return;
            };
            let Some(view) = element.get_untracked() else {
                return;
            };
            // The visible rectangle includes the view's offset in the window (window scrolling):
            // the view's own scroll position is without it.
            let (x, y) = ((rect.x - offset.0).max(0.0), (rect.y - offset.1).max(0.0));
            let before = (view.scroll_left().abs(), view.scroll_top());
            if (before.0 - x).abs() < 1.0 && (before.1 - y).abs() < 1.0 {
                return;
            }
            // Right-to-left: the specified negative offsets from the right edge.
            let rtl = direction.get_untracked() == WritingDirection::Rtl;
            view.set_scroll_left(if rtl { -x } else { x });
            view.set_scroll_top(y);
            // The browser may clamp the position: expect the scroll event of where it went (none
            // if it didn't move).
            let after = (view.scroll_left().abs(), view.scroll_top());
            let moved = (after.0 - before.0).abs() >= 1.0 || (after.1 - before.1).abs() >= 1.0;
            let _ = expected_position.try_set_value(moved.then_some(after));
        });
    });

    UseScrollViewReturn {
        scroll_view_styles,
        content_styles,
        is_scrolling: is_scrolling.into(),
        scroll_to,
    }
}
