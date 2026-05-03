use std::fmt;

use leptos::prelude::*;

use crate::utils::color::ColorValue;

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/color/src/useColorChannelFieldState.ts

// ## INTENTIONAL DEVIATIONS
//
// - Hook-owned state: The hook owns its color state internally.
//
// - Does not wrap `useNumberFieldState` directly. Instead, provides the
//   channel value as a signal that can be passed to `use_number_field`.

/// Input parameters for `use_color_channel_field_state`.
#[derive(Debug, Clone)]
pub struct UseColorChannelFieldStateInput<C: ColorValue> {
    /// The initial color value.
    pub default_value: C,

    /// Which channel this field edits.
    pub channel: C::Channel,

    /// Callback fired when the color changes.
    pub on_change: Option<Callback<C>>,
}

/// Return value of `use_color_channel_field_state`.
pub struct UseColorChannelFieldStateReturn<C: ColorValue> {
    /// The current full color.
    pub color_value: Signal<C>,

    /// Update the full color.
    pub set_color_value: Callback<C>,

    /// The value of the specific channel (for binding to a number input).
    pub channel_value: Signal<Option<f64>>,

    /// Update the channel value (maps back to full color).
    pub set_channel_value: Callback<Option<f64>>,

    /// The channel this field edits.
    pub channel: C::Channel,

    /// The minimum value for this channel.
    pub min_value: f64,

    /// The maximum value for this channel.
    pub max_value: f64,

    /// The step value for this channel.
    pub step: f64,
}

impl<C: ColorValue> Clone for UseColorChannelFieldStateReturn<C> {
    fn clone(&self) -> Self {
        Self {
            color_value: self.color_value,
            set_color_value: self.set_color_value,
            channel_value: self.channel_value,
            set_channel_value: self.set_channel_value,
            channel: self.channel,
            min_value: self.min_value,
            max_value: self.max_value,
            step: self.step,
        }
    }
}

impl<C: ColorValue> fmt::Debug for UseColorChannelFieldStateReturn<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseColorChannelFieldStateReturn")
            .field("channel", &self.channel)
            .finish_non_exhaustive()
    }
}

/// Creates state for a single-channel numeric input field.
///
/// Bridges between a full color value and a number input for one channel.
/// Changes to the channel value are mapped back to the full color via
/// `with_channel_value`.
pub fn use_color_channel_field_state<C: ColorValue>(
    input: &UseColorChannelFieldStateInput<C>,
) -> UseColorChannelFieldStateReturn<C> {
    let UseColorChannelFieldStateInput {
        default_value,
        channel,
        on_change,
    } = *input;

    let range = C::get_channel_range(channel);

    let (color_value, set_color_value_signal) = signal(default_value);

    let channel_value = Signal::derive(move || Some(color_value.get().get_channel_value(channel)));

    let update_color = move |new_color: C| {
        set_color_value_signal.set(new_color);
        if let Some(cb) = on_change {
            cb.run(new_color);
        }
    };

    let set_color_value = Callback::new(move |new_color: C| {
        update_color(new_color);
    });

    let set_channel_value = Callback::new(move |val: Option<f64>| {
        if let Some(v) = val {
            let clamped = v.clamp(range.min_value, range.max_value);
            let current = color_value.get_untracked();
            let new_color = current.with_channel_value(channel, clamped);
            update_color(new_color);
        }
    });

    UseColorChannelFieldStateReturn {
        color_value: color_value.into(),
        set_color_value,
        channel_value,
        set_channel_value,
        channel,
        min_value: range.min_value,
        max_value: range.max_value,
        step: range.step,
    }
}
