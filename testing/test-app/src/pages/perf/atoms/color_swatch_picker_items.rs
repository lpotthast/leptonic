//! `ColorSwatchPickerItems` (an item with a `ColorSwatch` of 20 × 20 pixels per color) of a `ColorSwatchPicker`
//! (`.leptonic-ColorSwatchPicker`) of [`FEW`] RGB colors (or [`MANY`], control `use-many` while
//! unmounted), red first and picked, for its performance test
//! (`leptonic/tests/perf_tests/atoms/color_swatch_picker_items.rs`), mounted at first. Its
//! controls ([`Control`]) mount and unmount them, and add a color and remove it again.

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

/// `count` distinct colors, red first.
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

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let colors = RwSignal::new(palette(FEW));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("use-many", move || colors.set(palette(MANY))),
        Control::action("add-gray", move || {
            colors.update(|colors| colors.push(GRAY));
        }),
        Control::action("remove-gray", move || {
            colors.update(|colors| {
                colors.pop();
            });
        }),
    ];
    view! {
        <h1>"ColorSwatchPickerItems performance"</h1>
        <PerfControls controls />
        <ColorSwatchPicker<RGB8> colors=colors default_value=RED>
            <Show when=move || mounted.get()>
                <ColorSwatchPickerItems>
                    <ColorSwatch styles=Styles::new()
                        .add_unchecked("width", "20px")
                        .add_unchecked("height", "20px") />
                </ColorSwatchPickerItems>
            </Show>
        </ColorSwatchPicker<RGB8>>
    }
}
