use std::borrow::Cow;

use leptos::prelude::*;

use crate::{
    hooks::slider::{
        SliderOrientation, UseSliderInput, UseSliderReturn, UseSliderThumbInput,
        UseSliderThumbReturn, use_slider, use_slider_thumb,
    },
    utils::color::ColorValue,
};

use super::use_color_slider_state::UseColorSliderStateReturn;

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/color/src/useColorSlider.ts

// ## INTENTIONAL DEVIATIONS
//
// - Delegates to `use_slider` and `use_slider_thumb` for interaction logic.
//   Adds color-specific gradient computation and ARIA valuetext.

/// Input parameters for `use_color_slider`.
#[derive(Debug, Clone)]
pub struct UseColorSliderInput<C: ColorValue> {
    /// The color slider state (from `use_color_slider_state`).
    pub state: UseColorSliderStateReturn<C>,

    /// Whether the slider is disabled.
    pub disabled: Signal<bool>,

    /// Slider orientation.
    pub orientation: Signal<SliderOrientation>,

    /// An accessibility label for the slider. When `None`, auto-generates
    /// from the channel name (e.g., "Hue", "Saturation").
    pub aria_label: Option<&'static str>,

    /// Whether to use RTL layout.
    pub is_rtl: bool,

    /// The name attribute for form submission.
    pub name: Option<&'static str>,
}

/// Return value of `use_color_slider`.
pub struct UseColorSliderReturn {
    /// The underlying slider hook return (group, label, output, track props).
    pub slider: UseSliderReturn,

    /// The slider thumb hook return (thumb, input props).
    pub thumb: UseSliderThumbReturn,

    /// CSS gradient background for the track.
    pub background: Signal<String>,

    /// CSS style string for the track including gradient and `forced-color-adjust: none`.
    pub track_style: Signal<String>,
}

/// Creates behavior and ARIA props for a color channel slider.
///
/// Composes `use_slider` and `use_slider_thumb` with color-specific
/// gradient generation and ARIA labels.
pub fn use_color_slider<C: ColorValue>(input: UseColorSliderInput<C>) -> UseColorSliderReturn {
    let UseColorSliderInput {
        state,
        disabled,
        orientation,
        aria_label,
        is_rtl,
        name,
    } = input;

    let channel = state.channel;

    // Auto-generate aria-label from channel name when none is provided.
    let effective_label = aria_label.unwrap_or_else(|| C::get_channel_name(state.channel));

    let slider_return = use_slider(UseSliderInput {
        state: state.slider_state,
        aria_label: Some(effective_label),
        aria_labelledby: None,
        is_rtl,
    });

    // Build enriched ARIA valuetext with hue/color name appended.
    let value_signal = state.value;
    let enriched_valuetext = Signal::derive(move || {
        let color = value_signal.get();
        let mut text = color.format_channel_value(channel);
        if let Some(hue_name) = color.get_hue_name_for_channel(channel) {
            text.push_str(", ");
            text.push_str(hue_name);
        }
        text
    });

    let thumb_return = use_slider_thumb(UseSliderThumbInput {
        state: state.slider_state,
        track: slider_return.track_ref,
        index: 0,
        name,
        aria_label: Some(Cow::Borrowed(effective_label)),
        aria_labelledby: None,
        disabled,
        validation_state: crate::hooks::ValidationState::Valid,
        is_rtl,
        decimal_places: None,
        is_required: false,
        aria_describedby: None,
        aria_details: None,
        aria_errormessage: None,
        aria_valuetext: Some(enriched_valuetext),
    });

    // Generate gradient background using display_color and channel-specific stops.
    let display_color_signal = state.display_color;
    let background = Signal::derive(move || {
        let color = display_color_signal.get();
        let range = C::get_channel_range(channel);

        let stops: Vec<String> = if let Some(fixed_stops) = range.gradient_stops {
            fixed_stops
                .iter()
                .map(|&val| color.with_channel_value(channel, val).to_css_string())
                .collect()
        } else {
            vec![
                color
                    .with_channel_value(channel, range.min_value)
                    .to_css_string(),
                color
                    .with_channel_value(channel, range.max_value)
                    .to_css_string(),
            ]
        };

        let direction = match orientation.get_untracked() {
            SliderOrientation::Horizontal => {
                if is_rtl {
                    "to left"
                } else {
                    "to right"
                }
            }
            SliderOrientation::Vertical => "to top",
        };

        format!("linear-gradient({direction}, {})", stops.join(", "))
    });

    // Combined track style with forced-color-adjust for high contrast mode.
    let track_style = Signal::derive(move || {
        format!(
            "background: {}; forced-color-adjust: none;",
            background.get()
        )
    });

    UseColorSliderReturn {
        slider: slider_return,
        thumb: thumb_return,
        background,
        track_style,
    }
}
