// Upstream: react-stately/src/color/useColorSliderState.ts @ 99e6102368
use leptos::prelude::*;

use crate::{
    hooks::slider::{SliderState, UseSliderStateInput, use_slider_state},
    utils::{
        ValueBinding,
        color::ColorValue,
        i18n::{Locale, use_locale},
        orientation::Orientation,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the color type (`ColorValue`), the color space is the type.
// - Hook-owned value (C4): `default_value` + `on_change`, or `value` bound to app state.
// - A `Copy` struct (C3) holding the one-thumb `SliderState` of the channel (react-aria: the
//   slider state spread into the color slider state).
//
// =============================================================================

/// Input of [`use_color_slider_state`].
#[derive(Debug, Clone, Copy)]
pub struct UseColorSliderStateInput<C: ColorValue> {
    /// The initial color.
    pub default_value: C,
    /// The color as app state, replacing `default_value`.
    pub value: Option<ValueBinding<C>>,
    /// The channel the slider changes.
    pub channel: C::Channel,
    pub is_disabled: Signal<bool>,
    pub orientation: Signal<Orientation>,
    /// Called with the color whenever it changes, also while dragging.
    pub on_change: Option<Callback<C>>,
    /// Called with the color when the user stops dragging (or after a keyboard change).
    pub on_change_end: Option<Callback<C>>,
}

/// The state of a color slider: the color, and the slider of its channel.
#[derive(Debug, Clone, Copy)]
pub struct ColorSliderState<C: ColorValue> {
    /// The color.
    pub value: Signal<C>,
    /// The channel the slider changes.
    pub channel: C::Channel,
    /// The slider of the channel's value (one thumb).
    pub slider: SliderState<f64>,
    /// Whether the thumb is being dragged.
    pub is_dragging: Signal<bool>,
    binding: ValueBinding<C>,
    default_value: StoredValue<C>,
    locale: Signal<Locale>,
}

impl<C: ColorValue> ColorSliderState<C> {
    /// Sets the color.
    pub fn set_value(&self, color: C) {
        self.binding.set(color);
    }

    /// The color the slider started with (for form resets).
    pub fn default_value(&self) -> C {
        self.default_value.get_value()
    }

    /// The color to draw the track with: for a hue, the hue at full saturation (react-aria's
    /// `getDisplayColor`).
    pub fn display_color(&self) -> Signal<C> {
        let (value, channel) = (self.value, self.channel);
        Signal::derive(move || value.get().display_color(channel))
    }

    /// The channel's value, formatted for the locale.
    pub fn formatted_value(&self) -> Signal<String> {
        let (value, channel, locale) = (self.value, self.channel, self.locale);
        Signal::derive(move || value.get().format_channel_value(channel, &locale.get()))
    }
}

/// Creates the state of a slider changing one channel of a color.
pub fn use_color_slider_state<C: ColorValue>(
    input: UseColorSliderStateInput<C>,
) -> ColorSliderState<C> {
    let UseColorSliderStateInput {
        default_value,
        value,
        channel,
        is_disabled,
        orientation,
        on_change,
        on_change_end,
    } = input;

    let binding = value.unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_value)));
    let default_value = StoredValue::new(binding.value.get_untracked());
    let color = binding.value;
    // Several changes can come before the binding updates (dragging): the latest color.
    let latest = StoredValue::new(color.get_untracked());
    Effect::new(move || latest.set_value(color.get()));
    let binding = ValueBinding::new(
        color,
        Callback::new(move |new_color: C| {
            latest.set_value(new_color);
            binding.set(new_color);
            if let Some(on_change) = on_change {
                on_change.run(new_color);
            }
        }),
    );

    let locale = use_locale();
    let range = C::channel_range(channel);
    let slider = use_slider_state(UseSliderStateInput {
        value: Some(ValueBinding::new(
            Signal::derive(move || vec![color.get().channel_value(channel)]),
            Callback::new(move |values: Vec<f64>| {
                if let Some(&value) = values.first() {
                    binding.set(latest.get_value().with_channel_value(channel, value));
                }
            }),
        )),
        step: Signal::stored(range.step),
        is_disabled,
        orientation,
        // The channel's formatting and page size (react-stately overrides both).
        value_label: Some(Callback::new(move |_| {
            color.get().format_channel_value(channel, &locale.get())
        })),
        page_size: Some(Signal::stored(range.page_size)),
        // `on_change` already ran with the color; this only reports the end.
        on_change_end: on_change_end.map(|on_change_end| {
            Callback::new(move |values: Vec<f64>| {
                if let Some(&value) = values.first() {
                    on_change_end.run(latest.get_value().with_channel_value(channel, value));
                }
            })
        }),
        default_values: None,
        min_value: Signal::stored(range.min_value),
        max_value: Signal::stored(range.max_value),
        format_options: Signal::default(),
        on_change: None,
    });

    ColorSliderState {
        value: color,
        channel,
        slider,
        is_dragging: Signal::derive(move || slider.is_thumb_dragging(0)),
        binding,
        default_value,
        locale,
    }
}

