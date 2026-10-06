use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::color_picker::ColorPickerConceptDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageColorPickerOverview() -> impl IntoView {
    view! {
        <DocPage title="Color Picker">
            <p>
                "A color picker lets users choose any color. It combines several controls that all edit one color: an area "
                "for two channels at once, sliders or a wheel for single channels, and fields for exact values. Each part "
                "suits a different user: the area and the sliders for picking by eye, the fields for typing a known color."
            </p>
            <p>
                "Each part also works on its own; the picker keeps them in sync. Leptonic offers a themed picker, an atom "
                "that shares one color among the color atoms of a picker of your own design, and a shared state for parts "
                "built from hooks."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Let users choose any color, by eye or by value"</TableCell>
                        <TableCell><b>"Color Picker"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick only two channels, such as saturation and brightness"</TableCell>
                        <TableCell><Link href=routes::doc::ColorArea.materialize()>"Color Area"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Adjust a single channel"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::ColorSlider.materialize()>"Color Slider"</Link>" / "
                            <Link href=routes::doc::ColorWheel.materialize()>"Color Wheel"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Type a hex code"</TableCell>
                        <TableCell><Link href=routes::doc::ColorField.materialize()>"Color Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Choose one of a few predefined colors"</TableCell>
                        <TableCell><Link href=routes::doc::ColorSwatchPicker.materialize()>"Color Swatch Picker"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>
                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::color_picker::Hook.materialize()>"use_color_picker_state"</Link></TableCell>
                        <TableCell>"The shared color for parts built from the color hooks, and how to bind each part to it."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::color_picker::Atom.materialize()>"Color Picker Atom"</Link></TableCell>
                        <TableCell>
                            "A "<Code inline=true>"ColorPicker"</Code>" atom that renders nothing itself: the unstyled color "
                            "atoms inside it (areas, sliders, wheels, fields, swatches and swatch pickers) share its color, "
                            "each in its own color space."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::color_picker::Component.materialize()>"Color Picker Components"</Link></TableCell>
                        <TableCell>
                            "A themed "<Code inline=true>"ColorPicker"</Code>": a preview, a saturation and brightness area, a "
                            "hue slider, and fields for the HSB and RGB channels and the hex code. Its parts "
                            <Code inline=true>"ColorPreview"</Code>", "<Code inline=true>"ColorPalette"</Code>" and "
                            <Code inline=true>"HueSlider"</Code>" are available on their own."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Pass the themed component a "<Link href=format!("{}#color", routes::doc::Color.materialize())>"Color"</Link>
                    " signal as "<Code inline=true>"value"</Code>" and "<Code inline=true>"set_value"</Code>". Read the color in "
                    "the space you need with "<Code inline=true>"to"</Code>":"
                </p>
                <Demo
                    description="Themed color picker kept in a signal, showing the color as HSL and by name"
                    source=include_str!("demos/color_picker.rs")
                    source_open=true
                >
                    <ColorPickerConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "WAI-ARIA has no pattern for a color picker as a whole: it is a group of controls with patterns of their own. "
                    "Make each part usable on its own, so that users who can\u{2019}t drag or see the gradient still reach "
                    "every color."
                </p>
                <ul>
                    <li>
                        "Areas, sliders and wheels follow the WAI-ARIA "
                        <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/slider/" target=LinkTarget::Blank>"Slider pattern"</Link>
                        " and end their value text with the color\u{2019}s name, or the hue\u{2019}s on a hue (\u{201c}120\u{00b0}, "
                        "green\u{201d}). The hex field is a labelled text box, the channel fields are labelled number fields, "
                        "and swatches are images named after their color."
                    </li>
                    <li>
                        "In the "<Code inline=true>"ColorPicker"</Code>" component, every part works with the keyboard and "
                        "screen readers: the saturation and brightness area (named \u{201c}Color picker\u{201d}), the hue slider "
                        "(named \u{201c}Hue\u{201d}), the fields labelled with their channels and \u{201c}Hex\u{201d}, and the "
                        "preview named after the color."
                    </li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="Tab / Shift + Tab">"Moves focus between the parts of the picker."</KeyRow>
                    <KeyRow keys="Arrow keys">"On a slider, a wheel or an area: change the focused channel."</KeyRow>
                    <KeyRow keys="ArrowUp / ArrowDown">"In a field: increase or decrease the value by one step."</KeyRow>
                    <KeyRow keys="Enter">"In a field: commit the typed value."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::color_picker::Hook.materialize()>"use_color_picker_state"</Link></li>
                <li><Link href=routes::doc::color_picker::Atom.materialize()>"Color Picker Atom"</Link></li>
                <li><Link href=routes::doc::color_picker::Component.materialize()>"Color Picker Components"</Link></li>
                <li><Link href=routes::doc::ColorArea.materialize()>"Color Area"</Link></li>
                <li><Link href=routes::doc::ColorSwatchPicker.materialize()>"Color Swatch Picker"</Link></li>
                <li><Link href=routes::doc::Color.materialize()>"Color"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
