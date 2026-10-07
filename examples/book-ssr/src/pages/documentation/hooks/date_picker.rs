use indoc::indoc;
use leptos::prelude::*;

use super::demos::date_picker::DatePickerHookDemo;
use crate::{kit::*, routes};

/// A link to a section of the Date Field Hooks page.
fn field_section(anchor: &str) -> String {
    format!("{}#{anchor}", routes::doc::date_field::Hook.materialize())
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageDatePickerHooks() -> impl IntoView {
    view! {
        <DocPage title="Date Picker Hooks">
            <p>
                "The date picker hooks combine a "<Link href=routes::doc::date_field::Hook.materialize()>"date field"</Link>
                " (or two, for a range) with a button opening a calendar in a popover: the picker\u{2019}s state and the "
                "attributes connecting its parts. They are in "<Code inline=true>"leptonic::hooks::datepicker"</Code>". See the "
                <Link href=routes::doc::DatePicker.materialize()>"Date Picker overview"</Link>" for the concept and its keys."
            </p>

            <ReactAria hook="useDatePicker"/>
            <ReactAria hook="useDateRangePicker"/>

            <Section title="Demo">
                <p>
                    "Type a date, or open the calendar with the button (or "<Keys keys="Alt + ArrowDown"/>" in the field) and "
                    "pick a day. The demo combines "<Code inline=true>"use_date_picker_state"</Code>", "
                    <Code inline=true>"use_date_picker"</Code>", a field of the "<Link href=routes::doc::date_field::Hook.materialize()>"Date Field Hooks"</Link>
                    ", the "<Link href=routes::doc::popover::Atom.materialize()>"Popover"</Link>" atom and the "
                    <Link href=routes::doc::calendar::Atom.materialize()>"Calendar Atoms"</Link>". The "
                    <Link href=routes::doc::date_picker::Atom.materialize()>"Date Picker Atoms"</Link>" do all of this for you."
                </p>

                <Demo description="Departure date picker of the hooks with a calendar popover and a disabled toggle" source=include_str!("demos/date_picker.rs")>
                    <DatePickerHookDemo/>
                </Demo>
            </Section>

            <Section title="Composition">
                <DocTable headers=&["Part", "From"]>
                    <TableRow>
                        <TableCell>"The label and the group around the field and the button"</TableCell>
                        <TableCell><Code inline=true>"use_date_picker"</Code>": "<Code inline=true>"label_props"</Code>", "<Code inline=true>"group_props"</Code></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"The field"</TableCell>
                        <TableCell>
                            <Link href=field_section("use-date-field-state")>"use_date_field_state"</Link>" with the picker\u{2019}s "
                            <Code inline=true>"value"</Code>" (a "<Code inline=true>"ValueBinding"</Code>" of "<Code inline=true>"state.value"</Code>
                            " and "<Code inline=true>"state.set_value"</Code>"), "<Code inline=true>"granularity"</Code>" and "
                            <Code inline=true>"validation"</Code>"; "<Link href=field_section("use-date-field")>"use_date_field"</Link>
                            " with "<Code inline=true>"picker"</Code>" (the picker\u{2019}s "<Code inline=true>"state.overlay"</Code>"); the segments\u{2019} "
                            <Code inline=true>"aria_labelledby"</Code>" from "<Code inline=true>"labelledby"</Code>" and "
                            <Code inline=true>"aria_describedby"</Code>" from "<Code inline=true>"field_describedby"</Code>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"The button"</TableCell>
                        <TableCell><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" with "<Code inline=true>"button"</Code></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"The popover and its dialog"</TableCell>
                        <TableCell>
                            "A popover bound to "<Code inline=true>"state.overlay.is_open"</Code>" and "<Code inline=true>"state.set_open"</Code>
                            ", positioned at the group; an element with "<Code inline=true>"role=\"dialog\""</Code>", the id "
                            "passed as "<Code inline=true>"dialog_id"</Code>", and "<Code inline=true>"dialog_labelledby"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"The calendar"</TableCell>
                        <TableCell>
                            "A "<Code inline=true>"Calendar"</Code>" with "<Code inline=true>"state.date_value"</Code>" as its value, "
                            "calling "<Code inline=true>"state.select_date"</Code>", focused when it opens"
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="use_date_picker_state">
                <p>
                    "The state of a date picker: a date field\u{2019}s value with a popover to pick it. Selecting a date in the "
                    "calendar keeps the value\u{2019}s time; with "<Code inline=true>"should_close_on_select"</Code>" off, a value "
                    "with a time waits for a time selected in the popover ("<Code inline=true>"select_time"</Code>"), or for the "
                    "popover to close. The value is validated as a date field\u{2019}s."
                </p>

                <Section title="Input" id="use-date-picker-state-input">
                    <ApiTable kind=ApiKind::Input of="datepicker::use_date_picker_state::UseDatePickerStateInput">
                        <ApiRow name="default_value" ty="Option<V>" default="None">"The initial value."</ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<Option<V>>>" default="None">"The value as app state."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<V>>>" default="None">"Called with each new value."</ApiRow>
                        <ApiRow name="placeholder_value" ty="Signal<Option<V>>" default="None">"The type, time and zone of a value picked without one. Default: today, midnight."</ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<Option<V>>" default="None">"The earliest and latest valid value."</ApiRow>
                        <ApiRow name="is_date_unavailable" ty="Option<Callback<V, bool>>" default="None">"Whether a date can\u{2019}t be chosen."</ApiRow>
                        <ApiRow name="granularity, hour_cycle, hide_time_zone, should_force_leading_zeros" ty="see use_date_field_state" default="None, None, false, false">
                            "As for "<Link href=field_section("use-date-field-state-input")>"use_date_field_state"</Link>"."
                        </ApiRow>
                        <ApiRow name="should_close_on_select" ty="Signal<bool>" default="true">"Whether selecting a date closes the popover."</ApiRow>
                        <ApiRow name="default_open" ty="bool" default="false">"Whether the popover starts open."</ApiRow>
                        <ApiRow name="is_open" ty="Option<ValueBinding<bool>>" default="None">"The open state as app state."</ApiRow>
                        <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">"Called when the popover opens or closes."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the value invalid."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<V>>>" default="None">"Custom validation."</ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Aria">"When errors show."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The name for server errors."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-date-picker-state-return">
                    <p>"A "<Code inline=true>"DatePickerState<V>"</Code>", "<Code inline=true>"Copy"</Code>":"</p>
                    <ApiTable kind=ApiKind::Return of="datepicker::use_date_picker_state::DatePickerState">
                        <ApiRow name="value" ty="Signal<Option<V>>">"The value."</ApiRow>
                        <ApiRow name="date_value" ty="Signal<Option<Date>>">"The calendar\u{2019}s date: the one selected in the popover, else the value\u{2019}s."</ApiRow>
                        <ApiRow name="time_value" ty="Signal<Option<Time>>">"The time: the one selected in the popover, else the value\u{2019}s."</ApiRow>
                        <ApiRow name="granularity" ty="Signal<Granularity>">"The finest segment, for the field."</ApiRow>
                        <ApiRow name="has_time" ty="Signal<bool>">"Whether the value has a time."</ApiRow>
                        <ApiRow name="overlay" ty="OverlayTriggerState">"The popover\u{2019}s open state ("<Code inline=true>"overlay.is_open"</Code>")."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the shown validation fails."</ApiRow>
                        <ApiRow name="validation" ty="FormValidationState">"The validation, shared with the field."</ApiRow>
                    </ApiTable>
                    <p>
                        "Methods: "<Code inline=true>"set_value(value)"</Code>", "<Code inline=true>"select_date(date)"</Code>
                        " (from the calendar), "<Code inline=true>"select_time(time)"</Code>", "<Code inline=true>"set_open(is_open)"</Code>
                        " (closing sets a selected date), "<Code inline=true>"date_to_value(date)"</Code>" (a calendar date on the "
                        "value\u{2019}s time and zone) and "<Code inline=true>"format_value()"</Code>" (\u{201c}June 15, 2024\u{201d})."
                    </p>
                </Section>
            </Section>

            <Section title="use_date_picker">
                <p>
                    "The attributes of a picker: the group of the field and the button is labelled by the label and described "
                    "by the value (\u{201c}Selected Date: June 15, 2024\u{201d}), "<Keys keys="Alt + ArrowDown"/>" in it opens "
                    "the popover, and the button (\u{201c}Calendar\u{201d}) opens it."
                </p>

                <Section title="Input" id="use-date-picker-input">
                    <p>"Pass a "<Code inline=true>"UseDatePickerInput"</Code>" with every field named:"</p>
                    <ApiTable kind=ApiKind::Input of="datepicker::use_date_picker::UseDatePickerInput">
                        <ApiRow name="state" ty="DatePickerState<V>">"From "<Code inline=true>"use_date_picker_state"</Code>". Required."</ApiRow>
                        <ApiRow name="group" ty="CapturedElement">
                            "The group of the field and the button, captured by "<Code inline=true>"group_props"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="options" ty="DatePickerOptions" default="DatePickerOptions::default()">"The other settings, below."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="DatePickerOptions">
                    <p>
                        <Code inline=true>"DatePickerOptions"</Code>" implements "<Code inline=true>"Default"</Code>". "
                        <Code inline=true>"use_date_range_picker"</Code>" takes them too."
                    </p>
                    <ApiTable kind=ApiKind::Fields of="DatePickerOptions">
                        <ApiRow name="id" ty="Option<String>" default="None">"The group\u{2019}s id. Generated when not given."</ApiRow>
                        <ApiRow name="has_label" ty="Signal<bool>" default="false">"Whether you render a label with "<Code inline=true>"label_props"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the picker without a label."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">"Ids of further elements labelling or describing the picker."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">
                            "Disable the button; "<Code inline=true>"is_disabled"</Code>" also sets "
                            <Code inline=true>"aria-disabled"</Code>" on the group."
                        </ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">
                            "Called when the focus enters or leaves the picker; moving into the popover doesn\u{2019}t leave it."
                        </ApiRow>
                        <ApiRow name="on_key_down, on_key_up" ty="Option<Callback<KeyboardEvent>>" default="None">"Called with the keys in the group while the popover is closed."</ApiRow>
                        <ApiRow name="dialog_id" ty="Signal<Option<String>>" default="None">
                            "The id of the dialog in the popover: moving the focus into it doesn\u{2019}t leave the picker."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-date-picker-return">
                    <ApiTable kind=ApiKind::Return of="datepicker::use_date_picker::UseDatePickerReturn">
                        <ApiRow name="label_props" ty="UseDateFieldLabelProps">"For the label; pressing it focuses the first segment."</ApiRow>
                        <ApiRow name="group_props" ty="PropsWithStyles<UseDateFieldProps>">"For the group of the field and the button ("<Code inline=true>"role=\"group\""</Code>")."</ApiRow>
                        <ApiRow name="field_describedby" ty="Signal<Option<String>>">"What describes the segments: the picker\u{2019}s value and description."</ApiRow>
                        <ApiRow name="button" ty="UseButtonInput">
                            "The button\u{2019}s input for "<Code inline=true>"use_button"</Code>": named \u{201c}Calendar\u{201d} with "
                            "the label, "<Code inline=true>"aria-haspopup=\"dialog\""</Code>", "<Code inline=true>"aria-expanded"</Code>
                            ", opening the popover."
                        </ApiRow>
                        <ApiRow name="dialog_labelledby" ty="Signal<Option<String>>">"What names the dialog: the button and the label."</ApiRow>
                        <ApiRow name="description_props, error_message_props" ty="SlotProps">"For the description and the error message."</ApiRow>
                        <ApiRow name="labelledby" ty="Signal<Option<String>>">"What labels the picker, for its fields."</ApiRow>
                        <ApiRow name="focus_manager" ty="FocusManager">"Moves through the segments of the picker\u{2019}s fields, without the button."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_date_range_picker_state">
                <p>
                    "The state of a date range picker: a "<AnchorLink href="#rangevalue"><Code inline=true>"RangeValue"</Code></AnchorLink>
                    " edited in a start and an end field, or picked in a range calendar. The range is set once both ends are "
                    "there ("<Code inline=true>"start"</Code>" and "<Code inline=true>"end"</Code>" show an incomplete one); an end "
                    "before the start is invalid (\u{201c}Start date must be before end date.\u{201d})."
                </p>

                <Section title="Input" id="use-date-range-picker-state-input">
                    <ApiTable kind=ApiKind::Input of="datepicker::use_date_range_picker_state::UseDateRangePickerStateInput">
                        <ApiRow name="default_value" ty="Option<RangeValue<V>>" default="None">"The initial range."</ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<Option<RangeValue<V>>>>" default="None">"The range as app state."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<RangeValue<V>>>>" default="None">"Called with each new range."</ApiRow>
                        <ApiRow name="placeholder_value" ty="Signal<Option<V>>" default="None">"The type, time and zone of dates picked without a value."</ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<Option<V>>" default="None">"The earliest and latest valid date of either end."</ApiRow>
                        <ApiRow name="is_date_unavailable" ty="Option<Callback<V, bool>>" default="None">"Whether a date can\u{2019}t be chosen."</ApiRow>
                        <ApiRow name="granularity, hour_cycle, hide_time_zone, should_force_leading_zeros" ty="see use_date_field_state" default="None, None, false, false">"For the fields."</ApiRow>
                        <ApiRow name="should_close_on_select" ty="Signal<bool>" default="true">"Whether selecting a range closes the popover."</ApiRow>
                        <ApiRow name="default_open, is_open, on_open_change" ty="see use_date_picker_state" default="false, None, None">"The popover\u{2019}s open state."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the range invalid."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<RangeValue<V>>>>" default="None">"Custom validation of the range."</ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Aria">"When errors show."</ApiRow>
                        <ApiRow name="start_name, end_name" ty="Option<String>" default="None">"The names of the ends in forms; server errors for the start\u{2019}s name apply to the range."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-date-range-picker-state-return">
                    <p>"A "<Code inline=true>"DateRangePickerState<V>"</Code>", "<Code inline=true>"Copy"</Code>":"</p>
                    <ApiTable kind=ApiKind::Return of="datepicker::use_date_range_picker_state::DateRangePickerState">
                        <ApiRow name="value" ty="Signal<Option<RangeValue<V>>>">"The complete range ("<Code inline=true>"None"</Code>" while an end is missing)."</ApiRow>
                        <ApiRow name="start, end" ty="Signal<Option<V>>">"The ends shown, also of an incomplete range: the fields\u{2019} values."</ApiRow>
                        <ApiRow name="date_range" ty="Signal<Option<DateRange>>">"The range calendar\u{2019}s range."</ApiRow>
                        <ApiRow name="granularity" ty="Signal<Granularity>">"The finest segment, for the fields."</ApiRow>
                        <ApiRow name="has_time" ty="Signal<bool>">"Whether the values have a time."</ApiRow>
                        <ApiRow name="overlay" ty="OverlayTriggerState">"The popover\u{2019}s open state."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the shown validation fails."</ApiRow>
                        <ApiRow name="validation" ty="FormValidationState">"The validation, shared with the fields."</ApiRow>
                    </ApiTable>
                    <p>
                        "Methods: "<Code inline=true>"set_value(start, end)"</Code>", "<Code inline=true>"set_date_time(part, value)"</Code>
                        " (from a field), "<Code inline=true>"select_range(range)"</Code>" (from the calendar), "
                        <Code inline=true>"select_time(part, time)"</Code>", "<Code inline=true>"set_open(is_open)"</Code>", "
                        <Code inline=true>"date_to_value(date)"</Code>" and "<Code inline=true>"format_value()"</Code>" (the start and the end)."
                    </p>
                </Section>
            </Section>

            <Section title="use_date_range_picker">
                <p>
                    "The attributes of a range picker: as "<AnchorLink href="#use-date-picker">"use_date_picker"</AnchorLink>
                    ", described by the range (\u{201c}Selected Range: June 1, 2024 to June 15, 2024\u{201d}). It takes a "
                    <Code inline=true>"UseDateRangePickerInput"</Code>" (the same fields, with a "<Code inline=true>"DateRangePickerState"</Code>
                    ") and returns a "<Code inline=true>"UseDatePickerReturn"</Code>
                    ". Give each end a field: "<Code inline=true>"use_date_field_state"</Code>" with a binding of "
                    <Code inline=true>"state.start"</Code>" (or "<Code inline=true>"end"</Code>") and "
                    <Code inline=true>"state.set_date_time"</Code>", and "<Code inline=true>"use_date_field"</Code>" named "
                    "\u{201c}Start Date\u{201d} (\u{201c}End Date\u{201d}), labelled by "<Code inline=true>"labelledby"</Code>
                    ", with the picker\u{2019}s "<Code inline=true>"focus_manager"</Code>", so that the arrow keys move across both."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let field = |part: RangePart, value: Signal<Option<Date>>, name: &str| {
                            let field_state = use_date_field_state(UseDateFieldStateInput {
                                value: Some(ValueBinding::new(
                                    value,
                                    Callback::new(move |value| state.set_date_time(part, value)),
                                )),
                                granularity: Signal::derive(move || Some(state.granularity.get())),
                                validation: Some(state.validation),
                                ..UseDateFieldStateInput::default()
                            });
                            let mut field = use_date_field(UseDateFieldInput {
                                state: field_state,
                                element: CapturedElement::new(),
                                input_element: CapturedElement::new(),
                                options: DateFieldOptions {
                                    aria_label: name.to_owned().into(),
                                    aria_labelledby: picker.labelledby.get_untracked(),
                                    picker: Some(DateFieldPicker {
                                        overlay: state.overlay,
                                        focus_manager: Some(picker.focus_manager.clone()),
                                    }),
                                    ..DateFieldOptions::default()
                                },
                            });
                            // The segments are described by the picker: its description and its value.
                            field.data.aria_describedby = picker.field_describedby;
                            field
                        };
                        let start = field(RangePart::Start, state.start, "Start Date");
                        let end = field(RangePart::End, state.end, "End Date");
                    "#)}
                </Code>
            </Section>

            <Section title="RangeValue">
                <p>
                    "A range of values, "<Code inline=true>"RangeValue<V> { start: V, end: V }"</Code>"; "
                    <Code inline=true>"RangePart"</Code>" names an end ("<Code inline=true>"Start"</Code>" or "
                    <Code inline=true>"End"</Code>")."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::DatePicker.materialize()>"Date Picker overview"</Link></li>
                <li><Link href=routes::doc::date_picker::Atom.materialize()>"Date Picker Atoms"</Link></li>
                <li><Link href=routes::doc::date_field::Hook.materialize()>"Date Field Hooks"</Link></li>
                <li><Link href=routes::doc::calendar::Hook.materialize()>"Calendar Hooks"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
