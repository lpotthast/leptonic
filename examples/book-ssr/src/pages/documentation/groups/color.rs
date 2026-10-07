use indoc::indoc;
use leptos::prelude::*;

use super::demos::{
    color::ColorPopoverDemo, color_alpha::ColorAlphaDemo, color_names::ColorNamesDemo,
};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageColor() -> impl IntoView {
    view! {
        <DocPage title="Color">
            <p>
                "The color concepts let users view and choose colors in different ways: 2D gradient areas, channel "
                "sliders, hue wheels, text fields, swatches and swatch pickers. Each one suits a different task, and a color "
                "picker combines several of them on one shared color."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Color.materialize()/>
            </Section>

            <Section title="Relationships">
                <ul>
                    <li>
                        "Every color control edits a color of one of the types "<Code inline=true>"HSV"</Code>", "
                        <Code inline=true>"HSL"</Code>" or "<Code inline=true>"RGB8"</Code>". Their hooks and atoms are generic "
                        "over the "<AnchorLink href="#colorvalue">"ColorValue"</AnchorLink>" trait of these types; a picker "
                        "shares one "<AnchorLink href="#color">"Color"</AnchorLink>" among them."
                    </li>
                    <li>
                        <Link href=routes::doc::ColorPicker.materialize()>"Color Picker"</Link>" puts several of them "
                        "together: the "<Link href=routes::doc::color_picker::Atom.materialize()>"ColorPicker atom"</Link>
                        " shares its color with the atoms inside it, and "
                        <Link href=routes::doc::color_picker::Hook.materialize()>"use_color_picker_state"</Link>
                        " holds the color for parts built from hooks."
                    </li>
                    <li>
                        <Link href=routes::doc::ColorArea.materialize()>"Color Area"</Link>", "
                        <Link href=routes::doc::ColorSlider.materialize()>"Color Slider"</Link>" and "
                        <Link href=routes::doc::ColorWheel.materialize()>"Color Wheel"</Link>" are dragged with the pointer "
                        "or moved with the keyboard. "<Link href=routes::doc::ColorField.materialize()>"Color Field"</Link>
                        " takes the same color as text, for users who know the exact value."
                    </li>
                    <li>
                        <Link href=routes::doc::ColorSwatch.materialize()>"Color Swatch"</Link>" only shows a color, "
                        "for example the result next to a picker; "
                        <Link href=routes::doc::ColorSwatchPicker.materialize()>"Color Swatch Picker"</Link>" lets users pick "
                        "one of a few swatches."
                    </li>
                </ul>
            </Section>

            <Section title="Decision Guide">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Build a picker of your own from several parts sharing one color"</TableCell>
                        <TableCell><Link href=routes::doc::color_picker::Atom.materialize()>"Color Picker Atom"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick two channels at once, such as saturation and brightness"</TableCell>
                        <TableCell><Link href=routes::doc::ColorArea.materialize()>"Color Area"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick one channel, such as the hue or the red value, along a track"</TableCell>
                        <TableCell><Link href=routes::doc::ColorSlider.materialize()>"Color Slider"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick a hue on a circle"</TableCell>
                        <TableCell><Link href=routes::doc::ColorWheel.materialize()>"Color Wheel"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Let users type a hex code or the value of one channel"</TableCell>
                        <TableCell><Link href=routes::doc::ColorField.materialize()>"Color Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Let users pick one of a few predefined colors"</TableCell>
                        <TableCell><Link href=routes::doc::ColorSwatchPicker.materialize()>"Color Swatch Picker"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a color with an accessible name"</TableCell>
                        <TableCell><Link href=routes::doc::ColorSwatch.materialize()>"Color Swatch"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "A button showing the current color opens a "
                    <Link href=routes::doc::color_picker::Atom.materialize()>"ColorPicker"</Link>" in a "
                    <Link href=routes::doc::Popover.materialize()>"popover"</Link>"; the "
                    <Link href=routes::doc::color_swatch::Atom.materialize()>"ColorSwatch"</Link>" in the button follows "
                    "every change, and the button\u{2019}s label names the color:"
                </p>

                <Demo description="A button with a color swatch opening a color picker in a popover" source=include_str!("demos/color.rs") source_open=true>
                    <ColorPopoverDemo/>
                </Demo>
            </Section>

            <Section title="Color Values">
                <p>
                    "The color types and traits live in "<Code inline=true>"leptonic::utils::color"</Code>"."
                </p>

                <Section title="Color Types">
                    <p>
                        "Three color types hold a color in one color space each. They convert into each other with "
                        <Code inline=true>"From"</Code>" ("<Code inline=true>"RGB8::from(hsv)"</Code>"); conversions to RGB "
                        "round each channel to the nearest integer."
                    </p>
                    <DocTable headers=&["Type", "Channels", "Ranges"]>
                        <TableRow>
                            <TableCell><Code inline=true>"HSV"</Code></TableCell>
                            <TableCell><Code inline=true>"HsvChannel::{Hue, Saturation, Brightness}"</Code></TableCell>
                            <TableCell>"Hue 0\u{2013}360 (step 1, page 15); saturation and brightness 0\u{2013}1 (step 0.01, page 0.1)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"HSL"</Code></TableCell>
                            <TableCell><Code inline=true>"HslChannel::{Hue, Saturation, Lightness}"</Code></TableCell>
                            <TableCell>"Hue 0\u{2013}360 (step 1, page 15); saturation and lightness 0\u{2013}1 (step 0.01, page 0.1)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"RGB8"</Code></TableCell>
                            <TableCell><Code inline=true>"RgbChannel::{Red, Green, Blue}"</Code></TableCell>
                            <TableCell>"0\u{2013}255 (step 1, page 17)"</TableCell>
                        </TableRow>
                    </DocTable>
                    <p>
                        <Code inline=true>"RGB8"</Code>" displays as a hex code ("<Code inline=true>"#4287F5"</Code>") and parses "
                        "one with "<Code inline=true>"RGB8::from_hex"</Code>"."
                    </p>
                </Section>

                <Section title="Alpha">
                    <p>
                        <Code inline=true>"Alpha<C>"</Code>" adds an alpha channel to any of the three types, from "
                        <Code inline=true>"0.0"</Code>" (transparent) to "<Code inline=true>"1.0"</Code>" (opaque): "
                        <Code inline=true>"Alpha::new(hsv).with_alpha(0.5)"</Code>". Its channels are the color\u{2019}s, wrapped "
                        "in "<Code inline=true>"AlphaChannel::Color"</Code>", plus "<Code inline=true>"AlphaChannel::Alpha"</Code>
                        ", so every color hook and atom edits alpha too. In "<Code inline=true>"view!"</Code>", put a "
                        "turbofish in braces: "<Code inline=true>"<ColorSlider channel={AlphaChannel::<HsvChannel>::Alpha}>"</Code>
                        ". An alpha slider\u{2019}s value text is only the percentage, without a color name."
                    </p>
                    <Demo description="A hue and an alpha slider of one color, with its swatch on a checkerboard" source=include_str!("demos/color_alpha.rs")>
                        <ColorAlphaDemo/>
                    </Demo>
                </Section>

                <Section title="ColorValue">
                    <p>
                        "The color hooks and atoms are generic over the "<Code inline=true>"ColorValue"</Code>" trait, which "
                        "the three types and their "<Code inline=true>"Alpha"</Code>" versions implement. Each type brings its own channel enum, so you can\u{2019}t ask an RGB color "
                        "for its hue: the compiler rejects it. Props that only show a color, such as a swatch\u{2019}s or a "
                        "preview\u{2019}s "<Code inline=true>"color"</Code>", take a "<Code inline=true>"ColorProp"</Code>
                        " instead: any color value or a signal of one."
                    </p>
                    <Code language=Language::Rust>
                        {indoc!(r"
                            pub trait ColorValue: Clone + Copy + PartialEq + Debug + Send + Sync + From<Color> + Into<Color> + 'static {
                                type Channel: ColorChannel<Color = Self>;

                                fn channel_value(&self, channel: Self::Channel) -> f64;
                                fn with_channel_value(&self, channel: Self::Channel, value: f64) -> Self;
                                const HAS_ALPHA: bool = false;

                                fn channels() -> Vec<Self::Channel>;
                                fn channel_range(channel: Self::Channel) -> ColorChannelRange;
                                fn channel_name(channel: Self::Channel) -> &'static str;
                                fn is_alpha_channel(channel: Self::Channel) -> bool;
                                fn format_channel_value(&self, channel: Self::Channel) -> String;
                                fn to_css_string(&self) -> String;
                                fn to_css_string_with_alpha(&self, alpha: f64) -> String;
                                fn to_rgb8(&self) -> RGB8;
                                fn hue_channel() -> Option<Self::Channel>;
                                fn color_name(&self) -> String;
                                fn hue_name(&self) -> String;
                                // ... plus axis, display-color and gradient helpers the hooks use.
                            }
                        ")}
                    </Code>
                    <p>
                        "The channel enums implement "<Code inline=true>"ColorChannel"</Code>", which names their color type. "
                        "Atoms that take a channel infer the color type from it: "
                        <Code inline=true>"<ColorSlider channel=HsvChannel::Hue>"</Code>" is a slider of an "
                        <Code inline=true>"HSV"</Code>" color, without naming the type."
                    </p>
                </Section>

                <Section title="Color">
                    <p>
                        <Code inline=true>"Color"</Code>" holds a color of any of the three types, with alpha: it is "
                        <Code inline=true>"Alpha<OpaqueColor>"</Code>", where "<Code inline=true>"OpaqueColor"</Code>" is one of "
                        <Code inline=true>"Rgb"</Code>", "<Code inline=true>"Hsv"</Code>" and "<Code inline=true>"Hsl"</Code>
                        ". Create one with "<Code inline=true>"Color::from(..)"</Code>". It keeps the space it was set in: a gray set as HSV keeps its hue, which RGB would lose. That makes it the color that parts of "
                        "different spaces share, such as those of a "<Link href=routes::doc::ColorPicker.materialize()>"color picker"</Link>
                        ". Its default is black."
                    </p>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::utils::color::{Color, HSV, RGB8};

                            let color = Color::from(HSV { hue: 210.0, saturation: 0.6, brightness: 0.8 });
                            let rgb: RGB8 = color.to::<RGB8>();

                            // CSS-like text: #rgb, #rgba, #rrggbb, #rrggbbaa, rgb(r, g, b), rgba(r, g, b, a),
                            // hsb(h, s%, b%), hsba(..), hsl(h, s%, l%) and hsla(..).
                            let parsed = "hsl(210, 60%, 50%)".parse::<Color>(); // Ok(Color::from(..))
                            let invalid = "teal".parse::<Color>(); // Err(ParseColorError), no color keywords

                            // Displays as a CSS color: "rgb(r, g, b)", or "rgba(r, g, b, a)" when transparent.
                            let css = color.to_string();
                        "#)}
                    </Code>
                </Section>

                <Section title="Color Names">
                    <p>
                        <Code inline=true>"color_name()"</Code>" describes a color in words, such as \u{201c}dark vibrant "
                        "blue\u{201d} or \u{201c}very light grayish green\u{201d}, from its lightness, chroma and hue in the "
                        "perceptual OKLCH space; "<Code inline=true>"hue_name()"</Code>" names only its hue (\u{201c}blue\u{201d}). "
                        "The color controls use them so that colors aren\u{2019}t conveyed by sight alone: a swatch is named "
                        "after its color, and the value texts of areas, sliders and wheels end with the color\u{2019}s name (the "
                        "hue\u{2019}s, on a hue). A transparent color\u{2019}s name ends with its transparency (\u{201c}vibrant "
                        "red, 80% transparent\u{201d}); a fully transparent swatch is named \u{201c}transparent\u{201d}. The names are "
                        "English for every locale."
                    </p>
                    <p>"Type a color to see how it parses and what it is called:"</p>
                    <Demo description="A text field parsing a CSS-like color, with its swatch, color name and hue name" source=include_str!("demos/color_names.rs")>
                        <ColorNamesDemo/>
                    </Demo>
                </Section>
            </Section>
        </DocPage>
    }
}
