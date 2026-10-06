use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    color_palette::ColorPaletteDemo, color_picker_full::ColorPickerFullDemo,
    color_preview::ColorPreviewDemo, hue_slider::HueSliderDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageColorPicker() -> impl IntoView {
    view! {
        <DocPage title="ColorPicker component">
            <p>
                "The themed "<Code inline=true>"ColorPicker"</Code>" lets you select a color with a saturation/value "
                "area, a hue slider and numeric inputs. "
                "See the "<Link href=routes::doc::Color.materialize()>"Color overview"</Link>" for concept guidance."
            </p>

            <Demo description="Full color picker" source=include_str!("demos/color_picker_full.rs")>
                <ColorPickerFullDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="ColorPicker">
                    <ApiRow name="hsv" ty="Signal<HSV>">"The selected color. Required."</ApiRow>
                    <ApiRow name="set_hsv" ty="Out<HSV>">"Receives the new color on every change. Required."</ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles."
                    </ApiRow>
                </ApiTable>
                <p>
                    "The picker works on an "<Code inline=true>"HSV"</Code>" color from "
                    <Code inline=true>"leptonic::utils::color"</Code>". Below the area and the slider, it shows "
                    "number inputs for hue, saturation and value, read-only inputs for the RGB channels and the hex code."
                </p>
            </Section>

            <Section title="Parts">
                <p>
                    <Code inline=true>"ColorPicker"</Code>" is built from three components you can use on their own "
                    "to build your own color picker."
                </p>

                <Section title="ColorPreview">
                    <p>"Displays a patch of the given RGB color."</p>

                    <ApiTable kind=ApiKind::Props of="ColorPreview">
                        <ApiRow name="rgb" ty="Signal<RGB8>">"The color to show. Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles. Set the size of the patch here."
                        </ApiRow>
                    </ApiTable>

                    <Demo description="Color preview of a fixed color" source=include_str!("demos/color_preview.rs")>
                        <ColorPreviewDemo/>
                    </Demo>
                </Section>

                <Section title="ColorPalette">
                    <p>
                        "Displays the saturation/value gradient of the current hue. Drag the handle, or click anywhere "
                        "on the area, to choose the saturation (x-axis) and value (y-axis) of the color."
                    </p>

                    <ApiTable kind=ApiKind::Props of="ColorPalette">
                        <ApiRow name="hsv" ty="Signal<HSV>">
                            "The current color. Its hue colors the gradient. Required."
                        </ApiRow>
                        <ApiRow name="set_saturation" ty="Out<f64>">"Receives the new saturation (0 to 1). Required."</ApiRow>
                        <ApiRow name="set_value" ty="Out<f64>">"Receives the new value (0 to 1). Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles. Set the size of the area here."
                        </ApiRow>
                    </ApiTable>

                    <Demo description="Saturation/value area" source=include_str!("demos/color_palette.rs")>
                        <ColorPaletteDemo/>
                    </Demo>
                </Section>

                <Section title="HueSlider">
                    <p>
                        "A "<Link href=routes::doc::slider::Component.materialize()>"Slider"</Link>
                        " for picking a hue between 0\u{b0} and 360\u{b0}. Its track shows the hue range as a color band, "
                        "its thumb shows the selected hue at full saturation and value."
                    </p>

                    <ApiTable kind=ApiKind::Props of="HueSlider">
                        <ApiRow name="hue" ty="Signal<f64>">"The selected hue in degrees. Required."</ApiRow>
                        <ApiRow name="set_hue" ty="Out<f64>">"Receives the new hue. Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                    </ApiTable>

                    <Demo description="Hue slider" source=include_str!("demos/hue_slider.rs")>
                        <HueSliderDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt the color palette to your design:"</p>
                <CssVariables prefix="--color-palette-" scss=theme_scss!("color_picker")/>
                <p>
                    "The hue slider is a slider: style it with the "
                    <Link href=routes::doc::slider::Component.materialize()>"slider variables"</Link>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Color.materialize()>"Color overview"</Link></li>
                <li><Link href=routes::doc::color::Hooks.materialize()>"Color hooks"</Link></li>
                <li><Link href=routes::doc::slider::Component.materialize()>"Slider component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
