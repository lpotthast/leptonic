use leptonic::{
    atoms::prelude::{
        ColorArea, ColorField, ColorPicker, ColorSlider, ColorSliderOutput, ColorSliderTrack,
        ColorSwatch, ColorSwatchPicker, ColorSwatchPickerItems, ColorThumb, Input, Label,
    },
    utils::color::{Color, ColorValue, HSV, HsvChannel, RGB8},
};
use leptos::prelude::*;

const PRESETS: [RGB8; 4] = [
    RGB8 { r: 170, g: 0, b: 0 },
    RGB8 {
        r: 255,
        g: 136,
        b: 0,
    },
    RGB8 { r: 0, g: 136, b: 0 },
    RGB8 {
        r: 0,
        g: 136,
        b: 255,
    },
];

#[component]
pub fn ColorPickerAtomDemo() -> impl IntoView {
    // Every atom inside the picker shows and changes this color, each in its own color space.
    let color = RwSignal::new(Color::from(HSV {
        hue: 210.0,
        saturation: 0.6,
        brightness: 0.8,
    }));

    view! {
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
                        <ColorSliderOutput classes="demo-color-atoms-slider-output"/>
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
                    <ColorSwatchPicker<RGB8> colors=PRESETS.to_vec() aria_label="Presets" classes="demo-color-swatch-picker">
                        <ColorSwatchPickerItems classes="demo-color-swatch-picker-item">
                            <ColorSwatch classes="demo-color-swatch-picker-swatch"/>
                        </ColorSwatchPickerItems>
                    </ColorSwatchPicker<RGB8>>
                </div>
            </div>
        </ColorPicker>
        <p class="demo-status">
            {move || {
                let rgb = color.get().to::<RGB8>();
                format!("Color: {rgb}, {}", rgb.color_name())
            }}
        </p>
    }
}
