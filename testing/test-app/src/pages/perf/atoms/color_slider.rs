//! A `ColorSlider` (`.leptonic-ColorSlider`) of the red channel of an RGB color, with its
//! `ColorSliderOutput`, its `ColorSliderTrack` (200 × 20 pixels) and its `ColorThumb` (10 × 20
//! pixels), for its performance test (`leptonic/tests/perf_tests/atoms/color_slider.rs`): the
//! value bound (`value` + `set_value`, `#8000FF` at first), between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, change its reactive props (disabled, orientation, the
//! value, the label), and read its value (`value`, as a number `0xRRGGBB`) and how often it called
//! `on_change` (`changes`) and `on_change_end` (`change-ends`).

use leptonic::{
    Orientation, RGB8, RgbChannel,
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
const BLUE: RGB8 = RGB8 { r: 0, g: 0, b: 255 };

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
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let orientation = RwSignal::new(Orientation::Horizontal);
    let value = RwSignal::new(PURPLE);
    let label = RwSignal::new(None::<String>);
    let change_count = StoredValue::new(0_u32);
    let end_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("make-vertical", move || {
            orientation.set(Orientation::Vertical);
        }),
        Control::action("make-horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
        Control::action("set-blue", move || value.set(BLUE)),
        Control::action("set-purple", move || value.set(PURPLE)),
        Control::action("set-label", move || label.set(Some("Redness".to_owned()))),
        Control::action("unset-label", move || label.set(None)),
        Control::read("value", move || number(value.get_untracked())),
        Control::read("changes", move || f64::from(change_count.get_value())),
        Control::read("change-ends", move || f64::from(end_count.get_value())),
    ];
    view! {
        <h1>"ColorSlider performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <ColorSlider
                channel=RgbChannel::Red
                value=value
                set_value=value
                on_change=move |_: RGB8| change_count.update_value(|count| *count += 1)
                on_change_end=move |_: RGB8| end_count.update_value(|count| *count += 1)
                is_disabled=disabled
                orientation=orientation
                aria_label=label
            >
                <ColorSliderOutput />
                <ColorSliderTrack styles=size("200px", "20px")>
                    <ColorThumb styles=size("10px", "20px") />
                </ColorSliderTrack>
            </ColorSlider>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
