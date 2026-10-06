use leptonic::{components::prelude::*, hooks::Orientation};
use leptos::prelude::*;

#[component]
pub fn SliderVerticalDemo() -> impl IntoView {
    let level = RwSignal::new(40.0_f64);

    view! {
        // Vertical sliders grow from the bottom up; the theme makes them 10em tall.
        <Slider value=level set_value=level min_value=0.0 max_value=100.0 step=1.0 orientation=Orientation::Vertical aria_label="Level"/>
        <p class="demo-status">{move || format!("Level: {:.0}%.", level.get())}</p>
    }
}
