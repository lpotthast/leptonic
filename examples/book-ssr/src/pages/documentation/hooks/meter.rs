use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{meter_battery::MeterBatteryDemo, meter_disk::MeterDiskDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageUseMeter() -> impl IntoView {
    view! {
        <DocPage title="use_meter">
            <p>
                "A meter shows a scalar value within a known range, such as disk usage, battery level or a password\u{2019}s "
                "strength. The "<Code inline=true>"use_meter"</Code>" hook gives an element the "<Code inline=true>"meter"</Code>
                " role and the value attributes screen readers announce, and computes the percentage you size a fill with."
            </p>

            <ReactAria hook="useMeter"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseMeterInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    ", so you only name the fields you care about."
                </p>

                <ApiTable kind=ApiKind::Input of="UseMeterInput">
                    <ApiRow name="value" ty="Signal<f64>" default="0.0">"The current value."</ApiRow>
                    <ApiRow name="min_value" ty="f64" default="0.0">"The lower bound of the range."</ApiRow>
                    <ApiRow name="max_value" ty="f64" default="100.0">"The upper bound of the range."</ApiRow>
                    <ApiRow name="label" ty="Option<String>" default="None">
                        "Whether the meter has a visible label. When set, the meter gets "<Code inline=true>"aria-labelledby"</Code>
                        " pointing to "<Code inline=true>"label_props.id"</Code>"; the text itself is not used, so render it in your "
                        "label element."
                    </ApiRow>
                    <ApiRow name="show_value_label" ty="bool" default="true">
                        "Currently has no effect: "<Code inline=true>"value_label"</Code>" is always computed."
                    </ApiRow>
                    <ApiRow name="format_options" ty="Option<MeterFormatOptions>" default="None">
                        "How "<Code inline=true>"value_label"</Code>" is formatted, see "<a href="#format-options">"Format Options"</a>
                        ". "<Code inline=true>"None"</Code>" shows the rounded percentage."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseMeterReturn">
                    <ApiRow name="meter_props" ty="UseMeterProps">
                        "Attributes for the meter element, see "<a href="#aria-attributes">"ARIA Attributes"</a>
                        ". Spread with "<Code inline=true>"{..meter_props.into_attrs()}"</Code>"."
                    </ApiRow>
                    <ApiRow name="label_props" ty="UseMeterLabelProps">"The "<Code inline=true>"id"</Code>" for your label element."</ApiRow>
                    <ApiRow name="percentage" ty="Signal<f64>">
                        "The value as a percentage of the range, clamped to 0 to 100, for sizing a fill."
                    </ApiRow>
                    <ApiRow name="value_label" ty="Signal<String>">
                        "The formatted value, e.g. \u{201c}45%\u{201d}. Also used as "<Code inline=true>"aria-valuetext"</Code>"."
                    </ApiRow>
                    <ApiRow name="meter_id" ty="String">"The generated id of the meter."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let UseMeterReturn { meter_props, label_props, percentage, value_label, .. } = use_meter(
                            UseMeterInput {
                                value: Signal::derive(|| 72.5),
                                label: Some("Disk Usage".to_string()),
                                ..Default::default()
                            }
                        );

                        view! {
                            <label id=label_props.id>"Disk Usage"</label>
                            <span>{move || value_label.get()}</span>
                            <div {..meter_props.into_attrs()} class="track">
                                // Size the fill from `percentage`.
                                <div class="fill"></div>
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>"The fill color changes with the value. Both meters use the hook\u{2019}s "<Code inline=true>"value_label"</Code>" as text:"</p>

                <Demo description="Disk usage meter with buttons to change the value" source=include_str!("demos/meter_disk.rs")>
                    <MeterDiskDemo/>
                </Demo>

                <Demo description="Battery level meter with charge and discharge buttons" source=include_str!("demos/meter_battery.rs")>
                    <MeterBatteryDemo/>
                </Demo>
            </Section>

            <Section title="Meter vs Progress Bar">
                <DocTable headers=&["Use", "For"]>
                    <TableRow>
                        <TableCell><b>"Meter"</b></TableCell>
                        <TableCell>"A value within a range that is not a task: disk usage, battery level, ratings."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::progress::Hook.materialize()>"Progress bar"</Link></TableCell>
                        <TableCell>"The completion of a task over time: file uploads, loading."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Format Options">
                <p>
                    <Code inline=true>"MeterFormatOptions"</Code>" controls "<Code inline=true>"value_label"</Code>": "
                    <Code inline=true>"MeterFormatStyle::Percent"</Code>" shows the percentage of the range (\u{201c}72.5%\u{201d}), "
                    <Code inline=true>"MeterFormatStyle::Decimal"</Code>" shows the value itself (\u{201c}72.5\u{201d}). "
                    <Code inline=true>"decimals"</Code>" sets the number of decimal places."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        UseMeterInput {
                            format_options: Some(MeterFormatOptions {
                                style: MeterFormatStyle::Percent, // or Decimal
                                decimals: 1,
                            }),
                            ..Default::default()
                        }
                    ")}
                </Code>
            </Section>

            <Section title="ARIA Attributes">
                <p>"The meter element gets:"</p>

                <ul>
                    <li><Code inline=true>"role=\"meter\""</Code></li>
                    <li><Code inline=true>"aria-valuenow"</Code>" \u{2014} the current value"</li>
                    <li><Code inline=true>"aria-valuemin"</Code>" / "<Code inline=true>"aria-valuemax"</Code>" \u{2014} the range"</li>
                    <li><Code inline=true>"aria-valuetext"</Code>" \u{2014} "<Code inline=true>"value_label"</Code></li>
                    <li><Code inline=true>"aria-labelledby"</Code>" \u{2014} the label, if "<Code inline=true>"label"</Code>" is set"</li>
                </ul>

                <p>"Without a "<Code inline=true>"label"</Code>", give the element an "<Code inline=true>"aria-label"</Code>" yourself."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::DataDisplay.materialize()>"Data Display"</Link></li>
                <li><Link href=routes::doc::Progress.materialize()>"Progress"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
