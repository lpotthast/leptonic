// Upstream: react-aria-components/src/ColorSlider.tsx @ 99e6102368
// Upstream: react-aria-components/test/ColorSlider.test.js @ 99e6102368
//! Headless color slider atoms: a slider changing one channel of a color.

use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;

use super::{
    color_picker::ColorPickerContext,
    color_thumb::{ColorThumbContext, ThumbParts},
    field::{LabelContext, LabelPresence},
};
use crate::{
    Orientation, Out, Part, PropsWithStyles, ValueBinding,
    hooks::{
        color::{
            UseColorSliderInput, UseColorSliderReturn, UseColorSliderStateInput, use_color_slider,
            use_color_slider_state,
        },
        form::UseLabelProps,
        interactions::{UseHoverInput, use_hover},
        slider::{
            UseSliderGroupProps, UseSliderLabelProps, UseSliderOutputProps, UseSliderTrackProps,
        },
    },
    utils::{
        color::{ColorChannel, ColorValue},
        data_attributes::data_flags,
        default_class::with_default_class,
        i18n::use_locale,
        signal_attr::signal_attr,
        styles::Styles,
    },
};

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
#[derive(Clone)]
struct ColorSliderContext {
    orientation: Signal<Orientation>,
    is_disabled: Signal<bool>,
    /// The track, also the slider's group (with the gradient styles).
    track: Part<PropsWithStyles<UseSliderTrackProps>>,
    group: Part<UseSliderGroupProps>,
    track_styles: Styles,
    output: Part<UseSliderOutputProps>,
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
    /// Default: horizontal.
    #[prop(into, default = Signal::stored(Orientation::Horizontal))]
    orientation: Signal<Orientation>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// Names the slider when it has no `Label`. Without either, the channel's name does.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: MaybeProp<String>,
    /// Elements describing the slider.
    #[prop(into, optional)]
    aria_describedby: MaybeProp<String>,
    /// Elements with details about the slider.
    #[prop(into, optional)]
    aria_details: MaybeProp<String>,
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
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-ColorSlider", classes);
    let label_presence = LabelPresence::new(aria_label, aria_labelledby);
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
        aria_labelledby: crate::utils::id::id_ref_prop(aria_labelledby),
        name,
        form,
        state,
        aria_describedby: crate::utils::id::id_ref_prop(aria_describedby),
        aria_details: crate::utils::id::id_ref_prop(aria_details),
    });

    let UseSliderLabelProps { id, on_click } = slider.label.into_props();
    let label = LabelContext::span(Part::stored(UseLabelProps { id, html_for: None }))
        .with_on_click(on_click)
        .with_presence(label_presence)
        .with_default_text({
            let locale = use_locale();
            Signal::derive(move || <Ch::Color as ColorValue>::channel_name(channel, &locale.get()))
        });
    let context = ColorSliderContext {
        orientation,
        is_disabled,
        track: slider.track,
        group: slider.group,
        track_styles,
        output: slider.output,
        formatted: state.formatted_value(),
    };
    let color = state.display_color();
    let thumb_context = ColorThumbContext::new(
        Signal::derive(move || color.get().to_css_string()),
        state.is_dragging,
        thumb.is_disabled,
        ThumbParts::Slider {
            thumb: thumb.thumb,
            input: thumb
                .input
                .map(move |input| PropsWithStyles::new(input, input_styles.clone())),
        },
    );

    view! {
        <Provider value=context>
            <Provider value=thumb_context>
                <Provider value=label>
                    <div
                        class=classes
                        style={styles}
                        {..signal_attr("data-orientation", orientation)}
                        {..data_flags([("data-disabled", is_disabled)])}
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
/// Outside a [`ColorSlider`] it renders nothing (with a development warning).
///
/// Default class: `leptonic-ColorSliderTrack`.
#[component]
pub fn ColorSliderTrack(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-ColorSliderTrack", classes);
    let ColorSliderContext {
        orientation,
        is_disabled,
        track,
        group,
        track_styles,
        ..
    } = expect_color_slider()?;
    let (attrs, slider_track_styles) = track.attrs();
    let track_styles = slider_track_styles.merge(track_styles);
    // Hovered also while disabled (react-aria-components' `SliderTrack`).
    let hover = use_hover(UseHoverInput::default());
    Some(view! {
        <div
            {..group.attrs()}
            {..attrs}
            {..hover.target.attrs()}
            class=classes
            style=track_styles.merge(styles)
            {..data_flags([("data-hovered", hover.is_hovered)])}
            {..signal_attr("data-orientation", orientation)}
            {..data_flags([("data-disabled", is_disabled)])}
        >
            {children()}
        </div>
    })
}

/// The channel's formatted value of the [`ColorSlider`] around it.
///
/// Data attributes: `data-orientation`, `data-disabled`.
///
/// Outside a [`ColorSlider`] it renders nothing (with a development warning).
///
/// Default class: `leptonic-ColorSliderOutput`.
#[component]
pub fn ColorSliderOutput(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-ColorSliderOutput", classes);
    let ColorSliderContext {
        orientation,
        is_disabled,
        output,
        formatted,
        ..
    } = expect_color_slider()?;
    let attrs = output.attrs();
    Some(view! {
        <output
            {..attrs}
            class=classes
            style={styles}
            {..signal_attr("data-orientation", orientation)}
            {..data_flags([("data-disabled", is_disabled)])}
        >
            {formatted}
        </output>
    })
}

/// The context of the [`ColorSlider`] around a part, or a development warning.
fn expect_color_slider() -> Option<ColorSliderContext> {
    let context = use_context::<ColorSliderContext>();
    if context.is_none() {
        crate::utils::dev_warn!("ColorSlider parts must be inside a <ColorSlider>.");
    }
    context
}
