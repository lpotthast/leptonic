//! # Slider Hooks
//!
//! Accessible slider components with multi-thumb support, based on
//! [react-aria's slider](https://react-spectrum.adobe.com/react-aria/useSlider.html).
//!
//! ## Architecture
//!
//! Following react-aria's pattern, sliders use N+1 movement handler instances for N thumbs:
//! - **Track-level (`use_slider`):** One movement handler handles track clicks,
//!   finds closest thumb via `find_closest_thumb()`, and drags that thumb.
//! - **Per-thumb (`use_slider_thumb`):** Each thumb has its own movement handler
//!   for direct thumb-initiated drags.
//!
//! Both share the same track element as container. This allows:
//! - Track clicks to move the closest thumb with continuous drag support
//! - Direct thumb drags to work independently
//! - Clean separation without thumbs needing to know about each other
//!
//! ## Pixel Accumulation Pattern
//!
//! To avoid precision loss during continuous dragging, we use pixel accumulation:
//! - On drag start: Initialize `current_position` to `thumb_percent * track_size` (in pixels)
//! - On move: Accumulate pixel deltas into `current_position`
//! - Convert back to percent: `percent = current_position / track_size`
//!
//! This is critical because `set_thumb_percent` snaps values to steps. If we naively
//! added `delta_percent` to the current (snapped) `thumb_percent`, small movements would
//! be lost. By tracking position in floating-point pixels, we preserve sub-pixel
//! movements until they accumulate enough to cross a step boundary.
//!
//! ## Deviations from react-aria
//!
//! ### Input Type
//! React-aria uses `type="range"` for the hidden input, which provides better
//! screen reader semantics. We use `type="hidden"` for simpler form integration
//! since the thumb element already has the slider role and ARIA attributes.
//!
//! ### Number Formatting
//! React-aria uses `Intl.NumberFormat` for locale-specific number display.
//! We use basic string formatting with configurable decimal places.
//! This avoids `wasm_bindgen` complexity for internationalization.

use crate::hooks::slider::{SliderOrientation, UseSliderStateReturn};
use crate::hooks::{
    interactions::use_move::MoveAxis, use_move, MoveEndEvent, MoveEvent, MoveStartEvent,
    UseMoveInput,
};
use crate::utils::aria::{AriaDisabled, AriaLive};
use crate::utils::element_capture::{CapturedElement, ElementCaptureAttr};
use crate::utils::{EventHandler, EventTargetExt};
use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use leptos::tachys::html::style::{style, Style};
use leptos_use::use_event_listener;
use uuid::Uuid;
use web_sys::PointerEvent;

/// Input parameters for the `use_slider` hook.
#[derive(Clone)]
pub struct UseSliderInput {
    /// The slider state (from `use_slider_state`). Also provides the `disabled` info.
    pub state: UseSliderStateReturn,

    /// An accessibility label for the slider group.
    pub aria_label: Option<&'static str>,

    /// ID of an element that labels the slider group.
    pub aria_labelledby: Option<String>,

    /// Whether to use RTL layout (reverses arrow key direction).
    pub is_rtl: bool,
}

/// The return value of the `use_slider` hook.
#[derive(Clone)]
pub struct UseSliderReturn {
    /// Props for the slider group/container element.
    pub group_props: UseSliderGroupProps,

    /// Props for the label element.
    pub label_props: UseSliderLabelProps,

    /// Props for the output/value display element.
    pub output_props: UseSliderOutputProps,

    /// Props for the slider track element.
    /// Includes an `ElementCaptureAttr` that captures the DOM element.
    pub track_props: UseSliderTrackProps,

    /// Reactive handle to the captured track element.
    /// Pass this to `use_slider_thumb`.
    pub track_ref: CapturedElement,
}

#[derive(Debug, Clone)]
pub struct UseSliderGroupProps {
    role: &'static str,
    id: String,
    aria_label: Option<&'static str>,
    aria_labelledby: Option<String>,
    aria_disabled: Signal<Option<AriaDisabled>>,
}

