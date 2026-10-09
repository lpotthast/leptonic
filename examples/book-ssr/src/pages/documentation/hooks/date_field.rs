use indoc::indoc;
use leptos::prelude::*;

use super::demos::date_field::DateFieldHookDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageDateFieldHooks() -> impl IntoView {
    view! {
        <DocPage title="Date Field Hooks">
            <p>
                "The date field hooks give a date, or a date and time, edited in segments: the field\u{2019}s state, the "
                "attributes of the field and its label, and each segment\u{2019}s behavior. They are in "
                <Code inline=true>"leptonic::hooks::datepicker"</Code>". See the "
                <Link href=routes::doc::DateField.materialize()>"Date Field overview"</Link>" for the concept and its keys."
            </p>

            <ReactAria hook="useDateField"/>

            <Section title="Demo">
                <p>
                    "A field of the hooks: "<Code inline=true>"use_date_field_state"</Code>" holds the value and the segments, "
                    <Code inline=true>"use_date_field"</Code>" wires the group, the label, the description and the error "
                    "message, and a small "<Code inline=true>"Segment"</Code>" Leptos component renders each segment with "
                    <Code inline=true>"use_date_segment"</Code>". Weekends fail its validation."
                </p>

                <Demo description="Delivery date field of the hooks with weekday validation and a disabled toggle" source=include_str!("demos/date_field.rs")>
                    <DateFieldHookDemo/>
                </Demo>
            </Section>

            <Section title="use_date_field_state">
                <p>
                    "The state of a date field, generic over its "<AnchorLink href="#datevalue">"value"</AnchorLink>": the "
                    "value, the segments in the locale\u{2019}s order, their editing and the validation against "
                    <Code inline=true>"min_value"</Code>", "<Code inline=true>"max_value"</Code>", unavailable dates and "
                    <Code inline=true>"validate"</Code>". While segments are empty, the value is "<Code inline=true>"None"</Code>
                    " and the field keeps what was typed; a complete date is set at once. An impossible date (February 30) "
                    "stays on screen until the field is left ("<Code inline=true>"confirm_placeholder"</Code>"), which "
                    "constrains it."
                </p>

                <Section title="Input" id="use-date-field-state-input">
                    <ApiTable kind=ApiKind::Input of="datepicker::use_date_field_state::UseDateFieldStateInput">
                        <ApiRow name="default_value" ty="Option<V>" default="None">"The initial value."</ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<Option<V>>>" default="None">
                            "The value as app state, replacing "<Code inline=true>"default_value"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<V>>>" default="None">
                            "Called with each new value: complete, or "<Code inline=true>"None"</Code>" once all segments are cleared."
                        </ApiRow>
                        <ApiRow name="placeholder_value" ty="Signal<Option<V>>" default="None">
                            "Where empty segments start when stepped, and the time of a value whose time has no segments. "
                            "Default: today, midnight."
                        </ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<Option<V>>" default="None">
                            "The earliest and latest valid value (\u{201c}Value must be 3/1/2026 or later.\u{201d})."
                        </ApiRow>
                        <ApiRow name="is_date_unavailable" ty="Option<Callback<V, bool>>" default="None">
                            "Whether a date can\u{2019}t be chosen (\u{201c}Selected date unavailable.\u{201d})."
                        </ApiRow>
                        <ApiRow name="granularity" ty="Signal<Option<Granularity>>" default="None">"The finest segment. Default: the minute for values with a time, else the day."</ApiRow>
                        <ApiRow name="max_granularity" ty="Signal<MaxGranularity>" default="Year">
                            "The coarsest segment, e.g. "<Code inline=true>"Month"</Code>" for a month and a day."
                        </ApiRow>
                        <ApiRow name="hour_cycle" ty="Signal<Option<HourCycle>>" default="None">"A 12- or 24-hour clock. Default: the locale\u{2019}s."</ApiRow>
                        <ApiRow name="hide_time_zone" ty="Signal<bool>" default="false">"Hides the time zone of zoned values."</ApiRow>
                        <ApiRow name="should_force_leading_zeros" ty="Signal<bool>" default="false">"Pads months, days and hours to two digits."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only, is_required, is_invalid" ty="Signal<bool>" default="false">
                            "Disables or locks the editing, marks the field required, or marks the value invalid."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<V>>>" default="None">
                            "Custom validation: "<Code inline=true>"Err(messages)"</Code>" for an invalid value."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Aria">
                            <Code inline=true>"Aria"</Code>" shows errors while you edit, "<Code inline=true>"Native"</Code>" when the form is submitted."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The field\u{2019}s name in forms (and for server errors)."</ApiRow>
                        <ApiRow name="validation" ty="Option<FormValidationState>" default="None">
                            "The validation of a "<Link href=routes::doc::date_picker::Hook.materialize()>"date picker"</Link>
                            " the field belongs to, used instead of its own."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-date-field-state-return">
                    <p>
                        "A "<Code inline=true>"DateFieldState<V>"</Code>", "<Code inline=true>"Copy"</Code>":"
                    </p>
                    <ApiTable kind=ApiKind::Return of="datepicker::use_date_field_state::DateFieldState">
                        <ApiRow name="value" ty="Signal<Option<V>>">"The value ("<Code inline=true>"None"</Code>" while empty or incomplete)."</ApiRow>
                        <ApiRow name="segments" ty="Signal<Vec<DateSegment>>">"The segments, in the locale\u{2019}s order."</ApiRow>
                        <ApiRow name="date_value" ty="Signal<V>">"The shown value completed by the placeholder."</ApiRow>
                        <ApiRow name="granularity" ty="Signal<Granularity>">"The finest segment."</ApiRow>
                        <ApiRow name="max_granularity" ty="Signal<MaxGranularity>">"The coarsest segment."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only, is_required" ty="Signal<bool>">"The input\u{2019}s flags."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the shown validation fails."</ApiRow>
                        <ApiRow name="validation" ty="FormValidationState">"The validation, e.g. "<Code inline=true>"validation_errors"</Code>" for the error message."</ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior">"When errors show."</ApiRow>
                    </ApiTable>

                    <p>"Its methods edit the field, a segment at a time (the segment hook calls them):"</p>
                    <DocTable headers=&["Method", "Does"]>
                        <TableRow>
                            <TableCell><Code inline=true>"set_value(value)"</Code></TableCell>
                            <TableCell>"Sets the value; "<Code inline=true>"None"</Code>" clears the field."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"increment(kind)"</Code>", "<Code inline=true>"decrement(kind)"</Code></TableCell>
                            <TableCell>"Steps a segment by one, wrapping around."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"increment_page(kind)"</Code>", "<Code inline=true>"decrement_page(kind)"</Code></TableCell>
                            <TableCell>"Steps by 5 years, 2 months, 7 days, 2 hours, or 15 minutes or seconds."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"increment_to_max(kind)"</Code>", "<Code inline=true>"decrement_to_min(kind)"</Code></TableCell>
                            <TableCell>"Sets a segment to its maximum or minimum."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_segment(kind, value)"</Code></TableCell>
                            <TableCell>"Sets a segment: a number, the era\u{2019}s index (0: BC, 1: AD), the day period (0: AM, 1: PM)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"clear_segment(kind)"</Code></TableCell>
                            <TableCell>"Empties a segment."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"confirm_placeholder()"</Code></TableCell>
                            <TableCell>"Sets a complete but impossible shown date as the value, constrained (when the field is left)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"format_value()"</Code></TableCell>
                            <TableCell>"The value for descriptions (\u{201c}June 15, 2024\u{201d}); empty without a value."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>
            </Section>

            <Section title="use_date_field">
                <p>
                    "The attributes of a field: a "<Code inline=true>"role=\"group\""</Code>" around the segments, labelled by "
                    "the label (a "<Code inline=true>"<span>"</Code>": pressing it focuses the first segment), described by "
                    "the value (\u{201c}Selected Date: \u{2026}\u{201d}), the description and the error message, with the arrow "
                    "keys between segments. Leaving the field confirms the shown date and commits the validation. A hidden "
                    "input carries the value (ISO 8601) in forms and resets the field with its form."
                </p>
                <Section title="Input" id="use-date-field-input">
                    <p>"Pass a "<Code inline=true>"UseDateFieldInput"</Code>" with every field named:"</p>
                    <ApiTable kind=ApiKind::Input of="datepicker::use_date_field::UseDateFieldInput">
                        <ApiRow name="state" ty="DateFieldState<V>">"From "<Code inline=true>"use_date_field_state"</Code>". Required."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The group of segments, captured by "<Code inline=true>"field_props"</Code>". Required."</ApiRow>
                        <ApiRow name="input_element" ty="CapturedElement">
                            "The hidden input (form reset and native validation), captured by "<Code inline=true>"input_props"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="options" ty="DateFieldOptions" default="DateFieldOptions::default()">"The other settings, below."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="DateFieldOptions">
                    <p><Code inline=true>"DateFieldOptions"</Code>" implements "<Code inline=true>"Default"</Code>"."</p>
                    <ApiTable kind=ApiKind::Fields of="DateFieldOptions">
                        <ApiRow name="id" ty="Option<String>" default="None">"The group\u{2019}s id. Generated when not given."</ApiRow>
                        <ApiRow name="has_label" ty="Signal<bool>" default="false">"Whether you render a label with "<Code inline=true>"label_props"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the field without a label."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">"Ids of further elements labelling or describing the field."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the first segment when rendered."</ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form the hidden input belongs to, when outside it."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the focus enters or leaves the field."</ApiRow>
                        <ApiRow name="on_key_down, on_key_up" ty="Option<Callback<KeyboardEvent>>" default="None">"Called with the keys in the field."</ApiRow>
                        <ApiRow name="picker" ty="Option<DateFieldPicker>" default="None">
                            "The date picker the field belongs to: its popover state ("<Keys keys="Alt + ArrowDown"/>" opens it) and, "
                            "for a range picker, its focus manager. The field then gets no group role, as the picker\u{2019}s group "
                            "is labelled and described."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-date-field-return">
                    <ApiTable kind=ApiKind::Return of="datepicker::use_date_field::UseDateFieldReturn">
                        <ApiRow name="label_props" ty="UseDateFieldLabelProps">"For the label element. Spread with "<Code inline=true>".into_attrs()"</Code>"."</ApiRow>
                        <ApiRow name="field_props" ty="PropsWithStyles<UseDateFieldProps>">
                            "For the group around the segments. Spread "<Code inline=true>".into_parts()"</Code>": attributes and styles."
                        </ApiRow>
                        <ApiRow name="input_props" ty="UseDateFieldInputProps">"For the hidden "<Code inline=true>"<input>"</Code>"."</ApiRow>
                        <ApiRow name="description_props, error_message_props" ty="SlotProps">
                            "For the description and the error message: referenced while they are rendered."
                        </ApiRow>
                        <ApiRow name="data" ty="DateFieldData<V>">"What each segment needs, for "<AnchorLink href="#use-date-segment">"use_date_segment"</AnchorLink>"."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="DateFieldData">
                    <p>
                        <Code inline=true>"Copy"</Code>". Pass this data to every segment. Inside a picker, supply the picker's "
                        <Code inline=true>"labelledby"</Code>" and "<Code inline=true>"field_describedby"</Code>" through "
                        <Code inline=true>"DateFieldPicker { labelledby, describedby, .. }"</Code>" when creating the field."
                    </p>
                    <ApiTable kind=ApiKind::Fields of="datepicker::use_date_field::DateFieldData">
                        <ApiRow name="state" ty="DateFieldState<V>">"The field\u{2019}s state."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>">"The field\u{2019}s name, added to each segment\u{2019}s."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Signal<Option<String>>">"What labels the field."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Signal<Option<String>>">"What describes the first segment (and every segment while invalid)."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-date-field-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{
                                CapturedElement,
                                IntoAttrs,
                                hooks::datepicker::*,
                                jiff::civil::Date,
                            };
                            use leptos::prelude::*;

                            let state = use_date_field_state(UseDateFieldStateInput::<Date>::default());
                            let field = use_date_field(UseDateFieldInput {
                                state,
                                element: CapturedElement::new(),
                                input_element: CapturedElement::new(),
                                options: DateFieldOptions { has_label: true.into(), ..DateFieldOptions::default() },
                            });
                            let (field_attrs, field_styles) = field.field_props.into_parts();

                            view! {
                                <span {..field.label_props.into_attrs()}>"Birthday"</span>
                                <div {..field_attrs} style=field_styles>
                                    // A segment per `state.segments`, see `use_date_segment`.
                                </div>
                                <input {..field.input_props.into_attrs()}/>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_date_segment">
                <p>
                    "Makes a segment a spin button you type into: the arrow keys, "<Keys keys="PageUp"/>"/"<Keys keys="PageDown"/>
                    " and "<Keys keys="Home"/>"/"<Keys keys="End"/>" step it, digits fill it (moving on once no further digit "
                    "fits), letters choose the day period and the era, "<Keys keys="Backspace"/>" removes a digit. The element "
                    "is "<Code inline=true>"contenteditable"</Code>", so that phones show a numeric keyboard; the selection stays "
                    "collapsed. Literal segments need no hook: render them hidden from assistive technology."
                </p>
                <p>
                    "Render the segments keyed by position and kind, so that an element and its focus stay while its text "
                    "changes."
                </p>

                <Section title="Input" id="use-date-segment-input">
                    <p>"The input has no defaults: set every field."</p>
                    <ApiTable kind=ApiKind::Input of="datepicker::use_date_segment::UseDateSegmentInput">
                        <ApiRow name="segment" ty="Signal<DateSegment>">"The segment, from the state\u{2019}s "<Code inline=true>"segments"</Code>"; its kind must stay. Required."</ApiRow>
                        <ApiRow name="data" ty="DateFieldData<V>">
                            "The field\u{2019}s "<AnchorLink href="#datefielddata"><Code inline=true>"DateFieldData"</Code></AnchorLink>". Required."
                        </ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The segment element, captured by the props. Required."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-date-segment-return">
                    <ApiTable kind=ApiKind::Return of="datepicker::use_date_segment::UseDateSegmentReturn">
                        <ApiRow name="segment_props" ty="PropsWithStyles<UseDateSegmentProps>">
                            <Code inline=true>"role=\"spinbutton\""</Code>", "<Code inline=true>"aria-valuenow"</Code>"/"
                            <Code inline=true>"-valuetext"</Code>"/"<Code inline=true>"-valuemin"</Code>"/"<Code inline=true>"-valuemax"</Code>
                            ", the label (\u{201c}month, Birthday\u{201d}), "<Code inline=true>"aria-invalid"</Code>", "
                            <Code inline=true>"data-placeholder"</Code>", "<Code inline=true>"tabindex"</Code>", the editing "
                            "attributes and the handlers. Spread "<Code inline=true>".into_parts()"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-date-segment-example">
                    <p>"The demo\u{2019}s "<Code inline=true>"Segment"</Code>" Leptos component:"</p>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            #[component]
                            fn Segment(segment: Signal<DateSegment>, data: DateFieldData<Date>) -> impl IntoView {
                                let kind = segment.with_untracked(|segment| segment.kind);
                                let text = move || segment.with(|segment| segment.text.clone());
                                if kind == DateSegmentType::Literal {
                                    return view! { <span aria-hidden="true">{text}</span> }.into_any();
                                }
                                let segment = use_date_segment(UseDateSegmentInput {
                                    segment,
                                    data,
                                    element: CapturedElement::new(),
                                });
                                let (attrs, styles) = segment.segment_props.into_parts();
                                view! { <span {..attrs} style=styles data-type=kind.as_str()>{text}</span> }.into_any()
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_date_picker_group">
                <p>
                    "The keys and presses of the element holding a field\u{2019}s segments, which "<Code inline=true>"use_date_field"</Code>
                    " and the date picker hooks include: "<Keys keys="ArrowLeft"/>" and "<Keys keys="ArrowRight"/>" move between "
                    "the segments (by position in right-to-left languages), "<Keys keys="Alt + ArrowDown"/>" opens the "
                    "picker\u{2019}s popover, and pressing the element outside the segments focuses the segment before "
                    "the pointer. It returns "<Code inline=true>"PropsWithStyles<UseDatePickerGroupProps>"</Code>"."
                </p>
                <ApiTable kind=ApiKind::Input of="UseDatePickerGroupInput">
                    <ApiRow name="element" ty="CapturedElement">"The element holding the segments. Required."</ApiRow>
                    <ApiRow name="arrow_keys" ty="GroupArrowKeys">
                        <Code inline=true>"MoveBetweenSegments"</Code>", or "<Code inline=true>"Ignore"</Code>" for a field inside "
                        "a date picker, whose group moves across its fields. Required."
                    </ApiRow>
                    <ApiRow name="overlay" ty="Option<OverlayTriggerState>">
                        "The popover "<Keys keys="Alt + ArrowDown"/>" opens, inside a date picker. Required ("<Code inline=true>"None"</Code>" outside one)."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="use_hidden_date_input">
                <p>
                    "A visually hidden "<Code inline=true>"<input type=\"date\">"</Code>" (or "
                    <Code inline=true>"datetime-local"</Code>") beside a date field, so that browsers can autofill the "
                    "field, e.g. with a birthday. It shows the field\u{2019}s value and sets what the browser fills in. "
                    "It isn\u{2019}t submitted with a form (the field\u{2019}s own hidden input is) and can\u{2019}t be focused."
                </p>

                <Section title="Input" id="use-hidden-date-input-input">
                    <p>"The input has no defaults: set every field."</p>
                    <ApiTable kind=ApiKind::Input of="UseHiddenDateInputInput">
                        <ApiRow name="value" ty="Signal<Option<V>>">"The field\u{2019}s value: the state\u{2019}s "<Code inline=true>"value"</Code>". Required."</ApiRow>
                        <ApiRow name="base" ty="Signal<V>">
                            "The value an autofilled date is put on (its time zone, and its time for dates): the state\u{2019}s "
                            <Code inline=true>"date_value"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="granularity" ty="Signal<Granularity>">
                            "The field\u{2019}s granularity: a "<Code inline=true>"date"</Code>" input for days, else "
                            <Code inline=true>"datetime-local"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Callback<Option<V>>">"Sets the autofilled value, e.g. with the state\u{2019}s "<Code inline=true>"set_value"</Code>". Required."</ApiRow>
                        <ApiRow name="auto_complete" ty="Option<String>">
                            "What the browser may fill in, the "<Code inline=true>"autocomplete"</Code>" attribute (e.g. "
                            <Code inline=true>"\"bday\""</Code>"). Required."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>">"The field\u{2019}s name, which tells autofill what it is. Required."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Disables the input. Required."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-hidden-date-input-return">
                    <ApiTable kind=ApiKind::Return of="UseHiddenDateInputReturn">
                        <ApiRow name="container_props" ty="UseHiddenDateContainerProps">
                            "For a "<Code inline=true>"<div>"</Code>" around the input: "<Code inline=true>"aria-hidden"</Code>", "
                            "kept out of focus walks, and visually hidden "<Code inline=true>"styles"</Code>" (fixed at the "
                            "top left, so that the browser\u{2019}s focusing it doesn\u{2019}t scroll the page)."
                        </ApiRow>
                        <ApiRow name="input_props" ty="UseHiddenDateInputProps">
                            "For the "<Code inline=true>"<input>"</Code>": its type, "<Code inline=true>"tabindex=-1"</Code>", "
                            <Code inline=true>"autocomplete"</Code>", "<Code inline=true>"name"</Code>", "<Code inline=true>"step"</Code>
                            ", the value and the input handler."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-hidden-date-input-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let autofill = use_hidden_date_input(UseHiddenDateInputInput {
                                value: state.value,
                                base: state.date_value,
                                granularity: state.granularity,
                                set_value: Callback::new(move |value| state.set_value(value)),
                                auto_complete: Some("bday".to_owned()),
                                name: Some("birthday".to_owned()),
                                is_disabled: state.is_disabled,
                            });
                            let UseHiddenDateInputReturn { container_props, input_props } = autofill;
                            let styles = container_props.styles.clone();

                            view! {
                                <div {..container_props.into_attrs()} style=styles>
                                    <input {..input_props.into_attrs()}/>
                                </div>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="DateValue">
                <p>
                    "The trait of a field\u{2019}s value. Fields and pickers are generic over it and return the type they "
                    "were given:"
                </p>
                <DocTable headers=&["Type", "Edits"]>
                    <TableRow>
                        <TableCell><Code inline=true>"jiff::civil::Date"</Code></TableCell>
                        <TableCell>"A date (granularity day)."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"jiff::civil::DateTime"</Code></TableCell>
                        <TableCell>"A date and time of day."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"jiff::Zoned"</Code></TableCell>
                        <TableCell>
                            "A date and time in its time zone, shown with the zone. A time that occurs twice (when the clocks "
                            "go back) keeps its offset."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="TimeValue">
                <p>
                    "The trait of a "<Link href=routes::doc::TimeField.materialize()>"time field"</Link>"\u{2019}s value: a "
                    <Code inline=true>"civil::Time"</Code>" (edited as a "<Code inline=true>"civil::DateTime"</Code>" on today), "
                    "or a "<Code inline=true>"civil::DateTime"</Code>" or "<Code inline=true>"Zoned"</Code>" whose time is edited."
                </p>
            </Section>

            <Section title="Granularity, MaxGranularity and HourCycle">
                <ul>
                    <li>
                        <Code inline=true>"Granularity"</Code>": the finest segment, "<Code inline=true>"Day"</Code>", "
                        <Code inline=true>"Hour"</Code>", "<Code inline=true>"Minute"</Code>" or "<Code inline=true>"Second"</Code>
                        ". A finer granularity than the value type has (a time of a "<Code inline=true>"civil::Date"</Code>") is the day."
                    </li>
                    <li>
                        <Code inline=true>"MaxGranularity"</Code>": the coarsest segment, "<Code inline=true>"Year"</Code>" (the default) "
                        "to "<Code inline=true>"Second"</Code>"."
                    </li>
                    <li><Code inline=true>"HourCycle"</Code>": "<Code inline=true>"H12"</Code>" (with AM/PM) or "<Code inline=true>"H24"</Code>"."</li>
                </ul>
            </Section>

            <Section title="DateSegment">
                <ApiTable kind=ApiKind::Fields of="hooks::datepicker::types::DateSegment">
                    <ApiRow name="kind" ty="DateSegmentType">
                        <Code inline=true>"Era"</Code>", "<Code inline=true>"Year"</Code>", "<Code inline=true>"Month"</Code>", "
                        <Code inline=true>"Day"</Code>", "<Code inline=true>"Hour"</Code>", "<Code inline=true>"Minute"</Code>", "
                        <Code inline=true>"Second"</Code>", "<Code inline=true>"DayPeriod"</Code>", "<Code inline=true>"Literal"</Code>
                        " or "<Code inline=true>"TimeZoneName"</Code>". "<Code inline=true>"as_str()"</Code>" gives its "
                        <Code inline=true>"data-type"</Code>"."
                    </ApiRow>
                    <ApiRow name="text" ty="String">"What to show: the formatted value, the placeholder, or the literal text."</ApiRow>
                    <ApiRow name="value" ty="Option<i32>">"The number, the era\u{2019}s index or the day period (0: AM, 1: PM)."</ApiRow>
                    <ApiRow name="min_value, max_value" ty="Option<i32>">"The segment\u{2019}s range; the day\u{2019}s follows the month."</ApiRow>
                    <ApiRow name="is_placeholder" ty="bool">"Whether the segment is empty."</ApiRow>
                    <ApiRow name="placeholder" ty="String">"The placeholder in the locale (\u{201c}mm\u{201d}, \u{201c}\u{2013}\u{2013}\u{201d})."</ApiRow>
                    <ApiRow name="is_editable" ty="bool">"Whether the user edits it (not a literal or the time zone)."</ApiRow>
                </ApiTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::DateField.materialize()>"Date Field overview"</Link></li>
                <li><Link href=routes::doc::date_field::Atom.materialize()>"Date Field Atoms"</Link></li>
                <li><Link href=routes::doc::time_field::Hook.materialize()>"Time Field Hooks"</Link></li>
                <li><Link href=routes::doc::date_picker::Hook.materialize()>"Date Picker Hooks"</Link></li>
                <li><Link href=routes::doc::utilities::UseSpinButton.materialize()>"use_spin_button"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
