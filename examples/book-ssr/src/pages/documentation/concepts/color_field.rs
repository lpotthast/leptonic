use leptos::prelude::*;

use super::demos::color_field::ColorFieldConceptDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageColorFieldOverview() -> impl IntoView {
    view! {
        <DocPage title="Color Field">
            <p>
                "A color field lets users type a color: a whole color as a hex code ("<Code inline=true>"#1E6EC8"</Code>"), "
                "or one channel of it as a number, such as its hue or its red value. It suits users who know the exact "
                "color, e.g. from a style guide, and complements the parts of a "
                <Link href=routes::doc::ColorPicker.materialize()>"color picker"</Link>" that are used by eye."
            </p>
            <p>
                "Typed text is committed when the user presses "<Keys keys="Enter"/>" or leaves the field; text that is no "
                "color reverts to the last one. The arrow keys and the scroll wheel step the color while the field has focus."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Type a hex code, or the value of one channel"</TableCell>
                        <TableCell><b>"Color Field"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick a color by eye"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::ColorArea.materialize()>"Color Area"</Link>", "
                            <Link href=routes::doc::ColorSlider.materialize()>"Color Slider"</Link>" or "
                            <Link href=routes::doc::ColorWheel.materialize()>"Color Wheel"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick one of a few predefined colors"</TableCell>
                        <TableCell><Link href=routes::doc::ColorSwatchPicker.materialize()>"Color Swatch Picker"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Combine a field with the visual parts"</TableCell>
                        <TableCell><Link href=routes::doc::ColorPicker.materialize()>"Color Picker"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Enter a number that isn\u{2019}t part of a color"</TableCell>
                        <TableCell><Link href=routes::doc::NumberField.materialize()>"Number Field"</Link></TableCell>
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
                        <TableCell><Link href=routes::doc::color_field::Hook.materialize()>"Color Field Hooks"</Link></TableCell>
                        <TableCell>
                            "The states of a hex field and of a channel field, and the attributes of the label and the input, "
                            "for markup you write yourself; the channel field hook also returns stepper buttons, which the atom doesn\u{2019}t render."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::color_field::Atom.materialize()>"Color Field Atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"ColorField"</Code>" and "<Code inline=true>"ColorChannelField"</Code>
                            " atoms holding an "<Code inline=true>"Input"</Code>", a "<Code inline=true>"Label"</Code>" and "
                            "optionally a description and an error message, styled through data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Give the "<Code inline=true>"ColorField"</Code>" atom the color as "<Code inline=true>"value"</Code>
                    " and "<Code inline=true>"set_value"</Code>" (an "<Code inline=true>"Option<RGB8>"</Code>": the field can "
                    "be empty), a "<Code inline=true>"Label"</Code>" and an "<Code inline=true>"Input"</Code>":"
                </p>
                <Demo
                    description="A hex color field showing the committed color and its name"
                    source=include_str!("demos/color_field.rs")
                    source_open=true
                >
                    <ColorFieldConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "Both fields are labelled text inputs: the hex field a plain "<Code inline=true>"textbox"</Code>
                        ", the channel field a number field (\u{201c}Number field\u{201d} as role description). Without a "
                        "label, a channel field is named after its channel (\u{201c}Hue\u{201d}, \u{201c}Red\u{201d})."
                    </li>
                    <li>
                        "Typing in the hex field accepts only hex digits (at most six, after an optional "
                        <Code inline=true>"#"</Code>"). Validation errors are linked to the input, as in every "
                        <Link href=routes::doc::Forms.materialize()>"form field"</Link>"."
                    </li>
                    <li>
                        "Show the typed color next to the field, e.g. with a "
                        <Link href=routes::doc::ColorSwatch.materialize()>"color swatch"</Link>
                        ", so users can check it without decoding the hex code."
                    </li>
                </ul>
                <p>"The keys do nothing while a field is disabled or read-only."</p>

                <Section title="Hex Field">
                    <KeyboardTable>
                        <KeyRow keys="Enter">"Commit the typed color."</KeyRow>
                        <KeyRow keys="ArrowUp / ArrowDown">"Increase or decrease the hex value by one."</KeyRow>
                        <KeyRow keys="PageUp / PageDown">"Increase or decrease the hex value by one, like the arrow keys."</KeyRow>
                        <KeyRow keys="Home">"Set the color to "<Code inline=true>"#000000"</Code>"."</KeyRow>
                        <KeyRow keys="End">"Set the color to "<Code inline=true>"#FFFFFF"</Code>"."</KeyRow>
                    </KeyboardTable>
                </Section>

                <Section title="Channel Field">
                    <p>
                        "As in a "<Link href=routes::doc::NumberField.materialize()>"number field"</Link>
                        ", with the channel\u{2019}s step:"
                    </p>
                    <KeyboardTable>
                        <KeyRow keys="Enter">"Commit the typed value."</KeyRow>
                        <KeyRow keys="ArrowUp / ArrowDown">"Increase or decrease the channel by one step."</KeyRow>
                        <KeyRow keys="PageUp / PageDown">"Increase or decrease the channel by one step, like the arrow keys."</KeyRow>
                        <KeyRow keys="Home / End">"Set the channel to its minimum or maximum."</KeyRow>
                    </KeyboardTable>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::color_field::Hook.materialize()>"Color Field Hooks"</Link></li>
                <li><Link href=routes::doc::color_field::Atom.materialize()>"Color Field Atoms"</Link></li>
                <li><Link href=routes::doc::ColorPicker.materialize()>"Color Picker"</Link></li>
                <li><Link href=routes::doc::NumberField.materialize()>"Number Field"</Link></li>
                <li><Link href=routes::doc::Color.materialize()>"Color"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
