//! `use_color_swatch` on a plain `<div class="perf-swatch">` (20 × 20 pixels) of a color, for its
//! performance test (`leptonic/tests/perf_tests/hooks/use_color_swatch.rs`): red at first,
//! mounted at first. Its controls ([`Control`]) mount and unmount it, and change its inputs (the
//! color, its name, the label).

use leptonic::{
    Color, RGB8,
    hooks::color::{UseColorSwatchInput, use_color_swatch},
    leptos_styles::Styles,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

const RED: RGB8 = RGB8 { r: 255, g: 0, b: 0 };
const BLUE: RGB8 = RGB8 { r: 0, g: 0, b: 255 };

/// The swatch: `use_color_swatch` on a plain `<div>`.
#[component]
fn Swatch(
    color: RwSignal<Color>,
    name: RwSignal<Option<String>>,
    label: RwSignal<Option<String>>,
) -> impl IntoView {
    let (attrs, styles) = use_color_swatch(UseColorSwatchInput {
        color: color.into(),
        color_name: name.into(),
        aria_label: label.into(),
        aria_labelledby: Signal::default(),
        id: None,
    })
    .swatch
    .attrs();
    let styles = styles.merge(
        Styles::new()
            .add_unchecked("width", "20px")
            .add_unchecked("height", "20px"),
    );
    view! { <div {..attrs} class="perf-swatch" style=styles></div> }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let color = RwSignal::new(Color::from(RED));
    let name = RwSignal::new(None::<String>);
    let label = RwSignal::new(None::<String>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-blue", move || color.set(Color::from(BLUE))),
        Control::action("set-red", move || color.set(Color::from(RED))),
        Control::action("set-name", move || name.set(Some("Fire truck".to_owned()))),
        Control::action("unset-name", move || name.set(None)),
        Control::action("set-label", move || {
            label.set(Some("Background".to_owned()));
        }),
        Control::action("unset-label", move || label.set(None)),
    ];
    view! {
        <h1>"use_color_swatch performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Swatch color name label />
        </Show>
    }
}
