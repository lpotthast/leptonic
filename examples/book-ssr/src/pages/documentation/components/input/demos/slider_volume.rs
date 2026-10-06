use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;

#[component]
pub fn SliderVolumeDemo() -> impl IntoView {
    let (volume, set_volume) = signal(0.5);

    view! {
        <div class="demo-control-row">
            <Icon icon=icondata::BsVolumeDownFill classes="demo-volume-icon"/>
            <Slider min=0.0 max=1.0 value=volume set_value=set_volume classes="demo-volume-slider"/>
            <Icon icon=icondata::BsVolumeUpFill classes="demo-volume-icon"/>
            <span class="demo-status">{move || format!("{:.0}%", volume.get() * 100.0)}</span>
        </div>
    }
}
