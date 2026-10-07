use indoc::indoc;
use leptos::prelude::*;

use super::demos::forms::FormsDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageForms() -> impl IntoView {
    view! {
        <DocPage title="Forms & Validation">
            <p>
                "Leptonic\u{2019}s field hooks ("<Link href=routes::doc::text_field::Hook.materialize()>"text fields"</Link>", "
                <Link href=routes::doc::number_field::Hook.materialize()>"number fields"</Link>", "
                <Link href=routes::doc::checkbox::Hook.materialize()>"checkboxes"</Link>", "
                <Link href=routes::doc::radio::Hook.materialize()>"radio groups"</Link>", "
                <Link href=routes::doc::switch::Hook.materialize()>"switches"</Link>", \u{2026}) work inside native "
                <Code inline=true>"<form>"</Code>"s: they render "<Code inline=true>"name"</Code>", "<Code inline=true>"form"</Code>
                " and "<Code inline=true>"required"</Code>" attributes, restore their initial value when the form is reset, "
                "and validate their value, in realtime or when the form is submitted."
            </p>

            <Section title="Demo">
                <p>
                    "A "<Link href=routes::doc::Form.materialize()>"Form"</Link>" atom with two "
                    <Link href=routes::doc::text_field::Atom.materialize()>"TextField"</Link>"s, both required. The "
                    "username shows errors as you type; the email field waits for submission and uses the browser\u{2019}s "
                    "constraint validation (try submitting an invalid address). \u{201c}Simulate server error\u{201d} "
                    "reports an error for the username field, as a server action would; it shows until the value changes."
                </p>
                <Demo description="Form with realtime and native validation, server errors and reset" source=include_str!("demos/forms.rs")>
                    <FormsDemo/>
                </Demo>
            </Section>

            <Section title="Validating a Field">
                <p>
                    "Field hooks take a "<Code inline=true>"validate"</Code>" function: "<Code inline=true>"Ok(())"</Code>
                    " for a valid value, "<Code inline=true>"Err(messages)"</Code>" otherwise. The native constraints ("
                    <Code inline=true>"is_required"</Code>", "<Code inline=true>"min_length"</Code>", "<Code inline=true>"pattern"</Code>
                    ", the input type, \u{2026}) are checked by the browser. The hooks return the result as "
                    <Code inline=true>"is_invalid"</Code>" and "<Code inline=true>"validation_errors"</Code>", set "
                    <Code inline=true>"aria-invalid"</Code>", and reference the error message element ("
                    <Code inline=true>"error_message_props"</Code>") while you render it. Field atoms show the errors with a "
                    <Link href=routes::doc::field::Atom.materialize()>"FieldError"</Link>"."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use std::sync::Arc;

                        use leptonic::hooks::*;
                        use leptos::prelude::*;

                        let state = use_text_field_state(UseTextFieldStateInput::default());

                        let field = use_text_field(UseTextFieldInput {
                            state,
                            name: Some("username".to_owned()),
                            is_required: true.into(),
                            validate: Some(Arc::new(|value: &String| {
                                if value.chars().count() >= 3 { Ok(()) } else { Err(vec!["At least 3 characters.".to_owned()]) }
                            })),
                            validation_behavior: ValidationBehavior::Aria,
                            // Everything else off:
                            validation: None,
                            is_invalid: false.into(),
                            id: None,
                            element: TextFieldElement::Input,
                            input_type: Signal::stored(InputType::Text),
                            is_disabled: false.into(),
                            is_read_only: false.into(),
                            form: None,
                            placeholder: MaybeProp::default(),
                            pattern: None,
                            min_length: None,
                            max_length: None,
                            auto_complete: None,
                            auto_capitalize: None,
                            auto_correct: None,
                            spell_check: None,
                            input_mode: None,
                            enter_key_hint: None,
                            auto_focus: false,
                            exclude_from_tab_order: false,
                            label_id: None,
                            has_label: false.into(),
                            aria_label: MaybeProp::default(),
                            aria_labelledby: None,
                            aria_describedby: None,
                            aria_errormessage: None,
                            aria_activedescendant: Signal::stored(None),
                            aria_autocomplete: None,
                            aria_haspopup: None,
                            aria_controls: Signal::stored(None),
                            on_focus: None,
                            on_blur: None,
                            on_focus_change: None,
                            on_key_down: None,
                            on_key_up: None,
                            shortcuts: None,
                        });
                    "#)}
                </Code>
            </Section>

            <Section title="Validation Behavior">
                <p>
                    "The field hooks default to "<Code inline=true>"Aria"</Code>". The field atoms default to the behavior of "
                    "the surrounding "<Link href=routes::doc::Form.materialize()>"Form"</Link>" atom, else "
                    <Code inline=true>"Native"</Code>", which also moves focus to the first invalid field on submission."
                </p>
                <DocTable headers=&["ValidationBehavior", "Errors are shown"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Aria"</Code></TableCell>
                        <TableCell>
                            "As the user edits. Errors from "<Code inline=true>"validate"</Code>" only mark the field invalid "
                            "with ARIA and don\u{2019}t block submission (native constraints like "<Code inline=true>"required"</Code>
                            " still do)."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Native"</Code></TableCell>
                        <TableCell>
                            "When the value is committed or the form is submitted, and they clear once a corrected value is "
                            "committed. Errors are also passed to the browser ("<Code inline=true>"setCustomValidity"</Code>
                            "), which blocks submission of an invalid form."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Validation Sources">
                <p>"A field combines several sources; the first one reporting a result wins:"</p>
                <ol>
                    <li>
                        <b>"Explicit"</b>": the field\u{2019}s "<Code inline=true>"is_invalid"</Code>" input marks it invalid "
                        "while "<Code inline=true>"true"</Code>"; while "<Code inline=true>"false"</Code>", the next source decides."
                    </li>
                    <li><b>"Server"</b>": errors from a "<Code inline=true>"FormValidationContext"</Code>", matched by the field\u{2019}s "<Code inline=true>"name"</Code>"."</li>
                    <li><b>"Client"</b>": the "<Code inline=true>"validate"</Code>" function."</li>
                    <li><b>"Native"</b>": the browser\u{2019}s constraint validation."</li>
                </ol>
            </Section>

            <Section title="Server Errors">
                <p>
                    "Provide a "<Code inline=true>"FormValidationContext"</Code>" around the form and fill it with the errors a "
                    "server action returned, keyed by field name (the "<Link href=routes::doc::Form.materialize()>"Form"</Link>
                    " atom does this for its "<Code inline=true>"validation_errors"</Code>"):"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use std::collections::HashMap;

                        use leptonic::hooks::FormValidationContext;

                        let errors = RwSignal::new(HashMap::<String, Vec<String>>::new());

                        view! {
                            <Provider value=FormValidationContext { errors: errors.into() }>
                                <form>/* fields with `name`s */</form>
                            </Provider>
                        }

                        // After the server rejected the form:
                        errors.set(HashMap::from([("email".to_owned(), vec!["Email already in use.".to_owned()])]));
                    "#)}
                </Code>
            </Section>

            <Section title="Form Reset">
                <p>
                    "When a form is reset, every field restores its initial value (e.g. "<Code inline=true>"default_value"</Code>
                    " of "<Code inline=true>"use_text_field_state"</Code>") and its displayed validation."
                </p>
            </Section>

            <Section title="Building Your Own Fields">
                <p>"The field hooks are built from three hooks, which you can use for fields of your own:"</p>
                <DocTable headers=&["Hook", "Purpose"]>
                    <TableRow>
                        <TableCell><Code inline=true>"use_form_validation_state"</Code></TableCell>
                        <TableCell>
                            "Combines the validation sources for a value ("<Code inline=true>"value"</Code>", "
                            <Code inline=true>"validate"</Code>", "<Code inline=true>"validation_behavior"</Code>", "
                            <Code inline=true>"is_invalid"</Code>", "<Code inline=true>"name"</Code>") into the displayed result "
                            "and the result for the browser. "<Code inline=true>"commit_validation"</Code>" shows deferred errors, "
                            <Code inline=true>"reset_validation"</Code>" hides them."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"use_form_validation"</Code></TableCell>
                        <TableCell>
                            "Connects that state to a native input: sets its custom validity (native behavior), commits on "
                            <Code inline=true>"change"</Code>" and "<Code inline=true>"invalid"</Code>", resets on form reset."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"use_form_reset"</Code></TableCell>
                        <TableCell>"Calls "<Code inline=true>"on_reset"</Code>" with the initial value when the element\u{2019}s form is reset."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

        </DocPage>
    }
}
