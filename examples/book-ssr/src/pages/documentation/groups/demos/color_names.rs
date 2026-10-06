use leptonic::{
    atoms::prelude::ColorSwatch,
    components::prelude::*,
    utils::color::{Color, ColorValue, HSV, RGB8},
};
use leptos::prelude::*;

#[component]
pub fn ColorNamesDemo() -> impl IntoView {
    let text = RwSignal::new("hsb(212, 84%, 90%)".to_owned());
    // `None` while the text is no color.
    let color = Signal::derive(move || text.with(|text| text.parse::<Color>().ok()));

    view! {
        <div class="demo-color-names">
            <TextField
                label="Color"
                description="#rgb, #rrggbb, rgb(r, g, b), hsb(h, s%, b%) or hsl(h, s%, l%)"
                value=text
                set_value=text
            />
            <Show when=move || color.get().is_some()>
                <ColorSwatch
                    color=Signal::derive(move || color.get().unwrap_or_default().to::<RGB8>())
                    classes="demo-color-names-swatch"
                />
            </Show>
        </div>
        <p class="demo-status">
            {move || match color.get() {
                Some(color) => {
                    let hsv = color.to::<HSV>();
                    format!(
                        "{}: {} (hue: {}, {:.0}\u{00b0})",
                        color.to::<RGB8>(),
                        hsv.color_name(),
                        hsv.hue_name(),
                        hsv.hue,
                    )
                }
                None => "Not a color".to_owned(),
            }}
        </p>
    }
}
