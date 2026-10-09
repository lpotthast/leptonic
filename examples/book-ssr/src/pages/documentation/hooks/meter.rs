use indoc::indoc;
use leptos::prelude::*;

use super::demos::meter_disk::MeterDiskDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseMeter() -> impl IntoView {
    view! {
        <DocPage title="use_meter">
            <p>
                "The "<Code inline=true>"use_meter"</Code>" hook gives an element the "<Code inline=true>"meter"</Code>
                " role and the value attributes screen readers announce, and computes the percentage you size a fill with. "
                "See the "<Link href=routes::doc::Meter.materialize()>"Meter overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useMeter"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseMeterInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    ", so you only name the fields you care about."
                </p>

                <ApiTable kind=ApiKind::Input of="UseMeterInput">
                    <ApiRow name="value" ty="Signal<T>" default="0">"The value, clamped to the range. Its number type "<Code inline=true>"T"</Code>" (e.g. "<Code inline=true>"f64"</Code>" or "<Code inline=true>"u64"</Code>") is the meter\u{2019}s."</ApiRow>
                    <ApiRow name="min_value" ty="Signal<T>" default="0">"The start of the range."</ApiRow>
                    <ApiRow name="max_value" ty="Signal<T>" default="100">"The end of the range."</ApiRow>
                    <ApiRow name="format_options" ty="Signal<NumberFormatOptions>" default="percent">
                        "How the value text is formatted: a percent style formats the percentage, other styles the value."
                    </ApiRow>
                    <ApiRow name="value_label" ty="MaybeProp<String>" default="None">"Replaces the formatted value text (e.g. \u{201c}1 of 4\u{201d})."</ApiRow>
                    <ApiRow name="id" ty="Option<String>" default="None">"The element\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                    <ApiRow name="has_label" ty="Signal<bool>" default="false">"Whether a visible label is rendered (with "<Code inline=true>"label_props"</Code>")."</ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names it when there is no visible label."</ApiRow>
                    <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Ids of further elements naming it."</ApiRow>
                    <ApiRow name="aria_describedby" ty="Option<String>" default="None">"Ids of elements describing it."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <p>
                    <Code inline=true>"use_meter"</Code>" returns the "<Code inline=true>"UseProgressBarReturn"</Code>" of "
                    <Link href=routes::doc::progress_bar::Hook.materialize()>"use_progress_bar"</Link>", with "
                    <Code inline=true>"role=\"meter\""</Code>" instead of "<Code inline=true>"progressbar"</Code>"."
                </p>
                <ApiTable kind=ApiKind::Return of="UseProgressBarReturn">
                    <ApiRow name="props" ty="UseProgressBarProps">
                        "Spread on the meter element: "<Code inline=true>"role"</Code>", "<Code inline=true>"aria-valuenow"</Code>", "<Code inline=true>"aria-valuemin"</Code>", "<Code inline=true>"aria-valuemax"</Code>", "
                        <Code inline=true>"aria-valuetext"</Code>", id and labelling."
                    </ApiRow>
                    <ApiRow name="label_props" ty="UseLabelProps">"Spread on the visible label, a "<Code inline=true>"<span>"</Code>"."</ApiRow>
                    <ApiRow name="percentage" ty="Signal<Option<Fraction>>">
                        "The value as a share of the range, a "<Code inline=true>"Fraction"</Code>" from 0 to 1 ("
                        <Code inline=true>"as_percent()"</Code>": 0 to 100), to size a fill with. Always "<Code inline=true>"Some"</Code>
                        " for a meter: the type is shared with "<Code inline=true>"use_progress_bar"</Code>", whose value can be unknown."
                    </ApiRow>
                    <ApiRow name="value_text" ty="Signal<Option<String>>">
                        "The formatted value (or "<Code inline=true>"value_label"</Code>"), also the "<Code inline=true>"aria-valuetext"</Code>
                        ". Always "<Code inline=true>"Some"</Code>" for a meter."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            computed_pct,
                            computed_size,
                            hooks::{
                                meter::{UseMeterInput, use_meter},
                                progress::UseProgressBarReturn,
                            },
                            leptos_styles::{Styles, property::WidthProperty},
                        };

                        let UseProgressBarReturn { props, label_props, percentage, value_text } = use_meter(UseMeterInput {
                            value: Signal::stored(72.5),
                            has_label: true.into(),
                            ..UseMeterInput::default()
                        });
                        let fill = Styles::new().add_reactive(move || {
                            WidthProperty.declare(computed_size(computed_pct(percentage.get().unwrap_or_default().as_percent())))
                        });

                        view! {
                            <span {..label_props.into_attrs()}>"Disk Usage"</span>
                            <span>{value_text}</span>
                            <div {..props.into_attrs()} class="track">
                                <div class="fill" style=fill></div>
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "The label and the value text come from the hook; the demo sizes the fill from "
                    <Code inline=true>"percentage"</Code>" and colors it by how full the disk is:"
                </p>

                <Demo description="Disk usage meter with buttons to change the value" source=include_str!("demos/meter_disk.rs")>
                    <MeterDiskDemo/>
                </Demo>
            </Section>

            <Section title="Value Text">
                <p>
                    <Code inline=true>"format_options"</Code>" decides how "<Code inline=true>"value_text"</Code>
                    " (and with it "<Code inline=true>"aria-valuetext"</Code>") reads: the default percent style shows the "
                    "percentage of the range (\u{201c}45%\u{201d}), any other style the value itself, e.g. with a unit. "
                    "For text no number format can express, set "<Code inline=true>"value_label"</Code>"."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        UseMeterInput {
                            value: Signal::stored(3),
                            max_value: Signal::stored(4),
                            value_label: "3 of 4 GB".into(),
                            ..UseMeterInput::default()
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="ARIA Attributes">
                <p>"The meter element gets:"</p>

                <ul>
                    <li><Code inline=true>"role=\"meter\""</Code></li>
                    <li><Code inline=true>"aria-valuenow"</Code>" \u{2014} the current value"</li>
                    <li><Code inline=true>"aria-valuemin"</Code>" / "<Code inline=true>"aria-valuemax"</Code>" \u{2014} the range"</li>
                    <li><Code inline=true>"aria-valuetext"</Code>" \u{2014} "<Code inline=true>"value_text"</Code></li>
                    <li><Code inline=true>"aria-labelledby"</Code>" \u{2014} the label, with "<Code inline=true>"has_label"</Code></li>
                </ul>

                <p>
                    "Without a visible label, name the meter with "<Code inline=true>"aria_label"</Code>" or "
                    <Code inline=true>"aria_labelledby"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Meter.materialize()>"Meter overview"</Link></li>
                <li><Link href=routes::doc::meter::Atom.materialize()>"Meter Atoms"</Link></li>
                <li><Link href=routes::doc::progress_bar::Hook.materialize()>"use_progress_bar"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
