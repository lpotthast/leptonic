use leptonic::{
    atoms::slider::{
        Slider as SliderAtom, SliderOutput, SliderThumb, SliderTrack, SliderTrackFill,
    },
    hooks::SliderValues,
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
                <SliderTrack classes="demo-slider-track">
                    <SliderTrackFill classes="demo-slider-fill" />
                    <SliderThumb aria_label="Minimum" classes="demo-slider-thumb" />
                    <SliderThumb aria_label="Maximum" classes="demo-slider-thumb" />
                </SliderTrack>
                <SliderOutput let:attrs let:values>
                    <output {..attrs} class="demo-slider-output demo-slider-output-wide">
                        {move || {
                            let vals = values.get();
                            let v1 = vals.first().copied().unwrap_or(0.0);
                            let v2 = vals.get(1).copied().unwrap_or(0.0);
                            format!("{v1:.0}% - {v2:.0}%")
                        }}
                    </output>
                </SliderOutput>
            </SliderAtom>
        </div>
    }
}
