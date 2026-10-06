use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, pages::documentation::hooks::demos::date_picker::DatePickerDemo, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageDatePickerOverview() -> impl IntoView {
    view! {
        <DocPage title="Date Picker">
            <p>
                "A date picker combines a date field with a calendar in a popover. Users who know the date type it segment by "
                "segment (year, month, day); users who think in weeks open the calendar and pick the day there. Both edit "
                "the same value, a "<Code inline=true>"time::OffsetDateTime"</Code>", which can also carry a time of day."
            </p>
            <p>
                "Leptonic builds accessible date pickers from hooks: a date picker hook connects a "
                <Link href=routes::doc::DateField.materialize()>"date field"</Link>", a button, a popover and a "
                <Link href=routes::doc::Calendar.materialize()>"calendar"</Link>", and you render each part. The themed "
                <Code inline=true>"DateTimeInput"</Code>" predates these hooks and is unfinished."
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
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>
                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::date_picker::Hook.materialize()>"Date Picker Hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_date_picker_state"</Code>" and "<Code inline=true>"use_date_picker"</Code>
                            ": the value, the open state, validation and the attributes connecting the field, the button, the "
                            "popover and the calendar. You combine them with "<Code inline=true>"use_date_field"</Code>", a "
                            "popover and the calendar hooks."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::date_picker::Component.materialize()>"Date Picker Component"</Link></TableCell>
                        <TableCell>
                            "The themed "<Code inline=true>"DateTimeInput"</Code>", which opens the "
                            <Link href=routes::doc::calendar::Component.materialize()><Code inline=true>"DateSelector"</Code></Link>
                            " calendar below it. It predates the hooks and is unfinished: it can\u{2019}t be typed into, "
                            "needs a value before it opens, and its calendar works with the mouse only."
                        </TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "There are no date picker atoms yet. Until there are, build accessible date pickers from the hooks."
                </p>
            </Section>

            <Section title="Quick Start">
                <p>
                    "A date picker built from the hooks: type the date into the segments, or open the calendar with the button "
                    "(or "<Keys keys="Alt + ArrowDown"/>" in the field) and pick a day. The "
                    <Link href=routes::doc::date_picker::Hook.materialize()>"Date Picker Hooks"</Link>
                    " page explains how the parts fit together. Until leptonic has date picker atoms, a picker built from "
                    "the hooks is a longer piece of code: open the source below to see all of it."
                </p>
                <Demo
                    description="Departure date picker with a segmented field and a calendar popover, showing the picked date"
                    source=include_str!("../hooks/demos/date_picker.rs")
                >
                    <DatePickerDemo/>
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
                        "The field and the button are a "<Code inline=true>"role=\"group\""</Code>" named by the label. Each "
                        "editable segment is a "<Code inline=true>"spinbutton"</Code>"; the field is one tab stop."
                    </li>
                    <li>
                        "The button is named \u{201c}Calendar\u{201d} together with the label, and has "
                        <Code inline=true>"aria-haspopup=\"dialog\""</Code>" and "<Code inline=true>"aria-expanded"</Code>
                        ". It and the group are described by the selected date."
                    </li>
                    <li>
                        "The popover is a modal "<Code inline=true>"role=\"dialog\""</Code>" holding the calendar, a "
                        <Code inline=true>"grid"</Code>" of days that takes the focus when it opens. When it closes, the "
                        "focus returns to where it was; the "<Link href=routes::doc::popover::Atom.materialize()>"Popover"</Link>
                        " atom does this for you."
                    </li>
                    <li>
                        "The error message is a polite "<Code inline=true>"role=\"alert\""</Code>". Labels, announcements and "
                        "the segment order are English only for now."
                    </li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="Tab / Shift + Tab">"Moves focus between the field, the button and the rest of the page."</KeyRow>
                    <KeyRow keys="0\u{2013}9">"In the field: type into the focused segment."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowRight">"In the field: move to the previous or next segment."</KeyRow>
                    <KeyRow keys="ArrowUp / ArrowDown">"In the field: increase or decrease the focused segment."</KeyRow>
                    <KeyRow keys="Alt + ArrowDown / Alt + ArrowUp">"In the field: open the calendar."</KeyRow>
                    <KeyRow keys="Enter / Space">"On the button: open the calendar. In the calendar: pick the focused day."</KeyRow>
                    <KeyRow keys="Arrow keys">"In the calendar: move by a day or a week."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"In the calendar: move to the previous or next month."</KeyRow>
                    <KeyRow keys="Shift + PageUp / Shift + PageDown">"In the calendar: move to the previous or next year."</KeyRow>
                    <KeyRow keys="Home / End">"In the calendar: move to the first or last day of the month."</KeyRow>
                    <KeyRow keys="Escape">"Close the calendar without picking."</KeyRow>
                </KeyboardTable>
                <p>
                    "In the calendar, the keys move the focused date, but the day cells don\u{2019}t move the browser "
                    "focus yet: your calendar focuses the new day itself, as the Quick Start does (see "
                    <Link href=routes::doc::Calendar.materialize()>"Calendar"</Link>")."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::date_picker::Hook.materialize()>"Date Picker Hooks"</Link></li>
                <li><Link href=routes::doc::date_picker::Component.materialize()>"Date Picker Component"</Link></li>
                <li><Link href=routes::doc::DateField.materialize()>"Date Field Hooks"</Link></li>
                <li><Link href=routes::doc::Calendar.materialize()>"Calendar"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
