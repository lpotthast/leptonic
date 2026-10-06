use leptonic::{
    atoms::slider::{
        Slider as SliderAtom, SliderOutput, SliderThumb, SliderTrack, SliderTrackFill,
    },
    components::prelude::Checkbox,
    hooks::SliderValues,
};
use leptos::prelude::*;

#[component]
pub fn SliderDisabledDemo() -> impl IntoView {
    let disabled = RwSignal::new(true);

    view! {
        <div class="demo-frame">
            <SliderAtom
                values=SliderValues::Uncontrolled(vec![30.0])
                is_disabled=disabled
                classes=["demo-slider", "demo-slider-disableable"]
            >
                <SliderTrack classes="demo-slider-track">
                    <SliderTrackFill classes="demo-slider-fill" />
                    <SliderThumb aria_label="Opacity" classes="demo-slider-thumb" />
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
            <Checkbox state=disabled>"Disabled"</Checkbox>
        </div>
    }
}
