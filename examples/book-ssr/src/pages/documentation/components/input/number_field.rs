use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::number_field::NumberFieldDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageNumberField() -> impl IntoView {
    view! {
        <DocPage title="Number Field Component">
            <p>
                "The themed "<Code inline=true>"NumberField"</Code>" component: a number input with stepper buttons, its label, "
                "description and validation errors, styled by leptonic\u{2019}s theme. See the "
                <Link href=routes::doc::NumberField.materialize()>"Number Field overview"</Link>" for concept guidance."
            </p>

            <Demo
                description="Order form with an integer quantity, a price in euros and a percentage discount, with a disabled toggle"
                source=include_str!("demos/number_field.rs")
            >
                <NumberFieldDemo/>
            </Demo>

            <Section title="Props">
                <Section title="NumberField">
                    <ApiTable kind=ApiKind::Props of="components::number_field::NumberField">
                        <ApiRow name="label" ty="MaybeProp<String>" default="None">
                            "The visible label. Without it, set "<Code inline=true>"aria_label"</Code>"."
                        </ApiRow>
                        <ApiRow name="description" ty="MaybeProp<String>" default="None">"Help text below the input."</ApiRow>
                        <ApiRow name="default_value" ty="Option<T>" default="None">
                            "The initial value, unless "<Code inline=true>"value"</Code>" is set; "<Code inline=true>"None"</Code>
                            " starts empty. A form reset restores the value the field started with."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<OptionalNumberSignal<T>>" default="None">
                            "The value (controlled), replacing "<Code inline=true>"default_value"</Code>": a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Option<T>>>" default="None">
                            "Receives the new value: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<T>>>" default="None">"Called with the value when a committed value changes."</ApiRow>
                        <ApiRow name="min_value, max_value" ty="MaybeProp<T>" default="None">
                            "The range. Without them, integer types stop at their own bounds."
                        </ApiRow>
                        <ApiRow name="step" ty="MaybeProp<T>" default="None">
                            "The step of increments; without one, 1 (0.01 for percentages). Typed values snap to it only when it is set."
                        </ApiRow>
                        <ApiRow name="format_options" ty="Signal<NumberFormatOptions>" default="decimal, grouped">
                            "Formatting: style (decimal, percent, currency, unit), grouping, digits, sign display."
                        </ApiRow>
                        <ApiRow name="commit_behavior" ty="CommitBehavior" default="Snap">
                            <Code inline=true>"Snap"</Code>" clamps typed values and rounds them to the step; "
                            <Code inline=true>"Validate"</Code>" keeps them and reports them as invalid."
                        </ApiRow>
                        <ApiRow name="placeholder" ty="MaybeProp<String>" default="None">"Shown while the field is empty."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the field or makes it read-only."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">
                            "Marks the field as required (the native "<Code inline=true>"required"</Code>" with "
                            <Code inline=true>"Native"</Code>" validation, "<Code inline=true>"aria-required"</Code>" with "
                            <Code inline=true>"Aria"</Code>")."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the field invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<T>>>" default="None">
                            "Validates the value, returning "<Code inline=true>"Ok(())"</Code>" or "<Code inline=true>"Err(messages)"</Code>"."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown, see "<Link href=format!("{}#validation", routes::doc::text_field::Component.materialize())>"Validation"</Link>"."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "The name of a hidden input holding the value, for form submission and server errors."
                        </ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the input when it mounts."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"The accessible name, for a field without label."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the input gains or loses focus."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the field\u{2019}s "<Code inline=true>"<div>"</Code>"."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Values and Formatting">
                <p>
                    <Code inline=true>"NumberField<T>"</Code>" works on any primitive integer or float type "
                    <Code inline=true>"T"</Code>", taken from its "<Code inline=true>"value"</Code>" or "
                    <Code inline=true>"default_value"</Code>". Its value is an "<Code inline=true>"Option<T>"</Code>": "
                    <Code inline=true>"None"</Code>" while the field is empty. The stepper buttons, "<Keys keys="ArrowUp"/>" and "
                    <Keys keys="ArrowDown"/>" change it by "<Code inline=true>"step"</Code>" within "
                    <Code inline=true>"min_value"</Code>" and "<Code inline=true>"max_value"</Code>"; "<Keys keys="Home"/>" and "
                    <Keys keys="End"/>" jump to the limits. "<Code inline=true>"format_options"</Code>" formats it for the "
                    "current locale, as a currency, a percentage or a unit."
                </p>
                <p>
                    "Typed values are committed when the input loses focus or on "<Keys keys="Enter"/>". By default ("
                    <Code inline=true>"CommitBehavior::Snap"</Code>"), a value outside the range is clamped and rounded to the "
                    "step; with "<Code inline=true>"CommitBehavior::Validate"</Code>" it is kept and reported as invalid."
                </p>
            </Section>

            <Section title="Labels, State and Validation">
                <p>
                    "Labels, descriptions, the value state and validation work as for the "
                    <Link href=routes::doc::text_field::Component.materialize()>"Text Field Component"</Link>
                    ": see its sections on "
                    <Link href=format!("{}#labels-and-descriptions", routes::doc::text_field::Component.materialize())>"labels and descriptions"</Link>", "
                    <Link href=format!("{}#state", routes::doc::text_field::Component.materialize())>"state"</Link>" and "
                    <Link href=format!("{}#validation", routes::doc::text_field::Component.materialize())>"validation"</Link>"."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The field shares the input styling of leptonic\u{2019}s theme (see the "
                    <Link href=format!("{}#styling", routes::doc::text_field::Component.materialize())>"Text Field Component\u{2019}s CSS variables"</Link>
                    "). Its "<Code inline=true>"<div>"</Code>" has the classes "<Code inline=true>"leptonic-text-field"</Code>" and "
                    <Code inline=true>"leptonic-number-field"</Code>". For full control over the markup, build the field from the "
                    <Link href=routes::doc::number_field::Atom.materialize()>"Number Field Atoms"</Link>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::NumberField.materialize()>"Number Field overview"</Link></li>
                <li><Link href=routes::doc::number_field::Hook.materialize()>"Number Field Hooks"</Link></li>
                <li><Link href=routes::doc::number_field::Atom.materialize()>"Number Field Atoms"</Link></li>
                <li><Link href=routes::doc::text_field::Component.materialize()>"Text Field Component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
