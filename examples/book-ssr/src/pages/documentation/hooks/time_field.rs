use indoc::indoc;
use leptos::prelude::*;

use super::demos::time_field::TimeFieldHookDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageTimeFieldHooks() -> impl IntoView {
    view! {
        <DocPage title="Time Field Hooks">
            <p>
                "A time field is a date field of hours to seconds (with AM/PM on a 12-hour clock) whose value is a time. "
                <AnchorLink href="#use-time-field-state">"use_time_field_state"</AnchorLink>" holds its state, "
                <AnchorLink href="#use-time-field">"use_time_field"</AnchorLink>" returns the attributes of its label, group "
                "and hidden input, and the segments use "
                <Link href=format!("{}#use-date-segment", routes::doc::date_field::Hook.materialize())>"use_date_segment"</Link>
                ", as in a date field. See the "<Link href=routes::doc::TimeField.materialize()>"Time Field overview"</Link>
                " for the concept."
            </p>

            <ReactAria hook="useTimeField"/>

            <Section title="use_time_field_state">
            <Section title="Input" id="use-time-field-state-input">
                <p>
                    "Generic over the value, a "<Link href=format!("{}#timevalue", routes::doc::date_field::Hook.materialize())><Code inline=true>"TimeValue"</Code></Link>
                    ": a "<Code inline=true>"civil::Time"</Code>", or a "<Code inline=true>"civil::DateTime"</Code>" or "
                    <Code inline=true>"Zoned"</Code>" whose time it edits."
                </p>
                <ApiTable kind=ApiKind::Input of="datepicker::use_time_field_state::UseTimeFieldStateInput">
                    <ApiRow name="default_value" ty="Option<T>" default="None">"The initial value."</ApiRow>
                    <ApiRow name="value" ty="Option<ValueBinding<Option<T>>>" default="None">"The value as app state, replacing "<Code inline=true>"default_value"</Code>"."</ApiRow>
                    <ApiRow name="on_change" ty="Option<Callback<Option<T>>>" default="None">"Called with each new value."</ApiRow>
                    <ApiRow name="placeholder_value" ty="Signal<Option<T>>" default="None">"Where empty segments start when stepped. Default: midnight."</ApiRow>
                    <ApiRow name="min_value, max_value" ty="Signal<Option<Time>>" default="None">
                        "The earliest and latest valid time (\u{201c}Value must be 8:00 AM or later.\u{201d})."
                    </ApiRow>
                    <ApiRow name="granularity" ty="Signal<Option<Granularity>>" default="None">
                        "The finest segment: "<Code inline=true>"Hour"</Code>", "<Code inline=true>"Minute"</Code>" (the default) or "
                        <Code inline=true>"Second"</Code>"."
                    </ApiRow>
                    <ApiRow name="hour_cycle" ty="Signal<Option<HourCycle>>" default="None">"A 12- or 24-hour clock. Default: the locale\u{2019}s."</ApiRow>
                    <ApiRow name="hide_time_zone" ty="Signal<bool>" default="false">"Hides the time zone of a zoned value."</ApiRow>
                    <ApiRow name="should_force_leading_zeros" ty="Signal<bool>" default="false">"Pads the hours to two digits."</ApiRow>
                    <ApiRow name="is_disabled, is_read_only, is_required, is_invalid" ty="Signal<bool>" default="false">
                        "As for "<Link href=format!("{}#use-date-field-state-input", routes::doc::date_field::Hook.materialize())>"use_date_field_state"</Link>"."
                    </ApiRow>
                    <ApiRow name="validate" ty="Option<ValidateFn<Option<T>>>" default="None">"Custom validation."</ApiRow>
                    <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Aria">"When errors show."</ApiRow>
                    <ApiRow name="name" ty="Option<String>" default="None">"The field\u{2019}s name in forms."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return" id="use-time-field-state-return">
                <p>"A "<Code inline=true>"TimeFieldState<T>"</Code>", "<Code inline=true>"Copy"</Code>":"</p>
                <ApiTable kind=ApiKind::Return of="datepicker::use_time_field_state::TimeFieldState">
                    <ApiRow name="value" ty="Signal<Option<T>>">"The value."</ApiRow>
                    <ApiRow name="time_value" ty="Signal<Option<Time>>">"The time of the value."</ApiRow>
                    <ApiRow name="field" ty="DateFieldState<Field>">
                        "The "<Link href=format!("{}#use-date-field-state-return", routes::doc::date_field::Hook.materialize())>"date field state"</Link>
                        " editing it. A "<Code inline=true>"civil::Time"</Code>" is edited as a "<Code inline=true>"civil::DateTime"</Code>
                        " on today."
                    </ApiRow>
                </ApiTable>
            </Section>
            </Section>

            <Section title="use_time_field">
                <p>
                    <Code inline=true>"use_time_field"</Code>" is "
                    <Link href=format!("{}#use-date-field", routes::doc::date_field::Hook.materialize())>"use_date_field"</Link>" over "
                    <Code inline=true>"state.field"</Code>": its "<Code inline=true>"UseTimeFieldInput"</Code>" has the same fields as "
                    <Code inline=true>"UseDateFieldInput"</Code>" (with the time field state and the same "
                    <Code inline=true>"DateFieldOptions"</Code>") and it returns the same "<Code inline=true>"UseDateFieldReturn"</Code>" (label, field, hidden input, description "
                    "and error message attributes, and the "<Code inline=true>"data"</Code>" for the segments). Its hidden input "
                    "submits the time, e.g. "<Code inline=true>"08:30:00"</Code>", not a date and time."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::{
                            CapturedElement,
                            IntoAttrs,
                            hooks::datepicker::*,
                            jiff::civil::Time,
                        };
                        use leptos::prelude::*;

                        let state = use_time_field_state(UseTimeFieldStateInput::<Time>::default());
                        let field = use_time_field(UseTimeFieldInput {
                            state,
                            element: CapturedElement::new(),
                            input_element: CapturedElement::new(),
                            options: DateFieldOptions { has_label: true.into(), ..DateFieldOptions::default() },
                        });
                        // Render the label, the group with a segment per `state.field.segments`
                        // (`use_date_segment(UseDateSegmentInput { segment, data: field.data, element })`) and the hidden input.
                    ")}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "A pickup time between 8 AM and 6 PM, with the "<Code inline=true>"Segment"</Code>" Leptos component of the "
                    <Link href=format!("{}#use-date-segment-example", routes::doc::date_field::Hook.materialize())>"date field example"</Link>
                    ". A time outside fails the validation."
                </p>

                <Demo description="Pickup time field of the hooks with a minimum, a maximum and a disabled toggle" source=include_str!("demos/time_field.rs")>
                    <TimeFieldHookDemo/>
                </Demo>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::TimeField.materialize()>"Time Field overview"</Link></li>
                <li><Link href=routes::doc::time_field::Atom.materialize()>"Time Field Atom"</Link></li>
                <li><Link href=routes::doc::date_field::Hook.materialize()>"Date Field Hooks"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
