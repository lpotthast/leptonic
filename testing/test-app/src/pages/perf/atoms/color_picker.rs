//! A `ColorPicker` sharing its color with a `ColorSwatch` (`.leptonic-ColorSwatch`), a hue
//! `ColorSlider` (200-pixel track, with its thumb) and a `ColorField` (`.leptonic-ColorField`,
//! with its `Input`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/color_picker.rs`): the value bound (`value` + `set_value`,
//! `#00FFFF` at first), between two plain buttons (`#perf-before`, `#perf-after`) to tab from and
//! to, mounted at first. Its controls ([`Control`]) mount and unmount it, change the value, and
//! read it (`value`, as a number `0xRRGGBB`) and how often it called `on_change` (`changes`).

use leptonic::{
    Color, HsvChannel, RGB8,
    atoms::{
        color_field::ColorField,
        color_picker::ColorPicker,
        color_slider::{ColorSlider, ColorSliderTrack},
        color_swatch::ColorSwatch,
        color_thumb::ColorThumb,
        input::Input,
    },
    leptos_styles::Styles,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

const CYAN: RGB8 = RGB8 {
    r: 0,
    g: 255,
    b: 255,
};
const RED: RGB8 = RGB8 { r: 255, g: 0, b: 0 };

/// `color` as a number `0xRRGGBB`.
fn number(color: Color) -> f64 {
    let color = color.to::<RGB8>();
    f64::from((u32::from(color.r) << 16) | (u32::from(color.g) << 8) | u32::from(color.b))
}

fn size(width: &'static str, height: &'static str) -> Styles {
    Styles::new()
        .add_unchecked("width", width)
        .add_unchecked("height", height)
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(Color::from(CYAN));
    let change_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-red", move || value.set(Color::from(RED))),
        Control::action("set-cyan", move || value.set(Color::from(CYAN))),
        Control::read("value", move || number(value.get_untracked())),
        Control::read("changes", move || f64::from(change_count.get_value())),
    ];
    view! {
        <h1>"ColorPicker performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <ColorPicker
                value=value
                set_value=value
                on_change=move |_: Color| change_count.update_value(|count| *count += 1)
            >
                <ColorSwatch styles=size("20px", "20px") />
                <ColorSlider channel=HsvChannel::Hue>
                    <ColorSliderTrack styles=size("200px", "20px")>
                        <ColorThumb styles=size("10px", "20px") />
                    </ColorSliderTrack>
                </ColorSlider>
                <ColorField aria_label="Hex">
                    <Input />
                </ColorField>
            </ColorPicker>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
