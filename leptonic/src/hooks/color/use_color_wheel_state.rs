// Upstream: react-stately/src/color/useColorWheelState.ts @ 99e6102368
use leptos::prelude::*;

use crate::{
    ValueBinding,
    utils::{
        color::{Color, ColorChannelRange, ColorValue, HSL, HslChannel},
        point::Point,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the color type (`ColorValue`). A color space with a hue channel (HSV, HSL)
//   changes it; any other (RGB) changes the hue of its HSL form and converts back, keeping the
//   color type (react-aria converts the color to HSL and emits HSL colors). The hue the wheel set
//   last is remembered for the color it produced, so that the thumb stays put where RGB loses
//   the hue (grays) or rounds it.
// - Hook-owned value (C4): `default_value` + `on_change`, or `value` bound to app state.
// - A `Copy` struct with signals and methods (C3); points relative to the center are `Point`s
//   (C13).
//
// =============================================================================

/// Input of [`use_color_wheel_state`].
#[derive(Debug, Clone, Copy)]
pub struct UseColorWheelStateInput<C: ColorValue> {
    /// The initial color.
    pub default_value: C,
    /// The color as app state, replacing `default_value`.
    pub value: Option<ValueBinding<C>>,
    pub is_disabled: Signal<bool>,
    /// Called with the color whenever it changes, also while dragging.
    pub on_change: Option<Callback<C>>,
    /// Called with the color when the user stops dragging (or after a keyboard change).
    pub on_change_end: Option<Callback<C>>,
}

/// The state of a color wheel: the color and its hue.
#[derive(Debug, Clone, Copy)]
pub struct ColorWheelState<C: ColorValue> {
    /// The color.
    pub value: Signal<C>,
    /// Its hue, in degrees.
    pub hue: Signal<f64>,
    /// The hue's step.
    pub step: f64,
    /// The hue's page step (PageUp/PageDown, Shift + arrow keys).
    pub page_step: f64,
    pub is_disabled: Signal<bool>,
    /// Whether the thumb is being dragged.
    pub is_dragging: Signal<bool>,
    binding: ValueBinding<C>,
    default_value: StoredValue<C>,
    /// The color the current drag (or keyboard change) set last, for `on_change_end`
    /// (react-stately's `valueRef`).
    latest: StoredValue<C>,
    /// The color the wheel set last, with the hue it set.
    wheel_hue: RwSignal<Option<(C, f64)>>,
    dragging: RwSignal<bool>,
    on_change_end: Option<Callback<C>>,
}

/// The range, step and page size of a hue.
fn hue_range() -> ColorChannelRange {
    HSL::channel_range(HslChannel::Hue)
}

/// The hue of `color`: its hue channel, else its HSL form's.
fn hue_of<C: ColorValue>(color: C) -> f64 {
    if let Some(hue) = C::hue_channel() {
        color.channel_value(hue)
    } else {
        let color: Color = color.into();
        color.to::<HSL>().hue
    }
}

/// `color` with `hue`: its hue channel, else through its HSL form (keeping the alpha).
fn with_hue<C: ColorValue>(color: C, hue: f64) -> C {
    if let Some(channel) = C::hue_channel() {
        color.with_channel_value(channel, hue)
    } else {
        let color: Color = color.into();
        let hsl = color.to::<HSL>().with_hue(hue);
        C::from(Color::from(hsl).with_alpha(color.alpha))
    }
}

/// The hue of `color`, as the wheel set it if it produced `color` (tracks `wheel_hue`).
fn remembered_hue<C: ColorValue>(color: C, wheel_hue: RwSignal<Option<(C, f64)>>) -> f64 {
    wheel_hue
        .with(|set| set.filter(|(set, _)| *set == color).map(|(_, hue)| hue))
        .unwrap_or_else(|| hue_of(color))
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
        self.latest.set_value(color);
        if color != self.value.get_untracked() {
            self.binding.set(color);
        }
    }

    fn current_hue(&self) -> f64 {
        untrack(|| remembered_hue(self.value.get(), self.wheel_hue))
    }

    /// Sets the hue, snapped to the step (360 wraps around to 0).
    pub fn set_hue(&self, hue: f64) {
        let hue = if hue > 360.0 { 0.0 } else { hue };
        let hue = round_to_step(modulo(hue, 360.0), self.step);
        if hue != self.current_hue() {
            let color = with_hue(self.value.get_untracked(), hue);
            self.wheel_hue.set(Some((color, hue)));
            self.set_value(color);
        }
    }

    /// Sets the hue of `point`, relative to the wheel's center (`y` down).
    pub fn set_hue_from_point(&self, point: Point, radius: f64) {
        let degrees = (point.y / radius).atan2(point.x / radius).to_degrees();
        self.set_hue((degrees + 360.0) % 360.0);
    }

    /// The thumb's position relative to the center on a circle of `radius` (0° at 3 o'clock,
    /// clockwise). Tracked.
    pub fn thumb_position(&self, radius: f64) -> Point {
        let radians = (360.0 - self.hue.get() + 90.0).to_radians();
        Point::new(radians.sin() * radius, radians.cos() * radius)
    }

    /// Increases the hue by `step` (at least the step), wrapping around.
    pub fn increment(&self, step: f64) {
        let step = step.max(self.step);
        let range = hue_range();
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
        if dragging && !was_dragging {
            self.latest.set_value(self.value.get_untracked());
        }
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
        let hue = self.hue;
        Signal::derive(move || C::from(Color::from(HSL::from_hue_fully_saturated(hue.get()))))
    }
}

/// Creates the state of a color wheel changing a color's hue.
pub fn use_color_wheel_state<C: ColorValue>(
    input: UseColorWheelStateInput<C>,
) -> ColorWheelState<C> {
    let UseColorWheelStateInput {
        default_value,
        value,
        is_disabled,
        on_change,
        on_change_end,
    } = input;

    let binding = value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_value)));
    let default_value = StoredValue::new(binding.value.get_untracked());
    let latest = StoredValue::new(binding.value.get_untracked());
    let binding = ValueBinding::new(
        binding.value,
        Callback::new(move |color: C| {
            binding.set(color);
            if let Some(on_change) = on_change {
                on_change.run(color);
            }
        }),
    );
    let range = hue_range();
    let value = binding.value;
    let dragging = RwSignal::new(false);
    let wheel_hue = RwSignal::new(None);
    ColorWheelState {
        value,
        hue: Signal::derive(move || remembered_hue(value.get(), wheel_hue)),
        step: range.step,
        page_step: range.page_size,
        is_disabled,
        is_dragging: dragging.into(),
        binding,
        default_value,
        latest,
        wheel_hue,
        dragging,
        on_change_end,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::{
        testing::{flush_effects, with_owner},
        utils::color::{HSV, RGB8},
    };

    fn wheel(hue: f64) -> ColorWheelState<HSV> {
        use_color_wheel_state(UseColorWheelStateInput {
            default_value: HSV {
                hue,
                saturation: 1.0,
                brightness: 1.0,
            },
            value: None,
            is_disabled: Signal::stored(false),
            on_change: None,
            on_change_end: None,
        })
    }

    #[test]
    fn steps_wrap_around() {
        with_owner(|| {
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
        with_owner(|| {
            let state = wheel(0.0);
            let Point { x, y } = state.thumb_position(100.0);
            assert_that!(x.round()).is_equal_to(100.0);
            assert_that!(y.round().abs()).is_equal_to(0.0);
            // Below the center: 90° (clockwise).
            state.set_hue_from_point(Point::new(0.0, 50.0), 50.0);
            assert_that!(state.hue.get_untracked()).is_equal_to(90.0);
        });
    }

    #[test]
    fn rgb_colors_change_the_hue_of_their_hsl_form() {
        with_owner(|| {
            let state = use_color_wheel_state(UseColorWheelStateInput {
                default_value: RGB8 { r: 255, g: 0, b: 0 },
                value: None,
                is_disabled: Signal::stored(false),
                on_change: None,
                on_change_end: None,
            });
            assert_that!(state.hue.get_untracked()).is_equal_to(0.0);
            state.set_hue(120.0);
            assert_that!(state.value.get_untracked()).is_equal_to(RGB8 { r: 0, g: 255, b: 0 });
            // A hue that RGB rounds stays as the wheel set it.
            state.set_hue(37.0);
            assert_that!(state.hue.get_untracked()).is_equal_to(37.0);
            state.increment(1.0);
            assert_that!(state.hue.get_untracked()).is_equal_to(38.0);
        });
    }

    #[test]
    fn a_gray_rgb_color_keeps_the_hue_the_wheel_set() {
        with_owner(|| {
            let state = use_color_wheel_state(UseColorWheelStateInput {
                default_value: RGB8 {
                    r: 128,
                    g: 128,
                    b: 128,
                },
                value: None,
                is_disabled: Signal::stored(false),
                on_change: None,
                on_change_end: None,
            });
            state.set_hue(200.0);
            assert_that!(state.hue.get_untracked()).is_equal_to(200.0);
            assert_that!(state.thumb_position(100.0).y.round()).is_equal_to(-34.0);
        });
    }

    /// A keyboard step right after the app changed the color starts from the app's color, before
    /// any Effect ran (react-stately computes from the rendered `value`).
    #[test]
    fn steps_start_from_the_color_the_app_set_last() {
        with_owner(|| {
            let app = RwSignal::new(HSV::from_hue_fully_saturated(10.0));
            let state = use_color_wheel_state(UseColorWheelStateInput {
                default_value: app.get_untracked(),
                value: Some(ValueBinding::from(app)),
                is_disabled: Signal::stored(false),
                on_change: None,
                on_change_end: None,
            });
            flush_effects();
            app.set(HSV::from_hue_fully_saturated(100.0));
            state.increment(1.0);
            assert_that!(app.get_untracked()).is_equal_to(HSV::from_hue_fully_saturated(101.0));
        });
    }

    /// A color the app's state rejected is sent again when set again.
    #[test]
    fn a_color_the_app_rejected_is_sent_again() {
        with_owner(|| {
            let start = HSV::from_hue_fully_saturated(10.0);
            let sent = RwSignal::new(Vec::new());
            let state = use_color_wheel_state(UseColorWheelStateInput {
                default_value: start,
                // App state that ignores every change.
                value: Some(ValueBinding::new(
                    Signal::stored(start),
                    Callback::new(move |color| sent.update(|sent| sent.push(color))),
                )),
                is_disabled: Signal::stored(false),
                on_change: None,
                on_change_end: None,
            });
            state.set_hue(50.0);
            state.set_hue(50.0);
            let expected = HSV::from_hue_fully_saturated(50.0);
            assert_that!(sent.get_untracked()).is_equal_to(vec![expected, expected]);
        });
    }

    /// `on_change_end` reports the color the drag set last, even when the app changed the color
    /// before the drag and no Effect ran since.
    #[test]
    fn the_end_of_a_drag_reports_its_last_color() {
        with_owner(|| {
            let app = RwSignal::new(HSV::from_hue_fully_saturated(10.0));
            let ends = RwSignal::new(Vec::new());
            let state = use_color_wheel_state(UseColorWheelStateInput {
                default_value: app.get_untracked(),
                value: Some(ValueBinding::from(app)),
                is_disabled: Signal::stored(false),
                on_change: None,
                on_change_end: Some(Callback::new(move |c: HSV| ends.update(|e| e.push(c)))),
            });
            flush_effects();
            app.set(HSV::from_hue_fully_saturated(100.0));
            state.set_dragging(true);
            state.set_dragging(false);
            assert_that!(ends.get_untracked())
                .is_equal_to(vec![HSV::from_hue_fully_saturated(100.0)]);
        });
    }
}
