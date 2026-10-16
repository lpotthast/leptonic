//! A `ColorArea` (`.leptonic-ColorArea`, 200 × 200 pixels, red on x and green on y) of an RGB
//! color with its `ColorThumb` (`.leptonic-ColorThumb`, 10 × 10 pixels), for its performance test
//! (`leptonic/tests/perf_tests/atoms/color_area.rs`): the value bound (`value` + `set_value`,
//! `#FF00FF` at first), between two plain buttons (`#perf-before`, `#perf-after`) to tab from
//! and to, mounted at first. Its controls ([`Control`]) mount and unmount it, change its reactive
//! props (disabled, the value, the label), and read its value (`value`, as a number `0xRRGGBB`)
//! and how often it called `on_change` (`changes`) and `on_change_end` (`change-ends`).

use leptonic::{
    RGB8,
    atoms::{color_area::ColorArea, color_thumb::ColorThumb},
    leptos_styles::Styles,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

const MAGENTA: RGB8 = RGB8 {
    r: 255,
    g: 0,
    b: 255,
};
const BLUE: RGB8 = RGB8 { r: 0, g: 0, b: 255 };

/// `color` as a number `0xRRGGBB`.
fn number(color: RGB8) -> f64 {
    f64::from((u32::from(color.r) << 16) | (u32::from(color.g) << 8) | u32::from(color.b))
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let value = RwSignal::new(MAGENTA);
    let label = RwSignal::new(None::<String>);
    let change_count = StoredValue::new(0_u32);
    let end_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("set-blue", move || value.set(BLUE)),
        Control::action("set-magenta", move || value.set(MAGENTA)),
        Control::action("set-label", move || label.set(Some("Color".to_owned()))),
        Control::action("unset-label", move || label.set(None)),
        Control::read("value", move || number(value.get_untracked())),
        Control::read("changes", move || f64::from(change_count.get_value())),
        Control::read("change-ends", move || f64::from(end_count.get_value())),
    ];
    view! {
        <h1>"ColorArea performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <ColorArea
                value=value
                set_value=value
                on_change=move |_: RGB8| change_count.update_value(|count| *count += 1)
                on_change_end=move |_: RGB8| end_count.update_value(|count| *count += 1)
                is_disabled=disabled
                aria_label=label
                styles=Styles::new().add_unchecked("width", "200px").add_unchecked("height", "200px")
            >
                <ColorThumb styles=Styles::new()
                    .add_unchecked("width", "10px")
                    .add_unchecked("height", "10px") />
            </ColorArea>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
