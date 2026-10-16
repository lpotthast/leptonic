//! A `ColorSliderTrack` (`.leptonic-ColorSliderTrack`), the track (200 × 20 pixels, with its
//! `ColorThumb`) of a `ColorSlider` of the red channel of an RGB color (with its output), for its
//! performance test (`leptonic/tests/perf_tests/atoms/color_slider_track.rs`): the value bound
//! (`value` + `set_value`, `#8000FF` at first), between two plain buttons (`#perf-before`,
//! `#perf-after`), mounted at first. Its controls ([`Control`]) mount and unmount it, change the
//! slider's value and disable and enable the slider, and read the value (`value`, as a number
//! `0xRRGGBB`).

use leptonic::{
    RGB8, RgbChannel,
    atoms::{
        color_slider::{ColorSlider, ColorSliderOutput, ColorSliderTrack},
        color_thumb::ColorThumb,
    },
    leptos_styles::Styles,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

const PURPLE: RGB8 = RGB8 {
    r: 128,
    g: 0,
    b: 255,
};
const GREEN: RGB8 = RGB8 { r: 0, g: 255, b: 0 };

/// `color` as a number `0xRRGGBB`.
fn number(color: RGB8) -> f64 {
    f64::from((u32::from(color.r) << 16) | (u32::from(color.g) << 8) | u32::from(color.b))
}

fn size(width: &'static str, height: &'static str) -> Styles {
    Styles::new()
        .add_unchecked("width", width)
        .add_unchecked("height", height)
}

#[component]
pub fn PagePerf() -> impl IntoView {
    // Whether the slider shows its `ColorSliderTrack`.
    let part = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let value = RwSignal::new(PURPLE);
    let controls = vec![
        Control::action("mount", move || part.set(true)),
        Control::action("unmount", move || part.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("set-green", move || value.set(GREEN)),
        Control::action("set-purple", move || value.set(PURPLE)),
        Control::read("value", move || number(value.get_untracked())),
    ];
    view! {
        <h1>"ColorSliderTrack performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <ColorSlider channel=RgbChannel::Red value=value set_value=value is_disabled=disabled>
                <ColorSliderOutput />
                <Show when=move || part.get()>
                    <ColorSliderTrack styles=size("200px", "20px")>
                        <ColorThumb styles=size("10px", "20px") />
                    </ColorSliderTrack>
                </Show>
        </ColorSlider>
        <button id="perf-after">"After"</button>
    }
}
