use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;

#[component]
pub fn SliderVolumeDemo() -> impl IntoView {
    let volume = RwSignal::new(0.5_f64);

    view! {
        <div class="demo-control-row">
            <Icon icon=icondata::BsVolumeDownFill classes="demo-volume-icon"/>
            <Slider value=volume set_value=volume min_value=0.0 max_value=1.0 step=0.01 aria_label="Volume" classes="demo-volume-slider"/>
            <Icon icon=icondata::BsVolumeUpFill classes="demo-volume-icon"/>
        </div>
        <p class="demo-status">{move || format!("Volume: {:.0}%.", volume.get() * 100.0)}</p>
    }
}
