use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::date_field::DateFieldDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageDateFieldHooks() -> impl IntoView {
    view! {
        <DocPage title="Date Field Hooks">
            <p>
                "A date field lets users enter a date, or a date and time, without parsing free text: it splits the "
                "value into segments (year, month, day, hour, \u{2026}) that you fill in by typing digits or change with "
                "the arrow keys. For a time of day alone, use a "<Link href=routes::doc::TimeField.materialize()>"time field"</Link>
                "; to pick the date from a calendar as well, a "<Link href=routes::doc::DatePicker.materialize()>"date picker"</Link>
                ". See "<Link href=routes::doc::DateTime.materialize()>"Date & Time"</Link>" for how they relate."
            </p>

            <ReactAria hook="useDateField"/>

            <Section title="Demo">
                <p>
                    "Click the field (or tab to it) and type a date: the focus moves on to the next segment once a "
                    "segment is complete. The arrow keys change the focused segment. Weekend dates fail the field\u{2019}s "
                    "custom validation. The segments are rendered by the Leptos component shown in the "
                    <AnchorLink href="#use-date-segment-example">"use_date_segment example"</AnchorLink>"."
                </p>

                <Demo description="Date field with live value, weekday validation and a disabled toggle" source=include_str!("demos/date_field.rs")>
                    <DateFieldDemo/>
                </Demo>
            </Section>

            <Section title="How Segments Work">
                <p>
                    "A field doesn\u{2019}t use a text input. It renders one element per segment, and each editable segment "
                    "is a "<Code inline=true>"spinbutton"</Code>" that handles the keyboard itself. The field hook returns "
                    "the segments as a "<Code inline=true>"Signal<Vec<DateSegment>>"</Code>"; you render each one with "
                    <Code inline=true>"use_date_segment"</Code>"."
                </p>

                <DocTable headers=&["Field", "Segments"]>
                    <TableRow><TableCell><Code inline=true>"use_date_field"</Code></TableCell><TableCell><Code inline=true>"yyyy-mm-dd"</Code></TableCell></TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"use_date_field"</Code>" with "<Code inline=true>"show_time"</Code></TableCell>
                        <TableCell>
                            <Code inline=true>"yyyy-mm-dd --:--"</Code>", plus "<Code inline=true>"AM"</Code>"/"
                            <Code inline=true>"PM"</Code>" on a 12-hour clock"
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::TimeField.materialize()>"use_time_field"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"--:--"</Code>", "<Code inline=true>"--:--:--"</Code>" with "
                            <Code inline=true>"show_seconds"</Code>", plus "<Code inline=true>"AM"</Code>"/"
                            <Code inline=true>"PM"</Code>" on a 12-hour clock"
                        </TableCell>
                    </TableRow>
                </DocTable>

                <ul>
                    <li>
                        <b>"Editable and literal segments."</b>" Editable segments ("<Code inline=true>"is_editable"</Code>
                        ") hold a number. Literal segments ("<Code inline=true>"DateSegmentType::Literal"</Code>") are the "
                        "separators between them ("<Code inline=true>"-"</Code>", "<Code inline=true>":"</Code>", a space). "
                        "They can\u{2019}t be focused; hide them from screen readers with "<Code inline=true>"aria-hidden"</Code>"."
                    </li>
                    <li>
                        <b>"Placeholders."</b>" An empty segment has "<Code inline=true>"is_placeholder"</Code>" set and shows "
                        <Code inline=true>"yyyy"</Code>", "<Code inline=true>"mm"</Code>", "<Code inline=true>"dd"</Code>" or "
                        <Code inline=true>"--"</Code>". An empty AM/PM segment shows "<Code inline=true>"AM"</Code>"."
                    </li>
                    <li>
                        <b>"Incomplete values."</b>" While you edit, the field keeps every segment separately (in an "
                        <AnchorLink href="#incompletedate"><Code inline=true>"IncompleteDate"</Code></AnchorLink>"), so a value "
                        "can be partly filled in. "<Code inline=true>"on_change"</Code>" only fires once every segment is "
                        "filled (with the complete value) or every segment is cleared (with "<Code inline=true>"None"</Code>
                        "). A partly filled field keeps reporting its last complete value."
                    </li>
                    <li>
                        <b>"The placeholder date."</b>" Pressing an arrow key on an empty segment starts from a placeholder "
                        "date and steps once, so "<Keys keys="ArrowUp"/>" on an empty year shows next year. The field uses "
                        <Code inline=true>"default_value"</Code>", or the current UTC time when the field is created. It "
                        "also fills the parts it has no segments for (the time of a date-only field, the seconds) from the "
                        "placeholder."
                    </li>
                    <li>
                        <b>"Valid days."</b>" The day segment\u{2019}s maximum follows the month and year shown. Changing the "
                        "month clamps a day that no longer exists (31 becomes 30)."
                    </li>
                </ul>
            </Section>

            <Section title="use_date_field_state">
                <p>
                    "The state behind "<Code inline=true>"use_date_field"</Code>": the editing buffer, the segment list, the "
                    "segment callbacks and form validation. "<Code inline=true>"use_date_field"</Code>" creates it for you "
                    "and returns it as "<Code inline=true>"state"</Code>"; call it directly only to build a field with "
                    "different ARIA wiring."
                </p>

                <Section title="Input" id="use-date-field-state-input">
                    <p>"The input has no "<Code inline=true>"Default"</Code>": every field is required."</p>

                    <ApiTable kind=ApiKind::Input of="UseDateFieldStateInput">
                        <ApiRow name="value" ty="Signal<Option<OffsetDateTime>>">
                            "The current value. When it changes, the segments show it."
                        </ApiRow>
                        <ApiRow name="default_value" ty="Option<OffsetDateTime>">
                            "The placeholder date (see "<AnchorLink href="#how-segments-work">"How Segments Work"</AnchorLink>"). "
                            <Code inline=true>"None"</Code>" uses the current UTC time."
                        </ApiRow>
                        <ApiRow name="min, max" ty="Option<OffsetDateTime>">"Bounds the emitted value is clamped to."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<OffsetDateTime>>>">
                            "Called with the complete value, or "<Code inline=true>"None"</Code>" once every segment is cleared."
                        </ApiRow>
                        <ApiRow name="show_time" ty="bool">"Add hour and minute segments (and AM/PM on a 12-hour clock)."</ApiRow>
                        <ApiRow name="hour_cycle_24" ty="bool">"Use a 24-hour clock (hours 0\u{2013}23) instead of a 12-hour clock with AM/PM."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>">"Turn off all segment changes."</ApiRow>
                        <ApiRow name="is_required" ty="bool">"Unused by the state."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<OffsetDateTime>>>">
                            "Custom validation: return "<Code inline=true>"Err(messages)"</Code>" for an invalid value."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Marks the value invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation; "
                            <Code inline=true>"false"</Code>" leaves validation to the other sources."</ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior">
                            <Code inline=true>"Aria"</Code>" shows errors as you edit; "<Code inline=true>"Native"</Code>
                            " defers them until "<Code inline=true>"validation.commit_validation"</Code>" runs."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>">
                            "Matches server-side errors provided through "<Code inline=true>"FormValidationContext"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-date-field-state-return">
                    <ApiTable kind=ApiKind::Return of="UseDateFieldStateReturn">
                        <ApiRow name="value" ty="Signal<Option<OffsetDateTime>>">"The value passed in."</ApiRow>
                        <ApiRow name="segments" ty="Signal<Vec<DateSegment>>">"The segments to render, updated on every edit."</ApiRow>
                        <ApiRow name="validation" ty="UseFormValidationStateReturn">"The form validation state."</ApiRow>
                        <ApiRow name="set_segment" ty="Callback<(DateSegmentType, i32)>">"Sets a segment to a value (clamped to its range)."</ApiRow>
                        <ApiRow name="clear_segment" ty="Callback<DateSegmentType>">"Empties a segment."</ApiRow>
                        <ApiRow name="increment, decrement" ty="Callback<DateSegmentType>">"Step a segment by one, wrapping around (except the year)."</ApiRow>
                        <ApiRow name="increment_page, decrement_page" ty="Callback<DateSegmentType>">
                            "Step a segment by a larger amount: 5 years, 2 months, 7 days, 2 hours, 15 minutes or seconds."
                        </ApiRow>
                        <ApiRow name="increment_to_max, decrement_to_min" ty="Callback<DateSegmentType>">"Set a segment to its maximum or minimum."</ApiRow>
                        <ApiRow name="confirm_placeholder" ty="Callback<()>">
                            "Emits the value if it is complete (or "<Code inline=true>"None"</Code>" if it is cleared). Call it "
                            "when a segment loses focus."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_date_field">
                <p>
                    "Adds IDs and ARIA attributes for the field, its label, description and error message, and tracks "
                    "which segment is the field\u{2019}s tab stop. Pass the value in and update it in "
                    <Code inline=true>"on_change"</Code>"."
                </p>

                <Section title="Input" id="use-date-field-input">
                    <p>
                        <Code inline=true>"UseDateFieldInput"</Code>" implements "<Code inline=true>"Default"</Code>
                        ". It has the fields of "<Code inline=true>"use_date_field_state"</Code>"\u{2019}s input, plus "
                        "the label, description, error message and date picker settings."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseDateFieldInput">
                        <ApiRow name="value" ty="Signal<Option<OffsetDateTime>>" default="None">"The current value."</ApiRow>
                        <ApiRow name="default_value" ty="Option<OffsetDateTime>" default="None">
                            "The placeholder date (see "<AnchorLink href="#how-segments-work">"How Segments Work"</AnchorLink>"). "
                            <Code inline=true>"None"</Code>" uses the current UTC time."
                        </ApiRow>
                        <ApiRow name="min, max" ty="Option<OffsetDateTime>" default="None">"Bounds the emitted value is clamped to."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<OffsetDateTime>>>" default="None">
                            "Called with the complete value, or "<Code inline=true>"None"</Code>" once every segment is cleared."
                        </ApiRow>
                        <ApiRow name="show_time" ty="bool" default="false">
                            "Add hour and minute segments (and AM/PM on a 12-hour clock)."
                        </ApiRow>
                        <ApiRow name="hour_cycle_24" ty="bool" default="true">"Use a 24-hour clock."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Turn off all segment changes."</ApiRow>
                        <ApiRow name="is_required" ty="bool" default="false">"Sets "<Code inline=true>"aria-required"</Code>" on the field."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<OffsetDateTime>>>" default="None">
                            "Custom validation: return "<Code inline=true>"Err(messages)"</Code>" for an invalid value."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the value invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation; "
                            <Code inline=true>"false"</Code>" leaves validation to the other sources."</ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Aria">
                            <Code inline=true>"Aria"</Code>" shows errors as you edit; "<Code inline=true>"Native"</Code>
                            " defers them until "<Code inline=true>"state.validation.commit_validation"</Code>" runs."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "Matches server-side errors provided through "<Code inline=true>"FormValidationContext"</Code>"."
                        </ApiRow>
                        <ApiRow name="label" ty="Option<String>" default="None">
                            "Whether you render a label. "<Code inline=true>"Some"</Code>" points "
                            <Code inline=true>"aria-labelledby"</Code>" at "<Code inline=true>"label_props.id"</Code>
                            "; the text itself is yours to render."
                        </ApiRow>
                        <ApiRow name="description" ty="Option<String>" default="None">
                            "Whether you render a description. "<Code inline=true>"Some"</Code>" adds "
                            <Code inline=true>"description_props.id"</Code>" to "<Code inline=true>"aria-describedby"</Code>"."
                        </ApiRow>
                        <ApiRow name="error_message" ty="Option<String>" default="None">
                            "Whether you render an error message. "<Code inline=true>"Some"</Code>" always adds "
                            <Code inline=true>"error_props.id"</Code>" to "<Code inline=true>"aria-describedby"</Code>
                            "; otherwise it is added while the value is invalid."
                        </ApiRow>
                        <ApiRow name="is_date_picker" ty="bool" default="false">
                            "Set inside a "<Link href=routes::doc::date_picker::Hook.materialize()>"date picker"</Link>
                            ": the field gets "<Code inline=true>"role=\"presentation\""</Code>" instead of "
                            <Code inline=true>"group"</Code>", because the picker\u{2019}s group is the labelled element."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-date-field-return">
                    <ApiTable kind=ApiKind::Return of="UseDateFieldReturn">
                        <ApiRow name="field_props" ty="UseDateFieldProps">
                            "For the element around the segments: "<Code inline=true>"id"</Code>", "
                            <Code inline=true>"role"</Code>", "<Code inline=true>"aria-labelledby"</Code>", "
                            <Code inline=true>"aria-describedby"</Code>", "<Code inline=true>"aria-disabled"</Code>", "
                            <Code inline=true>"aria-invalid"</Code>", "<Code inline=true>"aria-required"</Code>" and a "
                            <Code inline=true>"keydown"</Code>" handler (the segments handle all keys). Spread with "
                            <Code inline=true>".into_attrs()"</Code>"."
                        </ApiRow>
                        <ApiRow name="label_props, description_props" ty="UseDateFieldLabelProps, UseDateFieldDescriptionProps">
                            "The "<Code inline=true>"id"</Code>" to give your label and description elements."
                        </ApiRow>
                        <ApiRow name="error_props" ty="UseDateFieldErrorProps">
                            <Code inline=true>"id"</Code>", "<Code inline=true>"role"</Code>" ("<Code inline=true>"alert"</Code>
                            ") and "<Code inline=true>"aria_live"</Code>" ("<Code inline=true>"\"polite\""</Code>") for the error message element."
                        </ApiRow>
                        <ApiRow name="segments" ty="Signal<Vec<DateSegment>>">"The segments to render."</ApiRow>
                        <ApiRow name="focused_segment" ty="Signal<Option<usize>>">
                            "Index of the segment that is the field\u{2019}s tab stop. "<Code inline=true>"None"</Code>" until "
                            "you set it."
                        </ApiRow>
                        <ApiRow name="focus_segment" ty="Callback<usize>">"Makes a segment the tab stop."</ApiRow>
                        <ApiRow name="focus_next, focus_previous" ty="Callback<()>">
                            "Move the tab stop to the next or previous editable segment. They only change "
                            <Code inline=true>"focused_segment"</Code>": moving the browser focus is up to you."
                        </ApiRow>
                        <ApiRow name="set_segment, clear_segment, increment, decrement, increment_page, decrement_page, increment_to_max, decrement_to_min, confirm_placeholder">
                            "The segment callbacks of "<Code inline=true>"use_date_field_state"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the displayed validation result is invalid."</ApiRow>
                        <ApiRow name="validation_errors" ty="Signal<Vec<String>>">"The displayed error messages."</ApiRow>
                        <ApiRow name="field_id" ty="String">"The field\u{2019}s "<Code inline=true>"id"</Code>"."</ApiRow>
                        <ApiRow name="state" ty="UseDateFieldStateReturn">"The underlying state."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-date-field-example">
                    <p>
                        "The field renders one element per segment. This minimal version wires the essential callbacks; the "
                        <AnchorLink href="#use-date-segment-example">"use_date_segment example"</AnchorLink>" adds paging, "
                        "clearing and moving the browser focus with "<Code inline=true>"focused_segment"</Code>"."
                    </p>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::hooks::*;
                            use leptos::prelude::*;
                            use time::OffsetDateTime;

                            let (birthday, set_birthday) = signal(None::<OffsetDateTime>);
                            let field = use_date_field(UseDateFieldInput {
                                value: birthday.into(),
                                label: Some("Birthday".to_owned()),
                                on_change: Some(Callback::new(move |date| set_birthday.set(date))),
                                ..Default::default()
                            });
                            let UseDateFieldReturn {
                                segments, focused_segment, set_segment, increment, decrement,
                                focus_next, focus_previous, confirm_placeholder, ..
                            } = field;

                            // The segment list never changes its shape: create every segment once.
                            let segment_views = segments.get_untracked().into_iter().enumerate().map(move |(index, segment)| {
                                let text = move || segments.with(|all| all.get(index).map(|it| it.text.clone()));
                                if !segment.is_editable {
                                    return view! { <span aria-hidden="true">{text}</span> }.into_any();
                                }
                                let ty = segment.segment_type;
                                let segment = use_date_segment(UseDateSegmentInput {
                                    segment,
                                    is_focused: Signal::derive(move || focused_segment.get() == Some(index)),
                                    on_change: Some(Callback::new(move |value| set_segment.run((ty, value)))),
                                    on_increment: Some(Callback::new(move |()| increment.run(ty))),
                                    on_decrement: Some(Callback::new(move |()| decrement.run(ty))),
                                    on_focus_next: Some(focus_next),
                                    on_focus_previous: Some(focus_previous),
                                    on_blur: Some(confirm_placeholder),
                                    ..Default::default()
                                });
                                view! { <span {..segment.segment_props.into_attrs()}>{text}</span> }.into_any()
                            });

                            view! {
                                <span id=field.label_props.id>"Birthday"</span>
                                <div {..field.field_props.into_attrs()}>{segment_views.collect_view()}</div>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_date_segment">
                <p>
                    "Makes one segment a "<Code inline=true>"spinbutton"</Code>": it handles typing, the arrow keys and "
                    <Keys keys="Backspace"/>", and calls the callbacks you wire to the field. Typed digits are buffered, so typing "
                    <Code inline=true>"1"</Code>" and "<Code inline=true>"2"</Code>" into the month gives 12. The focus "
                    "moves on as soon as another digit can\u{2019}t fit (typing "<Code inline=true>"4"</Code>
                    " into a day, or the second digit of a month)."
                </p>

                <p>
                    "The hook reads the "<Code inline=true>"segment"</Code>" you pass once. Create each segment element "
                    "once (the segment list never changes its shape) so that the digit buffer survives edits, and read "
                    "the text from the field\u{2019}s "<Code inline=true>"segments"</Code>" signal. The ARIA values "
                    "keep describing the segment the hook was created with (see "<AnchorLink href="#limitations">"Limitations"</AnchorLink>")."
                </p>

                <Section title="Input" id="use-date-segment-input">
                    <ApiTable kind=ApiKind::Input of="UseDateSegmentInput">
                        <ApiRow name="segment" ty="DateSegment" default="DateSegment::literal(\"\")">"The segment, as created."</ApiRow>
                        <ApiRow name="is_focused" ty="Signal<bool>" default="false">
                            "Whether this segment is the field\u{2019}s tab stop ("<Code inline=true>"tabindex=\"0\""</Code>")."
                        </ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Ignore all keys."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Sets "<Code inline=true>"aria-invalid"</Code>"."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<i32>>" default="None">
                            "Called with the typed number, or with 0 (AM) or 1 (PM) for the AM/PM segment."
                        </ApiRow>
                        <ApiRow name="on_increment, on_decrement" ty="Option<Callback<()>>" default="None"><Keys keys="ArrowUp"/>" and "<Keys keys="ArrowDown"/>"."</ApiRow>
                        <ApiRow name="on_increment_page, on_decrement_page" ty="Option<Callback<()>>" default="None"><Keys keys="PageUp"/>" and "<Keys keys="PageDown"/>"."</ApiRow>
                        <ApiRow name="on_increment_to_max, on_decrement_to_min" ty="Option<Callback<()>>" default="None"><Keys keys="End"/>" and "<Keys keys="Home"/>"."</ApiRow>
                        <ApiRow name="on_focus_next, on_focus_previous" ty="Option<Callback<()>>" default="None">
                            <Keys keys="ArrowRight"/>" and "<Keys keys="ArrowLeft"/>", auto-advance after typing, and "<Keys keys="Backspace"/>" on an empty segment."
                        </ApiRow>
                        <ApiRow name="on_clear" ty="Option<Callback<()>>" default="None"><Keys keys="Backspace"/>" and "<Keys keys="Delete"/>"."</ApiRow>
                        <ApiRow name="on_blur" ty="Option<Callback<()>>" default="None">
                            "Called when the segment loses focus. Wire it to "<Code inline=true>"confirm_placeholder"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-date-segment-return">
                    <ApiTable kind=ApiKind::Return of="UseDateSegmentReturn">
                        <ApiRow name="segment_props" ty="UseDateSegmentProps">
                            <Code inline=true>".into_attrs()"</Code>" spreads "<Code inline=true>"role"</Code>" ("
                            <Code inline=true>"spinbutton"</Code>", or "<Code inline=true>"presentation"</Code>" for literals), "
                            <Code inline=true>"tabindex"</Code>", "<Code inline=true>"aria-label"</Code>" (\u{201C}year\u{201D}, "
                            "\u{201C}month\u{201D}, \u{2026}), "<Code inline=true>"aria-valuenow"</Code>"/"
                            <Code inline=true>"-valuemin"</Code>"/"<Code inline=true>"-valuemax"</Code>"/"
                            <Code inline=true>"-valuetext"</Code>", "<Code inline=true>"aria-readonly"</Code>", "
                            <Code inline=true>"aria-disabled"</Code>", "<Code inline=true>"aria-invalid"</Code>" and the "
                            "keyboard, focus and blur handlers. The "<Code inline=true>"content_editable"</Code>", "
                            <Code inline=true>"input_mode"</Code>" and "<Code inline=true>"data_placeholder"</Code>
                            " fields are not part of the spread."
                        </ApiRow>
                        <ApiRow name="segment" ty="DateSegment">"The segment passed in."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-date-segment-example">
                    <p>
                        "leptonic has no segment atom yet, so the date field, "
                        <Link href=routes::doc::TimeField.materialize()>"Time Field Hooks"</Link>" and "
                        <Link href=routes::doc::date_picker::Hook.materialize()>"Date Picker Hooks"</Link>
                        " demos render their segments with this Leptos component. It wires "<Code inline=true>"use_date_segment"</Code>
                        " to either field, makes the first editable segment the tab stop, and moves the browser focus "
                        "along with "<Code inline=true>"focused_segment"</Code>"."
                    </p>

                    <Code language=Language::Rust>{include_str!("demos/date_segments.rs")}</Code>
                </Section>
            </Section>

            <Section title="DateSegment and DateSegmentType">
                <ApiTable kind=ApiKind::Fields of="DateSegment">
                    <ApiRow name="segment_type" ty="DateSegmentType">
                        <Code inline=true>"Year"</Code>", "<Code inline=true>"Month"</Code>", "<Code inline=true>"Day"</Code>", "
                        <Code inline=true>"Hour"</Code>", "<Code inline=true>"Minute"</Code>", "<Code inline=true>"Second"</Code>", "
                        <Code inline=true>"DayPeriod"</Code>" (AM/PM) or "<Code inline=true>"Literal"</Code>". "
                        <Code inline=true>"aria_label()"</Code>" names it for screen readers."
                    </ApiRow>
                    <ApiRow name="text" ty="String">"What to display: the zero-padded value, a placeholder, or the literal text."</ApiRow>
                    <ApiRow name="value" ty="Option<i32>">"The number; "<Code inline=true>"None"</Code>" for placeholders and literals."</ApiRow>
                    <ApiRow name="min_value, max_value" ty="Option<i32>">"The segment\u{2019}s range."</ApiRow>
                    <ApiRow name="is_editable" ty="bool">"Whether this is an editable segment rather than a literal."</ApiRow>
                    <ApiRow name="is_placeholder" ty="bool">"Whether the segment is empty."</ApiRow>
                </ApiTable>

                <p>
                    "Constructors ("<Code inline=true>"DateSegment::year(..)"</Code>", "<Code inline=true>"::month"</Code>", "
                    <Code inline=true>"::day"</Code>", "<Code inline=true>"::hour"</Code>", "<Code inline=true>"::minute"</Code>", "
                    <Code inline=true>"::second"</Code>", "<Code inline=true>"::day_period"</Code>", "
                    <Code inline=true>"::literal"</Code>") build segments for custom fields."
                </p>
            </Section>

            <Section title="IncompleteDate">
                <p>
                    "The editing buffer of the date field: every part is an "<Code inline=true>"Option"</Code>
                    " ("<Code inline=true>"year"</Code>", "<Code inline=true>"month"</Code>", "<Code inline=true>"day"</Code>", "
                    <Code inline=true>"hour"</Code>", "<Code inline=true>"minute"</Code>", "<Code inline=true>"second"</Code>", "
                    <Code inline=true>"day_period"</Code>"), and the hour is stored in the field\u{2019}s hour cycle. "
                    "You only need it to build your own field state: "<Code inline=true>"set"</Code>", "
                    <Code inline=true>"clear"</Code>", "<Code inline=true>"cycle"</Code>" (wrapping steps), "
                    <Code inline=true>"is_complete"</Code>", "<Code inline=true>"is_cleared"</Code>" and "
                    <Code inline=true>"to_date"</Code>" (fill missing parts from a placeholder) implement the behavior "
                    "described above."
                </p>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="0\u{2013}9">
                        "Type into the focused segment. The focus moves to the next segment once the segment is full "
                        "(four digits for the year, two otherwise) or no further digit fits. A number above the "
                        "segment\u{2019}s maximum is clamped: typing 1, 3 into a month gives 12."
                    </KeyRow>
                    <KeyRow keys="ArrowUp / ArrowDown">
                        "Increase or decrease the segment by one, wrapping around (12 \u{2192} 1 for months, 23 \u{2192} 0 "
                        "for hours). The year doesn\u{2019}t wrap. Toggles AM/PM."
                    </KeyRow>
                    <KeyRow keys="PageUp / PageDown">"Step by 5 years, 2 months, 7 days, 2 hours or 15 minutes/seconds."</KeyRow>
                    <KeyRow keys="Home / End">"Set the segment to its minimum or maximum."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowRight">"Move to the previous or next editable segment."</KeyRow>
                    <KeyRow keys="Backspace">
                        "Remove the last digit typed since the segment got focus; without typed digits, clear the segment. "
                        "On an empty segment, move to the previous segment."
                    </KeyRow>
                    <KeyRow keys="Delete">"Clear the segment."</KeyRow>
                    <KeyRow keys="A / P">"Set the AM/PM segment to AM or PM."</KeyRow>
                    <KeyRow keys="Tab / Shift + Tab">
                        "Move the focus into or out of the field. The field is a single tab stop: the segment you used last."
                    </KeyRow>
                </KeyboardTable>

                <p>"The keys do nothing while the field is disabled or read-only."</p>
            </Section>

            <Section title="Internationalization">
                <p>"The fields are not localized yet:"</p>

                <ul>
                    <li>
                        "The segment order and separators are fixed: "<Code inline=true>"yyyy-mm-dd"</Code>" (ISO 8601) and "
                        <Code inline=true>"hh:mm"</Code>", whatever the locale."
                    </li>
                    <li>
                        "The hour cycle comes from "<Code inline=true>"hour_cycle_24"</Code>", not from the locale. "
                        "The AM/PM segment always shows "<Code inline=true>"AM"</Code>" or "<Code inline=true>"PM"</Code>" "
                        "and reacts to the A and P keys."
                    </li>
                    <li>
                        "Placeholders ("<Code inline=true>"yyyy"</Code>", "<Code inline=true>"mm"</Code>", "
                        <Code inline=true>"dd"</Code>") and the segments\u{2019} "<Code inline=true>"aria-label"</Code>"s are English."
                    </li>
                    <li>"Only the Gregorian calendar is supported, and right-to-left layouts are not handled."</li>
                </ul>
            </Section>

            <Section title="Forms and Validation">
                <ul>
                    <li>
                        <Code inline=true>"use_date_field"</Code>" validates through "<Code inline=true>"use_form_validation_state"</Code>
                        ": a "<Code inline=true>"validate"</Code>" function, an external "<Code inline=true>"is_invalid"</Code>
                        " signal, and server errors matched by "<Code inline=true>"name"</Code>". The result sets "
                        <Code inline=true>"aria-invalid"</Code>" on the field and its segments; render "
                        <Code inline=true>"validation_errors"</Code>" in the error element (see the demo)."
                    </li>
                    <li>
                        <Code inline=true>"min"</Code>" and "<Code inline=true>"max"</Code>" clamp the emitted value instead of "
                        "reporting an error. "<Code inline=true>"is_required"</Code>" only sets "<Code inline=true>"aria-required"</Code>"."
                    </li>
                    <li>
                        "The fields render no "<Code inline=true>"<input>"</Code>". To submit the value with a native form, "
                        "add a hidden input bound to it. See "<Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link>"."
                    </li>
                </ul>
            </Section>

            <Section title="Limitations">
                <ul>
                    <li>
                        <b>"Segment snapshots."</b>" "<Code inline=true>"use_date_segment"</Code>" keeps the "
                        <Code inline=true>"DateSegment"</Code>" it was created with: "<Code inline=true>"aria-valuenow"</Code>" "
                        "and "<Code inline=true>"aria-valuetext"</Code>" keep their initial values, and "<Keys keys="Backspace"/>" decides "
                        "between clearing the segment and moving to the previous one by the initial state. In a field that "
                        "started empty, "<Keys keys="Backspace"/>" on a segment you filled moves to the previous segment instead of "
                        "clearing it."
                    </li>
                    <li>
                        <b>"Modifier keys."</b>" Segments react to the arrow keys even with Alt held, so the date picker\u{2019}s "
                        <Keys keys="Alt + ArrowDown"/>" also changes the focused segment."
                    </li>
                    <li>
                        <b>"Disabled fields stay focusable."</b>" The tab stop keeps "<Code inline=true>"tabindex=\"0\""</Code>" "
                        "while the field is disabled."
                    </li>
                    <li>
                        <b>"Date-only values carry a time."</b>" A field without "<Code inline=true>"show_time"</Code>" emits an "
                        <Code inline=true>"OffsetDateTime"</Code>" whose time (and every value\u{2019}s seconds) comes from the "
                        "placeholder, in UTC."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::DateTime.materialize()>"Date & Time"</Link></li>
                <li><Link href=routes::doc::TimeField.materialize()>"Time Field Hooks"</Link></li>
                <li><Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link></li>
                <li><Link href=routes::doc::Calendar.materialize()>"Calendar"</Link></li>
                <li><Link href=routes::doc::utilities::UseSpinButton.materialize()>"use_spin_button"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
