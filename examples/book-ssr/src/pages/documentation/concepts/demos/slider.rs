use leptonic::atoms::{
    checkbox::{CheckboxButton, CheckboxField},
    field::Label,
    slider::{Slider, SliderFill, SliderOutput, SliderThumb, SliderTrack},
};
use leptos::prelude::*;

#[component]
pub fn SliderConceptDemo() -> impl IntoView {
    // One value per thumb.
    let volume = RwSignal::new(vec![50_u8]);
    let disabled = RwSignal::new(false);

    view! {
        // The slider positions the fill and the thumb; the classes size and color them.
        <Slider min_value=0 max_value=100 values=volume set_values=volume is_disabled=disabled classes="demo-slider">
            <Label classes="demo-slider-label">"Volume"</Label>
            <SliderTrack classes="demo-slider-track">
                <SliderFill classes="demo-slider-fill"/>
                <SliderThumb classes="demo-slider-thumb"/>
            </SliderTrack>
            <SliderOutput classes="demo-slider-output"/>
        </Slider>
        <p class="demo-status">{move || format!("Volume: {}.", volume.with(|v| v[0]))}</p>
        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
