use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, pages::documentation::hooks::demos::birthday_field::BirthdayFieldDemo, routes};

#[component]
pub fn PageDateTime() -> impl IntoView {
    view! {
        <DocPage title="Date & Time">
            <p>
                "Dates and times are entered in two ways: by typing into a field whose parts (day, month, year, hour, \u{2026}) "
                "are separate segments, or by picking a day in a calendar. A date picker combines both: a date field with a "
                "button that opens a calendar in a popover."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::DateTime.materialize()/>
            </Section>

            <Section title="Relationships">
                <ul>
                    <li>
                        <Link href=routes::doc::DateField.materialize()>"Date Field"</Link>" and "
                        <Link href=routes::doc::TimeField.materialize()>"Time Field"</Link>" are the typed way in: they "
                        "build a date (optionally with a time) or a time of day from editable segments, each segment a "
                        <Link href=format!("{}#use-date-segment", routes::doc::DateField.materialize())>
                            <Code inline=true>"use_date_segment"</Code>
                        </Link>"."
                    </li>
                    <li>
                        <Link href=routes::doc::Calendar.materialize()>"Calendar"</Link>" is the picked way in: a grid of "
                        "the days of a month, selecting one date or a range of dates."
                    </li>
                    <li>
                        <Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link>" combines both: its hooks "
                        "connect a date field, a button, a "<Link href=routes::doc::Popover.materialize()>"popover"</Link>
                        " and a calendar that share one date."
                    </li>
                    <li>
                        "Calendar and Date Picker also have a themed component each, "
                        <Link href=routes::doc::calendar::Component.materialize()>"Calendar Component"</Link>" and "
                        <Link href=routes::doc::date_picker::Component.materialize()>"Date Picker Component"</Link>
                        ". They predate the hooks and work with the mouse only; build accessible date inputs from the hooks."
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
                        <TableCell>"Pick a date relative to today, such as an appointment, typed or picked"</TableCell>
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
                    "A date field built from "<Link href=routes::doc::DateField.materialize()><Code inline=true>"use_date_field"</Code></Link>
                    ": click it, or tab to it, and type a birthday. The segments are rendered by a small Leptos component "
                    "shown on the Date Field page, as leptonic has no segment atom yet. To add a calendar in a popover, "
                    "start from the "<Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link>" Quick Start."
                </p>

                <Demo
                    description="Birthday field with editable date segments"
                    source=include_str!("../hooks/demos/birthday_field.rs")
                    source_open=true
                >
                    <BirthdayFieldDemo/>
                </Demo>
            </Section>
        </DocPage>
    }
}
