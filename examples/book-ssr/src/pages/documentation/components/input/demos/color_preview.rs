use leptonic::{components::prelude::*, utils::color::HSV};
use leptos::prelude::*;

#[component]
pub fn ColorPreviewDemo() -> impl IntoView {
    let rgb = Signal::stored(HSV::from_hue_fully_saturated(200.0).into_rgb8());

    view! {
        <ColorPreview rgb=rgb classes="demo-color-preview-large"/>
        <p class="demo-status">{move || format!("#{:X}", rgb.get())}</p>
    }
}
