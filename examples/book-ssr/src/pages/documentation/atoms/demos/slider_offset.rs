use leptonic::atoms::{
    field::Label,
    slider::{Slider, SliderFill, SliderOutput, SliderThumb, SliderTrack},
};
use leptos::prelude::*;

#[component]
pub fn SliderOffsetDemo() -> impl IntoView {
    view! {
        <Slider min_value=-10 max_value=10 default_values=vec![4_i8] classes="demo-slider">
            <Label classes="demo-slider-label">"Balance"</Label>
            <SliderTrack classes="demo-slider-track">
                // The fill runs from the center (the value 0) to the thumb.
                <SliderFill offset=0.0 classes="demo-slider-fill"/>
                <SliderThumb classes="demo-slider-thumb"/>
            </SliderTrack>
            <SliderOutput classes="demo-slider-output"/>
        </Slider>
    }
}
