use std::fmt;

use leptos::prelude::*;

use crate::utils::{
    color::{ColorChannelRange, ColorValue},
    math::{decimal_precision, snap_value_to_step},
};

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/color/src/useColorAreaState.ts

// ## INTENTIONAL DEVIATIONS
//
// - Hook-owned state: The hook owns its WriteSignal internally and exposes a
//   read-only Signal<C>. React-aria uses useControlledState for
//   controlled/uncontrolled support. Callers must use the hook's mutation
//   callbacks.

/// Input parameters for `use_color_area_state`.
#[derive(Debug, Clone, Copy)]
pub struct UseColorAreaStateInput<C: ColorValue> {
    /// The initial color value.
    pub default_value: C,

    /// Which channel maps to the X axis.
    pub x_channel: C::Channel,

    /// Which channel maps to the Y axis.
    pub y_channel: C::Channel,

    /// Optional override for the X channel step.
    pub x_channel_step: Option<f64>,

    /// Optional override for the Y channel step.
    pub y_channel_step: Option<f64>,

    /// Callback fired when the color changes during interaction.
    pub on_change: Option<Callback<C>>,

    /// Callback fired when interaction ends (e.g. drag release).
    pub on_change_end: Option<Callback<C>>,
}

/// Return value of `use_color_area_state`.
pub struct UseColorAreaStateReturn<C: ColorValue> {
    /// The current color (read-only).
    pub value: Signal<C>,

    /// The X-axis channel value.
    pub x_value: Signal<f64>,

    /// The Y-axis channel value.
    pub y_value: Signal<f64>,

    /// Which channel is on the X axis.
    pub x_channel: C::Channel,

    /// Which channel is on the Y axis.
    pub y_channel: C::Channel,

    /// The remaining (Z) channel not on either axis.
    pub z_channel: C::Channel,

    /// Whether the user is currently dragging.
    pub is_dragging: Signal<bool>,

    /// Set the dragging state. Fires `on_change_end` on false transition.
    pub set_dragging: Callback<bool>,

    /// Update the color from a normalized point (0.0–1.0 for each axis).
    /// Y is inverted: 0.0 = top = max, 1.0 = bottom = min.
    pub set_color_from_point: Callback<(f64, f64)>,

    /// The current thumb position as normalized coordinates (0.0–1.0).
    /// Derived from the current channel values.
    pub thumb_position: Signal<(f64, f64)>,

    /// Update the full color value (for external sources like number inputs).
    pub set_value: Callback<C>,

    /// Increment the X channel by the given step (or default step if None).
    pub increment_x: Callback<Option<f64>>,

    /// Decrement the X channel by the given step (or default step if None).
    pub decrement_x: Callback<Option<f64>>,

    /// Increment the Y channel by the given step (or default step if None).
    pub increment_y: Callback<Option<f64>>,

    /// Decrement the Y channel by the given step (or default step if None).
    pub decrement_y: Callback<Option<f64>>,

    /// The step size for the X channel.
    pub x_channel_step: f64,

    /// The step size for the Y channel.
    pub y_channel_step: f64,

    /// The page step size for the X channel.
    pub x_channel_page_step: f64,

    /// The page step size for the Y channel.
    pub y_channel_page_step: f64,

    /// Set the X channel to a specific value. Fires `on_change`.
    pub set_x_value: Callback<f64>,

    /// Set the Y channel to a specific value. Fires `on_change`.
    pub set_y_value: Callback<f64>,

    /// The display color (same as value for non-alpha color spaces).
    pub display_color: Signal<C>,
}

impl<C: ColorValue> Clone for UseColorAreaStateReturn<C> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            x_value: self.x_value,
            y_value: self.y_value,
            x_channel: self.x_channel,
            y_channel: self.y_channel,
            z_channel: self.z_channel,
            is_dragging: self.is_dragging,
            set_dragging: self.set_dragging,
            set_color_from_point: self.set_color_from_point,
            thumb_position: self.thumb_position,
            set_value: self.set_value,
            increment_x: self.increment_x,
            decrement_x: self.decrement_x,
            increment_y: self.increment_y,
            decrement_y: self.decrement_y,
            x_channel_step: self.x_channel_step,
            y_channel_step: self.y_channel_step,
            x_channel_page_step: self.x_channel_page_step,
            y_channel_page_step: self.y_channel_page_step,
            set_x_value: self.set_x_value,
            set_y_value: self.set_y_value,
            display_color: self.display_color,
        }
    }
}

impl<C: ColorValue> fmt::Debug for UseColorAreaStateReturn<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseColorAreaStateReturn")
            .field("x_channel", &self.x_channel)
            .field("y_channel", &self.y_channel)
            .field("z_channel", &self.z_channel)
            .finish_non_exhaustive()
    }
}

/// Snaps a channel value to its step boundaries.
fn snap_channel_value<C: ColorValue>(channel: C::Channel, value: f64, step: f64) -> f64 {
    let range = C::get_channel_range(channel);
    let precision = decimal_precision(step);
    snap_value_to_step(value, range.min_value, range.max_value, step, precision)
}

/// Converts a normalized coordinate (0.0–1.0) to a channel value.
fn normalized_to_channel_value(normalized: f64, range: &ColorChannelRange) -> f64 {
    range.min_value + normalized * (range.max_value - range.min_value)
}

/// Converts a channel value to a normalized coordinate (0.0–1.0).
fn channel_value_to_normalized(value: f64, range: &ColorChannelRange) -> f64 {
    let span = range.max_value - range.min_value;
    if span == 0.0 {
        0.0
    } else {
        (value - range.min_value) / span
    }
}