#[cfg(test)]
mod tests {
    // Upstream: @adobe/react-spectrum/test/color/ColorSlider.test.tsx (the state's part of it).
    use assertr::prelude::*;

    use super::*;
    use crate::{
        testing::{flush_effects, with_owner},
        utils::color::{Alpha, AlphaChannel, HSL, HslChannel, RGB8, RgbChannel},
    };

    const BLACK: RGB8 = RGB8 { r: 0, g: 0, b: 0 };

    /// The colors a callback was called with.
    fn recorder<C: ColorValue>() -> (RwSignal<Vec<C>>, Callback<C>) {
        let calls = RwSignal::new(Vec::new());
        let callback = Callback::new(move |color| calls.update(|c| c.push(color)));
        (calls, callback)
    }

    fn input<C: ColorValue>(default_value: C, channel: C::Channel) -> UseColorSliderStateInput<C> {
        UseColorSliderStateInput {
            default_value,
            value: None,
            channel,
            is_disabled: Signal::default(),
            orientation: Signal::stored(Orientation::Horizontal),
            on_change: None,
            on_change_end: None,
        }
    }

    /// A keyboard change, as `use_slider_thumb` makes it: a drag around the change.
    fn key<C: ColorValue>(state: &ColorSliderState<C>, change: impl Fn(&SliderState<f64>)) {
        state.slider.set_thumb_dragging(0, true);
        change(&state.slider);
        state.slider.set_thumb_dragging(0, false);
    }

    fn red(r: u8) -> RGB8 {
        RGB8 { r, ..BLACK }
    }

    // Upstream: "sets input props", "sets aria-valuetext to formatted value".
    #[test]
    fn uses_the_channels_range_and_formatting() {
        with_owner(|| {
            let state = use_color_slider_state(input(BLACK, RgbChannel::Red));
            assert_that!(state.slider.min_value.get_untracked()).is_equal_to(0.0);
            assert_that!(state.slider.max_value.get_untracked()).is_equal_to(255.0);
            assert_that!(state.slider.step.get_untracked()).is_equal_to(1.0);
            assert_that!(state.formatted_value().get_untracked()).is_equal_to("0".to_owned());

            let hsl = HSL {
                hue: 10.0,
                saturation: 0.5,
                lightness: 0.5,
            };
            let state = use_color_slider_state(input(hsl, HslChannel::Hue));
            assert_that!(state.slider.max_value.get_untracked()).is_equal_to(360.0);
            assert_that!(state.slider.thumb_value(0)).is_equal_to(10.0);
            assert_that!(state.formatted_value().get_untracked()).is_equal_to("10°".to_owned());
            assert_that!(state.slider.thumb_value_label(0)).is_equal_to("10°".to_owned());
        });
    }

    // Upstream: "keyboard events" > "works".
    #[test]
    fn keyboard_changes_call_on_change_and_on_change_end() {
        with_owner(|| {
            let (changes, on_change) = recorder();
            let (ends, on_change_end) = recorder();
            let state = use_color_slider_state(UseColorSliderStateInput {
                on_change: Some(on_change),
                on_change_end: Some(on_change_end),
                ..input(BLACK, RgbChannel::Red)
            });
            let page = || Some(untrack(|| state.slider.page_size()));
            key(&state, |s| s.increment_thumb(0, None)); // ArrowRight
            key(&state, |s| s.decrement_thumb(0, None)); // ArrowLeft
            key(&state, |s| s.increment_thumb(0, page())); // PageUp
            key(&state, |s| s.increment_thumb(0, None)); // ArrowRight
            key(&state, |s| s.decrement_thumb(0, page())); // PageDown
            key(&state, |s| s.set_thumb_value(0, s.thumb_max_value(0))); // End
            key(&state, |s| s.decrement_thumb(0, page())); // PageDown
            key(&state, |s| s.set_thumb_value(0, s.thumb_min_value(0))); // Home

            let expected: Vec<RGB8> = [1, 0, 17, 18, 1, 255, 238, 0].map(red).to_vec();
            assert_that!(changes.get_untracked()).is_equal_to(expected.clone());
            assert_that!(ends.get_untracked()).is_equal_to(expected);
            assert_that!(state.value.get_untracked()).is_equal_to(BLACK);
        });
    }

