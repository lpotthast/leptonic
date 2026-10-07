use leptos::prelude::*;

use super::demos::color_wheel::ColorWheelConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageColorWheelOverview() -> impl IntoView {
    view! {
        <DocPage title="Color Wheel">
            <p>
                "A color wheel picks a hue on a ring of colors: 0\u{00b0} (red) at three o\u{2019}clock, going clockwise "
                "through yellow, green, cyan, blue and magenta. It changes only the hue; pair it with a "
                <Link href=routes::doc::ColorArea.materialize()>"color area"</Link>" or "
                <Link href=routes::doc::ColorSlider.materialize()>"color sliders"</Link>" for the other channels."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Pick a hue on a circle"</TableCell>
                        <TableCell><b>"Color Wheel"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Adjust a hue, or any other channel, along a straight track"</TableCell>
                        <TableCell><Link href=routes::doc::ColorSlider.materialize()>"Color Slider"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick two channels at once, such as saturation and brightness"</TableCell>
                        <TableCell><Link href=routes::doc::ColorArea.materialize()>"Color Area"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Type an exact hue"</TableCell>
                        <TableCell><Link href=routes::doc::ColorField.materialize()>"Color Field"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link>
                    " for how the layers relate."
                </p>
                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::color_wheel::Hook.materialize()>"Color Wheel Hooks"</Link></TableCell>
                        <TableCell>
                            "The color and its hue, and the attributes and styles of the ring, the thumb and the hidden "
                            "input, for markup you write yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::color_wheel::Atom.materialize()>"Color Wheel Atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"ColorWheel"</Code>", "<Code inline=true>"ColorWheelTrack"</Code>" and "
                            <Code inline=true>"ColorThumb"</Code>" atoms that draw the ring and position the thumb, styled "
                            "through data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Give the "<Code inline=true>"ColorWheel"</Code>" atom the color as "<Code inline=true>"value"</Code>" and "
                    <Code inline=true>"set_value"</Code>", its hue channel and the radii of the ring, a track and a thumb. Drag "
                    "the thumb, click the ring, or tab to the thumb and use the arrow keys:"
                </p>
                <Demo
                    description="Hue wheel showing the picked hue"
                    source=include_str!("demos/color_wheel.rs")
                    source_open=true
                >
                    <ColorWheelConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "The thumb holds a visually hidden "<Code inline=true>"<input type=\"range\">"</Code>" from 0 to 360, "
                        "following the WAI-ARIA "
                        <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/slider/" target=LinkTarget::Blank>"Slider pattern"</Link>
                        ". It takes the focus and carries the hue into forms."
                    </li>
                    <li>
                        "It is named \u{201c}Hue\u{201d} unless you name it; its value text is the hue and its name "
                        "(\u{201c}120\u{00b0}, green\u{201d})."
                    </li>
                    <li>
                        "Show the picked color next to the wheel, e.g. with a "
                        <Link href=routes::doc::ColorSwatch.materialize()>"color swatch"</Link>": a color shown only as a "
                        "position on a ring is lost on some users."
                    </li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus to the thumb."</KeyRow>
                    <KeyRow keys="ArrowRight / ArrowUp">"Increase the hue by 1\u{00b0}, wrapping around from 359\u{00b0} to 0\u{00b0}."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowDown">"Decrease the hue by 1\u{00b0}, wrapping around."</KeyRow>
                    <KeyRow keys="Shift + Arrow keys">"Change the hue by 15\u{00b0}."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"Increase or decrease the hue by 15\u{00b0}."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::color_wheel::Hook.materialize()>"Color Wheel Hooks"</Link></li>
                <li><Link href=routes::doc::color_wheel::Atom.materialize()>"Color Wheel Atoms"</Link></li>
                <li><Link href=routes::doc::ColorSlider.materialize()>"Color Slider"</Link></li>
                <li><Link href=routes::doc::ColorArea.materialize()>"Color Area"</Link></li>
                <li><Link href=routes::doc::ColorPicker.materialize()>"Color Picker"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
