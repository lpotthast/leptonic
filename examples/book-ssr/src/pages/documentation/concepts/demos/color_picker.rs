use leptonic::{
    components::prelude::*,
    utils::color::{Color, ColorValue, HSL, HSV},
};
use leptos::prelude::*;

#[component]
pub fn ColorPickerConceptDemo() -> impl IntoView {
    let color = RwSignal::new(Color::from(HSV {
        hue: 210.0,
        saturation: 0.6,
        value: 0.8,
    }));

    view! {
        <ColorPicker value=color set_value=color/>
        <p class="demo-status">
            {move || {
                let hsl = color.get().to::<HSL>();
                format!(
                    "hsl({:.0}, {:.0}%, {:.0}%), {}",
                    hsl.hue,
                    hsl.saturation * 100.0,
                    hsl.lightness * 100.0,
                    hsl.color_name(),
                )
            }}
        </p>
    }
}
