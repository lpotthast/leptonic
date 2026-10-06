use leptonic::{
    atoms::prelude::{ColorSwatch, ColorSwatchPicker, ColorSwatchPickerItems},
    utils::color::{ColorValue, RGB8},
};
use leptos::prelude::*;

/// The colors to pick from. They must differ as hex codes.
const PALETTE: [RGB8; 6] = [
    RGB8 { r: 170, g: 0, b: 0 },
    RGB8 { r: 255, g: 136, b: 0 },
    RGB8 { r: 0, g: 136, b: 0 },
    RGB8 { r: 0, g: 136, b: 255 },
    RGB8 { r: 0, g: 136, b: 136 },
    RGB8 { r: 0, g: 0, b: 136 },
];

#[component]
pub fn ColorSwatchPickerDemo() -> impl IntoView {
    let color = RwSignal::new(PALETTE[3]);

    view! {
        <ColorSwatchPicker
            colors=PALETTE.to_vec()
            value=color
            set_value=color
            aria_label="Background color"
            classes="demo-color-swatch-picker"
        >
            <ColorSwatchPickerItems classes="demo-color-swatch-picker-item">
                <ColorSwatch classes="demo-color-swatch-picker-swatch"/>
            </ColorSwatchPickerItems>
        </ColorSwatchPicker>
        <p class="demo-status">{move || format!("Background color: {}, {}.", color.get(), color.get().color_name())}</p>
    }
}