impl UseSliderGroupProps {
    pub fn into_attrs(self) -> UseSliderGroupAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::Id, self.id),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDisabled, self.aria_disabled),
        )
    }
}

/// Attributes for the slider group element.
pub type UseSliderGroupAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::Id, String>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
);

#[derive(Debug, Clone)]
pub struct UseSliderTrackProps {
    role: &'static str,
    style_touch_action: &'static str,
    on_pointerdown: EventHandler<PointerEvent>,
    element_capture: ElementCaptureAttr,
}

impl UseSliderTrackProps {
    pub fn into_attrs(self) -> UseSliderTrackAttrs {
        (
            Attr(attr::Role, self.role),
            style(("touch-action", self.style_touch_action)),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.element_capture,
        )
    }
}

/// Attributes for the slider track element.
pub type UseSliderTrackAttrs = (
    Attr<attr::Role, &'static str>,
    Style<(&'static str, &'static str)>, // touch-action: none
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    ElementCaptureAttr,
);

/// Props for the label element.
#[derive(Debug, Clone)]
pub struct UseSliderLabelProps {
    /// The id of the label element.
    pub id: String,
}

/// Props for the output/value display element.
#[derive(Debug, Clone)]
pub struct UseSliderOutputProps {
    /// The id of the output element.
    pub id: String,

    /// The "for" attribute linking to all thumb IDs (space-separated).
    pub html_for: String,

    /// The aria-live value for the output element. Use "off" to prevent
    /// screen reader announcements during dragging.
    pub aria_live: AriaLive,
}

impl UseSliderOutputProps {
    pub fn into_attrs(self) -> UseSliderOutputAttrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::For, self.html_for),
            Attr(attr::AriaLive, self.aria_live),
        )
    }
}

pub type UseSliderOutputAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::For, String>,
    Attr<attr::AriaLive, AriaLive>,
);

pub type ThumbIdx = usize;

/// Find the closest thumb to a click position.
fn find_closest_thumb(click_value: f64, values: &[f64]) -> Option<ThumbIdx> {
    if values.is_empty() {
        return None;
    }
    if values.len() == 1 {
        return Some(0);
    }

    // Find position where click_value would be inserted
    match values.iter().position(|&v| click_value < v) {
        Some(0) => Some(0),             // Click is before first thumb.
        None => Some(values.len() - 1), // Click is after last thumb.
        Some(i) => {
            // Click is between thumb i-1 and thumb i, find closer one.
            let prev_dist = (values[i - 1] - click_value).abs();
            let next_dist = (values[i] - click_value).abs();
            if prev_dist <= next_dist {
                Some(i - 1)
            } else {
                Some(i)
            }
        }
    }
}

