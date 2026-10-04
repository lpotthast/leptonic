use leptonic::{
    atoms::{
        label::Label as LabelAtom,
        slider::{Slider as SliderAtom, SliderOutput, SliderThumb, SliderTrack, SliderTrackFill},
    },
    hooks::*,
};
use leptos::prelude::*;

#[component]
pub fn SliderRangeDemo() -> impl IntoView {
    view! {
        <div class="demo-frame">
            <SliderAtom
                values=SliderValues::Uncontrolled(vec![20.0, 80.0])
                classes=["demo-slider", "demo-slider-blue"]
            >
                <LabelAtom classes="demo-slider-label">"Price Range"</LabelAtom>
                <SliderTrack classes="demo-slider-track">
                    <SliderTrackFill classes="demo-slider-fill" />
                    <SliderThumb aria_label="Minimum Price" classes="demo-slider-thumb" />
                    <SliderThumb aria_label="Maximum Price" classes="demo-slider-thumb" />
                </SliderTrack>
                <SliderOutput let:attrs let:values>
                    <output {..attrs} class="demo-slider-output demo-slider-output-wide">
                        {move || {
                            let val1 = values.get().first().copied().unwrap_or(0.0);
                            let val2 = values.get().get(1).copied().unwrap_or(0.0);
                            format!("{val1:.0}% - {val2:.0}%")
                        }}
                    </output>
                </SliderOutput>
            </SliderAtom>
        </div>
    }
}
