use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell},
    routes,
};

use super::demos::color::ColorConceptDemo;

#[component]
pub fn PageColorOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="color" class="anchor">
                "Color"
                <AnchorLink href="#color" description="Direct link to article header"/>
            </h1>

            <p>
                "Color components let users view and select colors through various "
                "interaction patterns: 2D gradient areas, channel sliders, hue wheels, "
                "hex text fields, and more."
            </p>

            <p>
                "Leptonic provides color picking at two abstraction levels. "
                "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                " for a detailed explanation of each layer. "
                "This page covers the color concept; choose a layer below to dive into specifics."
            </p>

            <h2 id="when-to-use" class="anchor">
                "When to Use"
                <AnchorLink href="#when-to-use" description="Direct link to section: When to Use"/>
            </h2>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>"If you want to\u{2026}"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Use"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell>"Drop in a complete color picker with preview, palette, sliders, and inputs"</TableCell>
                            <TableCell><b>"ColorPicker component"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Build a custom color UI with full control over layout and behavior"</TableCell>
                            <TableCell><b>"Color hooks"</b>" (use_color_area, use_color_slider, etc.)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Display a color swatch with proper accessibility"</TableCell>
                            <TableCell><b>"use_color_swatch"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Let users enter a hex color code"</TableCell>
                            <TableCell><b>"use_color_field"</b></TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="dive-deeper" class="anchor">
                "Dive Deeper"
                <AnchorLink href="#dive-deeper" description="Direct link to section: Dive Deeper"/>
            </h2>

            <p>
                "Not sure which layer to pick? Read the "
                <Link href=routes::doc::Architecture.materialize()>"architecture guide"</Link>
                ". Otherwise, pick a layer:"
            </p>

            <ul>
                <li><Link href=routes::doc::color::Hooks.materialize()>"Hooks: Color hooks overview"</Link>" \u{2014} 6 hook families for building custom color UIs"</li>
                <li><Link href=routes::doc::color::Component.materialize()>"Component: ColorPicker"</Link>" \u{2014} Full-featured, ready-to-use color picker"</li>
            </ul>

            <h2 id="hooks-overview" class="anchor">
                "Available Hooks"
                <AnchorLink href="#hooks-overview" description="Direct link to section: Available Hooks"/>
            </h2>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>"Hook"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"use_color_area"</Code></TableCell>
                            <TableCell>"2D gradient area for two-channel selection (e.g. saturation + brightness)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"use_color_slider"</Code></TableCell>
                            <TableCell>"Linear slider for a single color channel (e.g. hue, red)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"use_color_wheel"</Code></TableCell>
                            <TableCell>"Circular hue wheel for selecting hue (0\u{00b0}\u{2013}360\u{00b0})"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"use_color_field"</Code></TableCell>
                            <TableCell>"Text input for hex color values (#RRGGBB)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"use_color_channel_field"</Code></TableCell>
                            <TableCell>"Numeric input for a single channel value"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"use_color_swatch"</Code></TableCell>
                            <TableCell>"Display-only color preview with accessibility"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to add a color picker (component layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    use leptonic::prelude::*;

                    let (hsv, set_hsv) = signal(HSV::new());

                    view! {
                        <ColorPicker hsv=hsv set_hsv=set_hsv />
                    }
                ")}
            </Code>

            <h2 id="demo" class="anchor">
                "Demo"
                <AnchorLink href="#demo" description="Direct link to section: Demo"/>
            </h2>

            <p>"Click any swatch to open a color picker. Changes update the swatch in real time."</p>

            <DemoShell
                description="Theme palette with popover color pickers"
                source=include_str!("demos/color.rs")
            >
                <ColorConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <ul>
                <li>"Color area: Uses "<Code inline=true>"role=\"group\""</Code>" with keyboard navigation (arrow keys, Page Up/Down, Home/End)"</li>
                <li>"Color swatch: Uses "<Code inline=true>"role=\"img\""</Code>" with "<Code inline=true>"aria-roledescription=\"color swatch\""</Code></li>
                <li>"Color wheel: Uses "<Code inline=true>"role=\"slider\""</Code>" with "<Code inline=true>"aria-valuetext"</Code>" including hue name (e.g. \"120\u{00b0}, green\")"</li>
                <li>"Color slider: Enriched "<Code inline=true>"aria-valuetext"</Code>" with channel name and value"</li>
            </ul>

            <p>
                "All color hooks are based on "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useColorArea.html" target=leptonic::hooks::LinkTarget::_Blank>
                    "react-aria\u{2019}s color hooks"
                </LinkExt>
                "."
            </p>
        </Article>
    }
}
