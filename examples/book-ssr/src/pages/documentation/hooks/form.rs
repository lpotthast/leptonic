use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::form_coupon::FormCouponDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageFormHooks() -> impl IntoView {
    view! {
        <DocPage title="Form Hooks">
            <p>
                "The form hooks give a field you build yourself the validation and reset behavior of leptonic\u{2019}s "
                "fields, so that it takes part in a "<Code inline=true>"<form>"</Code>" like they do. See the "
                <Link href=routes::doc::Form.materialize()>"Form overview"</Link>"."
            </p>
            <p>
                "leptonic\u{2019}s field hooks ("<Link href=routes::doc::text_field::Hook.materialize()><Code inline=true>"use_text_field"</Code></Link>", "
                <Link href=routes::doc::checkbox::Hook.materialize()><Code inline=true>"use_checkbox"</Code></Link>", \u{2026}) "
                "call them for you. A form itself needs no hook: a plain "<Code inline=true>"<form>"</Code>" works, and a "
                <AnchorLink href="#formvalidationcontext">"FormValidationContext"</AnchorLink>" around it passes server "
                "errors to its fields."
            </p>

            <Section title="Example">
                <p>
                    "Validate the value with "<AnchorLink href="#use-form-validation-state">"use_form_validation_state"</AnchorLink>
                    ", connect the result to the input with "<AnchorLink href="#use-form-validation">"use_form_validation"</AnchorLink>
                    ", restore the value on reset with "<AnchorLink href="#use-form-reset">"use_form_reset"</AnchorLink>", and "
                    "label the field with "<Link href=format!("{}#use-field", routes::doc::field::Hook.materialize())>"use_field"</Link>":"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use std::sync::Arc;

                        use leptonic::{hooks::*, utils::CapturedElement};
                        use leptos::prelude::*;

                        let code = RwSignal::new(String::new());
                        let element = CapturedElement::new();

                        let validation = use_form_validation_state(UseFormValidationStateInput {
                            is_invalid: false.into(),
                            value: code.into(),
                            validate: Some(Arc::new(|code: &String| {
                                if code.len() == 8 { Ok(()) } else { Err(vec!["Use eight characters.".to_owned()]) }
                            })),
                            validation_behavior: ValidationBehavior::Native,
                            name: Some("coupon".to_owned()),
                        });
                        use_form_validation(UseFormValidationInput::new(element, validation, ValidationBehavior::Native));
                        use_form_reset(UseFormResetInput {
                            element,
                            initial_value: code.get_untracked(),
                            on_reset: Callback::new(move |initial| code.set(initial)),
                        });

                        view! {
                            <input
                                name="coupon"
                                prop:value=code
                                on:input=move |e| code.set(event_target_value(&e))
                                aria-invalid=move || validation.is_invalid.get().then_some("true")
                                {..element.attr()}
                            />
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Submit the empty form, or a code that isn\u{2019}t eight capital letters or digits: the browser blocks "
                    "the submission, the error appears and focus moves to the field. The error clears once you commit a "
                    "corrected value (leave the field). \u{201c}Simulate server error\u{201d} reports an error for the "
                    <Code inline=true>"coupon"</Code>" field until you change the value and leave the field; \u{201c}Reset\u{201d} restores the "
                    "empty field and hides its errors."
                </p>
                <Demo
                    description="Coupon code field built from the form hooks in a plain form, with submit, reset and a simulated server error"
                    source=include_str!("demos/form_coupon.rs")
                >
                    <FormCouponDemo/>
                </Demo>
            </Section>

            <Section title="use_form_validation_state">
                <p>
                    "Combines the validation sources of a value into the result the field shows. The first source reporting "
                    "an error wins: "<Code inline=true>"is_invalid"</Code>", then the server errors for the field\u{2019}s "
                    <Code inline=true>"name"</Code>", then "<Code inline=true>"validate"</Code>", then the browser\u{2019}s "
                    "constraint validation (read by "<Code inline=true>"use_form_validation"</Code>"). See "
                    <Link href=format!("{}#validation-sources", routes::doc::Forms.materialize())>"Validation Sources"</Link>"."
                </p>

                <Section title="Input" id="use-form-validation-state-input">
                    <p>
                        <Code inline=true>"UseFormValidationStateInput<T>"</Code>" has no "<Code inline=true>"Default"</Code>
                        ": set every field."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseFormValidationStateInput">
                        <ApiRow name="is_invalid" ty="Signal<bool>">
                            "Marks the field invalid while "<Code inline=true>"true"</Code>", before all other sources; while "
                            <Code inline=true>"false"</Code>", they decide. Required."
                        </ApiRow>
                        <ApiRow name="value" ty="Signal<T>">"The value "<Code inline=true>"validate"</Code>" checks. Required."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<T>>">
                            "Checks the value: "<Code inline=true>"Ok(())"</Code>" when valid, else "
                            <Code inline=true>"Err"</Code>" with the error messages. "<Code inline=true>"ValidateFn<T>"</Code>" is an "
                            <Code inline=true>"Arc<dyn Fn(&T) -> Result<(), Vec<String>>>"</Code>". Required ("<Code inline=true>"None"</Code>" for no check)."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior">
                            <Code inline=true>"Aria"</Code>" shows every error as the value changes; "<Code inline=true>"Native"</Code>
                            " shows the errors of "<Code inline=true>"validate"</Code>" and the browser once they are committed (on "
                            <Code inline=true>"change"</Code>" or submission). Required."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>">
                            "The field\u{2019}s "<Code inline=true>"name"</Code>", which selects its server errors. Required ("<Code inline=true>"None"</Code>" for no server errors)."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-form-validation-state-return">
                    <p>"All fields are "<Code inline=true>"Copy"</Code>"."</p>
                    <ApiTable kind=ApiKind::Return of="UseFormValidationStateReturn">
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the shown result is invalid. Set "<Code inline=true>"aria-invalid"</Code>" from it."</ApiRow>
                        <ApiRow name="validation_errors" ty="Signal<Vec<String>>">"The error messages of the shown result."</ApiRow>
                        <ApiRow name="display_validation" ty="Signal<ValidationResult>">
                            "The result the field shows, including which constraints fail."
                        </ApiRow>
                        <ApiRow name="realtime_validation" ty="Signal<ValidationResult>">
                            "The result for the current value, before it is committed. "<Code inline=true>"use_form_validation"</Code>
                            " passes it to the browser."
                        </ApiRow>
                        <ApiRow name="update_validation" ty="Callback<ValidationResult>">
                            "Reports a further result, such as the input\u{2019}s native validity. Shown right away with "
                            <Code inline=true>"Aria"</Code>", at the next commit with "<Code inline=true>"Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="commit_validation" ty="Callback<()>">
                            "Shows the current result and clears the server errors (the user changed the value)."
                        </ApiRow>
                        <ApiRow name="reset_validation" ty="Callback<()>">"Shows the field as valid again, e.g. when its form is reset."</ApiRow>
                        <ApiRow name="native_validity_readers" ty="NativeValidityReaders">
                            "Used by "<Code inline=true>"use_form_validation"</Code>" to read the input\u{2019}s native validity "
                            "before each commit."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="ValidationResult">
                    <ApiTable kind=ApiKind::Fields of="ValidationResult">
                        <ApiRow name="is_invalid" ty="bool">"Whether the value is invalid."</ApiRow>
                        <ApiRow name="validation_errors" ty="Vec<String>">"The error messages."</ApiRow>
                        <ApiRow name="validation_details" ty="ValidityStateSnapshot">
                            "Which constraints fail ("<Code inline=true>"value_missing"</Code>", "
                            <Code inline=true>"type_mismatch"</Code>", "<Code inline=true>"custom_error"</Code>", \u{2026}), "
                            "a snapshot of the browser\u{2019}s "<Code inline=true>"ValidityState"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_form_validation">
                <ReactAriaSource path="form/useFormValidation.ts"/>
                <p>
                    "Connects the validation state to the "<Code inline=true>"<input>"</Code>", "
                    <Code inline=true>"<textarea>"</Code>" or "<Code inline=true>"<select>"</Code>" holding the value. It "
                    "returns nothing."
                </p>
                <ul>
                    <li>
                        "With "<Code inline=true>"Native"</Code>", it passes the errors to the browser ("
                        <Code inline=true>"setCustomValidity"</Code>"), which then blocks the form\u{2019}s submission, and "
                        "reads the input\u{2019}s own constraints ("<Code inline=true>"required"</Code>", "
                        <Code inline=true>"type"</Code>", "<Code inline=true>"pattern"</Code>", \u{2026}) back."
                    </li>
                    <li>
                        "It commits the validation on "<Code inline=true>"change"</Code>" and when a submission finds the "
                        "field invalid, and then focuses the form\u{2019}s first invalid field."
                    </li>
                    <li>"It resets the validation when the form is reset."</li>
                </ul>

                <Section title="Input" id="use-form-validation-input">
                    <p>
                        "Create it with "<Code inline=true>"UseFormValidationInput::new(element, state, validation_behavior)"</Code>
                        ", which sets "<Code inline=true>"focus"</Code>" to "<Code inline=true>"None"</Code>"."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseFormValidationInput">
                        <ApiRow name="element" ty="CapturedElement">
                            "The "<Code inline=true>"<input>"</Code>", "<Code inline=true>"<textarea>"</Code>" or "
                            <Code inline=true>"<select>"</Code>" holding the value. Required."
                        </ApiRow>
                        <ApiRow name="state" ty="UseFormValidationStateReturn">
                            "The result of "<Code inline=true>"use_form_validation_state"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior">"The same behavior as the state\u{2019}s. Required."</ApiRow>
                        <ApiRow name="focus" ty="Option<Callback<()>>" default="None">
                            "Focuses the field when it is the form\u{2019}s first invalid field, for fields whose focusable "
                            "element isn\u{2019}t the validated one. "<Code inline=true>"None"</Code>" focuses "
                            <Code inline=true>"element"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_form_reset">
                <ReactAriaSource path="utils/useFormReset.ts"/>
                <p>
                    "Calls "<Code inline=true>"on_reset"</Code>" with the initial value when the element\u{2019}s form is "
                    "reset, so that the field restores its value with the form\u{2019}s native inputs. It returns nothing."
                </p>

                <Section title="Input" id="use-form-reset-input">
                    <p><Code inline=true>"UseFormResetInput<T>"</Code>" has no "<Code inline=true>"Default"</Code>": set every field."</p>
                    <ApiTable kind=ApiKind::Input of="UseFormResetInput">
                        <ApiRow name="element" ty="CapturedElement">
                            "An element inside the form (an "<Code inline=true>"<input>"</Code>", "
                            <Code inline=true>"<textarea>"</Code>" or "<Code inline=true>"<select>"</Code>"), to find it. Required."
                        </ApiRow>
                        <ApiRow name="initial_value" ty="T">"The value to restore. Required."</ApiRow>
                        <ApiRow name="on_reset" ty="Callback<T>">"Receives "<Code inline=true>"initial_value"</Code>" when the form is reset. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="FormValidationContext">
                <p>
                    "Provide it around a form to show the errors your server returned: fields built with "
                    <Code inline=true>"use_form_validation_state"</Code>" show the errors listed for their "
                    <Code inline=true>"name"</Code>" until the user commits a changed value. The "
                    <Link href=routes::doc::form::Atom.materialize()>"Form Atom"</Link>" provides it from its "
                    <Code inline=true>"validation_errors"</Code>". Provide it before the fields\u{2019} hooks run, e.g. with "
                    <Code inline=true>"provide_context"</Code>" in the component rendering the form."
                </p>
                <ApiTable kind=ApiKind::Fields of="FormValidationContext">
                    <ApiRow name="errors" ty="Signal<HashMap<String, Vec<String>>>">"The error messages, by field name."</ApiRow>
                </ApiTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Form.materialize()>"Form overview"</Link></li>
                <li><Link href=routes::doc::form::Atom.materialize()>"Form Atom"</Link></li>
                <li><Link href=routes::doc::field::Hook.materialize()>"Field Hooks"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
