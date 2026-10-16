//! A `ColorSwatchPicker` (`.leptonic-ColorSwatchPicker`) of [`FEW`] RGB colors (or [`MANY`],
//! control `use-many` while unmounted), red first, its swatches (20 × 20 pixels) by `ColorSwatchPickerItems`, for
//! its performance test (`leptonic/tests/perf_tests/atoms/color_swatch_picker.rs`): the value
//! bound (`value` + `set_value`, red at first), between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first. Its controls ([`Control`]) mount and
//! unmount it, change the value, add a color and remove it again, and read the value (`value`, as
//! a number `0xRRGGBB`) and how often it called `on_change` (`changes`).

use leptonic::{
    RGB8,
    atoms::{
        color_swatch::ColorSwatch,
        color_swatch_picker::{ColorSwatchPicker, ColorSwatchPickerItems},
    },
    leptos_styles::Styles,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The colors of the small picker.
const FEW: usize = 4;
/// The colors of the large picker.
const MANY: usize = 100;

const RED: RGB8 = RGB8 { r: 255, g: 0, b: 0 };
const GRAY: RGB8 = RGB8 {
    r: 128,
    g: 128,
    b: 128,
};

/// `count` distinct colors, red first, then the second.
fn palette(count: usize) -> Vec<RGB8> {
    (0..count)
        .map(|i| {
            let i = u8::try_from(i).expect("fewer than 256 colors");
            RGB8 {
                r: 255 - i,
                g: i,
                b: 0,
            }
        })
        .collect()
}

/// `color` as a number `0xRRGGBB`.
fn number(color: RGB8) -> f64 {
    f64::from((u32::from(color.r) << 16) | (u32::from(color.g) << 8) | u32::from(color.b))
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let colors = RwSignal::new(palette(FEW));
    let value = RwSignal::new(RED);
    let change_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("use-many", move || colors.set(palette(MANY))),
        Control::action("select-second", move || value.set(palette(2)[1])),
        Control::action("select-first", move || value.set(RED)),
        Control::action("add-gray", move || {
            colors.update(|colors| colors.push(GRAY));
        }),
        Control::action("remove-gray", move || {
            colors.update(|colors| {
                colors.pop();
            });
        }),
        Control::read("value", move || number(value.get_untracked())),
        Control::read("changes", move || f64::from(change_count.get_value())),
    ];
    view! {
        <h1>"ColorSwatchPicker performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <ColorSwatchPicker<RGB8>
                colors=colors
                value=value
                set_value=value
                on_change=move |_: RGB8| change_count.update_value(|count| *count += 1)
            >
                <ColorSwatchPickerItems>
                    <ColorSwatch styles=Styles::new()
                        .add_unchecked("width", "20px")
                        .add_unchecked("height", "20px") />
                </ColorSwatchPickerItems>
            </ColorSwatchPicker<RGB8>>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
