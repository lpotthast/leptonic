use std::fmt;

use leptos::prelude::*;

use crate::utils::color::ColorValue;

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/color/src/useColorWheelState.ts

// ## INTENTIONAL DEVIATIONS
//
// - Hook-owned state: The hook owns its WriteSignal internally and exposes a
//   read-only Signal<C>. React-aria uses useControlledState.
//
// - Coordinate convention: 0° at 12 o'clock (top), increasing clockwise.
//   React-aria uses 0° at 3 o'clock with `conic-gradient(from 90deg, ...)`.
//   Both are internally consistent; ours is arguably more intuitive for users.

/// Input parameters for `use_color_wheel_state`.
#[derive(Debug, Clone)]
pub struct UseColorWheelStateInput<C: ColorValue> {
    /// The initial color value. Defaults to red (hue=0) at full saturation.
    pub default_value: C,

    /// Which channel this wheel controls (typically the hue channel).
    pub channel: C::Channel,

    /// Whether the wheel is disabled.
    pub disabled: Signal<bool>,

    /// Callback fired when the color changes during interaction.
    pub on_change: Option<Callback<C>>,

    /// Callback fired when interaction ends.
    pub on_change_end: Option<Callback<C>>,
}

/// Return value of `use_color_wheel_state`.
pub struct UseColorWheelStateReturn<C: ColorValue> {
    /// The current full color.
    pub value: Signal<C>,

    /// Set the full color value (for external/programmatic updates).
    /// Does not fire `on_change`.
    pub set_value: Callback<C>,

    /// Which channel this wheel controls.
    pub channel: C::Channel,

    /// The current channel value (e.g. hue 0–360), derived from value.
    pub hue: Signal<f64>,

    /// Set the channel value directly (e.g. hue 0–360).
    pub set_hue: Callback<f64>,

    /// Set the channel value from Cartesian coordinates relative to the wheel center.
    /// Parameters: (x, y, radius) where x/y are relative to center.
    pub set_hue_from_point: Callback<(f64, f64, f64)>,

    /// Get the thumb position as Cartesian coordinates from center.
    /// Parameter: radius. Returns (x, y).
    pub get_thumb_position: Callback<f64, (f64, f64)>,

    /// Increment the channel value by step (or default step).
    pub increment: Callback<Option<f64>>,

    /// Decrement the channel value by step (or default step).
    pub decrement: Callback<Option<f64>>,

    /// Whether the user is dragging.
    pub is_dragging: Signal<bool>,

    /// Set dragging state. Fires `on_change_end` on false transition.
    pub set_dragging: Callback<bool>,

    /// Display color: the color at the current channel value with maximum vividness
    /// for gradient rendering.
    pub display_color: Signal<C>,

    /// The step size for channel value changes.
    pub step: f64,

    /// The page step size for channel value changes.
    pub page_step: f64,

    /// Whether the wheel is disabled.
    pub is_disabled: Signal<bool>,
}

// All fields are Copy (Signal<T>, Callback<T>, and C::Channel are Copy).
// Manual impls avoid the derive macro's incorrect generic bounds.
impl<C: ColorValue> Copy for UseColorWheelStateReturn<C> {}
#[allow(clippy::expl_impl_clone_on_copy)]
impl<C: ColorValue> Clone for UseColorWheelStateReturn<C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: ColorValue> fmt::Debug for UseColorWheelStateReturn<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseColorWheelStateReturn")
            .field("channel", &self.channel)
            .finish_non_exhaustive()
    }
}

/// Wraps a value to [min, max) using the channel range.
fn wrap_value(v: f64, min: f64, max: f64) -> f64 {
    let range = max - min;
    ((v - min) % range + range) % range + min
}

/// Rounds a value to the nearest step.
fn round_to_step(value: f64, step: f64) -> f64 {
    (value / step).round() * step
}

/// Floors a value, but returns `v - 1` if `v` is already an integer.
/// Used for decrement wrapping so that decrementing from the minimum reaches
/// the last valid step position below the maximum.
fn round_down(v: f64) -> f64 {
    let r = v.floor();
    if (r - v).abs() < f64::EPSILON {
        v - 1.0
    } else {
        r
    }
}

/// Converts Cartesian coordinates (relative to center) to an angular channel value.
/// 0° is at the top (12 o'clock), increasing clockwise.
/// The returned value is in the range [0, 360).
fn cartesian_to_angle(x: f64, y: f64) -> f64 {
    // atan2 gives angle from positive X axis, counter-clockwise.
    // We want 0° at top (negative Y), clockwise.
    let angle_rad = f64::atan2(x, -y);
    let angle_deg = angle_rad.to_degrees();
    ((angle_deg % 360.0) + 360.0) % 360.0
}

