// Upstream: react-stately/src/color/useColorAreaState.ts @ 99e6102368
use leptos::prelude::*;

use crate::{
    ValueBinding,
    utils::{
        color::{ColorChannelRange, ColorSpaceAxes, ColorValue},
        math::snap_value_to_step,
        point::Point,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the color type (`ColorValue`), whose channels are typed per color space; the
//   color space is the type (react-aria: a `colorSpace` prop converting the value).
// - Hook-owned value (C4): `default_value` + `on_change`, or `value` bound to app state.
// - A `Copy` struct with signals and methods (C3); points of the area are `Point`s (C13).
// - `x_channel_step`/`y_channel_step` override the channels' steps (leptonic addition).
//
// =============================================================================

/// Input of [`use_color_area_state`].
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
    /// The color the current drag (or keyboard change) set last, for `on_change_end`
    /// (react-stately's `valueRef`).
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
        self.latest.set_value(color);
        if color != self.value.get_untracked() {
            self.binding.set(color);
        }
    }

    /// Sets the x channel's value.
    pub fn set_x_value(&self, value: f64) {
        let color = self.value.get_untracked();
        if value != color.channel_value(self.x_channel) {
            self.set_value(color.with_channel_value(self.x_channel, value));
        }
    }

    /// Sets the y channel's value.
    pub fn set_y_value(&self, value: f64) {
        let color = self.value.get_untracked();
        if value != color.channel_value(self.y_channel) {
            self.set_value(color.with_channel_value(self.y_channel, value));
        }
    }

    /// Sets the color from a `point` of the area, each coordinate from 0 to 1 (`y` from the
    /// top). The values snap to the steps.
    pub fn set_color_from_point(&self, point: Point) {
        let Point { x, y } = point;
        let color = self.value.get_untracked();
        let x_range = C::channel_range(self.x_channel);
        let y_range = C::channel_range(self.y_channel);
        let new_x = x_range.min_value + x.clamp(0.0, 1.0) * (x_range.max_value - x_range.min_value);
        let new_y =
            y_range.min_value + (1.0 - y.clamp(0.0, 1.0)) * (y_range.max_value - y_range.min_value);
        let mut new_color = None;
        if new_x != color.channel_value(self.x_channel) {
            let snapped = snap(new_x, self.x_channel_step, &x_range);
            new_color = Some(color.with_channel_value(self.x_channel, snapped));
        }
        if new_y != color.channel_value(self.y_channel) {
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
    pub fn thumb_position(&self) -> Point {
        let x_range = C::channel_range(self.x_channel);
        let y_range = C::channel_range(self.y_channel);
        Point::new(
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
        let color = self.value.get_untracked();
        let range = C::channel_range(channel);
        let target = color.channel_value(channel) + delta;
        let value = if delta > 0.0 && target > range.max_value {
            range.max_value
        } else {
            snap(target, step, &range)
        };
        if value != color.channel_value(channel) {
            self.set_value(color.with_channel_value(channel, value));
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

    /// The color to show: without alpha (react-aria's `getDisplayColor`).
    pub fn display_color(&self) -> Signal<C> {
        let value = self.value;
        Signal::derive(move || value.get().opaque())
    }
}

fn snap(value: f64, step: f64, range: &ColorChannelRange) -> f64 {
    snap_value_to_step(value, Some(range.min_value), Some(range.max_value), step)
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

    let ColorSpaceAxes {
        x: x_channel,
        y: y_channel,
        z: z_channel,
    } = C::color_space_axes(x_channel, y_channel);
    let x_range = C::channel_range(x_channel);
    let y_range = C::channel_range(y_channel);

    let binding = value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_value)));
    // With app state, the color it holds at first is the one to reset to (react-aria).
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

    let dragging = RwSignal::new(false);
    let value = binding.value;
    ColorAreaState {
        value,
        x_value: Signal::derive(move || value.get().channel_value(x_channel)),
        y_value: Signal::derive(move || value.get().channel_value(y_channel)),
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
    use crate::{
        testing::{flush_effects, with_owner},
        utils::color::{HSV, HsvChannel},
    };

    fn input(color: HSV) -> UseColorAreaStateInput<HSV> {
        UseColorAreaStateInput {
            x_channel: Some(HsvChannel::Saturation),
            y_channel: Some(HsvChannel::Brightness),
            default_value: color,
            value: None,
            x_channel_step: None,
            y_channel_step: None,
            on_change: None,
            on_change_end: None,
        }
    }

    fn state(color: HSV) -> ColorAreaState<HSV> {
        use_color_area_state(input(color))
    }

    #[test]
    fn steps_snap_to_the_channel_step_and_stop_at_the_bounds() {
        with_owner(|| {
            let area = state(HSV {
                hue: 0.0,
                saturation: 0.5,
                brightness: 0.5,
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
        with_owner(|| {
            let area = state(HSV {
                hue: 0.0,
                saturation: 0.0,
                brightness: 0.0,
            });
            area.set_color_from_point(Point::new(0.25, 0.25));
            assert_that!(area.x_value.get_untracked()).is_equal_to(0.25);
            assert_that!(area.y_value.get_untracked()).is_equal_to(0.75);
            assert_that!(area.thumb_position()).is_equal_to(Point::new(0.25, 0.25));
        });
    }

    #[test]
    fn dragging_ends_with_on_change_end() {
        with_owner(|| {
            let ends = RwSignal::new(Vec::new());
            let area = use_color_area_state(UseColorAreaStateInput {
                on_change_end: Some(Callback::new(move |c: HSV| ends.update(|e| e.push(c)))),
                default_value: HSV {
                    hue: 0.0,
                    saturation: 0.5,
                    brightness: 0.5,
                },
                value: None,
                x_channel: None,
                y_channel: None,
                x_channel_step: None,
                y_channel_step: None,
                on_change: None,
            });
            area.set_dragging(true);
            area.set_color_from_point(Point::new(1.0, 0.0));
            area.set_dragging(false);
            assert_that!(ends.get_untracked().len()).is_equal_to(1);
        });
    }

    /// A keyboard step right after the app changed the color starts from the app's color, before
    /// any Effect ran (react-stately computes from the rendered `color`).
    #[test]
    fn steps_start_from_the_color_the_app_set_last() {
        with_owner(|| {
            let app = RwSignal::new(HSV {
                hue: 0.0,
                saturation: 0.5,
                brightness: 0.5,
            });
            let area = use_color_area_state(UseColorAreaStateInput {
                value: Some(ValueBinding::from(app)),
                ..input(app.get_untracked())
            });
            flush_effects();
            app.set(HSV {
                hue: 0.0,
                saturation: 0.2,
                brightness: 0.8,
            });
            area.increment_x(area.x_channel_step);
            assert_that!(app.get_untracked()).is_equal_to(HSV {
                hue: 0.0,
                saturation: 0.21,
                brightness: 0.8,
            });
        });
    }

    /// A color the app's state rejected is sent again when set again.
    #[test]
    fn a_color_the_app_rejected_is_sent_again() {
        with_owner(|| {
            let start = HSV {
                hue: 0.0,
                saturation: 0.5,
                brightness: 0.5,
            };
            let sent = RwSignal::new(Vec::new());
            let area = use_color_area_state(UseColorAreaStateInput {
                // App state that ignores every change.
                value: Some(ValueBinding::new(
                    Signal::stored(start),
                    Callback::new(move |color| sent.update(|sent| sent.push(color))),
                )),
                ..input(start)
            });
            let other = start.with_saturation(1.0);
            area.set_value(other);
            area.set_value(other);
            assert_that!(sent.get_untracked()).is_equal_to(vec![other, other]);
            assert_that!(area.value.get_untracked()).is_equal_to(start);
        });
    }

    /// `on_change_end` reports the color the drag set last, even when the app changed the color
    /// before the drag and no Effect ran since.
    #[test]
    fn the_end_of_a_drag_reports_its_last_color() {
        with_owner(|| {
            let app = RwSignal::new(HSV {
                hue: 0.0,
                saturation: 0.5,
                brightness: 0.5,
            });
            let ends = RwSignal::new(Vec::new());
            let area = use_color_area_state(UseColorAreaStateInput {
                value: Some(ValueBinding::from(app)),
                on_change_end: Some(Callback::new(move |c: HSV| ends.update(|e| e.push(c)))),
                ..input(app.get_untracked())
            });
            flush_effects();
            let changed = HSV {
                hue: 120.0,
                saturation: 0.5,
                brightness: 0.5,
            };
            app.set(changed);
            area.set_dragging(true);
            area.set_dragging(false);
            assert_that!(ends.get_untracked()).is_equal_to(vec![changed]);
        });
    }
}
