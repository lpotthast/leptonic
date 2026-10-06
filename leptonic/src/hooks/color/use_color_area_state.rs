// Upstream: react-stately/src/color/useColorAreaState.ts @ 99e6102368
use leptos::prelude::*;

use crate::utils::{
    ValueBinding,
    color::ColorValue,
    math::{decimal_precision, snap_value_to_step},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the color type (`ColorValue`), whose channels are typed per color space; the
//   color space is the type (react-aria: a `colorSpace` prop converting the value).
// - Hook-owned value (C4): `default_value` + `on_change`, or `value` bound to app state.
// - A `Copy` struct with signals and methods (C3).
// - `x_channel_step`/`y_channel_step` override the channels' steps (leptonic addition).
//
// =============================================================================

/// Input of [`use_color_area_state`]. Start from [`UseColorAreaStateInput::new`].
#[derive(Debug, Clone, Copy)]
pub struct UseColorAreaStateInput<C: ColorValue> {
    /// The initial color.
    pub default_value: C,
    /// The color as app state, replacing `default_value`.
    pub value: Option<ValueBinding<C>>,
    /// The channel on the horizontal axis. Default: the color space's first axis.
    pub x_channel: Option<C::Channel>,
    /// The channel on the vertical axis. Default: the color space's second axis.
    pub y_channel: Option<C::Channel>,
    /// The horizontal step. Default: the x channel's step.
    pub x_channel_step: Option<f64>,
    /// The vertical step. Default: the y channel's step.
    pub y_channel_step: Option<f64>,
    /// Called with the color whenever it changes, also while dragging.
    pub on_change: Option<Callback<C>>,
    /// Called with the color when the user stops dragging (or after a keyboard change).
    pub on_change_end: Option<Callback<C>>,
}

impl<C: ColorValue> UseColorAreaStateInput<C> {
    /// An area starting at `default_value`, with the color space's default axes.
    pub fn new(default_value: C) -> Self {
        Self {
            default_value,
            value: None,
            x_channel: None,
            y_channel: None,
            x_channel_step: None,
            y_channel_step: None,
            on_change: None,
            on_change_end: None,
        }
    }
}

/// The state of a 2D color area: the color, and the two channels its axes change.
#[derive(Debug, Clone, Copy)]
pub struct ColorAreaState<C: ColorValue> {
    /// The color.
    pub value: Signal<C>,
    /// The x channel's value.
    pub x_value: Signal<f64>,
    /// The y channel's value.
    pub y_value: Signal<f64>,
    /// The channel on the horizontal axis.
    pub x_channel: C::Channel,
    /// The channel on the vertical axis.
    pub y_channel: C::Channel,
    /// The third channel, fixed in the area.
    pub z_channel: C::Channel,
    /// The horizontal step.
    pub x_channel_step: f64,
    /// The vertical step.
    pub y_channel_step: f64,
    /// The horizontal page step (Home/End).
    pub x_channel_page_step: f64,
    /// The vertical page step (PageUp/PageDown).
    pub y_channel_page_step: f64,
    /// Whether the thumb is being dragged.
    pub is_dragging: Signal<bool>,
    binding: ValueBinding<C>,
    default_value: StoredValue<C>,
    /// The color as last set: dragging sets several times before the binding has updated.
    latest: StoredValue<C>,
    dragging: RwSignal<bool>,
    on_change_end: Option<Callback<C>>,
}

impl<C: ColorValue> ColorAreaState<C> {
    /// The color the area started with (for form resets).
    pub fn default_value(&self) -> C {
        self.default_value.get_value()
    }

    /// Sets the color.
    pub fn set_value(&self, color: C) {
        if color != self.latest.get_value() {
            self.latest.set_value(color);
            self.binding.set(color);
        }
    }

    /// Sets the x channel's value.
    pub fn set_x_value(&self, value: f64) {
        let color = self.latest.get_value();
        if value != color.get_channel_value(self.x_channel) {
            self.set_value(color.with_channel_value(self.x_channel, value));
        }
    }

    /// Sets the y channel's value.
    pub fn set_y_value(&self, value: f64) {
        let color = self.latest.get_value();
        if value != color.get_channel_value(self.y_channel) {
            self.set_value(color.with_channel_value(self.y_channel, value));
        }
    }

    /// Sets the color from a point of the area, each coordinate from 0 to 1 (`y` from the top).
    /// The values snap to the steps.
    pub fn set_color_from_point(&self, x: f64, y: f64) {
        let color = self.latest.get_value();
        let x_range = C::get_channel_range(self.x_channel);
        let y_range = C::get_channel_range(self.y_channel);
        let new_x = x_range.min_value + x.clamp(0.0, 1.0) * (x_range.max_value - x_range.min_value);
        let new_y =
            y_range.min_value + (1.0 - y.clamp(0.0, 1.0)) * (y_range.max_value - y_range.min_value);
        let mut new_color = None;
        if new_x != color.get_channel_value(self.x_channel) {
            let snapped = snap(new_x, self.x_channel_step, &x_range);
            new_color = Some(color.with_channel_value(self.x_channel, snapped));
        }
        if new_y != color.get_channel_value(self.y_channel) {
            let snapped = snap(new_y, self.y_channel_step, &y_range);
            new_color = Some(
                new_color
                    .unwrap_or(color)
                    .with_channel_value(self.y_channel, snapped),
            );
        }
        if let Some(new_color) = new_color {
            self.set_value(new_color);
        }
    }

    /// The thumb's position, each coordinate from 0 to 1 (`y` from the top). Tracked.
    pub fn thumb_position(&self) -> (f64, f64) {
        let x_range = C::get_channel_range(self.x_channel);
        let y_range = C::get_channel_range(self.y_channel);
        (
            (self.x_value.get() - x_range.min_value) / (x_range.max_value - x_range.min_value),
            1.0 - (self.y_value.get() - y_range.min_value)
                / (y_range.max_value - y_range.min_value),
        )
    }

    /// Increases the x channel by `step` (e.g. the page step).
    pub fn increment_x(&self, step: f64) {
        self.step_channel(self.x_channel, self.x_channel_step, step);
    }

    /// Decreases the x channel by `step`.
    pub fn decrement_x(&self, step: f64) {
        self.step_channel(self.x_channel, self.x_channel_step, -step);
    }

    /// Increases the y channel by `step`.
    pub fn increment_y(&self, step: f64) {
        self.step_channel(self.y_channel, self.y_channel_step, step);
    }

    /// Decreases the y channel by `step`.
    pub fn decrement_y(&self, step: f64) {
        self.step_channel(self.y_channel, self.y_channel_step, -step);
    }

    /// Adds `delta` to `channel`, snapped to the channel's `step` (react-aria: an increment past
    /// the maximum lands on the maximum, which may be off the step grid).
    fn step_channel(&self, channel: C::Channel, step: f64, delta: f64) {
        let color = self.latest.get_value();
        let range = C::get_channel_range(channel);
        let target = color.get_channel_value(channel) + delta;
        let value = if delta > 0.0 && target > range.max_value {
            range.max_value
        } else {
            snap(target, step, &range)
        };
        if value != color.get_channel_value(channel) {
            self.set_value(color.with_channel_value(channel, value));
        }
    }

    /// Starts or ends dragging; ending it calls `on_change_end`.
    pub fn set_dragging(&self, dragging: bool) {
        let was_dragging = self.dragging.get_untracked();
        self.dragging.set(dragging);
        if was_dragging
            && !dragging
            && let Some(on_change_end) = self.on_change_end
        {
            on_change_end.run(self.latest.get_value());
        }
    }

    /// The color to show (react-aria: without alpha; our color spaces have none).
    pub fn display_color(&self) -> Signal<C> {
        self.value
    }
}

fn snap(value: f64, step: f64, range: &crate::utils::color::ColorChannelRange) -> f64 {
    snap_value_to_step(
        value,
        range.min_value,
        range.max_value,
        step,
        decimal_precision(step),
    )
}

/// Creates the state of a 2D color area.
pub fn use_color_area_state<C: ColorValue>(input: UseColorAreaStateInput<C>) -> ColorAreaState<C> {
    let UseColorAreaStateInput {
        default_value,
        value,
        x_channel,
        y_channel,
        x_channel_step,
        y_channel_step,
        on_change,
        on_change_end,
    } = input;

    let (x_channel, y_channel, z_channel) = C::get_color_space_axes(x_channel, y_channel);
    let x_range = C::get_channel_range(x_channel);
    let y_range = C::get_channel_range(y_channel);

    let binding = value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_value)));
    // With app state, the color it holds at first is the one to reset to (react-aria).
    let default_value = StoredValue::new(binding.value.get_untracked());
    let latest = StoredValue::new(binding.value.get_untracked());
    // The binding's value may also change from outside.
    let bound = binding.value;
    Effect::new(move || latest.set_value(bound.get()));
    let binding = ValueBinding::new(
        binding.value,
        Callback::new(move |color: C| {
            binding.set(color);
            if let Some(on_change) = on_change {
                on_change.run(color);
            }
        }),
    );

    let dragging = RwSignal::new(false);
    let value = binding.value;
    ColorAreaState {
        value,
        x_value: Signal::derive(move || value.get().get_channel_value(x_channel)),
        y_value: Signal::derive(move || value.get().get_channel_value(y_channel)),
        x_channel,
        y_channel,
        z_channel,
        x_channel_step: x_channel_step.unwrap_or(x_range.step),
        y_channel_step: y_channel_step.unwrap_or(y_range.step),
        x_channel_page_step: x_range.page_size,
        y_channel_page_step: y_range.page_size,
        is_dragging: dragging.into(),
        binding,
        default_value,
        latest,
        dragging,
        on_change_end,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::utils::color::{HSV, HsvChannel};

    fn state(color: HSV) -> ColorAreaState<HSV> {
        use_color_area_state(UseColorAreaStateInput {
            x_channel: Some(HsvChannel::Saturation),
            y_channel: Some(HsvChannel::Brightness),
            ..UseColorAreaStateInput::new(color)
        })
    }

    #[test]
    fn steps_snap_to_the_channel_step_and_stop_at_the_bounds() {
        Owner::new().with(|| {
            let area = state(HSV {
                hue: 0.0,
                saturation: 0.5,
                value: 0.5,
            });
            area.increment_x(area.x_channel_page_step);
            assert_that!(area.x_value.get_untracked()).is_equal_to(0.6);
            area.decrement_y(area.y_channel_step);
            assert_that!(area.y_value.get_untracked()).is_equal_to(0.49);
            for _ in 0..10 {
                area.increment_x(area.x_channel_page_step);
            }
            assert_that!(area.x_value.get_untracked()).is_equal_to(1.0);
        });
    }

    #[test]
    fn a_point_sets_both_channels_with_y_from_the_top() {
        Owner::new().with(|| {
            let area = state(HSV {
                hue: 0.0,
                saturation: 0.0,
                value: 0.0,
            });
            area.set_color_from_point(0.25, 0.25);
            assert_that!(area.x_value.get_untracked()).is_equal_to(0.25);
            assert_that!(area.y_value.get_untracked()).is_equal_to(0.75);
            assert_that!(area.thumb_position()).is_equal_to((0.25, 0.25));
        });
    }

    #[test]
    fn dragging_ends_with_on_change_end() {
        Owner::new().with(|| {
            let ends = RwSignal::new(Vec::new());
            let area = use_color_area_state(UseColorAreaStateInput {
                on_change_end: Some(Callback::new(move |c: HSV| ends.update(|e| e.push(c)))),
                ..UseColorAreaStateInput::new(HSV {
                    hue: 0.0,
                    saturation: 0.5,
                    value: 0.5,
                })
            });
            area.set_dragging(true);
            area.set_color_from_point(1.0, 0.0);
            area.set_dragging(false);
            assert_that!(ends.get_untracked().len()).is_equal_to(1);
        });
    }
}
