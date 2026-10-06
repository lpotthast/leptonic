use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::slider::SliderConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageSliderOverview() -> impl IntoView {
    view! {
        <DocPage title="Slider">
            <p>
                "Sliders let users select a value or a range from an interval by dragging a thumb along a track. "
                "They fit best when the position within the range matters more than the exact number."
            </p>

            <p>"Leptonic provides sliders at all three layers, with any number of thumbs."</p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Pick a value from a continuous range"</TableCell><TableCell><b>"Slider"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Adjust both bounds of a range"</TableCell><TableCell><b>"Slider"</b>" with two thumbs"</TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Enter a precise number with the keyboard"</TableCell>
                        <TableCell><Link href=routes::doc::text_field::NumberFieldHook.materialize()>"Number field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Choose from a small set of discrete options"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::Select.materialize()>"Select"</Link>" / "
                            <Link href=routes::doc::Radio.materialize()>"Radio"</Link>
                        </TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "If the range is very large or precision matters more than visual feedback, a number field is the better choice."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::slider::Hook.materialize()>"Slider hooks"</Link></TableCell>
                        <TableCell>"State, dragging, keyboard handling and ARIA for markup you write and position yourself."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::slider::Atom.materialize()>"Slider atom"</Link></TableCell>
                        <TableCell>"Unstyled track, fill, thumb, output and mark components that position themselves."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::slider::Component.materialize()>"Slider component"</Link></TableCell>
                        <TableCell>"Themed "<Code inline=true>"Slider"</Code>" and "<Code inline=true>"RangeSlider"</Code>
                            " with marks, value popovers and variants."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The component shows "<Code inline=true>"value"</Code>" and reports changes through "
                    <Code inline=true>"set_value"</Code>":"
                </p>

                <Demo description="Volume slider showing its value" source=include_str!("demos/slider.rs") source_open=true>
                    <SliderConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Sliders follow the WAI-ARIA "
                    <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/slider/" target=LinkTarget::_Blank>"Slider pattern"</LinkExt>
                    ". All layers build on the same hooks."
                </p>

                <ul>
                    <li><Code inline=true>"role=\"group\""</Code>" on the slider container, with "<Code inline=true>"aria-disabled"</Code>" while disabled."</li>
                    <li>
                        "Each thumb contains a visually hidden "<Code inline=true>"<input type=\"range\">"</Code>
                        ". It receives focus and has the slider role, "<Code inline=true>"aria-valuenow"</Code>", "
                        <Code inline=true>"aria-valuemin"</Code>", "<Code inline=true>"aria-valuemax"</Code>", "
                        <Code inline=true>"aria-valuetext"</Code>" and "<Code inline=true>"aria-orientation"</Code>"."
                    </li>
                    <li>
                        "The minimum and maximum of a thumb are limited by its neighbors, so thumbs of a range slider cannot cross."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="ArrowRight / ArrowUp">"Increase by one step."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowDown">"Decrease by one step."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"Increase or decrease by the page size (also "<Keys keys="Shift"/>" + arrow keys)."</KeyRow>
                    <KeyRow keys="Home">"Set to the minimum."</KeyRow>
                    <KeyRow keys="End">"Set to the maximum."</KeyRow>
                </KeyboardTable>
            </Section>
        </DocPage>
    }
}
