use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::color_palette::ColorPaletteDemo;
use super::demos::color_picker_full::ColorPickerFullDemo;
use super::demos::color_preview::ColorPreviewDemo;
use super::demos::hue_slider::HueSliderDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageColorPicker() -> impl IntoView {
    view! {
        <Article>
            <h1 id="color-picker" class="anchor">
                "Color Picker"
                <AnchorLink href="#color-picker" description="Direct link to article header"/>
            </h1>

            <p>"Select colors using the "<Code inline=true>"<ColorPicker>"</Code>" component."</p>

            <DemoShell source=include_str!("demos/color_picker_full.rs")>
                <ColorPickerFullDemo />
            </DemoShell>

            <h2 id="parts" class="anchor">
                "Parts"
                <AnchorLink href="#parts" description="Direct link to section: Parts"/>
            </h2>

            <p>"The "<Code inline=true>"<ColorPicker>"</Code>" build on top of a few other components build to help work with colors. You may use them directly and build your own color picker."</p>

            <h3 id="part-color-preview" class="anchor">
                "ColorPreview"
                <AnchorLink href="#part-color-preview" description="Direct link to section: Part - ColorPreview"/>
            </h3>

            <p>"The "<Code inline=true>"<ColorPreview>"</Code>" component simply displays a reactive color patch based on the given RGB color signal."</p>

            <DemoShell source=include_str!("demos/color_preview.rs")>
                <ColorPreviewDemo />
            </DemoShell>

            <h3 id="part-color-palette" class="anchor">
                "ColorPalette"
                <AnchorLink href="#part-color-palette" description="Direct link to section: Part - ColorPalette"/>
            </h3>

            <p>
                "The "<Code inline=true>"<ColorPalette>"</Code>" component works on an HSV color signal, "
                "displays the color-gradient field for any given hue value and allows selecting new values for "
                "saturation (S, x-axis) and value (V, y-axis) of the HSV color by dragging a handle on the displayed surface."
            </p>

            <DemoShell source=include_str!("demos/color_palette.rs")>
                <ColorPaletteDemo />
            </DemoShell>

            <h3 id="part-hue-slider" class="anchor">
                "HueSlider"
                <AnchorLink href="#part-hue-slider" description="Direct link to section: Part - HueSlider"/>
            </h3>

            <p>
                "The "<Code inline=true>"<HueSlider>"</Code>" component renders a specialized "<Code inline=true>"<Slider>"</Code>", "
                "allowing you to pick a hue, a floating-point value between 0° and 360°. "
                "The slider background displays the hue range as a color band, the knob displays the currently selected hue value at maximum saturation and value."
            </p>

            <DemoShell source=include_str!("demos/hue_slider.rs")>
                <HueSliderDemo />
            </DemoShell>

            <p>"If you look at the source of Leptonic's <ColorPicker>, you will see that there is not much more to it as what you saw here!"</p>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    --color-palette-knob-size
                    --color-palette-knob-border-width
                    --color-palette-knob-border-color
                    --color-palette-knob-border-style
                    --color-palette-knob-background-color
                    --color-palette-knob-halo-size
                    --color-palette-knob-halo-size-while-dragged
                    --color-palette-knob-halo-opacity
                    --color-palette-knob-halo-background-color
                    --color-palette-knob-transition-speed
                    --color-palette-knob-box-shadow
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Color picker", link: "#color-picker" },
                Toc::Group { title: "Parts", link: "#parts", inner: vec![
                    Toc::Leaf { title: "ColorPreview", link: "#part-color-preview" },
                    Toc::Leaf { title: "ColorPalette", link: "#part-color-palette" },
                    Toc::Leaf { title: "HueSlider", link: "#part-hue-slider" },
                ]},
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
