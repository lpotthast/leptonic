use indoc::indoc;
use leptos::prelude::*;

use super::demos::{date_field::DateFieldAtomDemo, date_field_zoned::DateFieldZonedDemo};
use crate::{kit::*, routes};

/// A link to a section of the Date Field Hooks page.
fn hooks_section(anchor: &str) -> String {
    format!("{}#{anchor}", routes::doc::date_field::Hook.materialize())
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomDateField() -> impl IntoView {
    view! {
        <DocPage title="Date Field Atoms">
            <p>
                "The unstyled "<Code inline=true>"DateField"</Code>" holds a date, or a date and time, edited in segments; "
                "you compose it from a "<Code inline=true>"DateInput"</Code>" rendering a "<Code inline=true>"DateSegment"</Code>
                " per segment, and the "<Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>" for its label, "
                "description and error message. See the "<Link href=routes::doc::DateField.materialize()>"Date Field overview"</Link>
                " for when to use a date field and its keyboard interaction."
            </p>

            <ReactAria hook="DateField"/>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Built on"]>
                    <TableRow>
                        <TableCell><Code inline=true>"DateField"</Code></TableCell>
                        <TableCell>
                            <Link href=hooks_section("use-date-field-state")>"use_date_field_state"</Link>" and "
                            <Link href=hooks_section("use-date-field")>"use_date_field"</Link>
                            ". It hands the state to the atoms inside through a context."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"DateInput"</Code></TableCell>
                        <TableCell>"The field\u{2019}s props (with "<Link href=hooks_section("use-date-picker-group")>"use_date_picker_group"</Link>"\u{2019}s arrow keys)."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"DateSegment"</Code></TableCell>
                        <TableCell>
                            <Link href=hooks_section("use-date-segment")>"use_date_segment"</Link>", "
                            <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>" and "
                            <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>"."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <p>
                    "The atoms are in "<Code inline=true>"leptonic::atoms::datepicker"</Code>". "<Code inline=true>"DateInput"</Code>
                    " renders its children per segment. The type of the value decides what the field edits; name it on the "
                    "field ("<Code inline=true>"DateField<Date>"</Code>") when no prop gives it."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::{datepicker::*, field::{FieldError, Label}},
                            jiff::civil::Date,
                        };
                        use leptos::prelude::*;

                        let birthday = RwSignal::new(None::<Date>);

                        view! {
                            <DateField<Date> value=birthday set_value=birthday name="birthday">
                                <Label>"Birthday"</Label>
                                <DateInput children=|segment| view! { <DateSegment segment/> }/>
                                <FieldError/>
                            </DateField<Date>>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Type a date, or step the segments with the arrow keys. Weekends fail the field\u{2019}s "
                    <Code inline=true>"validate"</Code>" function, and the "<Code inline=true>"FieldError"</Code>" shows why."
                </p>

                <Demo description="Delivery date field of the atoms with weekday validation and a disabled toggle" source=include_str!("demos/date_field.rs")>
                    <DateFieldAtomDemo/>
                </Demo>
            </Section>

            <Section title="Values and Segments">
                <p>
                    "The field is generic over its value, a "<Link href=hooks_section("datevalue")><Code inline=true>"DateValue"</Code></Link>
                    ", and returns the type it is given:"
                </p>
                <DocTable headers=&["Value", "Segments"]>
                    <TableRow>
                        <TableCell><Code inline=true>"civil::Date"</Code></TableCell>
                        <TableCell>"Month, day and year, in the locale\u{2019}s order (\u{201c}mm/dd/yyyy\u{201d} in English)"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"civil::DateTime"</Code></TableCell>
                        <TableCell>"The date, then hour and minute, with AM/PM on a 12-hour clock"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Zoned"</Code></TableCell>
                        <TableCell>"The date and time, then the time zone, which can\u{2019}t be edited"</TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    <Code inline=true>"granularity"</Code>" sets the finest segment ("<Code inline=true>"Granularity::Hour"</Code>
                    " to "<Code inline=true>"Second"</Code>"), "<Code inline=true>"hour_cycle"</Code>" forces a 12- or 24-hour "
                    "clock over the locale\u{2019}s, and "<Code inline=true>"hide_time_zone"</Code>" drops the zone. Empty "
                    "segments start from "<Code inline=true>"placeholder_value"</Code>" (today, midnight, by default) when "
                    "stepped. This field edits a "<Code inline=true>"Zoned"</Code>" in New York on a 24-hour clock:"
                </p>

                <Demo description="Zoned date and time field of the atoms with read-only and disabled toggles" source=include_str!("demos/date_field_zoned.rs")>
                    <DateFieldZonedDemo/>
                </Demo>
            </Section>

            <Section title="DateField">
                <p>
                    "A "<Code inline=true>"<div>"</Code>" around the field\u{2019}s parts, and the hidden "
                    <Code inline=true>"<input>"</Code>" carrying the value (ISO 8601) in forms. Put a "
                    <Code inline=true>"Label"</Code>", a "<Code inline=true>"DateInput"</Code>" and optionally a "
                    <Code inline=true>"Description"</Code>" and a "<Code inline=true>"FieldError"</Code>" inside."
                </p>

                <Section title="Props" id="datefield-props">
                    <ApiTable kind=ApiKind::Props of="atoms::datepicker::DateField">
                        <ApiRow name="default_value" ty="Option<V>" default="None">"The initial value (uncontrolled)."</ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Option<V>>>" default="None">"The value (controlled): a value or any signal."</ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Option<V>>>" default="None">
                            "Receives the new value: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                            ", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<V>>>" default="None">
                            "Called with each new value: once all segments are filled, or with "<Code inline=true>"None"</Code>" once all are cleared."
                        </ApiRow>
                        <ApiRow name="placeholder_value" ty="Option<V>" default="None">"Where empty segments start when stepped. Default: today, midnight."</ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<Option<V>>" default="None">
                            "The earliest and latest valid value. A value outside is invalid (\u{201c}Value must be 3/1/2026 or later.\u{201d})."
                        </ApiRow>
                        <ApiRow name="is_date_unavailable" ty="Option<Callback<V, bool>>" default="None">"Whether a date can\u{2019}t be chosen; it makes the value invalid."</ApiRow>
                        <ApiRow name="granularity" ty="Option<Granularity>" default="None">"The finest segment. Default: the minute for values with a time, else the day."</ApiRow>
                        <ApiRow name="hour_cycle" ty="Option<HourCycle>" default="None">
                            <Code inline=true>"H12"</Code>" or "<Code inline=true>"H24"</Code>". Default: the locale\u{2019}s."
                        </ApiRow>
                        <ApiRow name="hide_time_zone" ty="bool" default="false">"Hides the time zone of zoned values."</ApiRow>
                        <ApiRow name="should_force_leading_zeros" ty="bool" default="false">"Pads months, days and hours to two digits."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Nothing can be focused or edited."</ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">"The segments take the focus, but can\u{2019}t be edited."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"Marks the segments required; with native validation, an empty field blocks submitting its form."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the value invalid, overriding the other validation."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<V>>>" default="None">
                            "Custom validation: "<Code inline=true>"Err(messages)"</Code>" for an invalid value."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            <Code inline=true>"Aria"</Code>" shows errors while you edit, "<Code inline=true>"Native"</Code>
                            " when the form is submitted. Default: the surrounding "<Code inline=true>"Form"</Code>"\u{2019}s, else "
                            <Code inline=true>"Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The hidden input\u{2019}s name in forms."</ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form the hidden input belongs to, when outside it."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the first segment when the field is rendered."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The group\u{2019}s id. Generated when not given."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the field when it has no "<Code inline=true>"Label"</Code>"."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">"Ids of further elements labelling or describing the field."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the focus enters or leaves the field."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the field element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The label, the "<Code inline=true>"DateInput"</Code>" and the other parts. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="DateInput">
                <p>
                    "The "<Code inline=true>"<div>"</Code>" with "<Code inline=true>"role=\"group\""</Code>" holding the segments. "
                    "Its children render a segment from a "<Code inline=true>"Signal<DateSegment>"</Code>" (see "
                    <Link href=hooks_section("datesegment")><Code inline=true>"DateSegment"</Code></Link>"); the elements stay "
                    "while their text changes, so the focus stays on the segment being edited. Pressing the group outside "
                    "the segments focuses the last segment before the pointer."
                </p>

                <Section title="Props" id="dateinput-props">
                    <ApiTable kind=ApiKind::Props of="datepicker::DateInput">
                        <ApiRow name="children" ty="Fn(Signal<DateSegment>) -> impl IntoView">
                            "Renders a segment, e.g. as a "<Code inline=true>"DateSegment"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="part" ty="Option<RangePart>" default="None">
                            "Inside a "<Link href=format!("{}#daterangepicker", routes::doc::date_picker::Atom.materialize())>"DateRangePicker"</Link>
                            ": which end it edits."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="DateSegment">
                <p>
                    "A "<Code inline=true>"<span>"</Code>" per segment: an editable one is a "<Code inline=true>"spinbutton"</Code>
                    " you type into (it is "<Code inline=true>"contenteditable"</Code>", with a numeric keyboard on phones), "
                    "a literal between them (\u{201c}/\u{201d}, \u{201c}:\u{201d}) is hidden from assistive technology."
                </p>

                <Section title="Props" id="datesegment-props">
                    <ApiTable kind=ApiKind::Props of="atoms::datepicker::DateSegment">
                        <ApiRow name="segment" ty="Signal<DateSegment>">"The segment, from "<Code inline=true>"DateInput"</Code>". Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the segment."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>"On "<Code inline=true>"DateSegment"</Code>":"</p>
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-type" ty="string">
                        "The segment\u{2019}s kind: "<Code inline=true>"year"</Code>", "<Code inline=true>"month"</Code>", "
                        <Code inline=true>"day"</Code>", "<Code inline=true>"hour"</Code>", "<Code inline=true>"minute"</Code>", "
                        <Code inline=true>"second"</Code>", "<Code inline=true>"dayPeriod"</Code>", "<Code inline=true>"era"</Code>", "
                        <Code inline=true>"timeZoneName"</Code>" or "<Code inline=true>"literal"</Code>"."
                    </ApiRow>
                    <ApiRow name="data-placeholder" ty="true">"The segment is empty and shows its placeholder."</ApiRow>
                    <ApiRow name="data-focused, data-focus-visible" ty="true">"The segment has the focus; by keyboard."</ApiRow>
                    <ApiRow name="data-hovered" ty="true">"A pointer is over the segment."</ApiRow>
                    <ApiRow name="data-readonly, data-disabled, data-invalid" ty="true">"The field is read-only, disabled or invalid."</ApiRow>
                </ApiTable>
                <p>
                    "Literals carry only "<Code inline=true>"data-type=\"literal\""</Code>". "<Code inline=true>"DateInput"</Code>
                    " has "<Code inline=true>"data-disabled"</Code>", "<Code inline=true>"data-invalid"</Code>", "
                    <Code inline=true>"data-hovered"</Code>", "<Code inline=true>"data-focus-within"</Code>" (a segment has the "
                    "focus) and "<Code inline=true>"data-focus-visible"</Code>" (by keyboard); "
                    <Code inline=true>"DateField"</Code>" has "<Code inline=true>"data-disabled"</Code>", "
                    <Code inline=true>"data-readonly"</Code>", "<Code inline=true>"data-required"</Code>" and "
                    <Code inline=true>"data-invalid"</Code>"."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. "<Code inline=true>"DateField"</Code>" renders a "<Code inline=true>"<div>"</Code>" with the class "<Code inline=true>"leptonic-DateField"</Code>", "
                    <Code inline=true>"DateInput"</Code>" the group of segments, a "<Code inline=true>"<div>"</Code>" with "<Code inline=true>"leptonic-DateInput"</Code>", and "<Code inline=true>"DateSegment"</Code>" "
                    "a "<Code inline=true>"<span>"</Code>" with "<Code inline=true>"leptonic-DateSegment"</Code>", each followed by the "<Code inline=true>"classes"</Code>" you pass; the "
                    "label, description and error are the field atoms "<Code inline=true>"Label"</Code>", "<Code inline=true>"Description"</Code>" and "<Code inline=true>"FieldError"</Code>". "
                    "A segment being edited has the focus, so mark it with a background rather than an outline, and the group with "
                    <Code inline=true>"data-focus-within"</Code>". The demos above use this CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .demo-date-field { display: flex; flex-direction: column; align-items: flex-start; gap: 0.25rem; }
                        .demo-date-input { display: inline-flex; flex-wrap: wrap; align-items: center; padding: 0.25rem 0.5rem; border: 1px solid var(--border); border-radius: 8px; font-variant-numeric: tabular-nums; }
                        .demo-date-input[data-focus-within] { border-color: var(--focus); }
                        .demo-date-input[data-disabled] { opacity: 0.5; }
                        .demo-date-segment { padding: 0 0.1em; border-radius: 4px; outline: none; }
                        .demo-date-segment:is([data-placeholder], [data-type="literal"]) { color: var(--muted); }
                        .demo-date-segment[data-type="literal"] { white-space: pre; }
                        .demo-date-segment[data-focused] { background: var(--accent); color: var(--surface); }
                    "#)}
                </Code>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <Section title="Composition">
                <ul>
                    <li>
                        "A "<Link href=routes::doc::time_field::Atom.materialize()><Code inline=true>"TimeField"</Code></Link>
                        " is composed the same way; so is the field inside a "
                        <Link href=routes::doc::date_picker::Atom.materialize()><Code inline=true>"DatePicker"</Code></Link>"."
                    </li>
                    <li>
                        "Inside a "<Link href=routes::doc::form::Atom.materialize()><Code inline=true>"Form"</Code></Link>
                        ", the field takes the form\u{2019}s validation behavior, is reset with it, and shows server errors for its "
                        <Code inline=true>"name"</Code>"."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::DateField.materialize()>"Date Field overview"</Link></li>
                <li><Link href=routes::doc::date_field::Hook.materialize()>"Date Field Hooks"</Link></li>
                <li><Link href=routes::doc::time_field::Atom.materialize()>"Time Field Atom"</Link></li>
                <li><Link href=routes::doc::date_picker::Atom.materialize()>"Date Picker Atoms"</Link></li>
                <li><Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
