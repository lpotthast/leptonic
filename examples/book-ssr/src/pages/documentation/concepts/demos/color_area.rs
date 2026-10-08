use leptonic::{
    atoms::prelude::{ColorArea, ColorSwatch, ColorThumb},
    utils::{
        color::{ColorValue, HSV, HsvChannel},
        i18n::use_locale,
    },
};
use leptos::prelude::*;

#[component]
pub fn ColorAreaConceptDemo() -> impl IntoView {
    let locale = use_locale();
    let color = RwSignal::new(HSV {
        hue: 210.0,
        saturation: 0.6,
        brightness: 0.8,
    });

    view! {
        <div class="demo-color-atoms">
            <ColorArea
                value=color
                set_value=color
                x_channel=HsvChannel::Saturation
                y_channel=HsvChannel::Brightness
                aria_label="Saturation and brightness"
                classes="demo-color-atoms-area"
            >
                <ColorThumb classes="demo-color-atoms-thumb"/>
            </ColorArea>
            <ColorSwatch color=color classes="demo-color-atoms-swatch"/>
        </div>
        <p class="demo-status">{move || format!("Color: {}, {}", color.get().to_rgb8(), color.get().color_name(&locale.get()))}</p>
    }
}
