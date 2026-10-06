use leptonic::{
    hooks::*,
    utils::{color::RGB8, css::CssColor, style::BackgroundColorProperty, styles::Styles},
};
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
fn Swatch(color: RGB8, #[prop(optional, into)] color_name: Option<String>) -> impl IntoView {
    let swatch = use_color_swatch(UseColorSwatchInput {
        color: Signal::stored(color),
        color_name: color_name.clone().map(Signal::stored),
        aria_label: MaybeProp::default(),
    });

    let label = color_name.unwrap_or_else(|| swatch.background_color.get_untracked());
    let styles = Styles::new().add(BackgroundColorProperty.declare(CssColor::from(color)));

    view! {
        <figure class="demo-color-swatch-item">
            <div {..swatch.props.into_attrs()} class="demo-color-swatch-hook" style=styles></div>
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
