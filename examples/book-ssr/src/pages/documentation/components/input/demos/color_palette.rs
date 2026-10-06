use leptonic::{
    components::prelude::*,
    utils::color::{ColorValue, HSV, HsvChannel},
};
use leptos::prelude::*;

#[component]
pub fn ColorPaletteDemo() -> impl IntoView {
    let color = RwSignal::new(HSV::new());

    view! {
        <ColorPalette
            value=color
            set_value=color
            aria_label="Saturation and brightness"
            classes="demo-color-palette-small"
        />
        <p class="demo-status">
            {move || {
                let c = color.get();
                format!(
                    "Saturation: {}, brightness: {}",
                    c.format_channel_value(HsvChannel::Saturation),
                    c.format_channel_value(HsvChannel::Brightness),
                )
            }}
        </p>
    }
}
