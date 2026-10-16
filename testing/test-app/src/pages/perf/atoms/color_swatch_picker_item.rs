//! A `ColorSwatchPickerItem` (green, with its `ColorSwatch` of 20 × 20 pixels) between a
//! red and a blue item of a `ColorSwatchPicker` (`.leptonic-ColorSwatchPicker`), for its
//! performance test (`leptonic/tests/perf_tests/atoms/color_swatch_picker_item.rs`): red picked
//! at first, between two plain buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted
//! at first. Its controls ([`Control`]) mount and unmount it, disable and enable it (its
//! `is_disabled`), and read the picked color (`value`, as a number `0xRRGGBB`).

use leptonic::{
    RGB8,
    atoms::{
        color_swatch::ColorSwatch,
        color_swatch_picker::{ColorSwatchPicker, ColorSwatchPickerItem},
    },
    leptos_styles::Styles,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

const RED: RGB8 = RGB8 { r: 255, g: 0, b: 0 };
const GREEN: RGB8 = RGB8 { r: 0, g: 255, b: 0 };
const BLUE: RGB8 = RGB8 { r: 0, g: 0, b: 255 };

/// A swatch's size: 20 × 20 pixels.
fn size() -> Styles {
    Styles::new()
        .add_unchecked("width", "20px")
        .add_unchecked("height", "20px")
}

/// `color` as a number `0xRRGGBB`.
fn number(color: RGB8) -> f64 {
    f64::from((u32::from(color.r) << 16) | (u32::from(color.g) << 8) | u32::from(color.b))
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let value = RwSignal::new(RED);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("value", move || number(value.get_untracked())),
    ];
    view! {
        <h1>"ColorSwatchPickerItem performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <ColorSwatchPicker<RGB8>
            colors=Signal::stored(vec![RED, GREEN, BLUE])
            value=value
            set_value=value
        >
            <ColorSwatchPickerItem color=RED>
                <ColorSwatch styles=size() />
            </ColorSwatchPickerItem>
            <Show when=move || mounted.get()>
                <ColorSwatchPickerItem color=GREEN is_disabled=disabled>
                    <ColorSwatch styles=size() />
                </ColorSwatchPickerItem>
            </Show>
            <ColorSwatchPickerItem color=BLUE>
                <ColorSwatch styles=size() />
            </ColorSwatchPickerItem>
        </ColorSwatchPicker<RGB8>>
        <button id="perf-after">"After"</button>
    }
}