/// Creates state for a 2D color area component.
///
/// The color area lets users adjust two channels of a color simultaneously
/// by dragging a thumb within a 2D gradient area.
#[allow(clippy::too_many_lines)]
pub fn use_color_area_state<C: ColorValue>(
    input: UseColorAreaStateInput<C>,
) -> UseColorAreaStateReturn<C> {
    let UseColorAreaStateInput {
        default_value,
        x_channel,
        y_channel,
        x_channel_step: x_step_override,
        y_channel_step: y_step_override,
        on_change,
        on_change_end,
    } = input;

    // Determine the z channel (the one not on either axis).
    let (_, _, z_channel) = C::get_color_space_axes(Some(x_channel), Some(y_channel));

    // Get channel ranges and steps.
    let x_range = C::get_channel_range(x_channel);
    let y_range = C::get_channel_range(y_channel);
    let x_step = x_step_override.unwrap_or(x_range.step);
    let y_step = y_step_override.unwrap_or(y_range.step);

    // Hook-owned state.
    let (value, set_value_signal) = signal(default_value);
    let (is_dragging, set_is_dragging) = signal(false);

    // Derived channel value signals.
    let x_value = Signal::derive(move || value.get().get_channel_value(x_channel));
    let y_value = Signal::derive(move || value.get().get_channel_value(y_channel));

    // Thumb position as normalized coordinates.
    // Y is inverted: 0.0 = top = max_value, 1.0 = bottom = min_value.
    let thumb_position = Signal::derive(move || {
        let x_norm = channel_value_to_normalized(x_value.get(), &x_range);
        let y_norm = 1.0 - channel_value_to_normalized(y_value.get(), &y_range);
        (x_norm, y_norm)
    });

    // Display color (same as value for non-alpha spaces).
    let display_color = Signal::derive(move || value.get());

    // Internal: update color value and fire callbacks.
    // Skips no-op updates to prevent reactive loops.
    let update_color = move |new_color: C| {
        if new_color == value.get_untracked() {
            return;
        }
        set_value_signal.set(new_color);
        if let Some(cb) = on_change {
            cb.run(new_color);
        }
    };

    // Set color from normalized point.
    let set_color_from_point = Callback::new(move |(norm_x, norm_y): (f64, f64)| {
        let norm_x = norm_x.clamp(0.0, 1.0);
        let norm_y = norm_y.clamp(0.0, 1.0);

        let x_val = normalized_to_channel_value(norm_x, &x_range);
        // Y is inverted.
        let y_val = normalized_to_channel_value(1.0 - norm_y, &y_range);

        let x_snapped = snap_channel_value::<C>(x_channel, x_val, x_step);
        let y_snapped = snap_channel_value::<C>(y_channel, y_val, y_step);

        let current = value.get_untracked();
        let new_color = current
            .with_channel_value(x_channel, x_snapped)
            .with_channel_value(y_channel, y_snapped);
        update_color(new_color);
    });

    // Set full value (for external sources).
    // Does NOT fire on_change to prevent reactive loops when syncing
    // with an external signal that also listens to on_change.
    // Skips no-op updates.
    let set_value = Callback::new(move |new_color: C| {
        if new_color != value.get_untracked() {
            set_value_signal.set(new_color);
        }
    });

    // Set dragging with on_change_end support.
    let set_dragging = Callback::new(move |dragging: bool| {
        let was_dragging = is_dragging.get_untracked();
        set_is_dragging.set(dragging);
        if was_dragging && !dragging {
            if let Some(cb) = on_change_end {
                cb.run(value.get_untracked());
            }
        }
    });

    // Increment/decrement helpers.
    let make_adjuster =
        move |channel: C::Channel, step: f64, direction: f64| -> Callback<Option<f64>> {
            let range = C::get_channel_range(channel);
            Callback::new(move |custom_step: Option<f64>| {
                let s = custom_step.unwrap_or(step);
                let current = value.get_untracked();
                let current_val = current.get_channel_value(channel);
                let new_val = (current_val + s * direction).clamp(range.min_value, range.max_value);
                let snapped = snap_channel_value::<C>(channel, new_val, s);
                let new_color = current.with_channel_value(channel, snapped);
                update_color(new_color);
            })
        };

    let increment_x = make_adjuster(x_channel, x_step, 1.0);
    let decrement_x = make_adjuster(x_channel, x_step, -1.0);
    let increment_y = make_adjuster(y_channel, y_step, 1.0);
    let decrement_y = make_adjuster(y_channel, y_step, -1.0);

    // Direct channel value setters (for hidden range input onChange).
    let set_x_value = Callback::new(move |new_val: f64| {
        let snapped = snap_channel_value::<C>(x_channel, new_val, x_step);
        let new_color = value.get_untracked().with_channel_value(x_channel, snapped);
        update_color(new_color);
    });
    let set_y_value = Callback::new(move |new_val: f64| {
        let snapped = snap_channel_value::<C>(y_channel, new_val, y_step);
        let new_color = value.get_untracked().with_channel_value(y_channel, snapped);
        update_color(new_color);
    });

    UseColorAreaStateReturn {
        value: value.into(),
        x_value,
        y_value,
        x_channel,
        y_channel,
        z_channel,
        is_dragging: is_dragging.into(),
        set_dragging,
        set_color_from_point,
        thumb_position,
        set_value,
        increment_x,
        decrement_x,
        increment_y,
        decrement_y,
        x_channel_step: x_step,
        y_channel_step: y_step,
        x_channel_page_step: x_range.page_size,
        y_channel_page_step: y_range.page_size,
        set_x_value,
        set_y_value,
        display_color,
    }
}
