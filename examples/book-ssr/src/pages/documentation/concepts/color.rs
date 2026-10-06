use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::color::ColorConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageColorOverview() -> impl IntoView {
    view! {
        <DocPage title="Color">
            <p>
                "Color controls let users view and choose colors through different interaction patterns: 2D gradient "
                "areas, channel sliders, hue wheels, hex text fields and swatches. Each pattern suits a different task, "
                "and a full color picker combines several of them on one shared color."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Drop in a complete color picker"</TableCell>
                        <TableCell><Link href=routes::doc::color::Component.materialize()>"ColorPicker component"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Build a custom color UI with full control over layout and behavior"</TableCell>
                        <TableCell><Link href=routes::doc::color::Hooks.materialize()>"Color hooks"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Display a color with an accessible name"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::color::Atom.materialize()>"ColorSwatch atom"</Link>" or "
                            <Link href=routes::doc::hooks::UseColorSwatch.materialize()>"use_color_swatch"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Let users enter a hex color code"</TableCell>
                        <TableCell><Link href=routes::doc::hooks::UseColorField.materialize()>"use_color_field"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Color picking exists as hooks, as atoms and as a component. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate, and the "<Link href=routes::doc::color::Hooks.materialize()>"color hooks"</Link>
                    " page for what the hooks share."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::hooks::UseColorArea.materialize()>"use_color_area"</Link></TableCell>
                        <TableCell>"A 2D gradient area for two channels at once (e.g. saturation and brightness)."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::hooks::UseColorSlider.materialize()>"use_color_slider"</Link></TableCell>
                        <TableCell>"A linear slider for one channel (e.g. hue, red)."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::hooks::UseColorWheel.materialize()>"use_color_wheel"</Link></TableCell>
                        <TableCell>"A circular wheel for the hue (0\u{00b0}\u{2013}360\u{00b0})."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::hooks::UseColorField.materialize()>"use_color_field"</Link></TableCell>
                        <TableCell>"A text input for hex values ("<Code inline=true>"#RRGGBB"</Code>")."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::hooks::UseColorChannelField.materialize()>"use_color_channel_field"</Link></TableCell>
                        <TableCell>"A number input for one channel."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::hooks::UseColorSwatch.materialize()>"use_color_swatch"</Link></TableCell>
                        <TableCell>"A display-only color preview."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::color::Atom.materialize()>"ColorArea and ColorSwatch atoms"</Link></TableCell>
                        <TableCell>"The color area and the swatch as unstyled components built on the hooks; you add the size and look."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::color::Component.materialize()>"ColorPicker component"</Link></TableCell>
                        <TableCell>"A ready-made, themed color picker."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>"The simplest way to add a color picker is the component:"</p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        let (hsv, set_hsv) = signal(HSV::new());

                        view! {
                            <ColorPicker hsv=hsv set_hsv=set_hsv />
                        }
                    ")}
                </Code>

                <p>
                    "For your own picker layout, start from the "<Link href=routes::doc::color::Atom.materialize()>"color atoms"</Link>
                    " instead: a "<Code inline=true>"ColorArea"</Code>" for two channels and a "<Code inline=true>"ColorSwatch"</Code>
                    " showing the result."
                </p>

                <p>"Color pickers fit well into popovers. Click a swatch to edit its color; the swatch updates as you go."</p>

                <Demo description="Theme palette with popover color pickers" source=include_str!("demos/color.rs")>
                    <ColorConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "Color area: "<Code inline=true>"role=\"group\""</Code>" with a visually hidden range input, announced as "
                        "a \u{201c}2D slider\u{201d} whose value text names all three channels."
                    </li>
                    <li>
                        "Color slider and wheel: a visually hidden range input whose "<Code inline=true>"aria-valuetext"</Code>
                        " includes the hue name for hue channels (e.g. \u{201c}120\u{00b0}, green\u{201d})."
                    </li>
                    <li>"Color field: "<Code inline=true>"role=\"spinbutton\""</Code>" on the text input."</li>
                    <li>
                        "Color swatch: "<Code inline=true>"role=\"img\""</Code>" with "
                        <Code inline=true>"aria-roledescription=\"color swatch\""</Code>" and the color name as label."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="ArrowLeft / ArrowRight / ArrowUp / ArrowDown">"Change the value of a slider, wheel or field, or move a color area\u{2019}s thumb."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"Change the value in larger steps."</KeyRow>
                    <KeyRow keys="Home / End">"Jump to the minimum or maximum (in a color area: change the X channel in larger steps)."</KeyRow>
                </KeyboardTable>
            </Section>
        </DocPage>
    }
}
