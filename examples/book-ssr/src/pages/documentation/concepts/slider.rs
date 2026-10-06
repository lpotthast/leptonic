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

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Pick a value from a continuous range"</TableCell><TableCell><b>"Slider"</b></TableCell></TableRow>
                    <TableRow><TableCell>"Adjust both bounds of a range"</TableCell><TableCell><b>"Slider"</b>" with two thumbs"</TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Enter a precise number with the keyboard"</TableCell>
                        <TableCell><Link href=routes::doc::NumberField.materialize()>"Number Field"</Link></TableCell>
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
                        <TableCell><Link href=routes::doc::slider::Hook.materialize()>"Slider Hooks"</Link></TableCell>
                        <TableCell>"State, dragging, keyboard handling and ARIA for markup you write and position yourself."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::slider::Atom.materialize()>"Slider Atoms"</Link></TableCell>
                        <TableCell>"An unstyled "<Code inline=true>"Slider"</Code>" with track, fill, thumb, output and mark parts that position themselves."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::slider::Component.materialize()>"Slider Components"</Link></TableCell>
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
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/slider/" target=LinkTarget::Blank>"Slider pattern"</Link>
                    ". All layers build on the same hooks."
                </p>

                <ul>
                    <li><Code inline=true>"role=\"group\""</Code>" on the element around the thumbs, named by the slider\u{2019}s label."</li>
                    <li>
                        "Each thumb contains a visually hidden "<Code inline=true>"<input type=\"range\">"</Code>
                        ". It takes the focus and has the implicit slider role with the value, minimum and maximum, "
                        <Code inline=true>"aria-valuetext"</Code>" (the formatted value) and "<Code inline=true>"aria-orientation"</Code>
                        ". The thumbs of a range slider are named by the slider\u{2019}s label and their own name (\u{201c}Minimum\u{201d}, \u{201c}Maximum\u{201d})."
                    </li>
                    <li>
                        "The minimum and maximum of a thumb are limited by its neighbors, so the thumbs of a range slider can\u{2019}t cross."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus to the next thumb."</KeyRow>
                    <KeyRow keys="ArrowRight / ArrowUp">"Increases the value by one step."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowDown">"Decreases the value by one step."</KeyRow>
                    <KeyRow keys="PageUp / Shift + ArrowRight / Shift + ArrowUp">"Increases the value by a page: a tenth of the range, a multiple of the step."</KeyRow>
                    <KeyRow keys="PageDown / Shift + ArrowLeft / Shift + ArrowDown">"Decreases the value by a page."</KeyRow>
                    <KeyRow keys="Home">"Sets the value to the minimum, or to the previous thumb\u{2019}s value."</KeyRow>
                    <KeyRow keys="End">"Sets the value to the maximum, or to the next thumb\u{2019}s value."</KeyRow>
                </KeyboardTable>

                <p>
                    "In a right-to-left locale (of an enclosing "<Code inline=true>"I18nProvider"</Code>"), a horizontal slider "
                    "runs from right to left: the left and right arrow keys swap, and track presses are mirrored."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::slider::Hook.materialize()>"Slider Hooks"</Link></li>
                <li><Link href=routes::doc::slider::Atom.materialize()>"Slider Atoms"</Link></li>
                <li><Link href=routes::doc::slider::Component.materialize()>"Slider Components"</Link></li>
                <li><Link href=routes::doc::NumberField.materialize()>"Number Field"</Link></li>
                <li><Link href=routes::doc::ColorSlider.materialize()>"Color Slider"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
