use leptonic::{hooks::*, utils::color::RGB8};
use leptos::prelude::*;

#[component]
pub fn ColorSwatchDemo() -> impl IntoView {
    let color = Signal::derive(|| RGB8 {
        r: 66,
        g: 135,
        b: 245,
    });

    let swatch = use_color_swatch(UseColorSwatchInput {
        color,
        color_name: None,
        aria_label: None,
    });

    let bg = swatch.background_color;

    view! {
        <div style="margin: 1em 0;">
            <div
                {..swatch.props.into_attrs()}
                style=move || format!(
                    "width: 48px; height: 48px; border-radius: 4px; border: 1px solid #ccc; background: {};",
                    bg.get()
                )
            ></div>
            <p style="margin-top: 0.5em;">
                "Swatch color: "
                <code>"rgb(66, 135, 245)"</code>
            </p>
        </div>
    }
}
