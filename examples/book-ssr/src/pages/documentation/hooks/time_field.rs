use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::time_field::TimeFieldDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageTimeFieldHooks() -> impl IntoView {
    view! {
        <DocPage title="Time Field Hooks">
            <p>
                "A time field lets users enter a time of day, such as the start of a meeting. Like a "
                <Link href=routes::doc::DateField.materialize()>"date field"</Link>", it splits the value into segments "
                "(hour, minute, optionally second and AM/PM) that you fill in by typing digits or change with the arrow "
                "keys, so no free text has to be parsed. Use a date field with "<Code inline=true>"show_time"</Code>
                " for a date together with a time. See "<Link href=routes::doc::DateTime.materialize()>"Date & Time"</Link>
                " for the other date and time concepts."
            </p>

            <ReactAria hook="useTimeField"/>

            <Section title="Demo">
                <p>
                    "Click the field (or tab to it) and type a time: the focus moves on to the next segment once a "
                    "segment is complete. The arrow keys change the focused segment. The segments are rendered by the "
                    "Leptos component shown in the "
                    <Link href=format!("{}#use-date-segment-example", routes::doc::DateField.materialize())>"use_date_segment example"</Link>
                    ", as leptonic has no segment atom yet."
                </p>

                <Demo description="Time field with 24-hour and seconds toggles" source=include_str!("demos/time_field.rs")>
                    <TimeFieldDemo/>
                </Demo>
            </Section>

            <Section title="Segments">
                <p>
                    "The field renders "<Code inline=true>"--:--"</Code>", "<Code inline=true>"--:--:--"</Code>" with "
                    <Code inline=true>"show_seconds"</Code>", plus "<Code inline=true>"AM"</Code>"/"<Code inline=true>"PM"</Code>
                    " on a 12-hour clock. Its segments work as in the date field: each editable segment is a spin button "
                    "rendered with "<Link href=format!("{}#use-date-segment", routes::doc::DateField.materialize())>"use_date_segment"</Link>
                    ", literal segments are separators, and the field is a single tab stop. See "
                    <Link href=format!("{}#how-segments-work", routes::doc::DateField.materialize())>"How Segments Work"</Link>
                    ". Pressing an arrow key on an empty segment starts from midnight."
                </p>
            </Section>

            <Section title="use_time_field">
                <p>
                    "Builds the segments and adds IDs and ARIA attributes for the field, its label, description and error "
                    "message. Its value is a "<Code inline=true>"TimeValue"</Code>". Pass the value in and update it in "
                    <Code inline=true>"on_change"</Code>"."
                </p>

                <Section title="Input" id="use-time-field-input">
                    <ApiTable kind=ApiKind::Input of="UseTimeFieldInput">
                        <ApiRow name="value" ty="Signal<Option<TimeValue>>" default="None">"The current value."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<TimeValue>>>" default="None">
                            "Called with the complete value, or "<Code inline=true>"None"</Code>" once every segment is cleared."
                        </ApiRow>
                        <ApiRow name="hour_cycle_24" ty="bool" default="true">"Use a 24-hour clock instead of a 12-hour clock with AM/PM."</ApiRow>
                        <ApiRow name="show_seconds" ty="bool" default="false">"Add a seconds segment."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Turn off all segment changes."</ApiRow>
                        <ApiRow name="is_required" ty="bool" default="false">"Sets "<Code inline=true>"aria-required"</Code>"."</ApiRow>
                        <ApiRow name="label, description, error_message" ty="Option<String>" default="None">
                            "Whether you render these elements, as for "
                            <Link href=format!("{}#use-date-field-input", routes::doc::DateField.materialize())>"use_date_field"</Link>"."
                        </ApiRow>
                        <ApiRow name="min, max" ty="Option<TimeValue>" default="None">"Currently ignored."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-time-field-return">
                    <p>
                        "The same fields as "<Link href=format!("{}#use-date-field-return", routes::doc::DateField.materialize())>"use_date_field"</Link>
                        " ("<Code inline=true>"field_props"</Code>", "<Code inline=true>"label_props"</Code>", "
                        <Code inline=true>"description_props"</Code>", "<Code inline=true>"error_props"</Code>", "
                        <Code inline=true>"segments"</Code>", "<Code inline=true>"focused_segment"</Code>", "
                        <Code inline=true>"field_id"</Code>", the focus and segment callbacks), without "
                        <Code inline=true>"is_invalid"</Code>", "<Code inline=true>"validation_errors"</Code>" and "
                        <Code inline=true>"state"</Code>". The field never sets "<Code inline=true>"aria-invalid"</Code>"."
                    </p>
                </Section>

                <Section title="Example" id="use-time-field-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::hooks::*;
                            use leptos::prelude::*;

                            let (value, set_value) = signal(None::<TimeValue>);
                            let field = use_time_field(UseTimeFieldInput {
                                value: value.into(),
                                label: Some("Meeting time".to_owned()),
                                on_change: Some(Callback::new(move |v| set_value.set(v))),
                                ..Default::default()
                            });

                            view! {
                                <span id=field.label_props.id>"Meeting time"</span>
                                <div {..field.field_props.into_attrs()}>
                                    // One element per segment, made a spin button by `use_date_segment`.
                                </div>
                            }
                        "#)}
                    </Code>
                </Section>

                <Section title="TimeValue">
                    <ApiTable kind=ApiKind::Fields of="TimeValue">
                        <ApiRow name="hour" ty="u8">"0\u{2013}23."</ApiRow>
                        <ApiRow name="minute, second" ty="u8">"0\u{2013}59."</ApiRow>
                    </ApiTable>

                    <p>
                        <Code inline=true>"TimeValue::new(h, m, s)"</Code>" clamps to these ranges, "
                        <Code inline=true>"TimeValue::hm(h, m)"</Code>" sets the seconds to 0. "
                        <Code inline=true>"format_hm()"</Code>" and "<Code inline=true>"format_hms()"</Code>" format as "
                        <Code inline=true>"14:30"</Code>" and "<Code inline=true>"14:30:00"</Code>"; "
                        <Code inline=true>"to_12_hour()"</Code>" returns the 12-hour hour and whether it is PM."
                    </p>
                </Section>
            </Section>

            <Section title="use_time_field_state">
                <p>
                    "A value holder for a time field: a signal with the value and callbacks to change it. "
                    <Code inline=true>"use_time_field"</Code>" doesn\u{2019}t need it (it keeps its own editing buffer), "
                    "but it is a convenient place to keep the value:"
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        let time = use_time_field_state(Some(TimeValue::hm(9, 0)));
                        let field = use_time_field(UseTimeFieldInput {
                            value: time.value,
                            on_change: Some(time.set_value),
                            ..Default::default()
                        });
                    ")}
                </Code>

                <Section title="Input" id="use-time-field-state-input">
                    <p>
                        "The hook takes a single argument, the initial value: "<Code inline=true>"Option<TimeValue>"</Code>"."
                    </p>
                </Section>

                <Section title="Return" id="use-time-field-state-return">
                    <ApiTable kind=ApiKind::Return of="UseTimeFieldStateReturn">
                        <ApiRow name="value" ty="Signal<Option<TimeValue>>">"The value."</ApiRow>
                        <ApiRow name="set_value" ty="Callback<Option<TimeValue>>">"Replaces the value."</ApiRow>
                        <ApiRow name="clear" ty="Callback<()>">"Sets the value to "<Code inline=true>"None"</Code>"."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="IncompleteTime">
                <p>
                    "The editing buffer of the time field: every part is an "<Code inline=true>"Option"</Code>" ("
                    <Code inline=true>"hour"</Code>", "<Code inline=true>"minute"</Code>", "<Code inline=true>"second"</Code>", "
                    <Code inline=true>"day_period"</Code>"), and the hour is stored in the field\u{2019}s hour cycle. While you "
                    "edit, the field keeps every segment separately, so a value can be partly filled in; "
                    <Code inline=true>"on_change"</Code>" only fires once every segment is filled or every segment is "
                    "cleared. You only need the type to build your own field state: "<Code inline=true>"set"</Code>", "
                    <Code inline=true>"clear"</Code>", "<Code inline=true>"cycle"</Code>" (wrapping steps), "
                    <Code inline=true>"is_complete"</Code>", "<Code inline=true>"is_cleared"</Code>" and "
                    <Code inline=true>"to_time"</Code>" (fill missing parts from a placeholder) implement this behavior."
                </p>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="0\u{2013}9">
                        "Type into the focused segment. The focus moves to the next segment once the segment is full "
                        "(two digits) or no further digit fits. A number above the segment\u{2019}s maximum is clamped."
                    </KeyRow>
                    <KeyRow keys="ArrowUp / ArrowDown">
                        "Increase or decrease the segment by one, wrapping around (23 \u{2192} 0 for hours). Toggles AM/PM."
                    </KeyRow>
                    <KeyRow keys="PageUp / PageDown">"Step by 2 hours or 15 minutes/seconds."</KeyRow>
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

            <Section title="Limitations">
                <ul>
                    <li>
                        "The field is not localized: the separator is always "<Code inline=true>":"</Code>", the hour cycle "
                        "comes from "<Code inline=true>"hour_cycle_24"</Code>" instead of the locale, the AM/PM segment always "
                        "shows "<Code inline=true>"AM"</Code>" or "<Code inline=true>"PM"</Code>", and the segments\u{2019} "
                        <Code inline=true>"aria-label"</Code>"s are English."
                    </li>
                    <li>
                        "It has no validation yet, and "<Code inline=true>"min"</Code>" and "<Code inline=true>"max"</Code>" are ignored. "
                        "It renders no "<Code inline=true>"<input>"</Code>": to submit the value with a native form, add a "
                        "hidden input bound to it."
                    </li>
                    <li>
                        "The segments have the "<Link href=format!("{}#limitations", routes::doc::DateField.materialize())>"limitations"</Link>
                        " of the date field\u{2019}s segments."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::DateTime.materialize()>"Date & Time"</Link></li>
                <li><Link href=routes::doc::DateField.materialize()>"Date Field Hooks"</Link></li>
                <li><Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link></li>
                <li><Link href=routes::doc::utilities::UseSpinButton.materialize()>"use_spin_button"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
