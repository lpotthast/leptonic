use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, pages::documentation::hooks::demos::date_picker::DatePickerDemo, routes};

#[component]
pub fn PageDateTimeOverview() -> impl IntoView {
    view! {
        <DocPage title="Date & Time">
            <p>
                "Dates and times are entered in two ways: by typing into a field whose parts (day, month, year, hour, ...) "
                "are separate segments, or by picking a day in a calendar. A date picker combines both: a date field with a "
                "button that opens a calendar in a popover."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Let the user type a known date, such as a birthday"</TableCell>
                        <TableCell><Link href=routes::doc::date_time::DateFieldHooks.materialize()>"Date field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Enter a time of day"</TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-time-field", routes::doc::date_time::DateFieldHooks.materialize())>
                                "Time field"
                            </Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick a date relative to today, such as an appointment, typed or picked"</TableCell>
                        <TableCell><Link href=routes::doc::date_time::DatePickerHooks.materialize()>"Date picker"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a month and select a day or a range of days, always visible"</TableCell>
                        <TableCell><Link href=routes::doc::date_time::CalendarHooks.materialize()>"Calendar"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Dates exist as hooks and as a styled component; there are no atoms yet. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::date_time::CalendarHooks.materialize()>"Calendar hooks"</Link></TableCell>
                        <TableCell>
                            "State, grid and cell hooks for single-date and range calendars you render yourself: "
                            <Code inline=true>"use_calendar_state"</Code>", "<Code inline=true>"use_range_calendar_state"</Code>", "
                            <Code inline=true>"use_range_calendar"</Code>", "<Code inline=true>"use_calendar_grid"</Code>", "
                            <Code inline=true>"use_calendar_cell"</Code>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::date_time::DateFieldHooks.materialize()>"Date field hooks"</Link></TableCell>
                        <TableCell>
                            "Segmented date and time fields: "<Code inline=true>"use_date_field_state"</Code>", "
                            <Code inline=true>"use_date_field"</Code>", "<Code inline=true>"use_date_segment"</Code>", "
                            <Code inline=true>"use_time_field"</Code>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::date_time::DatePickerHooks.materialize()>"Date picker hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_date_picker_state"</Code>" and "<Code inline=true>"use_date_picker"</Code>
                            ", which connect a date field, a button, a popover and a calendar."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::date_time::Component.materialize()>"Date & Time components"</Link></TableCell>
                        <TableCell>
                            "The themed "<Code inline=true>"DateSelector"</Code>" calendar. It predates the hooks and only uses "
                            <Code inline=true>"use_calendar_state"</Code>": it works with the mouse only and screen readers "
                            "don\u{2019}t see a calendar grid."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "A date picker built from the hooks: type the date into the segments, or open the calendar with the button "
                    "(or "<Keys keys="Alt + ArrowDown"/>"). The "
                    <Link href=routes::doc::date_time::DatePickerHooks.materialize()>"date picker hooks"</Link>
                    " page explains how its parts fit together."
                </p>

                <Demo
                    description="Date picker with a segmented field and a calendar popover"
                    source=include_str!("../hooks/demos/date_picker.rs")
                >
                    <DatePickerDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "Each segment of a date field is a "<Code inline=true>"spinbutton"</Code>" with its value as text; the "
                        "arrow keys step it, digits type it."
                    </li>
                    <li>
                        "A calendar is a "<Code inline=true>"grid"</Code>" of days. The arrow keys move between days, "
                        <Keys keys="PageUp"/>" and "<Keys keys="PageDown"/>" between months, "<Keys keys="Enter"/>" selects."
                    </li>
                    <li>
                        "The date picker\u{2019}s popover is a dialog. Its calendar asks for focus when it opens ("
                        <Code inline=true>"auto_focus"</Code>"); returning the focus to the button when it closes is up to "
                        "you, as the demo shows."
                    </li>
                    <li>
                        "Labels, announcements and the segment order are English only for now. See "
                        "Internationalization on the hook pages."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::date_time::CalendarHooks.materialize()>"Calendar hooks"</Link></li>
                <li><Link href=routes::doc::date_time::DateFieldHooks.materialize()>"Date field hooks"</Link></li>
                <li><Link href=routes::doc::date_time::DatePickerHooks.materialize()>"Date picker hooks"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
