use indoc::indoc;
use leptos::prelude::*;

use super::demos::number_field::NumberFieldAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomNumberField() -> impl IntoView {
    view! {
        <DocPage title="Number Field Atoms">
            <p>
                "The number field atoms render an unstyled number field for values of any primitive integer or float type: "
                <Code inline=true>"NumberField"</Code>" holds the value, "<Code inline=true>"NumberFieldGroup"</Code>" wraps "
                "the "<Link href=format!("{}#input-textarea", routes::doc::text_field::Atom.materialize())>"Input"</Link>" and the stepper buttons, and the "
                <Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>" add a label, a description and an "
                "error message. See the "<Link href=routes::doc::NumberField.materialize()>"Number Field overview"</Link>
                " for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"NumberField"</Code>" calls "
                    <Link href=format!("{}#use-number-field-state", routes::doc::number_field::Hook.materialize())>"use_number_field_state"</Link>
                    " and "<Link href=format!("{}#use-number-field", routes::doc::number_field::Hook.materialize())>"use_number_field"</Link>". "<Code inline=true>"NumberFieldGroup"</Code>" calls "
                    <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>" and "
                    <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>", the stepper buttons "
                    <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" and "<Code inline=true>"use_hover"</Code>"."
                </p>
            </Section>

            <Section title="Example">
                <p>
                    "The value type "<Code inline=true>"T"</Code>" is inferred from "<Code inline=true>"default_value"</Code>
                    " or "<Code inline=true>"value"</Code>"; without either, name it: "
                    <Code inline=true>"<NumberField<u8>>\u{2026}</NumberField<u8>>"</Code>"."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::{
                            field::{Description, FieldError, Label},
                            input::Input,
                            number_field::*,
                        };

                        view! {
                            <NumberField default_value=1024_u32 min_value=1 name="quantity">
                                <Label>"Quantity"</Label>
                                <NumberFieldGroup>
                                    <NumberFieldDecrementButton>
                                        <span aria-hidden="true">"\u{2212}"</span>
                                    </NumberFieldDecrementButton>
                                    <Input/>
                                    <NumberFieldIncrementButton>
                                        <span aria-hidden="true">"+"</span>
                                    </NumberFieldIncrementButton>
                                </NumberFieldGroup>
                                <Description>"How many to order."</Description>
                                <FieldError/>
                            </NumberField>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "A temperature in an "<Code inline=true>"RwSignal<Option<i8>>"</Code>", passed as "<Code inline=true>"value"</Code>
                    " and "<Code inline=true>"set_value"</Code>". Without "
                    <Code inline=true>"min_value"</Code>" and "<Code inline=true>"max_value"</Code>", the type\u{2019}s "
                    "bounds apply: hold a button, or press "<Keys keys="Home"/>" or "<Keys keys="End"/>" in the input, and "
                    "the value stops at \u{2212}128 or 127. Typed values beyond them aren\u{2019}t accepted."
                </p>
                <Demo
                    description="Temperature number field of type i8 controlled by a signal, with stepper buttons and a disabled toggle"
                    source=include_str!("demos/number_field.rs")
                >
                    <NumberFieldAtomDemo/>
                </Demo>
            </Section>

            <Section title="NumberField">
                <p>
                    "Creates the field and renders a "<Code inline=true>"<div>"</Code>" around its children. With a "
                    <Code inline=true>"name"</Code>", it also renders a hidden input submitting the value with a form (the "
                    "visible input shows the formatted text)."
                </p>
                <Section title="Props" id="number-field-props">
                    <ApiTable kind=ApiKind::Props of="atoms::number_field::NumberField">
                        <ApiRow name="default_value" ty="Option<T>" default="None">
                            "The initial value, unless "<Code inline=true>"value"</Code>" is set; "<Code inline=true>"None"</Code>
                            ": empty. A form reset restores the value the field started with."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<OptionalNumberSignal<T>>" default="None">
                            "The value (controlled), replacing "<Code inline=true>"default_value"</Code>": a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Option<T>>>" default="None">
                            "Receives the new state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<T>>>" default="None">"Called when the committed value changes."</ApiRow>
                        <ApiRow name="min_value, max_value" ty="MaybeProp<T>" default="None">
                            "The allowed range, e.g. "<Code inline=true>"min_value=0"</Code>". Integer types are also bounded by "
                            "their own range."
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
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the field or makes it read-only."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">
                            "Marks the field as required: with the native "<Code inline=true>"required"</Code>" under "
                            <Code inline=true>"ValidationBehavior::Native"</Code>", with "<Code inline=true>"aria-required"</Code>
                            " under "<Code inline=true>"Aria"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the field invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<T>>>" default="None">"Validates the committed value."</ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown. "<Code inline=true>"None"</Code>": the behavior of the surrounding "
                            <Link href=routes::doc::Form.materialize()>"Form"</Link>", else "<Code inline=true>"Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "The hidden input\u{2019}s name, submitting the value with a form (also matching server errors)."
                        </ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form, if the field isn\u{2019}t inside it."</ApiRow>
                        <ApiRow name="placeholder" ty="MaybeProp<String>" default="None">"The input\u{2019}s placeholder."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the input when it mounts."</ApiRow>
                        <ApiRow name="is_wheel_disabled" ty="bool" default="false">"Leave the value alone on scroll."</ApiRow>
                        <ApiRow name="increment_aria_label, decrement_aria_label" ty="MaybeProp<String>" default="None">
                            "Replace the stepper buttons\u{2019} names (\u{201c}Increase \u{2026}\u{201d}, \u{201c}Decrease \u{2026}\u{201d})."
                        </ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The input\u{2019}s id."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Labels the field when it has no "<Code inline=true>"Label"</Code>"."
                        </ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">
                            "Further labelling and describing elements."
                        </ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the input gains or loses focus."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the field\u{2019}s "<Code inline=true>"<div>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"Required. The parts, and any other content."</ApiRow>
                    </ApiTable>
                    <p>
                        "See "<Link href=routes::doc::number_field::Hook.materialize()>"Number Field Hooks"</Link>
                        " for how these settings behave and which types "<Code inline=true>"T"</Code>" can be."
                    </p>
                </Section>
            </Section>

            <Section title="NumberFieldGroup">
                <p>
                    "A "<Code inline=true>"<div role=\"group\">"</Code>" around the input and the stepper buttons. Draw the "
                    "field\u{2019}s border on it, and its focus ring with "<Code inline=true>"data-focus-visible"</Code>"."
                </p>
                <Section title="Props" id="number-field-group-props">
                    <ApiTable kind=ApiKind::Props of="NumberFieldGroup">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group."</ApiRow>
                        <ApiRow name="children" ty="Children">"Required. The input and the stepper buttons."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="NumberFieldDecrementButton, NumberFieldIncrementButton">
                <p>
                    "The stepper "<Code inline=true>"<button>"</Code>"s, named \u{201c}Decrease\u{201d} and \u{201c}Increase\u{201d} "
                    "with the field\u{2019}s label. They aren\u{2019}t in the tab order (the arrow keys step from the input), "
                    "keep focus in the input, step repeatedly while held, and are disabled at the limits."
                </p>
                <Section title="Props" id="number-field-button-props">
                    <ApiTable kind=ApiKind::Props of="NumberFieldDecrementButton">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the button."</ApiRow>
                        <ApiRow name="children" ty="Children">"Required. The button\u{2019}s content, e.g. \u{201c}\u{2212}\u{201d}."</ApiRow>
                    </ApiTable>
                    <ApiTable kind=ApiKind::Props of="NumberFieldIncrementButton">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the button."</ApiRow>
                        <ApiRow name="children" ty="Children">"Required. The button\u{2019}s content, e.g. \u{201c}+\u{201d}."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>
                    "Set to "<Code inline=true>"true"</Code>" while the state applies. The input renders the data attributes "
                    "of the "<Link href=format!("{}#input-data-attributes", routes::doc::text_field::Atom.materialize())>"Input"</Link>"."
                </p>
                <Section title="NumberField" id="number-field-data-attributes">
                    <p>"On the field\u{2019}s "<Code inline=true>"<div>"</Code>":"</p>
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-disabled" ty="true">"The field is disabled."</ApiRow>
                        <ApiRow name="data-readonly" ty="true">"The field is read-only."</ApiRow>
                        <ApiRow name="data-required" ty="true">"The field is required."</ApiRow>
                        <ApiRow name="data-invalid" ty="true">"The value is invalid."</ApiRow>
                    </ApiTable>
                </Section>
                <Section title="NumberFieldGroup" id="number-field-group-data-attributes">
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-hovered" ty="true">"A mouse or pen is over the group."</ApiRow>
                        <ApiRow name="data-focus-within" ty="true">"The input has focus."</ApiRow>
                        <ApiRow name="data-focus-visible" ty="true">"The input has keyboard focus: show a focus ring."</ApiRow>
                        <ApiRow name="data-disabled" ty="true">"The field is disabled."</ApiRow>
                        <ApiRow name="data-invalid" ty="true">"The value is invalid."</ApiRow>
                    </ApiTable>
                </Section>
                <Section title="NumberFieldDecrementButton, NumberFieldIncrementButton" id="number-field-button-data-attributes">
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-pressed" ty="true">"The button is pressed."</ApiRow>
                        <ApiRow name="data-hovered" ty="true">"A mouse or pen is over the button."</ApiRow>
                        <ApiRow name="data-disabled" ty="true">"The button is disabled (at a limit, or with the field)."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Styling">
                <p>
                    "Draw the field\u{2019}s border and focus ring on the group, which knows whether the input has keyboard "
                    "focus, and leave the input inside it plain:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-group { display: inline-flex; border: 1px solid var(--border); border-radius: 4px; }
                        .my-group[data-hovered] { border-color: var(--accent); }
                        .my-group[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 2px; }
                        .my-group[data-invalid] { border-color: var(--danger); }
                        .my-input { border: none; background: transparent; }
                        .my-stepper[data-pressed] { background: var(--surface); }
                        .my-stepper[data-disabled] { opacity: 0.5; }
                    ")}
                </Code>
                <p>"The demo shows its complete styles under \u{201c}View styles\u{201d}."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::NumberField.materialize()>"Number Field overview"</Link></li>
                <li><Link href=routes::doc::number_field::Hook.materialize()>"Number Field Hooks"</Link></li>
                <li><Link href=routes::doc::text_field::Atom.materialize()>"Text Field Atoms"</Link></li>
                <li><Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link></li>
                <li><Link href=routes::doc::Form.materialize()>"Form"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
