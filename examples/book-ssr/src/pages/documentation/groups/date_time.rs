use leptos::prelude::*;

use crate::{
    kit::*, pages::documentation::concepts::demos::date_field::DateFieldConceptDemo, routes,
};

#[component]
pub fn PageDateTime() -> impl IntoView {
    view! {
        <DocPage title="Date & Time">
            <p>
                "Dates and times are entered in two ways: by typing into a field whose parts (day, month, year, hour, \u{2026}) "
                "are separate segments, or by picking a day in a calendar. A date picker combines both: a date field with a "
                "button that opens a calendar in a popover. All of them work with the dates of "
                <Code inline=true>"jiff"</Code>", re-exported as "<Code inline=true>"leptonic::jiff"</Code>"."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::DateTime.materialize()/>
            </Section>

            <Section title="Relationships">
                <ul>
                    <li>
                        <Link href=routes::doc::DateField.materialize()>"Date Field"</Link>" and "
                        <Link href=routes::doc::TimeField.materialize()>"Time Field"</Link>" are the typed way in: they "
                        "edit a date (optionally with a time) or a time of day in segments. A time field is a date field of "
                        "hours and minutes: both are rendered with the same "
                        <Link href=format!("{}#dateinput", routes::doc::date_field::Atom.materialize())><Code inline=true>"DateInput"</Code></Link>
                        " and "<Link href=format!("{}#datesegment", routes::doc::date_field::Atom.materialize())><Code inline=true>"DateSegment"</Code></Link>
                        " atoms (or "<Link href=format!("{}#use-date-segment", routes::doc::date_field::Hook.materialize())><Code inline=true>"use_date_segment"</Code></Link>")."
                    </li>
                    <li>
                        <Link href=routes::doc::Calendar.materialize()>"Calendar"</Link>" is the picked way in: a grid of "
                        "the days of a month, selecting one date or a range of dates."
                    </li>
                    <li>
                        <Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link>" combines both: a date field "
                        "(or two, for a range), a button, a "<Link href=routes::doc::Popover.materialize()>"popover"</Link>
                        " and a calendar sharing one value."
                    </li>
                </ul>
            </Section>

            <Section title="Decision Guide">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Let the user type a known date, such as a birthday"</TableCell>
                        <TableCell><Link href=routes::doc::DateField.materialize()>"Date Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Enter a time of day"</TableCell>
                        <TableCell><Link href=routes::doc::TimeField.materialize()>"Time Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick a date or a range relative to today, such as an appointment, typed or picked"</TableCell>
                        <TableCell><Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a month and select a day or a range of days, always visible"</TableCell>
                        <TableCell><Link href=routes::doc::Calendar.materialize()>"Calendar"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "A date field of the "<Link href=routes::doc::date_field::Atom.materialize()>"Date Field Atoms"</Link>
                    ": click it, or tab into it, and type a birthday. To add a calendar in a popover, start from the "
                    <Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link>" Quick Start."
                </p>

                <Demo
                    description="Birthday field of the atoms showing the entered date"
                    source=include_str!("../concepts/demos/date_field.rs")
                    source_open=true
                >
                    <DateFieldConceptDemo/>
                </Demo>
            </Section>
        </DocPage>
    }
}
