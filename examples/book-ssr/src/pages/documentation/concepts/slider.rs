use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::slider::SliderConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageSliderOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="slider" class="anchor">
                "Slider"
                <AnchorLink href="#slider" description="Direct link to article header"/>
            </h1>

            <p>
                "Sliders let users select a value or range from a continuous interval "
                "by dragging a thumb along a track. They are ideal when the exact numeric value "
                "matters less than its position within a range."
            </p>

            <p>
                "Leptonic provides sliders at three abstraction levels. "
                "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                " for a detailed explanation of each layer. "
                "This page covers the slider concept; choose a layer below to dive into specifics."
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
                            <TableCell>"Pick a value from a continuous range"</TableCell>
                            <TableCell><b>"Slider"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Enter a precise number via keyboard"</TableCell>
                            <TableCell>"Number Field"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Choose from a small set of discrete options"</TableCell>
                            <TableCell>"Select / Radio"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Compare or adjust two bounds of a range"</TableCell>
                            <TableCell>"Range Slider (Slider with two thumbs)"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <p>
                "If the range is very large or precision matters more than visual feedback, "
                "a NumberInput may be more appropriate."
            </p>

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
                <li><Link href=routes::doc::slider::Hook.materialize()>"Hook: use_slider"</Link></li>
                <li><Link href=routes::doc::slider::Atom.materialize()>"Atom: Slider"</Link></li>
                <li><Link href=routes::doc::slider::Component.materialize()>"Component: Slider"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use a slider (component layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    let (value, set_value) = signal(50.0_f64);

                    <Slider value=value set_value=set_value min=0.0 max=100.0 step=1.0 />
                ")}
            </Code>

            <DemoShell description="Slider with value display" source=include_str!("demos/slider.rs")>
                <SliderConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Leptonic sliders follow the WAI-ARIA Slider pattern. "
                "All layers produce the same accessibility behavior."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"role=\"slider\""</Code>" \u{2014} on each thumb element"</li>
                <li><Code inline=true>"aria-valuenow"</Code>", "<Code inline=true>"aria-valuemin"</Code>", "<Code inline=true>"aria-valuemax"</Code>" \u{2014} reactive value tracking"</li>
                <li><Code inline=true>"aria-orientation"</Code>" \u{2014} \"horizontal\" or \"vertical\""</li>
                <li><Code inline=true>"aria-disabled"</Code>" \u{2014} \"true\" when disabled"</li>
            </ul>

            <h3>"Keyboard interaction"</h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Key"</TableHeaderCell>
                            <TableHeaderCell>"Action"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"Right / Up"</Code></TableCell>
                            <TableCell>"Increase by one step"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Left / Down"</Code></TableCell>
                            <TableCell>"Decrease by one step"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Page Up"</Code></TableCell>
                            <TableCell>"Increase by page size"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Page Down"</Code></TableCell>
                            <TableCell>"Decrease by page size"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Home"</Code></TableCell>
                            <TableCell>"Set to minimum"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"End"</Code></TableCell>
                            <TableCell>"Set to maximum"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Slider", link: "#slider" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}
