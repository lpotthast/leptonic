use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SliderConceptDemo() -> impl IntoView {
    let (value, set_value) = signal(50.0_f64);

    view! {
        <Slider value=value set_value=set_value min=0.0 max=100.0 step=1.0 />
    }
}
