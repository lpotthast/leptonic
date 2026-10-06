// Upstream: react-stately/src/color/useColorWheelState.ts @ 99e6102368
use leptos::prelude::*;

use crate::utils::{ValueBinding, color::ColorValue};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the color type (`ColorValue`); `channel` names its hue channel (react-aria
//   converts the color to HSL and uses its hue).
// - Hook-owned value (C4): `default_value` + `on_change`, or `value` bound to app state.
// - A `Copy` struct with signals and methods (C3).
//
// =============================================================================

/// Input of [`use_color_wheel_state`]. Start from [`UseColorWheelStateInput::new`].
#[derive(Debug, Clone, Copy)]
pub struct UseColorWheelStateInput<C: ColorValue> {
    /// The initial color.
    pub default_value: C,
    /// The color as app state, replacing `default_value`.
    pub value: Option<ValueBinding<C>>,
    /// The color type's hue channel, which the wheel changes.
    pub channel: C::Channel,
    pub is_disabled: Signal<bool>,
    /// Called with the color whenever it changes, also while dragging.
    pub on_change: Option<Callback<C>>,
    /// Called with the color when the user stops dragging (or after a keyboard change).
    pub on_change_end: Option<Callback<C>>,
}

impl<C: ColorValue> UseColorWheelStateInput<C> {
    /// A wheel changing the hue `channel` of `default_value`.
    pub fn new(default_value: C, channel: C::Channel) -> Self {
        Self {
            default_value,
            value: None,
            channel,
            is_disabled: Signal::stored(false),
            on_change: None,
            on_change_end: None,
        }
    }
}

/// The state of a color wheel: the color and its hue.
#[derive(Debug, Clone, Copy)]
pub struct ColorWheelState<C: ColorValue> {
    /// The color.
    pub value: Signal<C>,
    /// Its hue, in degrees.
    pub hue: Signal<f64>,
    /// The hue channel.
    pub channel: C::Channel,
    /// The hue's step.
    pub step: f64,
    /// The hue's page step (PageUp/PageDown, Shift + arrow keys).
    pub page_step: f64,
    pub is_disabled: Signal<bool>,
    /// Whether the thumb is being dragged.
    pub is_dragging: Signal<bool>,
    binding: ValueBinding<C>,
    default_value: StoredValue<C>,
    latest: StoredValue<C>,
    dragging: RwSignal<bool>,
    on_change_end: Option<Callback<C>>,
}

fn round_to_step(value: f64, step: f64) -> f64 {
    (value / step).round() * step
}

/// The positive remainder of `n / m`.
fn modulo(n: f64, m: f64) -> f64 {
    ((n % m) + m) % m
}

/// `v` rounded down, and one less if it is whole already.
fn round_down(v: f64) -> f64 {
    let r = v.floor();
    if (r - v).abs() < f64::EPSILON {
        v - 1.0
    } else {
        r
    }
}

impl<C: ColorValue> ColorWheelState<C> {
    /// The color the wheel started with (for form resets).
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

    fn current_hue(&self) -> f64 {
        self.latest.get_value().get_channel_value(self.channel)
    }

    /// Sets the hue, snapped to the step (360 wraps around to 0).
    pub fn set_hue(&self, hue: f64) {
        let hue = if hue > 360.0 { 0.0 } else { hue };
        let hue = round_to_step(modulo(hue, 360.0), self.step);
        if hue != self.current_hue() {
            self.set_value(
                self.latest
                    .get_value()
                    .with_channel_value(self.channel, hue),
            );
        }
    }

    /// Sets the hue of the point (`x`, `y`) relative to the wheel's center (`y` down).
    pub fn set_hue_from_point(&self, x: f64, y: f64, radius: f64) {
        let degrees = (y / radius).atan2(x / radius).to_degrees();
        self.set_hue((degrees + 360.0) % 360.0);
    }

    /// The thumb's position relative to the center on a circle of `radius` (0° at 3 o'clock,
    /// clockwise). Tracked.
    pub fn thumb_position(&self, radius: f64) -> (f64, f64) {
        let radians = (360.0 - self.hue.get() + 90.0).to_radians();
        (radians.sin() * radius, radians.cos() * radius)
    }

    /// Increases the hue by `step` (at least the step), wrapping around.
    pub fn increment(&self, step: f64) {
        let step = step.max(self.step);
        let range = C::get_channel_range(self.channel);
        let mut hue = self.current_hue() + step;
        if hue >= range.max_value {
            hue = range.min_value;
        }
        self.set_hue(round_to_step(modulo(hue, 360.0), step));
    }

    /// Decreases the hue by `step` (at least the step), wrapping around.
    pub fn decrement(&self, step: f64) {
        let step = step.max(self.step);
        let hue = self.current_hue();
        if hue == 0.0 {
            // Not just 360 - step: the previous step may be closer to 0 than a step.
            self.set_hue(round_down(360.0 / step) * step);
        } else {
            self.set_hue(round_to_step(modulo(hue - step, 360.0), step));
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

    /// The hue at full saturation, to draw the thumb with.
    pub fn display_color(&self) -> Signal<C> {
        let (value, channel) = (self.value, self.channel);
        Signal::derive(move || value.get().get_display_color(channel))
    }
}

/// Creates the state of a color wheel changing a color's hue.
pub fn use_color_wheel_state<C: ColorValue>(
    input: UseColorWheelStateInput<C>,
) -> ColorWheelState<C> {
    let UseColorWheelStateInput {
        default_value,
        value,
        channel,
        is_disabled,
        on_change,
        on_change_end,
    } = input;

    let binding = value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_value)));
    let default_value = StoredValue::new(binding.value.get_untracked());
    let latest = StoredValue::new(binding.value.get_untracked());
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
    let range = C::get_channel_range(channel);
    let value = binding.value;
    let dragging = RwSignal::new(false);
    ColorWheelState {
        value,
        hue: Signal::derive(move || value.get().get_channel_value(channel)),
        channel,
        step: range.step,
        page_step: range.page_size,
        is_disabled,
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

    fn wheel(hue: f64) -> ColorWheelState<HSV> {
        use_color_wheel_state(UseColorWheelStateInput::new(
            HSV {
                hue,
                saturation: 1.0,
                value: 1.0,
            },
            HsvChannel::Hue,
        ))
    }

    #[test]
    fn steps_wrap_around() {
        Owner::new().with(|| {
            let state = wheel(359.0);
            state.increment(1.0);
            assert_that!(state.hue.get_untracked()).is_equal_to(0.0);
            state.decrement(1.0);
            assert_that!(state.hue.get_untracked()).is_equal_to(359.0);
            state.set_hue(0.0);
            state.decrement(state.page_step);
            assert_that!(state.hue.get_untracked()).is_equal_to(345.0);
        });
    }

    #[test]
    fn zero_degrees_is_at_three_o_clock() {
        Owner::new().with(|| {
            let state = wheel(0.0);
            let (x, y) = state.thumb_position(100.0);
            assert_that!(x.round()).is_equal_to(100.0);
            assert_that!(y.round().abs()).is_equal_to(0.0);
            // Below the center: 90° (clockwise).
            state.set_hue_from_point(0.0, 50.0, 50.0);
            assert_that!(state.hue.get_untracked()).is_equal_to(90.0);
        });
    }
}
