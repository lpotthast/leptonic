use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::color_swatch::ColorSwatchConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageColorSwatchOverview() -> impl IntoView {
    view! {
        <DocPage title="Color Swatch">
            <p>
                "A color swatch shows a color as a small preview: the color a picker currently holds, the entries of a "
                "palette, or the color of a tag or a theme. It only displays the color; it doesn\u{2019}t change it. Screen "
                "readers announce it as an image named after the color, so the color isn\u{2019}t conveyed by sight alone."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Show a color"</TableCell><TableCell><b>"Color Swatch"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Let users pick one of a few colors"</TableCell>
                        <TableCell><Link href=routes::doc::ColorSwatchPicker.materialize()>"Color Swatch Picker"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Let users pick any color"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::ColorPicker.materialize()>"Color Picker"</Link>", or its parts: "
                            <Link href=routes::doc::ColorArea.materialize()>"Color Area"</Link>", "
                            <Link href=routes::doc::ColorSlider.materialize()>"Color Slider"</Link>", "
                            <Link href=routes::doc::ColorWheel.materialize()>"Color Wheel"</Link>", "
                            <Link href=routes::doc::ColorField.materialize()>"Color Field"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow><TableCell>"Color an element purely for decoration"</TableCell><TableCell>"CSS"</TableCell></TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::color_swatch::Hook.materialize()>"use_color_swatch"</Link></TableCell>
                        <TableCell>"The role, the accessible name and the CSS color, for an element you render and style."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::color_swatch::Atom.materialize()>"Color Swatch Atom"</Link></TableCell>
                        <TableCell>"An unstyled "<Code inline=true>"<div>"</Code>" filled with the color; you give it a size and shape."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>"A swatch of a fixed color, named for screen readers with "<Code inline=true>"color_name"</Code>":"</p>

                <Demo description="A named ColorSwatch atom" source=include_str!("demos/color_swatch.rs") source_open=true>
                    <ColorSwatchConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>"A swatch is an image of a color. It is not focusable and has no keyboard interaction."</p>

                <ul>
                    <li><Code inline=true>"role=\"img\""</Code>" with "<Code inline=true>"aria-roledescription=\"color swatch\""</Code>"."</li>
                    <li>
                        <Code inline=true>"aria-label"</Code>": the color name you give it, or else the color\u{2019}s "
                        <Link href=format!("{}#color-names", routes::doc::Color.materialize())>"name"</Link>" (e.g. "
                        "\u{201c}dark vibrant green\u{201d}), followed by "<Code inline=true>"aria_label"</Code>" if set "
                        "(\u{201c}dark vibrant green, Background\u{201d}). Name your colors where a name says more, such as "
                        "brand colors."
                    </li>
                    <li>
                        <Code inline=true>"forced-color-adjust: none"</Code>" keeps the color in Windows high contrast mode. The "
                        "hook\u{2019}s styles set it."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::color_swatch::Hook.materialize()>"use_color_swatch"</Link></li>
                <li><Link href=routes::doc::color_swatch::Atom.materialize()>"Color Swatch Atom"</Link></li>
                <li><Link href=routes::doc::ColorSwatchPicker.materialize()>"Color Swatch Picker"</Link></li>
                <li><Link href=routes::doc::ColorPicker.materialize()>"Color Picker"</Link></li>
                <li><Link href=routes::doc::Color.materialize()>"Color"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
