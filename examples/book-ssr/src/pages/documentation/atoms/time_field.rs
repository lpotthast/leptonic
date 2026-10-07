use indoc::indoc;
use leptos::prelude::*;

use super::demos::time_field::TimeFieldAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomTimeField() -> impl IntoView {
    view! {
        <DocPage title="Time Field Atom">
            <p>
                "The unstyled "<Code inline=true>"TimeField"</Code>" holds a time of day edited in segments. Its parts are "
                "those of a date field: a "<Link href=format!("{}#dateinput", routes::doc::date_field::Atom.materialize())>"DateInput"</Link>
                " rendering a "<Link href=format!("{}#datesegment", routes::doc::date_field::Atom.materialize())>"DateSegment"</Link>
                " per segment, and the "<Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>". See the "
                <Link href=routes::doc::TimeField.materialize()>"Time Field overview"</Link>" for when to use a time field."
            </p>

            <ReactAria hook="TimeField"/>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"TimeField"</Code>" calls "
                    <Link href=format!("{}#use-time-field-state", routes::doc::time_field::Hook.materialize())>"use_time_field_state"</Link>" and "
                    <Link href=format!("{}#use-time-field", routes::doc::time_field::Hook.materialize())>"use_time_field"</Link>
                    "; its segments are those of the "<Link href=routes::doc::date_field::Atom.materialize()>"Date Field Atoms"</Link>"."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::{datepicker::{DateInput, DateSegment, TimeField}, field::Label},
                            jiff::civil::Time,
                        };
                        use leptos::prelude::*;

                        let alarm = RwSignal::new(None::<Time>);

                        view! {
                            <TimeField<Time> value=alarm set_value=alarm>
                                <Label>"Alarm"</Label>
                                <DateInput children=|segment| view! { <DateSegment segment/> }/>
                            </TimeField<Time>>
                        }
                    "#)}
                </Code>
                <p>
                    "The value is a "<Code inline=true>"civil::Time"</Code>", or a "<Code inline=true>"civil::DateTime"</Code>
                    " or "<Code inline=true>"Zoned"</Code>" whose time the field edits (a "
                    <Link href=format!("{}#timevalue", routes::doc::date_field::Hook.materialize())><Code inline=true>"TimeValue"</Code></Link>")."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "A pickup time between "<Code inline=true>"min_value"</Code>" and "<Code inline=true>"max_value"</Code>
                    ": a time outside is invalid, and the "<Code inline=true>"FieldError"</Code>" says why."
                </p>

                <Demo description="Pickup time field of the atoms with a minimum, a maximum and a disabled toggle" source=include_str!("demos/time_field.rs")>
                    <TimeFieldAtomDemo/>
                </Demo>
            </Section>

            <Section title="TimeField">
                <Section title="Props" id="timefield-props">
                    <ApiTable kind=ApiKind::Props of="atoms::datepicker::TimeField">
                        <ApiRow name="default_value" ty="Option<T>" default="None">"The initial value (uncontrolled)."</ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Option<T>>>" default="None">"The value (controlled): a value or any signal."</ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Option<T>>>" default="None">"Receives the new value."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<T>>>" default="None">"Called with each new value."</ApiRow>
                        <ApiRow name="placeholder_value" ty="MaybeProp<T>" default="None">"Where empty segments start when stepped. Default: midnight."</ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<Option<T>>" default="None">"The earliest and latest valid time."</ApiRow>
                        <ApiRow name="granularity" ty="MaybeProp<Granularity>" default="None">
                            "The finest segment: "<Code inline=true>"Hour"</Code>", "<Code inline=true>"Minute"</Code>" (the default) or "
                            <Code inline=true>"Second"</Code>"."
                        </ApiRow>
                        <ApiRow name="hour_cycle" ty="MaybeProp<HourCycle>" default="None">
                            <Code inline=true>"H12"</Code>" or "<Code inline=true>"H24"</Code>". Default: the locale\u{2019}s."
                        </ApiRow>
                        <ApiRow name="hide_time_zone" ty="Signal<bool>" default="false">"Hides the time zone of a zoned value."</ApiRow>
                        <ApiRow name="should_force_leading_zeros" ty="Signal<bool>" default="false">"Pads the hours to two digits."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only, is_required, is_invalid" ty="Signal<bool>" default="false">
                            "As on "<Link href=format!("{}#datefield-props", routes::doc::date_field::Atom.materialize())>"DateField"</Link>"."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<T>>>" default="None">"Custom validation."</ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "Default: the surrounding "<Code inline=true>"Form"</Code>"\u{2019}s, else "<Code inline=true>"Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="name, form" ty="Option<String>" default="None">"The hidden input\u{2019}s name and form."</ApiRow>
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

            <Section title="Data Attributes">
                <p>
                    "Those of a date field: "<Code inline=true>"TimeField"</Code>" has "<Code inline=true>"data-disabled"</Code>", "
                    <Code inline=true>"data-readonly"</Code>", "<Code inline=true>"data-required"</Code>" and "
                    <Code inline=true>"data-invalid"</Code>"; see the "
                    <Link href=format!("{}#data-attributes", routes::doc::date_field::Atom.materialize())>"Date Field Atoms"</Link>
                    " for the input and the segments. The hour, minute and AM/PM segments have the "<Code inline=true>"data-type"</Code>
                    "s "<Code inline=true>"hour"</Code>", "<Code inline=true>"minute"</Code>" and "<Code inline=true>"dayPeriod"</Code>"."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. "<Code inline=true>"TimeField"</Code>" renders a "<Code inline=true>"<div>"</Code>" with the class "<Code inline=true>"leptonic-TimeField"</Code>" "
                    "and the "<Code inline=true>"classes"</Code>" you pass; its "<Code inline=true>"DateInput"</Code>" and "<Code inline=true>"DateSegment"</Code>"s are those of a "
                    <Link href=format!("{}#styling", routes::doc::date_field::Atom.materialize())>"date field"</Link>". The demo above uses "
                    "the date field\u{2019}s CSS:"
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

            <SeeAlso>
                <li><Link href=routes::doc::TimeField.materialize()>"Time Field overview"</Link></li>
                <li><Link href=routes::doc::time_field::Hook.materialize()>"Time Field Hooks"</Link></li>
                <li><Link href=routes::doc::date_field::Atom.materialize()>"Date Field Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
