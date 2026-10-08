use leptos::prelude::*;

use super::demos::date_picker::DatePickerConceptDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageDatePickerOverview() -> impl IntoView {
    view! {
        <DocPage title="Date Picker">
            <p>
                "A date picker combines a "<Link href=routes::doc::DateField.materialize()>"date field"</Link>" with a "
                "calendar in a popover. Users who know the date type it segment by segment; users who think in weeks open "
                "the calendar and pick the day there. Both edit the same value, a "<Code inline=true>"jiff"</Code>" date, "
                "date and time, or zoned date and time. A date range picker does the same for a start and an end date."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Let users type or pick a date near today, such as an appointment or a departure"</TableCell>
                        <TableCell><b>"Date Picker"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Let users type a date they know, such as a birthday"</TableCell>
                        <TableCell><Link href=routes::doc::DateField.materialize()>"Date Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Enter a time of day"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::TimeField.materialize()>"Time Field"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a month and select a day or a range of days, always visible"</TableCell>
                        <TableCell><Link href=routes::doc::Calendar.materialize()>"Calendar"</Link></TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "See "<Link href=routes::doc::DateTime.materialize()>"Date & Time"</Link>" for how the date and time "
                    "concepts fit together."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link>
                    " for how the layers relate."
                </p>
                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::date_picker::Hook.materialize()>"Date Picker Hooks"</Link></TableCell>
                        <TableCell>
                            "The state of date and date range pickers and the attributes connecting the field, the button, "
                            "the popover and the calendar. You render every part, with the "
                            <Link href=routes::doc::date_field::Hook.materialize()>"Date Field Hooks"</Link>", a popover and a calendar."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::date_picker::Atom.materialize()>"Date Picker Atoms"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"DatePicker"</Code>" and "<Code inline=true>"DateRangePicker"</Code>" with a "
                            <Code inline=true>"DatePickerGroup"</Code>" and a "<Code inline=true>"DatePickerButton"</Code>", composed "
                            "with the date field, popover, dialog and calendar atoms. You style them through their data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "A departure date picker of the atoms: type the date into the segments, or open the calendar with the "
                    "button (or "<Keys keys="Alt + ArrowDown"/>" in the field) and pick a day. The CSS shown with the demo "
                    "styles the atoms through their classes and data attributes."
                </p>
                <Demo
                    description="Departure date picker with a calendar popover, showing the picked date"
                    source=include_str!("demos/date_picker.rs")
                    source_open=true
                >
                    <DatePickerConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "A date picker follows the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/examples/datepicker-dialog/" target=LinkTarget::Blank>
                        "Date Picker Dialog example"
                    </Link>":"
                </p>
                <ul>
                    <li>
                        "The field and the button are a "<Code inline=true>"role=\"group\""</Code>" named by the label and "
                        "described by the value (\u{201c}Selected Date: March 12, 2026\u{201d}). Each editable segment is a "
                        <Code inline=true>"spinbutton"</Code>" and a tab stop. A range picker\u{2019}s fields are named "
                        "\u{201c}Start Date\u{201d} and \u{201c}End Date\u{201d}."
                    </li>
                    <li>
                        "The button is named \u{201c}Calendar\u{201d} together with the label, and has "
                        <Code inline=true>"aria-haspopup=\"dialog\""</Code>" and "<Code inline=true>"aria-expanded"</Code>"."
                    </li>
                    <li>
                        "The popover is a modal dialog holding the calendar, a "<Code inline=true>"grid"</Code>" of days that "
                        "takes the focus when it opens. When it closes, the focus returns to the field."
                    </li>
                    <li>
                        "The segments follow the locale, and so do their names, the button\u{2019}s name, the descriptions "
                        "and the validation messages."
                    </li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="Tab / Shift + Tab">"Moves the focus through the segments, the button and the rest of the page."</KeyRow>
                    <KeyRow keys="0\u{2013}9">
                        "In the field: edit the segments, with the digits and the arrow keys of a "
                        <Link href=format!("{}#accessibility", routes::doc::DateField.materialize())>"date field"</Link>"."
                    </KeyRow>
                    <KeyRow keys="Alt + ArrowDown / Alt + ArrowUp">"In the field: open the calendar."</KeyRow>
                    <KeyRow keys="Enter / Space">"On the button: open the calendar. In the calendar: pick the focused day."</KeyRow>
                    <KeyRow keys="Arrow keys">"In the calendar: move by a day or a week."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"In the calendar: move to the previous or next month."</KeyRow>
                    <KeyRow keys="Shift + PageUp / Shift + PageDown">"In the calendar: move to the previous or next year."</KeyRow>
                    <KeyRow keys="Home / End">"In the calendar: move to the first or last day of the month."</KeyRow>
                    <KeyRow keys="Escape">"Closes the calendar without picking."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::date_picker::Hook.materialize()>"Date Picker Hooks"</Link></li>
                <li><Link href=routes::doc::date_picker::Atom.materialize()>"Date Picker Atoms"</Link></li>
                <li><Link href=routes::doc::DateField.materialize()>"Date Field"</Link></li>
                <li><Link href=routes::doc::Calendar.materialize()>"Calendar"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
