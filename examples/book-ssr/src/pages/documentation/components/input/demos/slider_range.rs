use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SliderRangeDemo() -> impl IntoView {
    let (opacity_from, set_opacity_from) = signal(0.5);
    let (opacity_to, set_opacity_to) = signal(0.75);
    let (stars_from, set_stars_from) = signal(2.0);
    let (stars_to, set_stars_to) = signal(4.0);

    view! {
        <div class="demo-control-stack">
            // Continuous: no `step`.
            <RangeSlider
                value_a=opacity_from set_value_a=set_opacity_from
                value_b=opacity_to set_value_b=set_opacity_to
                min=0.0 max=1.0
                value_display=move |v| format!("{v:.2}")/>

            // Stepped, with named marks.
            <RangeSlider
                value_a=stars_from set_value_a=set_stars_from
                value_b=stars_to set_value_b=set_stars_to
                min=1.0 max=5.0 step=1.0
                marks=SliderMarks::Automatic { create_names: true }
                value_display=move |v| format!("{v:.0}")/>
        </div>
    }
}
