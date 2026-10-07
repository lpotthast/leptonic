use leptos::prelude::*;

use super::demos::calendar::CalendarConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageCalendarOverview() -> impl IntoView {
    view! {
        <DocPage title="Calendar">
            <p>
                "A calendar shows the days of a month as a grid and lets users pick a date, or a range of dates, by "
                "clicking a day or moving through the grid with the keyboard. It suits picking a date close to today, "
                "where seeing the weekdays and the neighboring dates helps, such as an appointment or a booking."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Pick a date or a range of dates from a visible month"</TableCell><TableCell><b>"Calendar"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Type a known date, such as a birthday"</TableCell>
                        <TableCell><Link href=routes::doc::DateField.materialize()>"Date Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Offer both, with the calendar in a popover"</TableCell>
                        <TableCell><Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Enter a time of day"</TableCell>
                        <TableCell><Link href=routes::doc::TimeField.materialize()>"Time Field"</Link></TableCell>
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
                        <TableCell><Link href=routes::doc::calendar::Hook.materialize()>"Calendar Hooks"</Link></TableCell>
                        <TableCell>
                            "The state of single-date and range calendars, the calendar\u{2019}s page buttons and "
                            "announcements, the grid of days with its keyboard handling, and each day\u{2019}s cell. You "
                            "render all elements."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::calendar::Atom.materialize()>"Calendar Atoms"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"Calendar"</Code>" and "<Code inline=true>"RangeCalendar"</Code>", composed from "
                            "unstyled atoms for the heading, the page buttons, the grid and the days. You style them through "
                            "their data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "A calendar of the atoms. Click a day, or tab into the grid, move with the arrow keys and press "
                    <Keys keys="Enter"/>"."
                </p>

                <Demo
                    description="Calendar of the atoms showing the selected date"
                    source=include_str!("demos/calendar.rs")
                    source_open=true
                >
                    <CalendarConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "A calendar is named by its label and the visible month (\u{201c}Appointment date, March 2026\u{201d}) "
                    "and holds a "<Code inline=true>"role=\"grid\""</Code>" of days per month. Each day is a cell with a "
                    "button labelled with the full date. The grid is a single tab stop, the focused date, which takes the "
                    "browser focus as the keys move it. Paging with the buttons announces the new month, and selecting "
                    "announces the selection. In a range calendar, the focused day says whether pressing it starts or "
                    "finishes the range."
                </p>

                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves the focus into the grid, to the focused date, and out again."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowRight">
                        "Previous / next day, across month boundaries; the other way round in right-to-left languages."
                    </KeyRow>
                    <KeyRow keys="ArrowUp / ArrowDown">"Same weekday in the previous / next week."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"Same day in the previous / next month."</KeyRow>
                    <KeyRow keys="Shift + PageUp / Shift + PageDown">"Same day in the previous / next year."</KeyRow>
                    <KeyRow keys="Home / End">"First / last day of the month."</KeyRow>
                    <KeyRow keys="Enter / Space">
                        "Selects the focused date. In a range calendar, the first press starts the range and the second finishes it."
                    </KeyRow>
                    <KeyRow keys="Escape">"Range calendars: cancels the range in progress."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::calendar::Hook.materialize()>"Calendar Hooks"</Link></li>
                <li><Link href=routes::doc::calendar::Atom.materialize()>"Calendar Atoms"</Link></li>
                <li><Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link></li>
                <li><Link href=routes::doc::DateField.materialize()>"Date Field"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
