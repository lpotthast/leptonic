use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::color_slider::ColorSliderConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageColorSliderOverview() -> impl IntoView {
    view! {
        <DocPage title="Color Slider">
            <p>
                "A color slider changes one channel of a color, such as its hue, its lightness or its red value, along a "
                "track that shows the colors the channel goes through. Several sliders bound to one color edit it channel "
                "by channel; a hue slider next to a "<Link href=routes::doc::ColorArea.materialize()>"color area"</Link>
                " completes a picker."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Adjust one channel of a color along a track"</TableCell>
                        <TableCell><b>"Color Slider"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick a hue on a circle"</TableCell>
                        <TableCell><Link href=routes::doc::ColorWheel.materialize()>"Color Wheel"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick two channels at once, such as saturation and brightness"</TableCell>
                        <TableCell><Link href=routes::doc::ColorArea.materialize()>"Color Area"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Type the exact value of a channel"</TableCell>
                        <TableCell><Link href=routes::doc::ColorField.materialize()>"Color Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick a number that isn\u{2019}t part of a color"</TableCell>
                        <TableCell><Link href=routes::doc::Slider.materialize()>"Slider"</Link></TableCell>
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
                        <TableCell><Link href=routes::doc::color_slider::Hook.materialize()>"Color Slider Hooks"</Link></TableCell>
                        <TableCell>
                            "The color and the slider of its channel, and the attributes and styles of the track, the thumb "
                            "and the hidden input, for markup you write yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::color_slider::Atom.materialize()>"Color Slider Atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"ColorSlider"</Code>", "<Code inline=true>"ColorSliderTrack"</Code>", "
                            <Code inline=true>"ColorSliderOutput"</Code>" and "<Code inline=true>"ColorThumb"</Code>" atoms that "
                            "draw the gradient and position the thumb, styled through data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Give the "<Code inline=true>"ColorSlider"</Code>" atom the color as "<Code inline=true>"value"</Code>" and "
                    <Code inline=true>"set_value"</Code>", the channel, a label, an output and a track with a thumb. Drag the "
                    "thumb, click the track, or tab to the thumb and use the arrow keys:"
                </p>
                <Demo
                    description="Hue slider with a label and the formatted hue"
                    source=include_str!("demos/color_slider.rs")
                    source_open=true
                >
                    <ColorSliderConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "A color slider follows the WAI-ARIA "
                        <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/slider/" target=LinkTarget::Blank>"Slider pattern"</Link>
                        ": the thumb holds a visually hidden "<Code inline=true>"<input type=\"range\">"</Code>" that takes the "
                        "focus and carries the value into forms."
                    </li>
                    <li>
                        "It is named by its label; without one, by the channel (\u{201c}Hue\u{201d}). The value text is the "
                        "formatted channel value and a name: the hue\u{2019}s for a hue slider (\u{201c}120\u{00b0}, "
                        "green\u{201d}), the color\u{2019}s for the other channels."
                    </li>
                    <li>
                        "A color shown only as a position on a gradient is lost on some users: show the value with the "
                        "output, or the color with a "<Link href=routes::doc::ColorSwatch.materialize()>"color swatch"</Link>"."
                    </li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus to the thumb."</KeyRow>
                    <KeyRow keys="ArrowRight / ArrowUp">"Increase the channel by its step (on a horizontal track, reversed right to left)."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowDown">"Decrease the channel by its step (on a horizontal track, reversed right to left)."</KeyRow>
                    <KeyRow keys="Shift + Arrow keys">"Change the channel by its page step."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"Increase or decrease the channel by its page step."</KeyRow>
                    <KeyRow keys="Home / End">"Set the channel to its minimum or maximum."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::color_slider::Hook.materialize()>"Color Slider Hooks"</Link></li>
                <li><Link href=routes::doc::color_slider::Atom.materialize()>"Color Slider Atoms"</Link></li>
                <li><Link href=routes::doc::ColorArea.materialize()>"Color Area"</Link></li>
                <li><Link href=routes::doc::ColorWheel.materialize()>"Color Wheel"</Link></li>
                <li><Link href=routes::doc::Slider.materialize()>"Slider"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
