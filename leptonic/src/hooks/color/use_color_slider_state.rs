use std::fmt;

use leptos::prelude::*;

use crate::{
    hooks::slider::{
        SliderOrientation, SliderValues, UseSliderStateInput, UseSliderStateReturn,
        use_slider_state,
    },
    utils::color::ColorValue,
};

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/color/src/useColorSliderState.ts

// ## INTENTIONAL DEVIATIONS
//
// - Hook-owned state: The hook owns its color state internally and syncs
//   bidirectionally with the underlying slider state.
//
// - Wraps `use_slider_state` with a single thumb for the channel value,
//   mapping channel range to slider min/max/step.

/// Input parameters for `use_color_slider_state`.
#[derive(Debug, Clone)]
pub struct UseColorSliderStateInput<C: ColorValue> {
    /// The initial color value.
    pub default_value: C,

    /// Which channel this slider controls.
    pub channel: C::Channel,

    /// Whether the slider is disabled.
    pub disabled: Signal<bool>,

    /// The slider orientation. Affects keyboard navigation and pointer
    /// coordinate interpretation in the underlying slider state.
    pub orientation: Signal<SliderOrientation>,

    /// Callback fired when the color changes during interaction.
    pub on_change: Option<Callback<C>>,

    /// Callback fired when interaction ends.
    pub on_change_end: Option<Callback<C>>,
}

/// Return value of `use_color_slider_state`.
pub struct UseColorSliderStateReturn<C: ColorValue> {
    /// The current full color.
    pub value: Signal<C>,

    /// Update the full color.
    pub set_value: Callback<C>,

    /// The underlying slider state (single thumb).
    pub slider_state: UseSliderStateReturn,

    /// The display color for gradient rendering.
    /// For hue: full saturation and brightness at current hue.
    /// For other channels: color as-is (alpha stripping deferred until alpha support).
    pub display_color: Signal<C>,

    /// Formatted label for the current channel value.
    pub thumb_value_label: Signal<String>,

    /// The channel this slider controls.
    pub channel: C::Channel,

    /// Whether the color slider is currently being dragged.
    pub is_dragging: Signal<bool>,
}

impl<C: ColorValue> Clone for UseColorSliderStateReturn<C> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            set_value: self.set_value,
            slider_state: self.slider_state,
            display_color: self.display_color,
            thumb_value_label: self.thumb_value_label,
            channel: self.channel,
            is_dragging: self.is_dragging,
        }
    }
}

impl<C: ColorValue> fmt::Debug for UseColorSliderStateReturn<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseColorSliderStateReturn")
            .field("channel", &self.channel)
            .finish_non_exhaustive()
    }
}

/// Creates state for a color slider that adjusts a single channel.
///
/// Wraps `use_slider_state` with the channel's range (min, max, step)
/// and syncs changes back to the full color value.
pub fn use_color_slider_state<C: ColorValue>(
    input: &UseColorSliderStateInput<C>,
) -> UseColorSliderStateReturn<C> {
    let UseColorSliderStateInput {
        default_value,
        channel,
        disabled,
        orientation,
        on_change,
        on_change_end,
    } = *input;

    let range = C::get_channel_range(channel);

    // Hook-owned color state.
    let (color, set_color_signal) = signal(default_value);

    // Internal update helper.
    let update_color = move |new_color: C| {
        set_color_signal.set(new_color);
        if let Some(cb) = on_change {
            cb.run(new_color);
        }
    };

    // Controlled slider values derived from the color channel.
    let slider_values = Signal::derive(move || vec![color.get().get_channel_value(channel)]);

    let slider_on_change = Callback::new(move |values: Vec<f64>| {
        if let Some(&val) = values.first() {
            let current = color.get_untracked();
            let new_color = current.with_channel_value(channel, val);
            update_color(new_color);
        }
    });

    let slider_on_change_end = on_change_end.map(|cb| {
        Callback::new(move |values: Vec<f64>| {
            if let Some(&val) = values.first() {
                let current = color.get_untracked();
                cb.run(current.with_channel_value(channel, val));
            }
        })
    });

    let mut slider_state = use_slider_state(UseSliderStateInput {
        values: SliderValues::Controlled(slider_values),
        min_value: range.min_value,
        max_value: range.max_value,
        step: Some(range.step),
        disabled,
        orientation,
        on_change: Some(slider_on_change),
        on_change_end: slider_on_change_end,
    });

    // Override the auto-computed page_size with the channel's intended value.
    slider_state.page_size = range.page_size;

    let set_value = Callback::new(move |new_color: C| {
        update_color(new_color);
    });

    // Display color for gradient rendering (channel-specific).
    let display_color = Signal::derive(move || color.get().get_display_color(channel));

    // Formatted channel value label.
    let thumb_value_label = Signal::derive(move || color.get().format_channel_value(channel));

    let is_dragging = Signal::derive(move || slider_state.is_thumb_dragging.run(0));

    UseColorSliderStateReturn {
        value: color.into(),
        set_value,
        slider_state,
        display_color,
        thumb_value_label,
        channel,
        is_dragging,
    }
}
