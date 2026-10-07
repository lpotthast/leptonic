use leptonic::{
    atoms::{
        color_area::ColorArea,
        color_field::ColorField,
        color_picker::ColorPicker,
        color_slider::{ColorSlider, ColorSliderTrack},
        color_swatch::ColorSwatch,
        color_thumb::ColorThumb,
        input::Input,
    },
    utils::{
        color::{AlphaChannel, Color, HSV, HsvChannel, RGB8},
        styles::Styles,
    },
};
use leptos::prelude::*;

/// `ColorPicker` atom (react-aria-components' `ColorPicker.test.js`): a swatch, an HSV area, a
/// hue slider and a hex field sharing one color. Its colors are logged to `#test-cp-log`.
#[component]
pub fn PageAtomColorPicker() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let size = |width: &'static str, height: &'static str| {
        Styles::new()
            .add_unchecked("width", width)
            .add_unchecked("height", height)
    };
    view! {
        <div id="test-page-atom-color-picker">
            <h1>"Color picker"</h1>
            <ColorPicker
                default_value=Color::from(RGB8::from_hex_int(0xFF_00_00))
                on_change=move |c: Color| log.update(|l| l.push(format!("{:X}", c.to::<RGB8>())))
            >
                <div id="test-cp-swatch"><ColorSwatch styles=size("20px", "20px") /></div>
                <div id="test-cp-area">
                    <ColorArea<HSV>
                        x_channel=HsvChannel::Saturation
                        y_channel=HsvChannel::Brightness
                        styles=size("100px", "100px")
                    >
                        <ColorThumb />
                    </ColorArea<HSV>>
                </div>
                <div id="test-cp-hue">
                    <ColorSlider channel=HsvChannel::Hue>
                        <ColorSliderTrack styles=size("100px", "20px")>
                            <ColorThumb />
                        </ColorSliderTrack>
                    </ColorSlider>
                </div>
                <div id="test-cp-alpha">
                    <ColorSlider channel={AlphaChannel::<HsvChannel>::Alpha}>
                        <ColorSliderTrack styles=size("100px", "20px")>
                            <ColorThumb />
                        </ColorSliderTrack>
                    </ColorSlider>
                </div>
                <div id="test-cp-field">
                    <ColorField aria_label="hex">
                        <Input />
                    </ColorField>
                </div>
            </ColorPicker>
            <button id="test-cp-after">"After"</button>
            <div>"Log: " <span id="test-cp-log">{move || log.get().join(",")}</span></div>
        </div>
    }
}
