//! A `ColorSwatch` (`.leptonic-ColorSwatch`, 20 × 20 pixels) of an RGB color, for its
//! performance test (`leptonic/tests/perf_tests/atoms/color_swatch.rs`): red at first, mounted at
//! first. Its controls ([`Control`]) mount and unmount it, and change its reactive props (the
//! color, its name, the label).

use leptonic::{RGB8, atoms::color_swatch::ColorSwatch, leptos_styles::Styles};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

const RED: RGB8 = RGB8 { r: 255, g: 0, b: 0 };
const BLUE: RGB8 = RGB8 { r: 0, g: 0, b: 255 };

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let color = RwSignal::new(RED);
    let name = RwSignal::new(None::<String>);
    let label = RwSignal::new(None::<String>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-blue", move || color.set(BLUE)),
        Control::action("set-red", move || color.set(RED)),
        Control::action("set-name", move || name.set(Some("Fire truck".to_owned()))),
        Control::action("unset-name", move || name.set(None)),
        Control::action("set-label", move || {
            label.set(Some("Background".to_owned()));
        }),
        Control::action("unset-label", move || label.set(None)),
    ];
    view! {
        <h1>"ColorSwatch performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <ColorSwatch
                color=color
                color_name=name
                aria_label=label
                styles=Styles::new().add_unchecked("width", "20px").add_unchecked("height", "20px")
            />
        </Show>
    }
}
