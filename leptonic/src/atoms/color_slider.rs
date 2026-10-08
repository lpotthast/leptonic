//! Headless color slider atoms: a slider changing one channel of a color.
// Upstream: react-aria-components/src/ColorSlider.tsx @ 99e6102368

use leptos::{context::Provider, prelude::*};

use super::{
    color_picker::ColorPickerContext,
    color_thumb::{ColorThumbContext, SliderThumbParts, ThumbParts},
    field::{LabelContext, LabelPresence},
};
use crate::{
    Out,
    hooks::{
        IntoAttrs, UseColorSliderInput, UseColorSliderReturn, UseColorSliderStateInput,
        UseHoverInput, UseLabelProps, UseSliderGroupAttrs, UseSliderOutputAttrs,
        UseSliderTrackAttrs, use_color_slider, use_color_slider_state, use_hover,
    },
    utils::{
        ValueBinding,
        classes::Classes,
        color::{ColorChannel, ColorValue},
        data_attributes::flag,
        default_class::with_default_class,
        orientation::Orientation,
        styles::Styles,
    },
};
use crate::utils::i18n::use_locale;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Its own track and output atoms (`ColorSliderTrack`, `ColorSliderOutput`; react-aria-
//   components reuses `SliderTrack`/`SliderOutput` through their contexts). As upstream, the
//   track is the slider's group (`role="group"`, labelled by the `Label`), and a `Label` without
//   children shows the channel's name.
// - The value is split into `value` (a value or any signal) and `set_value` (an `Out`), plus
//   `default_value` and `on_change` (C4).
//
// =============================================================================

/// What the parts of a [`ColorSlider`] get from it.
#[derive(Clone, Copy)]
struct ColorSliderContext {
    orientation: Signal<Orientation>,
    is_disabled: Signal<bool>,
    /// The track's attributes (with the group's) and styles, cloned per render.
    track: StoredValue<(UseSliderGroupAttrs, UseSliderTrackAttrs, Styles)>,
    output: StoredValue<UseSliderOutputAttrs>,
    formatted: Signal<String>,
}

/// A slider changing one channel of a color: a `Label`, a [`ColorSliderOutput`] and a
/// [`ColorSliderTrack`] with a [`ColorThumb`](super::color_thumb::ColorThumb).
///
/// ```ignore
/// <ColorSlider default_value=HSV::default() channel=HsvChannel::Hue>
///     <Label>"Hue"</Label>
///     <ColorSliderOutput />
///     <ColorSliderTrack>
///         <ColorThumb />
///     </ColorSliderTrack>
/// </ColorSlider>
/// ```
///
/// Data attributes: `data-orientation`, `data-disabled`.
///
/// Default class: `leptonic-ColorSlider`.
#[component]
#[allow(clippy::too_many_arguments)]
pub fn ColorSlider<Ch: ColorChannel<Color: Default>>(
    /// The channel the slider changes.
    channel: Ch,
    /// The initial color. Default: the color type's default.
    #[prop(optional)]
    default_value: Option<Ch::Color>,
    /// The color (controlled): a value or any signal. Default: the `ColorPicker`'s around it.
    #[prop(into, optional)]
    value: Option<Signal<Ch::Color>>,
    /// Receives the new color: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Ch::Color>>,
    /// Called with the color whenever it changes, also while dragging.
    #[prop(into, optional)]
    on_change: Option<Callback<Ch::Color>>,
    /// Called with the color when the user stops dragging.
    #[prop(into, optional)]
    on_change_end: Option<Callback<Ch::Color>>,
    #[prop(into, default = Signal::stored(Orientation::Horizontal))] orientation: Signal<
        Orientation,
    >,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// Names the slider when it has no `Label`. Without either, the channel's name does.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    /// The name of the input, for form submission.
    #[prop(into, optional)]
    name: Option<String>,
    /// The id of a `<form>` the input belongs to.
    #[prop(into, optional)]
    form: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ColorSlider", classes);
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    let (binding, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let binding = binding.or_else(ColorPickerContext::binding::<Ch::Color>);
    let state = use_color_slider_state(UseColorSliderStateInput {
        value: binding,
        is_disabled,
        orientation,
        on_change,
        on_change_end,
        default_value: default_value.unwrap_or_default(),
        channel,
    });
    let UseColorSliderReturn {
        slider,
        thumb,
        track_styles,
        input_styles,
    } = use_color_slider(UseColorSliderInput {
        has_label: label_presence.has_label,
        aria_label,
        aria_labelledby,
        name,
        form,
        state,
        aria_describedby: None,
    });

    let label = LabelContext::span(UseLabelProps {
        id: slider.label_props.id,
        html_for: None,
    })
    .with_on_click(slider.label_props.on_click)
    .with_presence(label_presence)
    .with_default_text({
        let locale = use_locale();
        Signal::derive(move || <Ch::Color as ColorValue>::channel_name(channel, &locale.get()))
    });
    let (track_attrs, slider_track_styles) = slider.track_props.into_parts();
    let context = ColorSliderContext {
        orientation,
        is_disabled,
        track: StoredValue::new((
            slider.group_props.into_attrs(),
            track_attrs,
            slider_track_styles.merge(track_styles),
        )),
        output: StoredValue::new(slider.output_props.into_attrs()),
        formatted: state.formatted_value(),
    };
    let color = state.display_color();
    let thumb_context = ColorThumbContext::new(
        Signal::derive(move || color.get().to_css_string()),
        state.is_dragging,
        thumb.is_disabled,
        ThumbParts::Slider(Box::new(SliderThumbParts {
            thumb: thumb.thumb_props,
            input: thumb.input_props,
            input_styles,
        })),
    );

    view! {
        <Provider value=context>
            <Provider value=thumb_context>
                <Provider value=label>
                    <div
                        class=classes
                        style=styles
                        data-orientation=move || orientation.get().as_str()
                        data-disabled=flag(is_disabled)
                    >
                        {children()}
                    </div>
                </Provider>
            </Provider>
        </Provider>
    }
}

/// The track of the [`ColorSlider`] around it, drawn with the channel's gradient; put the
/// [`ColorThumb`](super::color_thumb::ColorThumb) in it. It is the slider's group
/// (`role="group"`, named by the slider's label).
///
/// Data attributes: `data-hovered`, `data-orientation`, `data-disabled`.
///
/// # Panics
///
/// Outside a [`ColorSlider`].
///
/// Default class: `leptonic-ColorSliderTrack`.
#[component]
pub fn ColorSliderTrack(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ColorSliderTrack", classes);
    let ColorSliderContext {
        orientation,
        is_disabled,
        track,
        ..
    } = expect_context::<ColorSliderContext>();
    let (group_attrs, attrs, track_styles) = track.get_value();
    let hover = use_hover(UseHoverInput {
        is_disabled,
        ..UseHoverInput::default()
    });
    view! {
        <div
            {..group_attrs}
            {..attrs}
            {..hover.props.into_attrs()}
            class=classes
            style=track_styles.merge(styles)
            data-hovered=flag(hover.is_hovered)
            data-orientation=move || orientation.get().as_str()
            data-disabled=flag(is_disabled)
        >
            {children()}
        </div>
    }
}

/// The channel's formatted value of the [`ColorSlider`] around it.
///
/// Data attributes: `data-orientation`, `data-disabled`.
///
/// # Panics
///
/// Outside a [`ColorSlider`].
///
/// Default class: `leptonic-ColorSliderOutput`.
#[component]
pub fn ColorSliderOutput(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ColorSliderOutput", classes);
    let ColorSliderContext {
        orientation,
        is_disabled,
        output,
        formatted,
        ..
    } = expect_context::<ColorSliderContext>();
    let attrs = output.get_value();
    view! {
        <output
            {..attrs}
            class=classes
            style=styles
            data-orientation=move || orientation.get().as_str()
            data-disabled=flag(is_disabled)
        >
            {formatted}
        </output>
    }
}