/// Provides the behavior and accessibility implementation for a slider track.
///
/// This hook manages track-level behavior including track clicks to move thumbs.
/// Use `use_slider_thumb` for individual thumb behavior.
///
/// # Example
///
/// ```ignore
/// let state = use_slider_state(UseSliderStateInput {
///     values: SliderValues::Uncontrolled(vec![50.0]),
///     ..Default::default()
/// });
///
/// let UseSliderReturn { group_props, track_props, track_ref, label_props, output_props } =
///     use_slider(UseSliderInput {
///         state,
///         label: Some("Volume".to_string()),
///         ..Default::default()
///     });
///
/// // Pass track_ref to each thumb
/// let UseSliderThumbReturn { thumb_props, .. } = use_slider_thumb(UseSliderThumbInput {
///     state,
///     track_ref,
///     index: 0,
///     ..Default::default()
/// });
/// ```
///
/// # Panics
///
/// Panics if the current target of the pointer event is not available.
#[allow(clippy::too_many_lines)]
pub fn use_slider(input: UseSliderInput) -> UseSliderReturn {
    let UseSliderInput {
        state,
        aria_label,
        aria_labelledby,
        is_rtl,
    } = input;

    let base_id = Uuid::new_v4();
    let group_id = format!("slider-group-{base_id}");
    let label_id = format!("slider-label-{base_id}");
    let output_id = format!("slider-output-{base_id}");

    let orientation = state.orientation;
    let disabled = state.disabled;

    let track_element = CapturedElement::new();

    // Generate thumb IDs for the output's "for" attribute
    let thumb_ids: Vec<String> = (0..state.num_thumbs)
        .map(|i| format!("slider-thumb-{base_id}-{i}"))
        .collect();
    let html_for = thumb_ids.join(" ");

    // Track-level drag state: which thumb index is being dragged via track interaction
    let dragging_thumb_index: StoredValue<Option<usize>> = StoredValue::new(None);
    // Pixel accumulation: track the thumb's position in pixels (not percent) to avoid
    // precision loss when the percent snaps to steps. See module docs for details.
    let current_position_px: StoredValue<Option<f64>> = StoredValue::new(None);

    // Cleanup functions for the global pointerup/pointercancel listeners registered
    // in on_track_pointerdown. Stored so we can remove them in the handler or on_cleanup.
    let track_cleanup: StoredValue<Option<Box<dyn Fn() + Send + Sync>>, LocalStorage> =
        StoredValue::new_local(None);

    // Handle track clicks immediately on pointerdown (before use_move processes the event).
    // This is the react-aria "onDownTrack" pattern: click-to-position happens here,
    // while use_move only handles subsequent drag deltas.
    let handle_track_pointerdown = move |e: PointerEvent| {
        if disabled.get_untracked() {
            return;
        }

        if let Some(track) = track_element.get_untracked().as_deref().cloned() {
            let rect = track.get_bounding_client_rect();

            // Convert page coordinates to client coordinates relative to track.
            // page_x/page_y include scroll offset, but getBoundingClientRect uses viewport coords.
            let scroll_x = web_sys::window()
                .and_then(|w| w.scroll_x().ok())
                .unwrap_or(0.0);
            let scroll_y = web_sys::window()
                .and_then(|w| w.scroll_y().ok())
                .unwrap_or(0.0);
            let client_x = f64::from(e.page_x()) - scroll_x;
            let client_y = f64::from(e.page_y()) - scroll_y;

            let (position, size): (f64, f64) = match orientation.get_untracked() {
                SliderOrientation::Horizontal => {
                    let pos = client_x - rect.left();
                    let adjusted_pos = if is_rtl { rect.width() - pos } else { pos };
                    (adjusted_pos, rect.width())
                }
                SliderOrientation::Vertical => {
                    // For vertical, bottom = 0%, top = 100%
                    (rect.bottom() - client_y, rect.height())
                }
            };

            let percent = (position / size).clamp(0.0, 1.0);
            let click_value = state.min_value + percent * (state.max_value - state.min_value);

            // Find closest thumb
            let values = state.values.get_untracked();
            let Some(closest_thumb) = find_closest_thumb(click_value, &values) else {
                return;
            };

            // Move the closest thumb to the click position
            state.set_thumb_percent.run((closest_thumb, percent));

            // Focus the thumb that was clicked
            state.set_focused_thumb.run(Some(closest_thumb));

            // Start tracking for continuous drag
            dragging_thumb_index.set_value(Some(closest_thumb));
            state.set_thumb_dragging.run((closest_thumb, true));

            // Initialize pixel position for accumulation pattern.
            // Use the click position (already adjusted for RTL/vertical).
            current_position_px.set_value(Some(position));

            // Register global pointerup/pointercancel listeners for cleanup.
            // These always fire (even without pointer movement), ensuring dragging state
            // is cleared for click-without-drag interactions.
            let pointer_id = e.pointer_id();
            let doc = e.current_target().unwrap().get_owner_document();

            let cleanup_up =
                use_event_listener(doc.clone(), ev::pointerup, move |e: PointerEvent| {
                    if e.pointer_id() != pointer_id {
                        return;
                    }
                    if let Some(idx) = dragging_thumb_index.get_value() {
                        state.set_thumb_dragging.run((idx, false));
                    }
                    dragging_thumb_index.set_value(None);
                    current_position_px.set_value(None);
                    // Remove our global listeners
                    track_cleanup.update_value(|c| {
                        if let Some(cleanup_fn) = c.take() {
                            cleanup_fn();
                        }
                    });
                });

            let cleanup_cancel =
                use_event_listener(doc, ev::pointercancel, move |e: PointerEvent| {
                    if e.pointer_id() != pointer_id {
                        return;
                    }
                    if let Some(idx) = dragging_thumb_index.get_value() {
                        state.set_thumb_dragging.run((idx, false));
                    }
                    dragging_thumb_index.set_value(None);
                    current_position_px.set_value(None);
                    // Remove our global listeners
                    track_cleanup.update_value(|c| {
                        if let Some(cleanup_fn) = c.take() {
                            cleanup_fn();
                        }
                    });
                });

            track_cleanup.set_value(Some(Box::new(move || {
                cleanup_up();
                cleanup_cancel();
            })));
        }
    };

    // Use use_move hook for track-level dragging. use_move handles pointer event
    // management (pointerdown, pointermove, pointerup, pointercancel) for drag deltas.
    // on_move_start is a no-op because we handle the initial click in on_track_pointerdown.
    let track_move_props = use_move(UseMoveInput {
        axis: Signal::derive(move || match orientation.get() {
            SliderOrientation::Horizontal => Some(MoveAxis::Horizontal),
            SliderOrientation::Vertical => Some(MoveAxis::Vertical),
        }),
        on_move_start: Callback::new(move |_: MoveStartEvent| {}),
        on_move: Callback::new(move |e: MoveEvent| {
            if let Some(idx) = dragging_thumb_index.get_value() {
                if let Some(track) = track_element.get_untracked().as_deref().cloned() {
                    let orientation = orientation.get_untracked();

                    let rect = track.get_bounding_client_rect();
                    let size = match orientation {
                        SliderOrientation::Horizontal => rect.width(),
                        SliderOrientation::Vertical => rect.height(),
                    };

                    // Get current position in pixels (initialized in on_track_pointerdown)
                    let pos = current_position_px
                        .get_value()
                        .unwrap_or_else(|| state.get_thumb_percent.run(idx) * size);

                    // use_move provides raw deltas: delta_x for horizontal, delta_y for vertical
                    // For vertical sliders, up should increase value (positive delta_y means cursor moved down)
                    let delta = match orientation {
                        SliderOrientation::Horizontal => e.delta_x,
                        SliderOrientation::Vertical => -e.delta_y,
                    };
                    let delta = if is_rtl && orientation == SliderOrientation::Horizontal {
                        -delta
                    } else {
                        delta
                    };

                    // Accumulate in pixels, then convert to percent
                    let new_pos = pos + delta;
                    current_position_px.set_value(Some(new_pos));

                    let new_percent = (new_pos / size).clamp(0.0, 1.0);
                    state.set_thumb_percent.run((idx, new_percent));
                }
            }
        }),
        on_move_end: Callback::new(move |_: MoveEndEvent| {
            // Idempotent: on_track_pointerdown's global pointerup handler may have
            // already cleared this state. Both handlers guard with `if let Some`.
            if let Some(idx) = dragging_thumb_index.get_value() {
                state.set_thumb_dragging.run((idx, false));
            }
            dragging_thumb_index.set_value(None);
            current_position_px.set_value(None);
        }),
    });

    on_cleanup(move || {
        track_cleanup.update_value(|c| {
            if let Some(cleanup_fn) = c.take() {
                cleanup_fn();
            }
        });
    });

    UseSliderReturn {
        group_props: UseSliderGroupProps {
            role: "group",
            id: group_id,
            aria_label,
            aria_labelledby,
            aria_disabled: Signal::derive(move || disabled.get().then_some(AriaDisabled::True)),
        },
        label_props: UseSliderLabelProps { id: label_id },
        output_props: UseSliderOutputProps {
            id: output_id,
            html_for,
            aria_live: AriaLive::Off,
        },
        track_props: UseSliderTrackProps {
            role: "presentation",
            style_touch_action: "none",
            on_pointerdown: EventHandler::new(handle_track_pointerdown)
                .chain(track_move_props.props.on_pointerdown),
            element_capture: track_element.attr(),
        },
        track_ref: track_element,
    }
}
