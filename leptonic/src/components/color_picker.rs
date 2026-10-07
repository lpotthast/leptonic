//! Styled color picker components, built on the color atoms.

use leptos::prelude::*;

use crate::{
    Out,
    atoms::{
        color_area::ColorArea,
        color_field::{ColorChannelField, ColorField},
        color_picker::ColorPicker as ColorPickerAtom,
        color_slider::{ColorSlider, ColorSliderTrack},
        color_swatch::ColorSwatch,
        color_thumb::ColorThumb,
        field::Label,
        input::Input,
    },
    utils::{
        classes::Classes,
        color::{Color, ColorChannel, ColorProp, ColorValue, HSV, HsvChannel, RgbChannel},
        styles::Styles,
    },
};

/// A square showing a color (a [`ColorSwatch`]).
#[component]
pub fn ColorPreview(
    /// The color: any color value or signal of one. Default: the `ColorPicker`'s around it.
    #[prop(into, optional)]
    color: Option<ColorProp>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    view! {
        <ColorSwatch
            nostrip:color=color
            classes=classes.add("leptonic-color-preview")
            styles=styles
        />
    }
}

/// A 2D area for the saturation (x) and brightness (y) of an HSV color (a [`ColorArea`]).
#[component]
pub fn ColorPalette(
    /// The initial color.
    #[prop(optional)]
    default_value: Option<HSV>,
    /// The color: a value or any signal. Default: the `ColorPicker`'s around it.
    #[prop(into, optional)]
    value: Option<Signal<HSV>>,
    /// Receives the new color: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<HSV>>,
    /// Called with every new color.
    #[prop(into, optional)]
    on_change: Option<Callback<HSV>>,
    /// Names the palette. Default: "Color picker".
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    view! {
        <ColorArea<HSV>
            nostrip:default_value=default_value
            nostrip:value=value
            nostrip:set_value=set_value
            nostrip:on_change=on_change
            x_channel=HsvChannel::Saturation
            y_channel=HsvChannel::Brightness
            aria_label=aria_label
            is_disabled=is_disabled
            classes=classes.add("leptonic-color-palette")
            styles=styles
        >
            <ColorThumb classes="leptonic-color-palette-knob" />
        </ColorArea<HSV>>
    }
}

/// A slider for the hue of an HSV color (a [`ColorSlider`]).
#[component]
pub fn HueSlider(
    /// The initial color.
    #[prop(optional)]
    default_value: Option<HSV>,
    /// The color: a value or any signal. Default: the `ColorPicker`'s around it.
    #[prop(into, optional)]
    value: Option<Signal<HSV>>,
    /// Receives the new color: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<HSV>>,
    /// Called with every new color.
    #[prop(into, optional)]
    on_change: Option<Callback<HSV>>,
    /// Names the slider. Default: "Hue".
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    view! {
        <ColorSlider
            channel=HsvChannel::Hue
            nostrip:default_value=default_value
            nostrip:value=value
            nostrip:set_value=set_value
            nostrip:on_change=on_change
            aria_label=aria_label
            is_disabled=is_disabled
            classes=classes.add("leptonic-hue-slider")
            styles=styles
        >
            <ColorSliderTrack classes="leptonic-hue-slider-track">
                <ColorThumb classes="leptonic-hue-slider-thumb" />
            </ColorSliderTrack>
        </ColorSlider>
    }
}

/// A field for one channel of the `ColorPicker`'s color.
fn channel_field<Ch: ColorChannel<Color: Default>>(channel: Ch) -> impl IntoView {
    view! {
        <ColorChannelField channel=channel classes=["leptonic-text-field", "leptonic-color-picker-field"]>
            <Label classes="leptonic-field-label">{<Ch::Color as ColorValue>::channel_name(channel)}</Label>
            <Input classes="leptonic-text-field-input" />
        </ColorChannelField>
    }
}

/// A color picker: a preview, a saturation and brightness palette, a hue slider, fields for the
/// HSB and RGB channels and the hex code, all showing and changing one color.
#[component]
pub fn ColorPicker(
    /// The initial color. Default: black.
    #[prop(optional)]
    default_value: Option<Color>,
    /// The color (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<Color>>,
    /// Receives the color: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Color>>,
    /// Called with every new color.
    #[prop(into, optional)]
    on_change: Option<Callback<Color>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    view! {
        <ColorPickerAtom
            nostrip:default_value=default_value
            nostrip:value=value
            nostrip:set_value=set_value
            nostrip:on_change=on_change
        >
            <div class=classes.add("leptonic-color-picker") style=styles>
                <div class="leptonic-color-picker-main">
                    <ColorPreview />
                    <ColorPalette />
                </div>
                <HueSlider />
                <div class="leptonic-color-picker-fields">
                    {channel_field(HsvChannel::Hue)}
                    {channel_field(HsvChannel::Saturation)}
                    {channel_field(HsvChannel::Brightness)}
                </div>
                <div class="leptonic-color-picker-fields">
                    {channel_field(RgbChannel::Red)}
                    {channel_field(RgbChannel::Green)}
                    {channel_field(RgbChannel::Blue)}
                </div>
                <ColorField classes=["leptonic-text-field", "leptonic-color-picker-field"]>
                    <Label classes="leptonic-field-label">"Hex"</Label>
                    <Input classes="leptonic-text-field-input" />
                </ColorField>
            </div>
        </ColorPickerAtom>
    }
}
