//! A `ColorChannelField` (`.leptonic-ColorChannelField`) of the hue of an HSV color with its
//! `Label` and `Input`, for its performance test
//! (`leptonic/tests/perf_tests/atoms/color_channel_field.rs`): the value bound (`value` +
//! `set_value`, hue 180 at first), between two plain buttons (`#perf-before`, `#perf-after`) to
//! tab from and to, mounted at first. Its controls ([`Control`]) mount and unmount it, change its
//! reactive props (disabled, read-only, required, invalid, the value), and read its hue (`hue`,
//! `-1` without a color).

use leptonic::{
    HSV, HsvChannel,
    atoms::{color_field::ColorChannelField, field::Label, input::Input},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

fn hsv(hue: f64) -> HSV {
    HSV {
        hue,
        saturation: 0.5,
        brightness: 1.0,
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let required = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let value = RwSignal::new(Some(hsv(180.0)));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("make-read-only", move || read_only.set(true)),
        Control::action("make-editable", move || read_only.set(false)),
        Control::action("require", move || required.set(true)),
        Control::action("unrequire", move || required.set(false)),
        Control::action("invalidate", move || invalid.set(true)),
        Control::action("validate", move || invalid.set(false)),
        Control::action("set-hue-90", move || value.set(Some(hsv(90.0)))),
        Control::action("set-hue-180", move || value.set(Some(hsv(180.0)))),
        Control::read("hue", move || {
            value.get_untracked().map_or(-1.0, |color| color.hue)
        }),
    ];
    view! {
        <h1>"ColorChannelField performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <ColorChannelField
                channel=HsvChannel::Hue
                value=value
                set_value=value
                is_disabled=disabled
                is_read_only=read_only
                is_required=required
                is_invalid=invalid
            >
                <Label>"Hue"</Label>
                <Input />
            </ColorChannelField>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
