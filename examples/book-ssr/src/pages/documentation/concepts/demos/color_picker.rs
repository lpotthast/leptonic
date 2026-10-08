use leptonic::{
    atoms::prelude::{
        ColorArea, ColorField, ColorPicker, ColorSlider, ColorSliderTrack, ColorSwatch, ColorThumb,
        Input, Label,
    },
    utils::{
        color::{Color, ColorValue, HSL, HSV, HsvChannel},
        i18n::use_locale,
    },
};
use leptos::prelude::*;

#[component]
pub fn ColorPickerConceptDemo() -> impl IntoView {
    let locale = use_locale();
    let color = RwSignal::new(Color::from(HSV {
        hue: 210.0,
        saturation: 0.6,
        brightness: 0.8,
    }));

    view! {
        // The atoms inside the picker show and change its color, each in its own color space.
        <ColorPicker value=color set_value=color>
            <div class="demo-color-atoms">
                <ColorArea<HSV>
                    x_channel=HsvChannel::Saturation
                    y_channel=HsvChannel::Brightness
                    aria_label="Saturation and brightness"
                    classes="demo-color-atoms-area"
                >
                    <ColorThumb classes="demo-color-atoms-thumb"/>
                </ColorArea<HSV>>
                <div class="demo-color-atoms-sliders">
                    <ColorSlider channel=HsvChannel::Hue classes="demo-color-atoms-slider">
                        <Label>"Hue"</Label>
                        <ColorSliderTrack classes="demo-color-atoms-slider-track">
                            <ColorThumb classes="demo-color-atoms-thumb"/>
                        </ColorSliderTrack>
                    </ColorSlider>
                    <div class="demo-color-atoms-result">
                        <ColorSwatch classes="demo-color-atoms-swatch"/>
                        <ColorField classes="demo-field">
                            <Label classes="demo-field-label">"Hex"</Label>
                            <Input classes="demo-color-atoms-input"/>
                        </ColorField>
                    </div>
                </div>
            </div>
        </ColorPicker>
        <p class="demo-status">
            {move || {
                let hsl = color.get().to::<HSL>();
                format!(
                    "hsl({:.0}, {:.0}%, {:.0}%), {}",
                    hsl.hue,
                    hsl.saturation * 100.0,
                    hsl.lightness * 100.0,
                    hsl.color_name(&locale.get()),
                )
            }}
        </p>
    }
}
