use leptonic::{
    atoms::slider::{
        Slider as SliderAtom, SliderOutput, SliderThumb, SliderTrack, SliderTrackFill,
    },
    hooks::SliderValues,
};
use leptos::prelude::*;

#[component]
pub fn SliderBasicDemo() -> impl IntoView {
    view! {
        <div class="demo-frame">
            <SliderAtom values=SliderValues::Uncontrolled(vec![50.0]) classes="demo-slider">
                <SliderTrack classes="demo-slider-track">
                    <SliderTrackFill classes="demo-slider-fill" />
                    <SliderThumb aria_label="Volume" classes="demo-slider-thumb" />
                </SliderTrack>
                <SliderOutput let:attrs let:values>
                    <output {..attrs} class="demo-slider-output">
                        {move || {
                            let val = values.get().first().copied().unwrap_or(0.0);
                            format!("{val:.0}%")
                        }}
                    </output>
                </SliderOutput>
            </SliderAtom>
        </div>
    }
}
