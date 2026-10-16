//! A `ColorWheel` (`.leptonic-ColorWheel`, radii 100 and 74) of an HSV color with its
//! `ColorWheelTrack` and its `ColorThumb` (20 × 20 pixels), for its performance test
//! (`leptonic/tests/perf_tests/atoms/color_wheel.rs`): the value bound (`value` + `set_value`,
//! hue 180 at first), between two plain buttons (`#perf-before`, `#perf-after`) to tab from and
//! to, mounted at first. Its controls ([`Control`]) mount and unmount it, change its reactive
//! props (disabled, the value, the radii, the label), and read its hue (`hue`) and how often it
//! called `on_change` (`changes`) and `on_change_end` (`change-ends`).

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

fn hsv(hue: f64) -> HSV {
    HSV {
        hue,
        saturation: 1.0,
        brightness: 1.0,
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let value = RwSignal::new(hsv(180.0));
    let outer_radius = RwSignal::new(100.0);
    let label = RwSignal::new(None::<String>);
    let change_count = StoredValue::new(0_u32);
    let end_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("set-hue-90", move || value.set(hsv(90.0))),
        Control::action("set-hue-180", move || value.set(hsv(180.0))),
        Control::action("grow", move || outer_radius.set(120.0)),
        Control::action("shrink", move || outer_radius.set(100.0)),
        Control::action("set-label", move || label.set(Some("Tint".to_owned()))),
        Control::action("unset-label", move || label.set(None)),
        Control::read("hue", move || value.get_untracked().hue),
        Control::read("changes", move || f64::from(change_count.get_value())),
        Control::read("change-ends", move || f64::from(end_count.get_value())),
    ];
    view! {
        <h1>"ColorWheel performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <ColorWheel
                outer_radius=outer_radius
                inner_radius=Signal::derive(move || outer_radius.get() - 26.0)
                value=value
                set_value=value
                on_change=move |_: HSV| change_count.update_value(|count| *count += 1)
                on_change_end=move |_: HSV| end_count.update_value(|count| *count += 1)
                is_disabled=disabled
                aria_label=label
            >
                <ColorWheelTrack />
                <ColorThumb styles=Styles::new()
                    .add_unchecked("width", "20px")
                    .add_unchecked("height", "20px") />
            </ColorWheel>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
