use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SliderConceptDemo() -> impl IntoView {
    let (volume, set_volume) = signal(50.0_f64);

    view! {
        <Slider value=volume set_value=set_volume min=0.0 max=100.0 step=1.0/>
        <p>{move || format!("Volume: {:.0}", volume.get())}</p>
    }
}
