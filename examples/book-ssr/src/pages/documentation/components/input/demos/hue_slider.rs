use leptonic::{
    components::prelude::*,
    utils::color::{ColorValue, HSV, HsvChannel},
};
use leptos::prelude::*;

#[component]
pub fn HueSliderDemo() -> impl IntoView {
    let color = RwSignal::new(HSV::new());

    view! {
        <HueSlider value=color set_value=color/>
        <p class="demo-status">
            {move || {
                let c = color.get();
                format!("Hue: {}, {}", c.format_channel_value(HsvChannel::Hue), c.hue_name())
            }}
        </p>
    }
}
