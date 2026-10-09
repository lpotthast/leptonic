use indoc::indoc;
use leptos::prelude::*;

use super::demos::{date_picker::DatePickerAtomDemo, date_range_picker::DateRangePickerAtomDemo};
use crate::{kit::*, routes};

/// A link to a section of the Date Picker Hooks page.
fn hooks_section(anchor: &str) -> String {
    format!("{}#{anchor}", routes::doc::date_picker::Hook.materialize())
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomDatePicker() -> impl IntoView {
    view! {
        <DocPage title="Date Picker Atoms">
            <p>
                "The unstyled "<Code inline=true>"DatePicker"</Code>" and "<Code inline=true>"DateRangePicker"</Code>
                " combine a date field with a button opening a calendar in a popover. You compose them from a "
                <Code inline=true>"DatePickerGroup"</Code>" holding the "<Link href=routes::doc::date_field::Atom.materialize()>"date field\u{2019}s"</Link>
                " "<Code inline=true>"DateInput"</Code>" and a "<Code inline=true>"DatePickerButton"</Code>", the generic "
                <Link href=routes::doc::popover::Atom.materialize()>"Popover"</Link>" and "
                <Link href=routes::doc::dialog::Atom.materialize()>"Dialog"</Link>", and the "
                <Link href=routes::doc::calendar::Atom.materialize()>"Calendar Atoms"</Link>". See the "
                <Link href=routes::doc::DatePicker.materialize()>"Date Picker overview"</Link>" for when to use a date picker "
                "and its keyboard interaction."
            </p>

            <ReactAria hook="DatePicker"/>
            <ReactAria hook="DateRangePicker"/>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Built on"]>
                    <TableRow>
                        <TableCell><Code inline=true>"DatePicker"</Code></TableCell>
                        <TableCell>
                            <Link href=hooks_section("use-date-picker-state")>"use_date_picker_state"</Link>" and "
                            <Link href=hooks_section("use-date-picker")>"use_date_picker"</Link>", with "
                            <Link href=routes::doc::date_field::Hook.materialize()>"use_date_field_state and use_date_field"</Link>
                            " for its field. It hands everything to the parts inside through contexts."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"DateRangePicker"</Code></TableCell>
                        <TableCell>
                            <Link href=hooks_section("use-date-range-picker-state")>"use_date_range_picker_state"</Link>" and "
                            <Link href=hooks_section("use-date-range-picker")>"use_date_range_picker"</Link>", with a field per end."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"DatePickerGroup"</Code></TableCell>
                        <TableCell>"The picker\u{2019}s group props (with "<Code inline=true>"use_date_picker_group"</Code>"\u{2019}s keys)."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"DatePickerButton"</Code></TableCell>
                        <TableCell><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" with the picker\u{2019}s "<Code inline=true>"button"</Code>"."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <p>
                    "The atoms are in "<Code inline=true>"leptonic::atoms::datepicker"</Code>". The "<Code inline=true>"Popover"</Code>
                    " opens from the group, and the "<Code inline=true>"Calendar"</Code>" inside takes its value, limits and "
                    "selection from the picker: it needs no props."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::{calendar::*, datepicker::*, dialog::Dialog, field::Label, popover::Popover},
                            jiff::civil::Date,
                        };
                        use leptos::prelude::*;

                        let departure = RwSignal::new(None::<Date>);

                        view! {
                            <DatePicker<Date> value=departure set_value=departure>
                                <Label>"Departure"</Label>
                                <DatePickerGroup>
                                    <DateInput children=|segment| view! { <DateSegment segment/> }/>
                                    <DatePickerButton>"\u{25BC}"</DatePickerButton>
                                </DatePickerGroup>
                                <Popover>
                                    <Dialog>
                                        <Calendar>
                                            // The heading, the page buttons and a CalendarGrid.
                                        </Calendar>
                                    </Dialog>
                                </Popover>
                            </DatePicker<Date>>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Type a date, or open the calendar with the button (or "<Keys keys="Alt + ArrowDown"/>" in the field) "
                    "and pick a day: the popover closes and the field shows the date. Days outside March and April 2026 are "
                    "disabled, and weekends are unavailable; a typed weekend date is invalid."
                </p>

                <Demo description="Appointment date picker of the atoms with limits, unavailable weekends and a disabled toggle" source=include_str!("demos/date_picker.rs")>
                    <DatePickerAtomDemo/>
                </Demo>
            </Section>

            <Section title="DatePicker">
                <p>
                    "A "<Code inline=true>"<div>"</Code>" around the picker\u{2019}s parts, and the hidden "<Code inline=true>"<input>"</Code>
                    " carrying the value (ISO 8601) in forms. Put a "<Code inline=true>"Label"</Code>", a "
                    <Code inline=true>"DatePickerGroup"</Code>", a "<Code inline=true>"Popover"</Code>" with a "
                    <Code inline=true>"Dialog"</Code>" and a "<Code inline=true>"Calendar"</Code>", and optionally a "
                    <Code inline=true>"Description"</Code>" and a "<Code inline=true>"FieldError"</Code>" inside. Its value "
                    "is generic as a "<Link href=format!("{}#values-and-segments", routes::doc::date_field::Atom.materialize())>"date field\u{2019}s"</Link>
                    "; with a time, selecting a day keeps the value\u{2019}s time."
                </p>

                <Section title="Props" id="datepicker-props">
                    <ApiTable kind=ApiKind::Props of="atoms::datepicker::DatePicker">
                        <ApiRow name="default_value" ty="Option<V>" default="None">"The initial value (uncontrolled)."</ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Option<V>>>" default="None">"The value (controlled): a value or any signal."</ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Option<V>>>" default="None">"Receives the new value."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<V>>>" default="None">"Called with each new value."</ApiRow>
                        <ApiRow name="placeholder_value" ty="MaybeProp<V>" default="None">
                            "Where empty segments start when stepped, and the month the calendar opens on. Default: today, midnight."
                        </ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<Option<V>>" default="None">
                            "The earliest and latest valid value; the calendar disables the days outside."
                        </ApiRow>
                        <ApiRow name="is_date_unavailable" ty="Option<Callback<V, bool>>" default="None">
                            "Whether a date can\u{2019}t be chosen: the calendar marks it unavailable, and a typed one is invalid."
                        </ApiRow>
                        <ApiRow name="granularity" ty="MaybeProp<Granularity>" default="None">"The finest segment. Default: the minute for values with a time, else the day."</ApiRow>
                        <ApiRow name="hour_cycle" ty="MaybeProp<HourCycle>" default="None">
                            <Code inline=true>"H12"</Code>" or "<Code inline=true>"H24"</Code>". Default: the locale\u{2019}s."
                        </ApiRow>
                        <ApiRow name="hide_time_zone" ty="Signal<bool>" default="false">"Hides the time zone of zoned values."</ApiRow>
                        <ApiRow name="should_force_leading_zeros" ty="Signal<bool>" default="false">"Pads months, days and hours to two digits."</ApiRow>
                        <ApiRow name="should_close_on_select" ty="Signal<bool>" default="true">
                            "Whether selecting a date closes the popover. Without, a value with a time waits for a time selected "
                            "in the popover, or for the popover to close."
                        </ApiRow>
                        <ApiRow name="is_open" ty="Option<Signal<bool>>" default="None">"Whether the popover is open (controlled)."</ApiRow>
                        <ApiRow name="set_open" ty="Option<Out<bool>>" default="None">"Receives the open state."</ApiRow>
                        <ApiRow name="default_open" ty="bool" default="false">"Whether the popover starts open (uncontrolled)."</ApiRow>
                        <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">"Called when the popover opens or closes."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"The segments can\u{2019}t be focused or edited, and the button is disabled."</ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">"The segments take the focus, but can\u{2019}t be edited, and the button is disabled."</ApiRow>
                        <ApiRow name="is_required, is_invalid" ty="Signal<bool>" default="false">
                            "As on "<Link href=format!("{}#datefield-props", routes::doc::date_field::Atom.materialize())>"DateField"</Link>"."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<V>>>" default="None">"Custom validation."</ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "Default: the surrounding "<Code inline=true>"Form"</Code>"\u{2019}s, else "<Code inline=true>"Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="name, form" ty="Option<String>" default="None">"The hidden input\u{2019}s name and form."</ApiRow>
                        <ApiRow name="auto_complete" ty="Option<String>" default="None">
                            "What the browser may autofill ("<Code inline=true>"autocomplete"</Code>", e.g. "<Code inline=true>"\"bday\""</Code>"), through a visually hidden date input."
                        </ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the first segment when the picker is rendered."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The group\u{2019}s id. Generated when not given."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the picker when it has no "<Code inline=true>"Label"</Code>"."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">"Ids of further elements labelling or describing the picker."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">
                            "Called when the focus enters or leaves the picker; moving into the popover doesn\u{2019}t leave it."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the picker element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The label, the group, the popover and the other parts. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="DateRangePicker">
                <p>
                    "A picker for a range, a "<Link href=format!("{}#rangevalue", routes::doc::date_picker::Hook.materialize())><Code inline=true>"RangeValue<V>"</Code></Link>
                    " with a "<Code inline=true>"start"</Code>" and an "<Code inline=true>"end"</Code>". Its group holds a "
                    <Code inline=true>"DateInput"</Code>" per end ("<Code inline=true>"part=RangePart::Start"</Code>" and "
                    <Code inline=true>"RangePart::End"</Code>"), and its popover a "<Code inline=true>"RangeCalendar"</Code>
                    ". The value is set once both ends are there; an end before the start is invalid (\u{201c}Start date must "
                    "be before end date.\u{201d})."
                </p>

                <Demo description="Trip date range picker of the atoms with a range calendar and a disabled toggle" source=include_str!("demos/date_range_picker.rs")>
                    <DateRangePickerAtomDemo/>
                </Demo>

                <Section title="Props" id="daterangepicker-props">
                    <p>
                        "The props shared with "<AnchorLink href="#datepicker-props">"DatePicker"</AnchorLink>" work the same; "
                        "the value is a range."
                    </p>
                    <ApiTable kind=ApiKind::Props of="atoms::datepicker::DateRangePicker">
                        <ApiRow name="default_value" ty="Option<RangeValue<V>>" default="None">"The initial range (uncontrolled)."</ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Option<RangeValue<V>>>>" default="None">"The range (controlled)."</ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Option<RangeValue<V>>>>" default="None">"Receives the new range."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<RangeValue<V>>>>" default="None">"Called with each new range."</ApiRow>
                        <ApiRow name="placeholder_value" ty="MaybeProp<V>" default="None">"Where empty segments start, and the month the calendar opens on."</ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<Option<V>>" default="None">"The earliest and latest valid date of either end."</ApiRow>
                        <ApiRow name="is_date_unavailable" ty="Option<Callback<V, bool>>" default="None">"Whether a date can\u{2019}t be chosen."</ApiRow>
                        <ApiRow name="allows_non_contiguous_ranges" ty="bool" default="false">"Whether a range may span unavailable dates."</ApiRow>
                        <ApiRow name="granularity" ty="MaybeProp<Granularity>" default="None">"The finest segment. Default: the minute for values with a time, else the day."</ApiRow>
                        <ApiRow name="hour_cycle" ty="MaybeProp<HourCycle>" default="None">
                            <Code inline=true>"H12"</Code>" or "<Code inline=true>"H24"</Code>". Default: the locale\u{2019}s."
                        </ApiRow>
                        <ApiRow name="hide_time_zone" ty="Signal<bool>" default="false">"Hides the time zone of zoned values."</ApiRow>
                        <ApiRow name="should_force_leading_zeros" ty="Signal<bool>" default="false">"Pads months, days and hours to two digits."</ApiRow>
                        <ApiRow name="should_close_on_select" ty="Signal<bool>" default="true">"Whether selecting a range closes the popover."</ApiRow>
                        <ApiRow name="is_open" ty="Option<Signal<bool>>" default="None">"Whether the popover is open (controlled)."</ApiRow>
                        <ApiRow name="set_open" ty="Option<Out<bool>>" default="None">"Receives the open state."</ApiRow>
                        <ApiRow name="default_open" ty="bool" default="false">"Whether the popover starts open (uncontrolled)."</ApiRow>
                        <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">"Called when the popover opens or closes."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only, is_required, is_invalid" ty="Signal<bool>" default="false">"As on "<Code inline=true>"DatePicker"</Code>"."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<RangeValue<V>>>>" default="None">"Custom validation of the range."</ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">"As on "<Code inline=true>"DatePicker"</Code>"."</ApiRow>
                        <ApiRow name="start_name, end_name" ty="Option<String>" default="None">"The names of the hidden inputs of the ends."</ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form the hidden inputs belong to."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the start\u{2019}s first segment when rendered."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The group\u{2019}s id."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the picker when it has no "<Code inline=true>"Label"</Code>"."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">"Ids of further elements labelling or describing the picker."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the focus enters or leaves the picker."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the picker element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The label, the group, the popover and the other parts. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Time Fields in the Popover">
                <p>
                    "Read a surrounding picker's state with "<Code inline=true>"use_date_picker_state_context::<V>()"</Code>
                    " or "<Code inline=true>"use_date_range_picker_state_context::<V>()"</Code>" from "
                    <Code inline=true>"leptonic::atoms::datepicker"</Code>". Both return "<Code inline=true>"Option"</Code>
                    "; use the same value type as the picker. For a "<Code inline=true>"TimeField"</Code>" in a single "
                    "picker's popover, bind "<Code inline=true>"value=state.time_value"</Code>" and send present time values to "
                    <Code inline=true>"state.select_time(time)"</Code>". For a range, read "<Code inline=true>"state.start_time"</Code>
                    " or "<Code inline=true>"state.end_time"</Code>" and call "
                    <Code inline=true>"state.select_time(RangePart::Start, time)"</Code>" or its end counterpart."
                </p>
            </Section>

            <Section title="DatePickerGroup">
                <p>
                    "The "<Code inline=true>"<div>"</Code>" with "<Code inline=true>"role=\"group\""</Code>" around the field and "
                    "the button, named by the picker\u{2019}s label and described by its value (\u{201c}Selected Date: "
                    "March 12, 2026\u{201d}). The popover opens at it. "<Keys keys="Alt + ArrowDown"/>" in it opens the popover."
                </p>

                <Section title="Props" id="datepickergroup-props">
                    <ApiTable kind=ApiKind::Props of="DatePickerGroup">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group."</ApiRow>
                        <ApiRow name="children" ty="Children">"The "<Code inline=true>"DateInput"</Code>"s and the "<Code inline=true>"DatePickerButton"</Code>". Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="DatePickerButton">
                <p>
                    "The "<Code inline=true>"<button>"</Code>" opening the popover, named \u{201c}Calendar\u{201d} with the "
                    "picker\u{2019}s label, with "<Code inline=true>"aria-haspopup=\"dialog\""</Code>" and "
                    <Code inline=true>"aria-expanded"</Code>". It is disabled while the picker is disabled or read-only."
                </p>

                <Section title="Props" id="datepickerbutton-props">
                    <ApiTable kind=ApiKind::Props of="DatePickerButton">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the button."</ApiRow>
                        <ApiRow name="children" ty="Children">"The button\u{2019}s content, e.g. an icon. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-open" ty="true">
                        "The popover is open. On "<Code inline=true>"DatePicker"</Code>", "<Code inline=true>"DateRangePicker"</Code>
                        " and "<Code inline=true>"DatePickerGroup"</Code>"."
                    </ApiRow>
                    <ApiRow name="data-disabled, data-invalid" ty="true">"The picker is disabled or invalid. On the pickers and the group."</ApiRow>
                    <ApiRow name="data-hovered, data-focus-within, data-focus-visible" ty="true">
                        "On "<Code inline=true>"DatePickerGroup"</Code>": a pointer is over it; a segment or the button has the "
                        "focus; by keyboard."
                    </ApiRow>
                    <ApiRow name="data-readonly, data-required" ty="true">"The picker is read-only or required. On the pickers."</ApiRow>
                    <ApiRow name="data-pressed, data-hovered, data-focused, data-focus-visible" ty="true">"On "<Code inline=true>"DatePickerButton"</Code>"."</ApiRow>
                </ApiTable>
                <p>
                    "The button sets "<Code inline=true>"data-disabled"</Code>" too. The segments have the "
                    <Link href=format!("{}#data-attributes", routes::doc::date_field::Atom.materialize())>"data attributes of a date field"</Link>
                    "; the days those of the "<Link href=format!("{}#data-attributes", routes::doc::calendar::Atom.materialize())>"Calendar Atoms"</Link>"."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. "<Code inline=true>"DatePicker"</Code>" and "<Code inline=true>"DateRangePicker"</Code>" render a "<Code inline=true>"<div>"</Code>" with the class "
                    <Code inline=true>"leptonic-DatePicker"</Code>" or "<Code inline=true>"leptonic-DateRangePicker"</Code>", "<Code inline=true>"DatePickerGroup"</Code>" one with "
                    <Code inline=true>"leptonic-DatePickerGroup"</Code>" and "<Code inline=true>"DatePickerButton"</Code>" a "<Code inline=true>"<button>"</Code>" with "
                    <Code inline=true>"leptonic-DatePickerButton"</Code>", each followed by the "<Code inline=true>"classes"</Code>" you pass. The segments are those of a "
                    <Link href=format!("{}#styling", routes::doc::date_field::Atom.materialize())>"date field"</Link>", the days those of the "
                    <Link href=format!("{}#styling", routes::doc::calendar::Atom.materialize())>"Calendar Atoms"</Link>". The button\u{2019}s "
                    "arrow is your own markup ("<Code inline=true>"aria-hidden"</Code>": the button is named \u{201c}Calendar\u{201d}). Give the group the "
                    "field\u{2019}s border, and the popover its own surface; the "<Code inline=true>"Popover"</Code>" positions and layers itself. The demos "
                    "above use this CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .demo-date-input { display: inline-flex; align-items: center; padding: 0.25rem 0.5rem; border: 1px solid var(--border); border-radius: 8px; }
                        .demo-date-input[data-focus-within] { border-color: var(--focus); }
                        .demo-date-segments { display: inline-flex; flex-wrap: wrap; align-items: center; }
                        .demo-date-picker-button { margin-left: 0.5rem; padding: 0.25rem; border: 1px solid var(--border); border-radius: 4px; background: var(--surface); cursor: pointer; }
                        .demo-date-picker-button[data-hovered] { background: var(--border); }
                        [data-open] > .demo-date-picker-button { border-color: var(--accent); }
                        .demo-date-picker-button[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 2px; }
                        .demo-date-picker-popover { padding: 1rem; border: 1px solid var(--border); border-radius: 8px; background: var(--surface); }
                    ")}
                </Code>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <Section title="Composition">
                <ul>
                    <li>
                        "The "<Code inline=true>"Calendar"</Code>" (or "<Code inline=true>"RangeCalendar"</Code>") in the popover "
                        "takes the picker\u{2019}s date, limits, unavailable dates and state; its own value props are ignored there. "
                        "Compose it from the "<Link href=routes::doc::calendar::Atom.materialize()>"Calendar Atoms"</Link>
                        ", e.g. with two "<Code inline=true>"CalendarGrid"</Code>"s for two months."
                    </li>
                    <li>
                        "The "<Code inline=true>"Popover"</Code>" is modal: it keeps the focus inside while open, closes on "
                        <Keys keys="Escape"/>" and on clicks outside, and returns the focus to the field."
                    </li>
                    <li>
                        "A "<Code inline=true>"Description"</Code>" and a "<Code inline=true>"FieldError"</Code>" inside the picker "
                        "describe its group and segments."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::DatePicker.materialize()>"Date Picker overview"</Link></li>
                <li><Link href=routes::doc::date_picker::Hook.materialize()>"Date Picker Hooks"</Link></li>
                <li><Link href=routes::doc::date_field::Atom.materialize()>"Date Field Atoms"</Link></li>
                <li><Link href=routes::doc::calendar::Atom.materialize()>"Calendar Atoms"</Link></li>
                <li><Link href=routes::doc::popover::Atom.materialize()>"Popover Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
