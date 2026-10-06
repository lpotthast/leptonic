use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SliderRangeDemo() -> impl IntoView {
    let opacity = RwSignal::new(0.5_f64..=0.75);
    let stars = RwSignal::new(2_u8..=4);

    view! {
        <div class="demo-control-stack">
            <RangeSlider value=opacity set_value=opacity min_value=0.0 max_value=1.0 step=0.01 aria_label="Opacity"/>

            // Stepped, with named marks.
            <RangeSlider
                value=stars
                set_value=stars
                min_value=1
                max_value=5
                marks=SliderMarks::Automatic { create_names: true }
                aria_label="Stars"
            />
        </div>
        <p class="demo-status">
            {move || opacity.with(|opacity| {
                format!("Opacity from {:.0}% to {:.0}%. ", opacity.start() * 100.0, opacity.end() * 100.0)
            })}
            {move || stars.with(|stars| format!("Stars from {} to {}.", stars.start(), stars.end()))}
        </p>
    }
}
