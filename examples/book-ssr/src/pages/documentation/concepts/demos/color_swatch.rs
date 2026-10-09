use leptonic::{RGB8, atoms::color_swatch::ColorSwatch};
use leptos::prelude::*;

#[component]
pub fn ColorSwatchConceptDemo() -> impl IntoView {
    let forest_green = RGB8 {
        r: 34,
        g: 139,
        b: 84,
    };

    view! {
        <ColorSwatch
            color=forest_green
            color_name="Forest green".to_owned()
            classes="demo-color-swatch-large"
        />
        <p class="demo-status">{format!("Forest green ({forest_green})")}</p>
    }
}
