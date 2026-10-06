use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    kit::*, pages::documentation::hooks::demos::calendar_single::CalendarSingleDemo, routes,
};

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
                            "The state of single-date and range calendars, the grid of days with its keyboard handling, and "
                            "each day\u{2019}s cell. You render the month header, the navigation buttons and the grid."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::calendar::Component.materialize()>"Calendar Component"</Link></TableCell>
                        <TableCell>
                            "The themed "<Code inline=true>"DateSelector"</Code>", with month and year overviews. It works "
                            "with the mouse only: it has no keyboard support, and screen readers don\u{2019}t see a calendar grid."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "A single-date calendar built from the hooks. Click a day, or move with the arrow keys and press "
                    <Keys keys="Enter"/>". The demo moves the browser focus along with the focused date through a small "
                    "helper, shown on the "
                    <Link href=format!("{}#focus-management", routes::doc::calendar::Hook.materialize())>"Calendar Hooks"</Link>
                    " page, because the day cells don\u{2019}t move it themselves yet."
                </p>

                <Demo
                    description="Single-date calendar built from use_calendar_state, use_calendar_grid and use_calendar_cell"
                    source=include_str!("../hooks/demos/calendar_single.rs")
                    source_open=true
                >
                    <CalendarSingleDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "A calendar built from the hooks is a "<Code inline=true>"role=\"grid\""</Code>" of days, named by its "
                    "label. Each day is a cell holding a button, and the grid is a single tab stop: the focused date. When "
                    "the focus enters another month, the new month and year are announced to screen readers. The keys "
                    "move the focused date, but the day cells don\u{2019}t move the browser focus yet: your calendar "
                    "focuses the new day itself, as the Quick Start does."
                </p>

                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus into the grid, to the focused date, and out again."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowRight">"Previous / next day, across month boundaries."</KeyRow>
                    <KeyRow keys="ArrowUp / ArrowDown">"Same weekday in the previous / next week."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">
                        "Previous / next month. The focused date currently lands on the month\u{2019}s first day "
                        "instead of the same day."
                    </KeyRow>
                    <KeyRow keys="Shift + PageUp / Shift + PageDown">"Same day in the previous / next year."</KeyRow>
                    <KeyRow keys="Home / End">"First / last day of the month."</KeyRow>
                    <KeyRow keys="Enter / Space">
                        "Selects the focused date. In a range calendar, the first press sets the anchor and the second completes the range."
                    </KeyRow>
                    <KeyRow keys="Escape">"Range calendars: cancels the selection in progress."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::calendar::Hook.materialize()>"Calendar Hooks"</Link></li>
                <li><Link href=routes::doc::calendar::Component.materialize()>"Calendar Component"</Link></li>
                <li><Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link></li>
                <li><Link href=routes::doc::DateField.materialize()>"Date Field Hooks"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
