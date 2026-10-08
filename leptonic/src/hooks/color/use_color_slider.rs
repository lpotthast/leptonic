// Upstream: react-aria/src/color/useColorSlider.ts @ 99e6102368
use leptos::prelude::*;

use super::use_color_slider_state::ColorSliderState;
use crate::{
    hooks::slider::{
        UseSliderInput, UseSliderReturn, UseSliderThumbInput, UseSliderThumbReturn, use_slider,
        use_slider_thumb,
    },
    utils::{
        color::ColorValue,
        css::ForcedColorAdjust,
        i18n::{use_direction, use_locale},
        locale::WritingDirection,
        orientation::Orientation,
        style::ForcedColorAdjustProperty,
        styles::Styles,
        visually_hidden::visually_hidden_full_size_styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns the `use_slider` and `use_slider_thumb` results with the color's additions: the
//   value text and `track_styles` (the gradient) and `input_styles` (visually hidden).
// - Disabled and orientation come from the state (C8), not repeated on the input.
//
// =============================================================================

/// Input of [`use_color_slider`].
#[derive(Debug, Clone)]
pub struct UseColorSliderInput<C: ColorValue> {
    pub state: ColorSliderState<C>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    /// Names the slider. Without any label, the channel's name does.
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    /// The name of the input, for form submission.
    pub name: Option<String>,
    /// The id of a `<form>` the input belongs to.
    pub form: Option<String>,
}

/// Return value of [`use_color_slider`].
#[derive(Debug)]
pub struct UseColorSliderReturn {
    /// The slider: label, output and track (whose styles are `track_styles`). Its
    /// `group_props` go on the track (react-aria merges them into the track props): the track is
    /// the slider's group.
    pub slider: UseSliderReturn,
    /// The thumb and its input (with the color's value text).
    pub thumb: UseSliderThumbReturn,
    /// The track's gradient (merge with the track props' styles).
    pub track_styles: Styles,
    /// Hides the input visually across the thumb (merge into the input's style).
    pub input_styles: Styles,
}

/// Behavior and accessibility of a slider changing one channel of a color, on top of
/// [`use_slider`](fn@use_slider) and [`use_slider_thumb`](fn@use_slider_thumb): a gradient track, the channel's value text.
pub fn use_color_slider<C: ColorValue>(input: UseColorSliderInput<C>) -> UseColorSliderReturn {
    let UseColorSliderInput {
        state,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        name,
        form,
    } = input;
    let channel = state.channel;

    // Without any label, the channel names the slider (react-aria).
    let has_other_label = aria_labelledby.is_some();
    let locale = use_locale();
    let aria_label = MaybeProp::derive(move || {
        aria_label.get().or_else(|| {
            (!has_label.get() && !has_other_label).then(|| C::channel_name(channel, &locale.get()))
        })
    });

    let slider = use_slider(UseSliderInput {
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        state: state.slider,
        id: None,
        aria_details: None,
    });
    let mut thumb = use_slider_thumb(UseSliderThumbInput {
        is_disabled: state.slider.is_disabled,
        name,
        form,
        state: state.slider,
        slider: slider.data.clone(),
        track: slider.track_element,
        index: 0,
        is_required: Signal::default(),
        is_invalid: Signal::default(),
        has_label: Signal::stored(false),
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        aria_describedby: None,
        aria_errormessage: None,
        aria_details: None,
    });
    let value = state.value;
    let formatted = state.formatted_value();
    thumb.input_props.aria_valuetext = Signal::derive(move || {
        // The hue names a hue slider, the color the other channels, nothing an alpha slider
        // (react-aria).
        let color = value.get();
        let text = formatted.get();
        if C::is_alpha_channel(channel) {
            text
        } else if C::hue_channel() == Some(channel) {
            format!("{text}, {}", color.hue_name(&locale.get()))
        } else {
            format!("{text}, {}", color.color_name(&locale.get()))
        }
    });

    let display_color = state.display_color();
    let orientation = state.slider.orientation;
    let direction = use_direction();
    let background = move || {
        let color = display_color.get();
        let range = C::channel_range(channel);
        let stops: Vec<String> = match range.gradient_stops {
            Some(stops) => stops
                .iter()
                .map(|&stop| color.with_channel_value(channel, stop).to_css_string())
                .collect(),
            None => [range.min_value, range.max_value]
                .map(|stop| color.with_channel_value(channel, stop).to_css_string())
                .to_vec(),
        };
        let to = match (orientation.get(), direction.get()) {
            (Orientation::Vertical, _) => "top",
            (Orientation::Horizontal, WritingDirection::Ltr) => "right",
            (Orientation::Horizontal, WritingDirection::Rtl) => "left",
        };
        Some(format!("linear-gradient(to {to}, {})", stops.join(", ")))
    };

    UseColorSliderReturn {
        slider,
        thumb,
        track_styles: Styles::new()
            .add(ForcedColorAdjustProperty.declare(ForcedColorAdjust::None))
            // A computed gradient: no checked grammar in `leptos-css` yet.
            .add_optional_unchecked("background", background),
        input_styles: visually_hidden_full_size_styles(),
    }
}
