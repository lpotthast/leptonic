use leptos::prelude::*;

use super::demos::time_field::TimeFieldConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageTimeFieldOverview() -> impl IntoView {
    view! {
        <DocPage title="Time Field">
            <p>
                "A time field lets users type a time of day, such as an alarm or the start of a meeting. Like a "
                <Link href=routes::doc::DateField.materialize()>"date field"</Link>", it splits the value into segments "
                "(hour, minute, optionally second, and AM/PM where the locale uses a 12-hour clock) that users fill in by "
                "typing digits or step with the arrow keys."
            </p>
            <p>
                "The value is a "<Code inline=true>"jiff::civil::Time"</Code>", or the time of a "
                <Code inline=true>"civil::DateTime"</Code>" or "<Code inline=true>"Zoned"</Code>" whose date stays as it is."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Type a time of day"</TableCell>
                        <TableCell><b>"Time Field"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Type a date with a time"</TableCell>
                        <TableCell><Link href=routes::doc::DateField.materialize()>"Date Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Type a date, or pick it from a calendar"</TableCell>
                        <TableCell><Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link></TableCell>
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
                        <TableCell><Link href=routes::doc::time_field::Hook.materialize()>"Time Field Hooks"</Link></TableCell>
                        <TableCell>
                            "The state of a time field and the attributes of its label, group and hidden input; the segments "
                            "use "<Code inline=true>"use_date_segment"</Code>" of the "
                            <Link href=routes::doc::date_field::Hook.materialize()>"Date Field Hooks"</Link>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::time_field::Atom.materialize()>"Time Field Atom"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"TimeField"</Code>", composed like a date field from a "
                            <Code inline=true>"DateInput"</Code>" of "<Code inline=true>"DateSegment"</Code>
                            "s and the field atoms."
                        </TableCell>
                    </TableRow>
                </DocTable>
                <p>"There is no themed time field component yet."</p>
            </Section>

            <Section title="Quick Start">
                <p>"An alarm time of the atoms: click the field, or tab into it, and type the time."</p>

                <Demo description="Alarm time field of the atoms showing the entered time" source=include_str!("demos/time_field.rs") source_open=true>
                    <TimeFieldConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "A time field is a "<Code inline=true>"role=\"group\""</Code>" of spin buttons, as a date field: see the "
                    <Link href=format!("{}#accessibility", routes::doc::DateField.materialize())>"Date Field overview"</Link>
                    " for its semantics and keys. Once it has a value, it is described by it (\u{201c}Selected Time: "
                    "2:30 PM\u{201d}, in English for now). "<Keys keys="PageUp"/>" and "<Keys keys="PageDown"/>" step "
                    "the hour by 2 and the minutes and seconds by 15."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::time_field::Hook.materialize()>"Time Field Hooks"</Link></li>
                <li><Link href=routes::doc::time_field::Atom.materialize()>"Time Field Atom"</Link></li>
                <li><Link href=routes::doc::DateField.materialize()>"Date Field"</Link></li>
                <li><Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
