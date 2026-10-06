use leptonic::{
    components::prelude::*,
    utils::color::{Color, ColorValue, RGB8},
};
use leptos::prelude::*;

#[component]
pub fn ColorPickerFullDemo() -> impl IntoView {
    let color = RwSignal::new(Color::Rgb(RGB8 { r: 30, g: 110, b: 200 }));

    view! {
        <ColorPicker value=color set_value=color/>
        <p class="demo-status">
            {move || {
                let rgb = color.get().to::<RGB8>();
                format!("Color: {rgb}, {}", rgb.color_name())
            }}
        </p>
    }
}
