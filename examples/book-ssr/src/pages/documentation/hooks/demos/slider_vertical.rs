use leptonic::{
    atoms::{
        label::Label as LabelAtom,
        slider::{Slider as SliderAtom, SliderOutput, SliderThumb, SliderTrack, SliderTrackFill},
    },
    hooks::*,
};
use leptos::prelude::*;

#[component]
pub fn SliderVerticalDemo() -> impl IntoView {
    view! {
        <div class="demo-frame">
            <SliderAtom
                values=SliderValues::Uncontrolled(vec![60.0])
                orientation=SliderOrientation::Vertical
                classes=["demo-slider", "demo-slider-purple", "demo-slider-vertical"]
            >
                <LabelAtom classes="demo-slider-label">"Vertical"</LabelAtom>
                <SliderTrack classes="demo-slider-track">
                    <SliderTrackFill classes="demo-slider-fill" />
                    <SliderThumb classes="demo-slider-thumb" />
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

        <p>"Vertical range slider"</p>

        <div class="demo-frame">
            <SliderAtom
                values=SliderValues::Uncontrolled(vec![20.0, 60.0])
                orientation=SliderOrientation::Vertical
                classes=["demo-slider", "demo-slider-purple", "demo-slider-vertical"]
            >
                <LabelAtom classes="demo-slider-label">"Vertical"</LabelAtom>
                <SliderTrack classes="demo-slider-track">
                    <SliderTrackFill classes="demo-slider-fill" />
                    <SliderThumb classes="demo-slider-thumb" />
                    <SliderThumb classes="demo-slider-thumb" />
                </SliderTrack>
                <SliderOutput let:attrs let:values>
                    <output {..attrs} class="demo-slider-output">
                        {move || {
                            let val1 = values.get().first().copied().unwrap_or(0.0);
                            let val2 = values.get().get(1).copied().unwrap_or(0.0);
                            format!("{val1}% - {val2}% ")
                        }}
                    </output>
                </SliderOutput>
            </SliderAtom>
        </div>
    }
}
