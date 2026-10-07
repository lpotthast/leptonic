use leptos::prelude::*;

use super::demos::color_area::ColorAreaConceptDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageColorAreaOverview() -> impl IntoView {
    view! {
        <DocPage title="Color Area">
            <p>
                "A color area picks two channels of a color at once, such as saturation and brightness, by dragging a thumb "
                "across a two-dimensional gradient. The third channel, often the hue, stays fixed; pair the area with a "
                <Link href=routes::doc::ColorSlider.materialize()>"color slider"</Link>" or a "
                <Link href=routes::doc::ColorWheel.materialize()>"color wheel"</Link>" to change it."
            </p>
            <p>
                "The area works with any of leptonic\u{2019}s color types (HSV, HSL and RGB) and any two of their "
                "channels. The arrow keys change the channels too, and two hidden range inputs carry them into forms."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Pick two channels of a color at once, by eye"</TableCell>
                        <TableCell><b>"Color Area"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Adjust a single channel, such as the hue"</TableCell>
                        <TableCell><Link href=routes::doc::ColorSlider.materialize()>"Color Slider"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick a hue on a circle"</TableCell>
                        <TableCell><Link href=routes::doc::ColorWheel.materialize()>"Color Wheel"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Type an exact color, as a hex code or channel values"</TableCell>
                        <TableCell><Link href=routes::doc::ColorField.materialize()>"Color Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a color without editing it"</TableCell>
                        <TableCell><Link href=routes::doc::ColorSwatch.materialize()>"Color Swatch"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Offer a complete picker that combines these"</TableCell>
                        <TableCell><Link href=routes::doc::ColorPicker.materialize()>"Color Picker"</Link></TableCell>
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
                        <TableCell><Link href=routes::doc::color_area::Hook.materialize()>"Color Area Hooks"</Link></TableCell>
                        <TableCell>
                            "The color, custom channel steps, and the attributes of the area, its thumb and the hidden inputs, "
                            "for markup you write and position yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::color_area::Atom.materialize()>"Color Area Atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"ColorArea"</Code>" and "<Code inline=true>"ColorThumb"</Code>" atoms that "
                            "draw the gradient and position the thumb, styled through data attributes. Show the picked color "
                            "with a "
                            <Link href=routes::doc::color_swatch::Atom.materialize()>"ColorSwatch"</Link>"."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Give the "<Code inline=true>"ColorArea"</Code>" atom the color as "<Code inline=true>"value"</Code>" and "
                    <Code inline=true>"set_value"</Code>", the two channels, an accessible name and a "
                    <Code inline=true>"ColorThumb"</Code>". Drag the thumb, click anywhere in the area, or tab to it and use "
                    "the arrow keys:"
                </p>
                <Demo
                    description="Saturation and brightness area for a blue, with a swatch of the picked color"
                    source=include_str!("demos/color_area.rs")
                    source_open=true
                >
                    <ColorAreaConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "The area is a "<Code inline=true>"role=\"group\""</Code>" named by its "<Code inline=true>"aria_label"</Code>
                        " and \u{201c}Color picker\u{201d}. It has no visible label of its own, so always give it one."
                    </li>
                    <li>
                        "The thumb holds two visually hidden "<Code inline=true>"<input type=\"range\">"</Code>" elements, "
                        "one per channel, following the WAI-ARIA "
                        <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/slider/" target=LinkTarget::Blank>"Slider pattern"</Link>
                        ". Screen readers list a single \u{201c}2D slider\u{201d}, the X input, whose value text names all "
                        "three channels followed by the color\u{2019}s name, e.g. \u{201c}Saturation: 60%, Brightness: 80%, Hue: "
                        "210\u{00b0}, \u{2026}\u{201d}. After a keyboard change, the value text names only the changed channel "
                        "and the color."
                    </li>
                    <li>
                        "Name the picked color next to the area, e.g. with a "
                        <Link href=routes::doc::ColorSwatch.materialize()>"color swatch"</Link>": a color shown only as a "
                        "gradient position is invisible to some users."
                    </li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus to the area\u{2019}s thumb."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowRight">"Decrease or increase the X channel by its step (reversed right to left)."</KeyRow>
                    <KeyRow keys="ArrowDown / ArrowUp">"Decrease or increase the Y channel by its step."</KeyRow>
                    <KeyRow keys="Shift + Arrow keys">"Change the channel by its page step."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"Increase or decrease the Y channel by its page step."</KeyRow>
                    <KeyRow keys="Home / End">"Decrease or increase the X channel by its page step (reversed right to left)."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::color_area::Hook.materialize()>"Color Area Hooks"</Link></li>
                <li><Link href=routes::doc::color_area::Atom.materialize()>"Color Area Atoms"</Link></li>
                <li><Link href=routes::doc::ColorSlider.materialize()>"Color Slider"</Link></li>
                <li><Link href=routes::doc::ColorWheel.materialize()>"Color Wheel"</Link></li>
                <li><Link href=routes::doc::ColorPicker.materialize()>"Color Picker"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
