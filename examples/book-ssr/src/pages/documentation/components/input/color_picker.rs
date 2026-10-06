use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    color_palette::ColorPaletteDemo, color_picker_full::ColorPickerFullDemo,
    color_preview::ColorPreviewDemo, hue_slider::HueSliderDemo,
};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageColorPicker() -> impl IntoView {
    view! {
        <DocPage title="Color Picker Components">
            <p>
                "The themed "<Code inline=true>"ColorPicker"</Code>" picks any color with a saturation and brightness area, "
                "a hue slider and fields for the channels and the hex code. See the "
                <Link href=routes::doc::ColorPicker.materialize()>"Color Picker overview"</Link>" for the concept."
            </p>

            <Demo description="Full color picker" source=include_str!("demos/color_picker_full.rs")>
                <ColorPickerFullDemo/>
            </Demo>

            <Section title="ColorPicker">
                <p>
                    "Shows a preview of the color next to the saturation and brightness area, the hue slider below them, "
                    "fields for hue, saturation and brightness, fields for the red, green and blue channels, and a field for "
                    "the hex code. All of them edit the one "
                    <Link href=format!("{}#color", routes::doc::Color.materialize())><Code inline=true>"Color"</Code></Link>
                    " of the picker: it is the "<Link href=routes::doc::color_picker::Atom.materialize()>"ColorPicker atom"</Link>
                    " with themed parts."
                </p>

                <Section title="Props" id="color-picker-props">
                    <ApiTable kind=ApiKind::Props of="components::color_picker::ColorPicker">
                        <ApiRow name="default_value" ty="Option<Color>" default="None">
                            "The initial color, unless "<Code inline=true>"value"</Code>" is set; "<Code inline=true>"None"</Code>": black."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Color>>" default="None">"The color (controlled): a value or any signal."</ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Color>>" default="None">
                            "Receives the new color: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Color>>" default="None">"Called with every new color."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ColorPreview">
                <p>
                    "Shows a patch of an RGB color: a "<Link href=routes::doc::color_swatch::Atom.materialize()>"ColorSwatch"</Link>
                    ", named after the color (e.g. \u{201c}vibrant blue\u{201d}). Inside a "<Code inline=true>"ColorPicker"</Code>
                    ", it shows the picker\u{2019}s color."
                </p>

                <Section title="Props" id="color-preview-props">
                    <ApiTable kind=ApiKind::Props of="ColorPreview">
                        <ApiRow name="color" ty="Option<ColorProp>" default="None">
                            "The color to show: any color value or a signal of one. "<Code inline=true>"None"</Code>": the color of the "<Code inline=true>"ColorPicker"</Code>" around it."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles. Set the size of the patch here."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Demo description="Color preview of a fixed color" source=include_str!("demos/color_preview.rs")>
                    <ColorPreviewDemo/>
                </Demo>
            </Section>

            <Section title="ColorPalette">
                <p>
                    "Shows the saturation and brightness gradient of the current hue. Drag the handle, click anywhere in the "
                    "area, or tab to it and use the arrow keys to choose the saturation (x-axis) and brightness (y-axis) of "
                    "an HSV color. It is a themed "<Link href=routes::doc::color_area::Atom.materialize()>"ColorArea"</Link>"."
                </p>

                <Section title="Props" id="color-palette-props">
                    <ApiTable kind=ApiKind::Props of="ColorPalette">
                        <ApiRow name="default_value" ty="Option<HSV>" default="None">
                            "The initial color, unless "<Code inline=true>"value"</Code>" is set or a "<Code inline=true>"ColorPicker"</Code>
                            " is around it; "<Code inline=true>"None"</Code>": red."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<Signal<HSV>>" default="None">
                            "The color (controlled): a value or any signal. "<Code inline=true>"None"</Code>": the color of the "
                            <Code inline=true>"ColorPicker"</Code>" around it, if any."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<HSV>>" default="None">"Receives the new color."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<HSV>>" default="None">"Called with every new color."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the area. Its inputs are named \u{201c}<label>, Color picker\u{201d}, or \u{201c}Color "
                            "picker\u{201d} without it."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether it is disabled."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles. Set the size of the area here."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Demo description="Saturation and brightness area" source=include_str!("demos/color_palette.rs")>
                    <ColorPaletteDemo/>
                </Demo>
            </Section>

            <Section title="HueSlider">
                <p>
                    "A slider for the hue of an HSV color, from 0\u{b0} to 360\u{b0}: a themed "
                    <Link href=routes::doc::color_slider::Atom.materialize()>"ColorSlider"</Link>". Its track shows the hues "
                    "as a color band, its thumb the selected hue at full saturation and brightness. It is named "
                    "\u{201c}Hue\u{201d} and announces the hue with its name (\u{201c}210\u{b0}, blue\u{201d})."
                </p>

                <Section title="Props" id="hue-slider-props">
                    <ApiTable kind=ApiKind::Props of="HueSlider">
                        <ApiRow name="default_value" ty="Option<HSV>" default="None">
                            "The initial color, unless "<Code inline=true>"value"</Code>" is set or a "<Code inline=true>"ColorPicker"</Code>
                            " is around it; "<Code inline=true>"None"</Code>": red."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<Signal<HSV>>" default="None">
                            "The color (controlled): a value or any signal. "<Code inline=true>"None"</Code>": the color of the "
                            <Code inline=true>"ColorPicker"</Code>" around it, if any."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<HSV>>" default="None">"Receives the new color."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<HSV>>" default="None">"Called with every new color."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the slider. "<Code inline=true>"None"</Code>": \u{201c}Hue\u{201d}."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether it is disabled."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles."</ApiRow>
                    </ApiTable>
                </Section>

                <Demo description="Hue slider" source=include_str!("demos/hue_slider.rs")>
                    <HueSliderDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>
                    "Override any of these CSS variables to adapt the palette\u{2019}s handle and the hue slider\u{2019}s thumb "
                    "to your design:"
                </p>
                <CssVariables prefix="--color-palette-" scss=theme_scss!("color_picker")/>
                <p>
                    "The fields are themed like a "<Link href=routes::doc::text_field::Component.materialize()>"TextField"</Link>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ColorPicker.materialize()>"Color Picker"</Link></li>
                <li><Link href=routes::doc::color_picker::Hook.materialize()>"use_color_picker_state"</Link></li>
                <li><Link href=routes::doc::color_picker::Atom.materialize()>"Color Picker Atom"</Link></li>
                <li><Link href=routes::doc::color_area::Atom.materialize()>"Color Area Atoms"</Link></li>
                <li><Link href=routes::doc::color_slider::Atom.materialize()>"Color Slider Atoms"</Link></li>
                <li><Link href=routes::doc::color_swatch::Atom.materialize()>"Color Swatch Atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
