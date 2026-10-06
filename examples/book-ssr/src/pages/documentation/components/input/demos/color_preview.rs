use leptonic::{
    components::prelude::*,
    utils::color::{ColorValue, RGB8},
};
use leptos::prelude::*;

#[component]
pub fn ColorPreviewDemo() -> impl IntoView {
    let sky_blue = RGB8 { r: 0, g: 170, b: 255 };

    view! {
        <ColorPreview color=sky_blue classes="demo-color-preview-large"/>
        <p class="demo-status">{format!("Showing {sky_blue}, announced as \u{201c}{}\u{201d}", sky_blue.color_name())}</p>
    }
}
