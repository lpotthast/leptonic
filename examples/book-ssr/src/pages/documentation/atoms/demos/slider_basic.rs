use leptonic::{
    atoms::{
        field::Label,
        slider::{Slider, SliderFill, SliderOutput, SliderThumb, SliderTrack},
    },
    components::prelude::Checkbox,
};
use leptos::prelude::*;

#[component]
pub fn SliderBasicDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    view! {
        <Slider min_value=0 max_value=100 default_values=vec![50_u8] is_disabled=disabled classes="demo-slider">
            <Label classes="demo-slider-label">"Volume"</Label>
            <SliderTrack classes="demo-slider-track">
                <SliderFill classes="demo-slider-fill"/>
                <SliderThumb classes="demo-slider-thumb"/>
            </SliderTrack>
            <SliderOutput classes="demo-slider-output"/>
        </Slider>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
