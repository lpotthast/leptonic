use leptonic::{
    atoms::{
        field::Label,
        slider::{Slider, SliderFill, SliderOutput, SliderThumb, SliderTrack},
    },
    utils::orientation::Orientation,
};
use leptos::prelude::*;

#[component]
pub fn SliderVerticalDemo() -> impl IntoView {
    view! {
        <Slider
            min_value=0
            max_value=100
            default_values=vec![60_u8]
            orientation=Orientation::Vertical
            classes=["demo-slider", "demo-slider-purple"]
        >
            <Label classes="demo-slider-label">"Level"</Label>
            <SliderTrack classes="demo-slider-track">
                <SliderFill classes="demo-slider-fill"/>
                <SliderThumb classes="demo-slider-thumb"/>
            </SliderTrack>
            <SliderOutput classes="demo-slider-output"/>
        </Slider>
    }
}
