use leptonic::atoms::{
    field::Label,
    slider::{Slider, SliderFill, SliderOutput, SliderThumb, SliderTrack},
};
use leptos::prelude::*;

#[component]
pub fn SliderCallbacksDemo() -> impl IntoView {
    let (last_change, set_last_change) = signal(None::<u8>);
    let (last_change_end, set_last_change_end) = signal(None::<u8>);
    let show = |value: Option<u8>| value.map_or_else(|| "none yet".to_owned(), |value| value.to_string());

    view! {
        // `on_change`: every change, also while dragging. `on_change_end`: when the user lets go (or after a
        // keyboard change).
        <Slider
            min_value=0
            max_value=100
            default_values=vec![50_u8]
            on_change={move |values: Vec<u8>| set_last_change.set(values.first().copied())}
            on_change_end={move |values: Vec<u8>| set_last_change_end.set(values.first().copied())}
            classes=["demo-slider", "demo-slider-orange"]
        >
            <Label classes="demo-slider-label">"Brightness"</Label>
            <SliderTrack classes="demo-slider-track">
                <SliderFill classes="demo-slider-fill"/>
                <SliderThumb classes="demo-slider-thumb"/>
            </SliderTrack>
            <SliderOutput classes="demo-slider-output"/>
        </Slider>

        <p class="demo-status">
            {move || format!(
                "Last on_change: {}. Last on_change_end: {}.",
                show(last_change.get()),
                show(last_change_end.get()),
            )}
        </p>
    }
}
