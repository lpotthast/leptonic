use leptonic::{
    ColorValue, HSV, HsvChannel,
    atoms::{
        color_slider::{ColorSlider, ColorSliderOutput, ColorSliderTrack},
        color_thumb::ColorThumb,
        field::Label,
    },
    use_locale,
};
use leptos::prelude::*;

#[component]
pub fn ColorSliderConceptDemo() -> impl IntoView {
    let locale = use_locale();
    let color = RwSignal::new(HSV {
        hue: 210.0,
        saturation: 0.6,
        brightness: 0.8,
    });

    view! {
        <ColorSlider channel=HsvChannel::Hue value=color set_value=color classes="demo-color-atoms-slider">
            <Label>"Hue"</Label>
            <ColorSliderOutput classes="demo-color-atoms-slider-output"/>
            <ColorSliderTrack classes="demo-color-atoms-slider-track">
                <ColorThumb classes="demo-color-atoms-thumb"/>
            </ColorSliderTrack>
        </ColorSlider>
        <p class="demo-status">{move || format!("Color: {}, hue: {}", color.get().to_rgb8(), color.get().hue_name(&locale.get()))}</p>
    }
}
