use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::meter::MeterAtomDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomMeter() -> impl IntoView {
    view! {
        <DocPage title="Meter Atoms">
            <p>
                "The unstyled "<Code inline=true>"Meter"</Code>" atom shows a value within a known range, such as disk "
                "usage or a battery level. Its parts "<Code inline=true>"MeterFill"</Code>" and "
                <Code inline=true>"MeterValueText"</Code>" show the value; you draw the track. See the "
                <Link href=routes::doc::Meter.materialize()>"Meter overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"Meter"</Code>" calls "<Link href=routes::doc::meter::Hook.materialize()>"use_meter"</Link>
                    " for "<Code inline=true>"role=\"meter\""</Code>", the value attributes and the label, and provides the "
                    "percentage and the value text to its parts. A "<Link href=routes::doc::field::Atom.materialize()>"Label"</Link>
                    " inside it names the meter."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude::*;

                        let battery_level = RwSignal::new(68.0);

                        view! {
                            <Meter value=battery_level classes="meter">
                                <Label>"Battery"</Label>
                                <MeterValueText />
                                <div class="meter-track">
                                    <MeterFill classes="meter-fill" />
                                </div>
                            </Meter>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="Storage meter in gigabytes with its own value text" source=include_str!("demos/meter.rs")>
                    <MeterAtomDemo/>
                </Demo>
            </Section>

            <Section title="Meter">
                <p>"The meter element, a "<Code inline=true>"<div>"</Code>" holding the label, the parts and your track."</p>

                <Section title="Props" id="meter-props">
                    <ApiTable kind=ApiKind::Props of="atoms::meter::Meter">
                        <ApiRow name="value" ty="NumberSignal<T>">
                            "The value: a number or any signal of one, clamped to the range. The number type "
                            <Code inline=true>"T"</Code>" is inferred from it. Required."
                        </ApiRow>
                        <ApiRow name="min_value" ty="Option<Signal<T>>" default="0">"The start of the range."</ApiRow>
                        <ApiRow name="max_value" ty="Option<Signal<T>>" default="100">"The end of the range."</ApiRow>
                        <ApiRow name="format_options" ty="Option<Signal<NumberFormatOptions>>" default="percent">
                            "How the value text is formatted: a percent style formats the percentage, other styles the value."
                        </ApiRow>
                        <ApiRow name="value_label" ty="MaybeProp<String>" default="None">"Replaces the value text (e.g. \u{201c}3 of 4 GB\u{201d})."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The element\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the meter when it has no "<Code inline=true>"Label"</Code>"."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Ids of elements naming the meter."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">"Ids of elements describing the meter."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the meter element."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">"The label, the parts and your track."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="MeterFill">
                <p>
                    "A "<Code inline=true>"<div>"</Code>" as wide as the value, in percent of its container: put it into the "
                    "track you draw. Its width is an inline style; everything else is up to your classes."
                </p>

                <Section title="Props" id="meter-fill-props">
                    <ApiTable kind=ApiKind::Props of="MeterFill">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and further styles of the fill."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="MeterValueText">
                <p>"A "<Code inline=true>"<span>"</Code>" with the formatted value, the same text screen readers announce."</p>

                <Section title="Props" id="meter-value-text-props">
                    <ApiTable kind=ApiKind::Props of="MeterValueText">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the value text."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Styling">
                <p>
                    "The meter atoms add no classes and no data attributes: a meter has no states beyond its value. Give the "
                    "track a size and a background, and the fill a height and a color:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .meter { display: grid; grid-template-columns: 1fr auto; gap: 0.25em; }
                        .meter-track { grid-column: 1 / -1; height: 0.5em; overflow: hidden; border-radius: 4px; background: var(--border); }
                        .meter-fill { height: 100%; background: var(--accent); }
                    ")}
                </Code>
                <p>
                    "To color the fill by the value (e.g. red above 90%), compute the level from the value you pass and set it "
                    "as an attribute or class of your own, as the "<Link href=routes::doc::meter::Hook.materialize()>"use_meter"</Link>
                    " demo does."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Meter.materialize()>"Meter overview"</Link></li>
                <li><Link href=routes::doc::meter::Hook.materialize()>"use_meter"</Link></li>
                <li><Link href=routes::doc::meter::Component.materialize()>"Meter Component"</Link></li>
                <li><Link href=routes::doc::progress_bar::Atom.materialize()>"Progress Bar Atoms"</Link></li>
                <li><Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
