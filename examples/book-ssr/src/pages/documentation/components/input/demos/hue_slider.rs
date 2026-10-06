use leptonic::{components::prelude::*, utils::color::HSV};
use leptos::prelude::*;

#[component]
pub fn HueSliderDemo() -> impl IntoView {
    let (hsv, set_hsv) = signal(HSV::new());

    view! {
        <HueSlider
            hue=Signal::derive(move || hsv.get().hue)
            set_hue=move |hue| set_hsv.update(|hsv| hsv.hue = hue)
        />
        <p class="demo-status">{move || format!("Hue: {:.0}\u{b0}", hsv.get().hue)}</p>
    }
}