    // Upstream: "keyboard events" > "doesn't work when disabled".
    #[test]
    fn a_disabled_slider_ignores_changes() {
        with_owner(|| {
            let (changes, on_change) = recorder();
            let state = use_color_slider_state(UseColorSliderStateInput {
                is_disabled: Signal::stored(true),
                on_change: Some(on_change),
                ..input(BLACK, RgbChannel::Red)
            });
            key(&state, |s| s.increment_thumb(0, None));
            key(&state, |s| s.decrement_thumb(0, None));
            assert_that!(changes.get_untracked()).is_empty();
            assert_that!(state.value.get_untracked()).is_equal_to(BLACK);
        });
    }

    // Upstream: "dragging the thumb works" (the state's part: the changes while dragging, one
    // end, the other channels kept).
    #[test]
    fn dragging_changes_the_channel_and_reports_the_end_once() {
        with_owner(|| {
            let (changes, on_change) = recorder();
            let (ends, on_change_end) = recorder();
            let start = RGB8 {
                r: 0,
                g: 100,
                b: 200,
            };
            let state = use_color_slider_state(UseColorSliderStateInput {
                on_change: Some(on_change),
                on_change_end: Some(on_change_end),
                ..input(start, RgbChannel::Red)
            });
            state.slider.set_thumb_dragging(0, true);
            assert_that!(state.is_dragging.get_untracked()).is_true();
            state.slider.set_thumb_percent(0, 0.2);
            state.slider.set_thumb_percent(0, 0.5);
            assert_that!(ends.get_untracked()).is_empty();
            state.slider.set_thumb_dragging(0, false);
            assert_that!(state.is_dragging.get_untracked()).is_false();

            let with_red = |r| RGB8 { r, ..start };
            assert_that!(changes.get_untracked()).is_equal_to(vec![with_red(51), with_red(128)]);
            assert_that!(ends.get_untracked()).is_equal_to(vec![with_red(128)]);
            assert_that!(state.value.get_untracked()).is_equal_to(with_red(128));
        });
    }

    // Upstream: "supports form reset".
    #[test]
    fn remembers_the_initial_bound_color_for_form_resets() {
        with_owner(|| {
            let app = RwSignal::new(red(127));
            let state = use_color_slider_state(UseColorSliderStateInput {
                value: Some(ValueBinding::from(app)),
                ..input(BLACK, RgbChannel::Red)
            });
            assert_that!(state.slider.thumb_value(0)).is_equal_to(127.0);
            state.slider.set_thumb_value(0, 255.0);
            assert_that!(app.get_untracked()).is_equal_to(red(255));

            assert_that!(state.default_value()).is_equal_to(red(127));
            assert_that!(state.slider.default_values()).is_equal_to(vec![127.0]);
            state
                .slider
                .set_thumb_value(0, state.slider.default_values()[0]);
            assert_that!(app.get_untracked()).is_equal_to(red(127));
        });
    }

    #[test]
    fn changes_keep_the_other_channels_the_app_set() {
        with_owner(|| {
            let app = RwSignal::new(BLACK);
            let state = use_color_slider_state(UseColorSliderStateInput {
                value: Some(ValueBinding::from(app)),
                ..input(BLACK, RgbChannel::Red)
            });
            flush_effects();
            app.set(RGB8 {
                r: 10,
                g: 20,
                b: 30,
            });
            flush_effects();
            assert_that!(state.slider.thumb_value(0)).is_equal_to(10.0);
            key(&state, |s| s.increment_thumb(0, None));
            assert_that!(app.get_untracked()).is_equal_to(RGB8 {
                r: 11,
                g: 20,
                b: 30,
            });
        });
    }

    // Upstream: `getDisplayColor`.
    #[test]
    fn the_display_color_shows_the_channel() {
        with_owner(|| {
            let color = Alpha {
                color: HSL {
                    hue: 200.0,
                    saturation: 0.2,
                    lightness: 0.3,
                },
                alpha: 0.5,
            };
            let display = |channel| {
                use_color_slider_state(input(color, channel))
                    .display_color()
                    .get_untracked()
            };
            // The hue at full saturation (and opaque).
            assert_that!(display(AlphaChannel::Color(HslChannel::Hue))).is_equal_to(Alpha::new(
                HSL {
                    hue: 200.0,
                    saturation: 1.0,
                    lightness: 0.5,
                },
            ));
            // Other color channels: the color, opaque.
            assert_that!(display(AlphaChannel::Color(HslChannel::Lightness)))
                .is_equal_to(Alpha::new(color.color));
            // Alpha: the color as it is.
            assert_that!(display(AlphaChannel::Alpha)).is_equal_to(color);
        });
    }
}