/// Converts an angular channel value to Cartesian coordinates on a circle of given radius.
/// 0° is at the top (12 o'clock), increasing clockwise.
fn angle_to_cartesian(angle: f64, radius: f64) -> (f64, f64) {
    let angle_rad = (angle - 90.0).to_radians();
    let x = radius * angle_rad.cos();
    let y = radius * angle_rad.sin();
    (x, y)
}

/// Creates state for a circular channel wheel component.
///
/// Typically used for hue wheels, but works with any angular channel of a
/// [`ColorValue`] type.
#[allow(clippy::needless_pass_by_value)]
pub fn use_color_wheel_state<C: ColorValue>(
    input: UseColorWheelStateInput<C>,
) -> UseColorWheelStateReturn<C> {
    let UseColorWheelStateInput {
        default_value,
        channel,
        disabled,
        on_change,
        on_change_end,
    } = input;

    let range = C::get_channel_range(channel);
    let step = range.step;
    let page_step = range.page_size;
    let max_value = range.max_value;
    let min_value = range.min_value;

    let (value, set_value_signal) = signal(default_value);
    let (is_dragging, set_is_dragging) = signal(false);

    let hue = Signal::derive(move || value.get().get_channel_value(channel));

    let update_channel_value = move |new_val: f64| {
        // React-aria guard: if value exceeds max, snap to min so you can always
        // get back to the start of the range.
        let clamped = if new_val > max_value {
            min_value
        } else {
            new_val
        };
        let wrapped = wrap_value(round_to_step(clamped, step), min_value, max_value);
        let current = value.get_untracked();
        // Skip no-op updates (matches react-aria's `if (hue !== v)` guard).
        if (current.get_channel_value(channel) - wrapped).abs() < f64::EPSILON {
            return;
        }
        let new_color = current.with_channel_value(channel, wrapped);
        set_value_signal.set(new_color);
        if let Some(cb) = on_change {
            cb.run(new_color);
        }
    };

    let set_hue = Callback::new(move |h: f64| {
        update_channel_value(h);
    });

    let set_hue_from_point = Callback::new(move |(x, y, _radius): (f64, f64, f64)| {
        // Convert Cartesian to angular value. For hue (0-360), the angle maps 1:1.
        // For channels with different ranges, scale from the [0, 360) angle to the
        // channel range.
        let angle = cartesian_to_angle(x, y);
        let channel_val = min_value + (angle / 360.0) * (max_value - min_value);
        update_channel_value(channel_val);
    });

    let get_thumb_position = Callback::new(move |radius: f64| -> (f64, f64) {
        let channel_val = hue.get_untracked();
        // Map channel value to angle in [0, 360).
        let angle = (channel_val - min_value) / (max_value - min_value) * 360.0;
        angle_to_cartesian(angle, radius)
    });

    let increment = Callback::new(move |custom_step: Option<f64>| {
        let s = f64::max(custom_step.unwrap_or(step), step);
        let current_val = hue.get_untracked();
        let new_val = current_val + s;
        // Wrap to min when reaching or exceeding max.
        if new_val >= max_value {
            update_channel_value(min_value);
        } else {
            update_channel_value(round_to_step(wrap_value(new_val, min_value, max_value), s));
        }
    });

    let decrement = Callback::new(move |custom_step: Option<f64>| {
        let s = f64::max(custom_step.unwrap_or(step), step);
        let current_val = hue.get_untracked();
        if (current_val - min_value).abs() < f64::EPSILON {
            // At min: jump to the last valid step position below max.
            // E.g., for hue step=15: round_down(360/15) * 15 = round_down(24) * 15 = 23 * 15 = 345.
            let range_size = max_value - min_value;
            update_channel_value(min_value + round_down(range_size / s) * s);
        } else {
            update_channel_value(round_to_step(
                wrap_value(current_val - s, min_value, max_value),
                s,
            ));
        }
    });

    let set_dragging = Callback::new(move |dragging: bool| {
        let was_dragging = is_dragging.get_untracked();
        set_is_dragging.set(dragging);
        if was_dragging && !dragging {
            if let Some(cb) = on_change_end {
                cb.run(value.get_untracked());
            }
        }
    });

    // Programmatic set_value: does NOT fire on_change (matches use_color_area_state pattern).
    let set_value = Callback::new(move |new_color: C| {
        if new_color != value.get_untracked() {
            set_value_signal.set(new_color);
        }
    });

    let display_color = Signal::derive(move || value.get().get_display_color(channel));

    UseColorWheelStateReturn {
        value: value.into(),
        set_value,
        channel,
        hue,
        set_hue,
        set_hue_from_point,
        get_thumb_position,
        increment,
        decrement,
        is_dragging: is_dragging.into(),
        set_dragging,
        display_color,
        step,
        page_step,
        is_disabled: disabled,
    }
}
