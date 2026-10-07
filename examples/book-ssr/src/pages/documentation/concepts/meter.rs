use leptos::prelude::*;

use super::demos::meter_storage::MeterStorageDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageMeterOverview() -> impl IntoView {
    view! {
        <DocPage title="Meter">
            <p>
                "A meter shows a value within a known range, such as disk usage, a battery level or the strength of a "
                "password. Unlike a progress bar, it doesn\u{2019}t show a task advancing toward completion: its value "
                "can go up and down and stays meaningful at any point of the range."
            </p>
            <p>
                "Screen readers announce a meter with its name and formatted value, e.g. \u{201c}Storage, 75%\u{201d}. "
                "The value text is formatted for the user\u{2019}s locale: as a percentage by default, or with any "
                "number format, or replaced by your own text such as \u{201c}3 of 4 GB\u{201d}."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Show a value within a known range: disk usage, battery level, a score"</TableCell>
                        <TableCell><b>"Meter"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show how far a task has advanced, or that it is running"</TableCell>
                        <TableCell><Link href=routes::doc::ProgressBar.materialize()>"Progress Bar"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Let the user change the value"</TableCell>
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
                        <TableCell><Link href=routes::doc::meter::Hook.materialize()>"use_meter"</Link></TableCell>
                        <TableCell>
                            "The meter role, the value attributes, the label and the formatted value text, for "
                            "elements you render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::meter::Atom.materialize()>"Meter Atoms"</Link></TableCell>
                        <TableCell>
                            "An unstyled "<Code inline=true>"Meter"</Code>" with the parts "<Code inline=true>"MeterFill"</Code>
                            " and "<Code inline=true>"MeterValueText"</Code>", labelled by a "<Code inline=true>"Label"</Code>
                            "; you draw the track."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>"The component is the quickest way to a meter: give it a value and a label, optionally a range and your own value text:"</p>
                <Demo
                    description="A storage meter with its own value text"
                    source=include_str!("demos/meter_storage.rs")
                    source_open=true
                >
                    <MeterStorageDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "A meter has the "<Code inline=true>"meter"</Code>" role with "<Code inline=true>"aria-valuenow"</Code>
                    ", "<Code inline=true>"aria-valuemin"</Code>", "<Code inline=true>"aria-valuemax"</Code>" and "
                    <Code inline=true>"aria-valuetext"</Code>" (the formatted value). It needs a name: a visible label, or "
                    <Code inline=true>"aria_label"</Code>" when there is none."
                </p>
                <p>"A meter is not interactive: it takes no focus and has no keyboard interaction."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::meter::Hook.materialize()>"use_meter"</Link></li>
                <li><Link href=routes::doc::meter::Atom.materialize()>"Meter Atoms"</Link></li>
                <li><Link href=routes::doc::ProgressBar.materialize()>"Progress Bar"</Link></li>
                <li><Link href=routes::doc::Status.materialize()>"Status"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
