use leptonic::{hooks::*, utils::color::RGB8};
use leptos::prelude::*;

const BLUE: RGB8 = RGB8 {
    r: 66,
    g: 135,
    b: 245,
};
const GREEN: RGB8 = RGB8 {
    r: 46,
    g: 139,
    b: 87,
};

/// A swatch together with the accessible name it announces.
#[component]
fn Swatch(color: RGB8, #[prop(optional, into)] color_name: MaybeProp<String>) -> impl IntoView {
    let swatch = use_color_swatch(UseColorSwatchInput {
        color_name,
        ..UseColorSwatchInput::new(Signal::stored(color))
    });
    // The props carry the swatch's background color as a style.
    let (props, styles) = swatch.color_swatch_props.into_inner();
    let label = props.aria_label;

    view! {
        <figure class="demo-color-swatch-item">
            <div {..props.into_attrs()} class="demo-color-swatch-hook" style=styles></div>
            <figcaption>
                "aria-label: " <code>{label}</code>
            </figcaption>
        </figure>
    }
}

#[component]
pub fn ColorSwatchDemo() -> impl IntoView {
    view! {
        <div class="demo-color-swatch-row">
            <Swatch color=BLUE />
            <Swatch color=GREEN color_name="Sea green" />
        </div>
    }
}
