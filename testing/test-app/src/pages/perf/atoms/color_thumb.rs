//! `ColorThumb`s (`.leptonic-ColorThumb`, 10 × 10 pixels) in their parents, for their
//! performance test (`leptonic/tests/perf_tests/atoms/color_thumb.rs`): the thumb of a
//! `ColorArea` (`#perf-area`, 200 × 200 pixels, of `#FF00FF`), of a `ColorSlider`
//! (`#perf-slider`, the red channel, a 200-pixel track) and of a `ColorWheel` (`#perf-wheel`,
//! radii 100 and 74), between two plain buttons (`#perf-before`, `#perf-after`) to tab from and
//! to, the area's thumb mounted at first. Its controls ([`Control`]) mount and unmount each
//! thumb (`mount-area`, `mount-slider`, `mount-wheel`, `unmount-...`) and disable and enable
//! the area.

use leptonic::{
    HSV, RGB8, RgbChannel,
    atoms::{
        color_area::ColorArea,
        color_slider::{ColorSlider, ColorSliderTrack},
        color_thumb::ColorThumb,
        color_wheel::{ColorWheel, ColorWheelTrack},
    },
    leptos_styles::Styles,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

const MAGENTA: RGB8 = RGB8 {
    r: 255,
    g: 0,
    b: 255,
};

fn size(width: &'static str, height: &'static str) -> Styles {
    Styles::new()
        .add_unchecked("width", width)
        .add_unchecked("height", height)
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let area_thumb = RwSignal::new(true);
    let slider_thumb = RwSignal::new(false);
    let wheel_thumb = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let controls = vec![
        Control::action("mount-area", move || area_thumb.set(true)),
        Control::action("unmount-area", move || area_thumb.set(false)),
        Control::action("mount-slider", move || slider_thumb.set(true)),
        Control::action("unmount-slider", move || slider_thumb.set(false)),
        Control::action("mount-wheel", move || wheel_thumb.set(true)),
        Control::action("unmount-wheel", move || wheel_thumb.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
    ];
    view! {
        <h1>"ColorThumb performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div id="perf-area">
            <ColorArea default_value=MAGENTA is_disabled=disabled styles=size("200px", "200px")>
                <Show when=move || area_thumb.get()>
                    <ColorThumb styles=size("10px", "10px") />
                </Show>
            </ColorArea>
        </div>
        <button id="perf-after">"After"</button>
        <div id="perf-slider">
            <ColorSlider channel=RgbChannel::Red default_value=MAGENTA>
                <ColorSliderTrack styles=size("200px", "20px")>
                    <Show when=move || slider_thumb.get()>
                        <ColorThumb styles=size("10px", "20px") />
                    </Show>
                </ColorSliderTrack>
            </ColorSlider>
        </div>
        <div id="perf-wheel">
            <ColorWheel
                outer_radius=100.0
                inner_radius=74.0
                default_value=HSV {
                    hue: 0.0,
                    saturation: 1.0,
                    brightness: 1.0,
                }
            >
                <ColorWheelTrack />
                <Show when=move || wheel_thumb.get()>
                    <ColorThumb styles=size("20px", "20px") />
                </Show>
            </ColorWheel>
        </div>
    }
}
