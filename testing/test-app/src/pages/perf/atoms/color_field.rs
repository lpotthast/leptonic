//! A `ColorField` (`.leptonic-ColorField`) of an RGB color with its `Label` and `Input`, for its
//! performance test (`leptonic/tests/perf_tests/atoms/color_field.rs`): the value bound (`value`
//! and `set_value`, `#808080` at first), between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first. Its controls ([`Control`]) mount and
//! unmount it, change its reactive props (disabled, read-only, required, invalid, the value), and
//! read its value (`value`, as a number `0xRRGGBB`, `-1` without a color).
use leptonic::{
    RGB8,
    atoms::{color_field::ColorField, field::Label, input::Input},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

const GRAY: RGB8 = RGB8 {
    r: 128,
    g: 128,
    b: 128,
};
const RED: RGB8 = RGB8 { r: 255, g: 0, b: 0 };

/// `color` as a number `0xRRGGBB` (`-1`: none).
fn number(color: Option<RGB8>) -> f64 {
    color.map_or(-1.0, |color| {
        f64::from((u32::from(color.r) << 16) | (u32::from(color.g) << 8) | u32::from(color.b))
    })
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let required = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let value = RwSignal::new(Some(GRAY));
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
        Control::action("set-red", move || value.set(Some(RED))),
        Control::action("set-gray", move || value.set(Some(GRAY))),
        Control::read("value", move || number(value.get_untracked())),
    ];
    view! {
        <h1>"ColorField performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <ColorField
                value=value
                set_value=value
                is_disabled=disabled
                is_read_only=read_only
                is_required=required
                is_invalid=invalid
            >
                <Label>"Color"</Label>
                <Input />
            </ColorField>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
