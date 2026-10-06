use leptonic::{atoms::color_swatch::ColorSwatch, utils::color::RGB8};
use leptos::prelude::*;

/// Named colors. The name becomes the swatch's accessible label, replacing the generated one ("vibrant red").
const PALETTE: [(&str, RGB8); 6] = [
    (
        "Tomato red",
        RGB8 {
            r: 229,
            g: 72,
            b: 77,
        },
    ),
    (
        "Sunflower yellow",
        RGB8 {
            r: 245,
            g: 190,
            b: 30,
        },
    ),
    (
        "Forest green",
        RGB8 {
            r: 34,
            g: 139,
            b: 84,
        },
    ),
    (
        "Ocean blue",
        RGB8 {
            r: 30,
            g: 110,
            b: 200,
        },
    ),
    (
        "Grape violet",
        RGB8 {
            r: 120,
            g: 70,
            b: 180,
        },
    ),
    (
        "Slate gray",
        RGB8 {
            r: 100,
            g: 116,
            b: 139,
        },
    ),
];

#[component]
pub fn ColorSwatchPaletteDemo() -> impl IntoView {
    view! {
        <ul class="demo-color-atoms-palette">
            {PALETTE
                .into_iter()
                .map(|(name, color)| {
                    view! {
                        <li class="demo-color-atoms-palette-item">
                            <ColorSwatch
                                color=color
                                color_name=name.to_owned()
                                classes="demo-color-atoms-palette-swatch"
                            />
                            <span>{name}</span>
                            <code>{color.to_string()}</code>
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}
