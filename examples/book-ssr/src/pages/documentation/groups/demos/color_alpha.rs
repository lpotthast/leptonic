use leptonic::{
    atoms::prelude::{
        ColorSlider, ColorSliderOutput, ColorSliderTrack, ColorSwatch, ColorThumb, Label,
    },
    utils::color::{Alpha, AlphaChannel, ColorValue, HSV, HsvChannel},
};
use leptos::prelude::*;

#[component]
pub fn ColorAlphaDemo() -> impl IntoView {
    // An HSV color with an alpha channel: its channels are HSV's plus `AlphaChannel::Alpha`.
    let color = RwSignal::new(
        Alpha::new(HSV {
            hue: 0.0,
            saturation: 0.9,
            brightness: 0.9,
        })
        .with_alpha(0.6),
    );

    let sliders = [AlphaChannel::Color(HsvChannel::Hue), AlphaChannel::Alpha]
        .into_iter()
        .map(|channel| {
            view! {
                <ColorSlider channel value=color set_value=color classes="demo-color-atoms-slider">
                    <Label>{Alpha::<HSV>::channel_name(channel)}</Label>
                    <ColorSliderOutput classes="demo-color-atoms-slider-output"/>
                    <ColorSliderTrack classes="demo-color-atoms-slider-track">
                        <ColorThumb classes="demo-color-atoms-thumb"/>
                    </ColorSliderTrack>
                </ColorSlider>
            }
        })
        .collect_view();

    view! {
        <div class="demo-color-atoms">
            <div class="demo-color-atoms-sliders">{sliders}</div>
            // The checkerboard behind the swatch shows its transparency.
            <div class="demo-color-checkerboard">
                <ColorSwatch color=color classes="demo-color-atoms-swatch"/>
            </div>
        </div>
        <p class="demo-status">{move || format!("{}: {}.", color.get().to_css_string(), color.get().color_name())}</p>
    }
}
