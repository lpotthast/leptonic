use leptonic::{
    atoms::prelude::{ColorThumb, ColorWheel, ColorWheelTrack},
    utils::color::{ColorValue, HSV, HsvChannel},
};
use leptos::prelude::*;

#[component]
pub fn ColorWheelConceptDemo() -> impl IntoView {
    let locale = leptonic::utils::i18n::use_locale();
    let color = RwSignal::new(HSV {
        hue: 210.0,
        saturation: 1.0,
        brightness: 1.0,
    });

    view! {
        <ColorWheel
            outer_radius=100.0
            inner_radius=74.0
            value=color
            set_value=color
            classes="demo-color-atoms-wheel"
        >
            <ColorWheelTrack/>
            <ColorThumb classes="demo-color-atoms-thumb"/>
        </ColorWheel>
        <p class="demo-status">{move || format!("Hue: {}, {}", color.get().format_channel_value(HsvChannel::Hue, &locale.get()), color.get().hue_name())}</p>
    }
}
