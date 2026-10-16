//! A `ColorWheelTrack` (`.leptonic-ColorWheelTrack`) of a `ColorWheel` (radii 100 and 74, with
//! its `ColorThumb`) of an HSV color, for its performance test
//! (`leptonic/tests/perf_tests/atoms/color_wheel_track.rs`): the value bound (hue 180 at first),
//! mounted at first. Its controls ([`Control`]) mount and unmount it, disable and enable the
//! wheel, change its radii, and read the hue (`hue`).

use leptonic::{
    HSV,
    atoms::{
        color_thumb::ColorThumb,
        color_wheel::{ColorWheel, ColorWheelTrack},
    },
    leptos_styles::Styles,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    // Whether the wheel shows its track.
    let part = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let outer_radius = RwSignal::new(100.0);
    let value = RwSignal::new(HSV {
        hue: 180.0,
        saturation: 1.0,
        brightness: 1.0,
    });
    let controls = vec![
        Control::action("mount", move || part.set(true)),
        Control::action("unmount", move || part.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("grow", move || outer_radius.set(120.0)),
        Control::action("shrink", move || outer_radius.set(100.0)),
        Control::read("hue", move || value.get_untracked().hue),
    ];
    view! {
        <h1>"ColorWheelTrack performance"</h1>
        <PerfControls controls />
        <ColorWheel
            outer_radius=outer_radius
            inner_radius=Signal::derive(move || outer_radius.get() - 26.0)
            value=value
            set_value=value
            is_disabled=disabled
        >
            <Show when=move || part.get()>
                <ColorWheelTrack />
            </Show>
            <ColorThumb styles=Styles::new()
                .add_unchecked("width", "20px")
                .add_unchecked("height", "20px") />
        </ColorWheel>
    }
}
