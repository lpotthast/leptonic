use leptonic::{
    atoms::{
        color_slider::{ColorSlider, ColorSliderOutput, ColorSliderTrack},
        color_thumb::ColorThumb,
        field::Label,
    },
    utils::{
        color::{HSL, HslChannel, RGB8, RgbChannel},
        styles::Styles,
    },
};
use leptos::prelude::*;

/// `ColorSlider` atoms (react-spectrum's `ColorSlider.test.tsx`): horizontal tracks of 200
/// pixels; changes are logged as `change:<hex>`/`end:<hex>` to `#test-cs-log`.
#[component]
pub fn PageAtomColorSlider() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let bound = RwSignal::new(RGB8 { r: 127, g: 0, b: 0 });
    let track = || {
        Styles::new()
            .add_unchecked("width", "200px")
            .add_unchecked("height", "20px")
    };
    let thumb = || {
        Styles::new()
            .add_unchecked("width", "10px")
            .add_unchecked("height", "20px")
    };
    view! {
        <div id="test-page-atom-color-slider">
            <h1>"Color slider"</h1>
            <button id="test-cs-before">"Before"</button>
            <div id="test-cs-red">
                <ColorSlider
                    channel=RgbChannel::Red
                    default_value=RGB8 { r: 0, g: 0, b: 0 }
                    on_change=move |c: RGB8| log.update(|l| l.push(format!("change:{c:X}")))
                    on_change_end=move |c: RGB8| log.update(|l| l.push(format!("end:{c:X}")))
                >
                    <ColorSliderOutput />
                    <ColorSliderTrack styles=track()>
                        <ColorThumb styles=thumb() />
                    </ColorSliderTrack>
                </ColorSlider>
            </div>
            <div id="test-cs-hue">
                <ColorSlider
                    channel=HslChannel::Hue
                    default_value=HSL { hue: 10.0, saturation: 0.5, lightness: 0.5 }
                >
                    <Label>"Hue"</Label>
                    <ColorSliderTrack styles=track()>
                        <ColorThumb styles=thumb() />
                    </ColorSliderTrack>
                </ColorSlider>
            </div>
            <button id="test-cs-a">"A"</button>
            <div id="test-cs-disabled">
                <ColorSlider channel=RgbChannel::Red default_value=RGB8::default() is_disabled=true>
                    <ColorSliderTrack styles=track()>
                        <ColorThumb styles=thumb() />
                    </ColorSliderTrack>
                </ColorSlider>
            </div>
            <button id="test-cs-b">"B"</button>
            <form id="test-cs-form">
                <ColorSlider channel=RgbChannel::Red value=bound set_value=bound name="red">
                    <ColorSliderTrack styles=track()>
                        <ColorThumb styles=thumb() />
                    </ColorSliderTrack>
                </ColorSlider>
                <button type="reset" id="test-cs-reset">"Reset"</button>
            </form>
            <button id="test-cs-clear" on:click=move |_| log.set(Vec::new())>"Clear log"</button>
            <div>"Log: " <span id="test-cs-log">{move || log.get().join(",")}</span></div>
        </div>
    }
}
