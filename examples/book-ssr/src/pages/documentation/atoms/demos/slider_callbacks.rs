use leptonic::{
    atoms::slider::{
        Slider as SliderAtom, SliderOutput, SliderThumb, SliderTrack, SliderTrackFill,
    },
    hooks::SliderValues,
};
use leptos::prelude::*;

#[component]
pub fn SliderCallbacksDemo() -> impl IntoView {
    let (change_log, set_change_log) = signal(String::new());
    let (change_end_log, set_change_end_log) = signal(String::new());

    view! {
        <div class="demo-frame">
            <SliderAtom
                values=SliderValues::Uncontrolled(vec![50.0])
                on_change=Callback::new(move |values: Vec<f64>| {
                    set_change_log
                        .set(
                            format!(
                                "on_change: {:?}",
                                values.iter().map(|v| format!("{v:.0}")).collect::<Vec<_>>(),
                            ),
                        );
                })
                on_change_end=Callback::new(move |values: Vec<f64>| {
                    set_change_end_log
                        .set(
                            format!(
                                "on_change_end: {:?}",
                                values.iter().map(|v| format!("{v:.0}")).collect::<Vec<_>>(),
                            ),
                        );
                })
                classes="demo-slider"
            >
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
            <div class="demo-mono-log">
                <div>{move || change_log.get()}</div>
                <div>{move || change_end_log.get()}</div>
            </div>
        </div>
    }
}
