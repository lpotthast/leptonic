use leptonic::{
    atoms::{
        field::Label,
        slider::{Slider, SliderFill, SliderMark, SliderMarks, SliderThumb, SliderTrack},
    },
    hooks::SliderMarks as Marks,
};
use leptos::prelude::*;

#[component]
pub fn SliderMarksDemo() -> impl IntoView {
    let rating = RwSignal::new(vec![3_u8, 7]);

    view! {
        <Slider
            min_value=0
            max_value=10
            values=rating
            set_values=rating
            classes=["demo-slider", "demo-slider-stacked", "demo-slider-green"]
        >
            <Label classes="demo-slider-label">"Rating"</Label>
            <SliderTrack classes="demo-slider-track">
                <SliderFill classes="demo-slider-fill"/>
                <SliderThumb index=0 aria_label="Lowest" classes="demo-slider-thumb"/>
                <SliderThumb index=1 aria_label="Highest" classes="demo-slider-thumb"/>
            </SliderTrack>
            // One mark per step, named by its value. The children receive the computed marks.
            <SliderMarks marks=Marks::Automatic { create_names: true } classes="demo-slider-marks" let:marks>
                <For each=move || marks.get() key=|mark| mark.percentage.to_bits() let:mark>
                    <SliderMark mark=mark.clone() classes="demo-slider-mark">{mark.name.unwrap_or_default()}</SliderMark>
                </For>
            </SliderMarks>
        </Slider>

        <p class="demo-status">
            {move || rating.with(|rating| match rating.as_slice() {
                [lowest, highest] => format!("Ratings from {lowest} to {highest}."),
                _ => String::new(),
            })}
        </p>
    }
}
