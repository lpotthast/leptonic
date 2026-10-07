// Upstream: react-stately/src/color/useColorSliderState.ts @ 99e6102368
use leptos::prelude::*;

use crate::{
    hooks::slider::{SliderState, UseSliderStateInput, use_slider_state},
    utils::{ValueBinding, color::ColorValue, orientation::Orientation},
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
        Signal::derive(move || value.get().get_display_color(channel))
    }

    /// The channel's value, formatted.
    pub fn formatted_value(&self) -> Signal<String> {
        let (value, channel) = (self.value, self.channel);
        Signal::derive(move || value.get().format_channel_value(channel))
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

    let range = C::get_channel_range(channel);
    let slider = use_slider_state(UseSliderStateInput {
        value: Some(ValueBinding::new(
            Signal::derive(move || vec![color.get().get_channel_value(channel)]),
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
            color.get().format_channel_value(channel)
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
    }
}
