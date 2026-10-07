use leptos::prelude::*;

use super::demos::date_field::DateFieldConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageDateFieldOverview() -> impl IntoView {
    view! {
        <DocPage title="Date Field">
            <p>
                "A date field lets users type a date, or a date and time, without parsing free text: it splits the value "
                "into segments (month, day, year, hour, \u{2026}) in the order and with the placeholders of the user\u{2019}s "
                "locale. Users fill a segment in by typing digits, or step it with the arrow keys. It suits dates people "
                "know by heart, such as a birthday, where a calendar would only get in the way."
            </p>
            <p>
                "The value is a "<Code inline=true>"jiff"</Code>" type (re-exported as "<Code inline=true>"leptonic::jiff"</Code>
                "), and its type decides what the field edits: a "<Code inline=true>"civil::Date"</Code>" (a date), a "
                <Code inline=true>"civil::DateTime"</Code>" (a date and time of day) or a "<Code inline=true>"Zoned"</Code>
                " (a date and time in a time zone, shown with the zone)."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Type a date users know, such as a birthday, or a date with a time"</TableCell>
                        <TableCell><b>"Date Field"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Type a time of day alone"</TableCell>
                        <TableCell><Link href=routes::doc::TimeField.materialize()>"Time Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Type a date, or pick it from a calendar in a popover"</TableCell>
                        <TableCell><Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick a date or a range from an always visible month"</TableCell>
                        <TableCell><Link href=routes::doc::Calendar.materialize()>"Calendar"</Link></TableCell>
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
                        <TableCell><Link href=routes::doc::date_field::Hook.materialize()>"Date Field Hooks"</Link></TableCell>
                        <TableCell>
                            "The field\u{2019}s state (value, segments, editing, validation), the attributes of the field "
                            "and its label, and each segment\u{2019}s spin button behavior. You render every element."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::date_field::Atom.materialize()>"Date Field Atoms"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"DateField"</Code>" with a "<Code inline=true>"DateInput"</Code>" of "
                            <Code inline=true>"DateSegment"</Code>"s and the field atoms (label, description, error). You "
                            "style them through their data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>"A birthday field of the atoms: click it, or tab into it, and type the date."</p>

                <Demo description="Birthday field of the atoms showing the entered date" source=include_str!("demos/date_field.rs") source_open=true>
                    <DateFieldConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The field is a "<Code inline=true>"role=\"group\""</Code>" named by its label. Each editable segment is a "
                    <Code inline=true>"spinbutton"</Code>" named by its unit and the label (\u{201c}month, Birthday\u{201d}), with "
                    "its value as text (\u{201c}6 \u{2013} June\u{201d}); the separators between them are hidden from assistive "
                    "technology. Once the field has a value, it is described by it (\u{201c}Selected Date: June 15, 2024\u{201d}). "
                    "Every editable segment is a tab stop. The segments\u{2019} names and the description are English for now."
                </p>
                <p>
                    "A hidden input carries the value (ISO 8601) in forms. While segments are empty, the field has no "
                    "value. An impossible date, such as February 30, stays on screen while you edit and becomes a valid "
                    "date when you leave the field."
                </p>

                <KeyboardTable>
                    <KeyRow keys="Tab / Shift + Tab">"Moves to the next or previous segment, and into or out of the field."</KeyRow>
                    <KeyRow keys="0\u{2013}9">
                        "Types into the focused segment. The focus moves on once no further digit fits (\u{201c}4\u{201d} in a "
                        "month, two digits in a day)."
                    </KeyRow>
                    <KeyRow keys="A / P">"Sets the AM/PM segment (the first letters of the locale\u{2019}s day periods)."</KeyRow>
                    <KeyRow keys="ArrowUp / ArrowDown">"Increases or decreases the segment by one, wrapping around (12 \u{2192} 1 for months)."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"Steps by a larger amount: 5 years, 2 months, 7 days, 2 hours, 15 minutes or seconds."</KeyRow>
                    <KeyRow keys="Home / End">"Sets the segment to its minimum or maximum."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowRight">
                        "Moves to the previous or next segment; by position on the screen in right-to-left languages."
                    </KeyRow>
                    <KeyRow keys="Backspace / Delete">
                        "Removes the last digit, or clears the segment. On an empty segment, moves to the previous one."
                    </KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::date_field::Hook.materialize()>"Date Field Hooks"</Link></li>
                <li><Link href=routes::doc::date_field::Atom.materialize()>"Date Field Atoms"</Link></li>
                <li><Link href=routes::doc::TimeField.materialize()>"Time Field"</Link></li>
                <li><Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
