use leptonic::{
    atoms::prelude::{ColorSwatch, Description, Input, Label, TextField},
    utils::color::{Color, ColorValue, HSV},
};
use leptos::prelude::*;

#[component]
pub fn ColorNamesDemo() -> impl IntoView {
    let text = RwSignal::new("hsb(212, 84%, 90%)".to_owned());
    // `None` while the text is no color.
    let color = Signal::derive(move || text.with(|text| text.parse::<Color>().ok()));

    view! {
        <div class="demo-color-names">
            <TextField value=text set_value=text classes="demo-field">
                <Label classes="demo-field-label">"Color"</Label>
                <Input classes="demo-atom-input"/>
                <Description classes="demo-field-description">
                    "#rgb, #rrggbb, rgb(r, g, b), hsb(h, s%, b%), hsl(h, s%, l%), or with alpha: #rrggbbaa, rgba(..), hsla(..)"
                </Description>
            </TextField>
            <Show when=move || color.get().is_some()>
                // The checkerboard behind the swatch shows transparency.
                <div class="demo-color-checkerboard">
                    <ColorSwatch
                        color=Signal::derive(move || color.get().unwrap_or_default())
                        classes="demo-color-names-swatch"
                    />
                </div>
            </Show>
        </div>
        <p class="demo-status">
            {move || match color.get() {
                Some(color) => {
                    // `Color::color_name` includes the transparency; the hue comes from HSV.
                    let hsv = color.to::<HSV>();
                    format!(
                        "{}: {} (hue: {}, {:.0}\u{00b0}).",
                        color.to_css_string(),
                        color.color_name(),
                        hsv.hue_name(),
                        hsv.hue,
                    )
                }
                None => "Not a color.".to_owned(),
            }}
        </p>
    }
}
