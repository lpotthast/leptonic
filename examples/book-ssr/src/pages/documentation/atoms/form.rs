use indoc::indoc;
use leptos::prelude::*;

use super::demos::form::FormAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomForm() -> impl IntoView {
    view! {
        <DocPage title="Form Atom">
            <p>
                <Code inline=true>"Form"</Code>" renders an unstyled "<Code inline=true>"<form>"</Code>" whose fields share "
                "one validation behavior and show the validation errors your server returns. See the "
                <Link href=routes::doc::Form.materialize()>"Form overview"</Link>"."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"Form"</Code>" calls no hook of its own. It provides the contexts its fields read:"
                </p>
                <ul>
                    <li>
                        "a "<Link href=format!("{}#formvalidationcontext", routes::doc::form::Hook.materialize())>
                            <Code inline=true>"FormValidationContext"</Code>
                        </Link>" with its "<Code inline=true>"validation_errors"</Code>", read by "
                        <Link href=format!("{}#use-form-validation-state", routes::doc::form::Hook.materialize())>
                            "use_form_validation_state"
                        </Link>" in every field hook,"
                    </li>
                    <li>
                        "a "<AnchorLink href="#formcontext">"FormContext"</AnchorLink>" with its "
                        <Code inline=true>"validation_behavior"</Code>", read by the field atoms."
                    </li>
                </ul>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="atoms::form::Form">
                    <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Native">
                        "When the fields inside show their errors (see "
                        <AnchorLink href="#validation-behavior">"Validation Behavior"</AnchorLink>"). A field\u{2019}s own "
                        <Code inline=true>"validation_behavior"</Code>" takes precedence."
                    </ApiRow>
                    <ApiRow name="validation_errors" ty="Option<Signal<HashMap<String, Vec<String>>>>" default="None">
                        "Errors from the server, by field "<Code inline=true>"name"</Code>". A field shows its errors until the "
                        "user commits a changed value."
                    </ApiRow>
                    <ApiRow name="id" ty="Option<String>" default="None">
                        "The form\u{2019}s id, for fields outside of it (their "<Code inline=true>"form"</Code>" prop)."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<form>"</Code>"."</ApiRow>
                    <ApiRow name="children" ty="Children">"The fields, buttons and any other content. Required."</ApiRow>
                </ApiTable>
                <p>
                    "Listeners and attributes you set on the atom ("<Code inline=true>"on:submit"</Code>", "
                    <Code inline=true>"attr:action"</Code>", "<Code inline=true>"attr:aria-label"</Code>", \u{2026}) land on the "
                    <Code inline=true>"<form>"</Code>"."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::{
                                button::Button,
                                field::{FieldError, Label},
                                form::Form,
                                input::Input,
                                text_field::TextField,
                            },
                            hooks::{ButtonType, InputType},
                        };
                        use leptos::{ev::SubmitEvent, prelude::*};

                        view! {
                            <Form on:submit=move |e: SubmitEvent| {
                                e.prevent_default();
                                // Send the form data.
                            }>
                                <TextField name="email" input_type=InputType::Email is_required=true>
                                    <Label>"Email"</Label>
                                    <Input/>
                                    <FieldError/>
                                </TextField>
                                <Button button_type=ButtonType::Submit>"Subscribe"</Button>
                            </Form>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Submit the empty form, or an invalid address: the browser blocks the submission, the error appears below "
                    "the field and focus moves to it. Once you commit a corrected value (leave the field), the error clears. "
                    "\u{201c}Simulate server error\u{201d} reports an error for the "<Code inline=true>"email"</Code>
                    " field; it disappears once you change the value and leave the field."
                </p>
                <Demo
                    description="Newsletter form with a required email text field, a description, an error message, submit, reset and a simulated server error"
                    source=include_str!("demos/form.rs")
                >
                    <FormAtomDemo/>
                </Demo>
            </Section>

            <Section title="Validation Behavior">
                <DocTable headers=&["ValidationBehavior", "Errors are shown"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Native"</Code>" (default)"</TableCell>
                        <TableCell>
                            "When the form is submitted. The fields use the browser\u{2019}s constraint validation, so an invalid "
                            "form isn\u{2019}t submitted, and focus moves to the first invalid field. An error clears once the "
                            "edited value is committed (on "<Code inline=true>"change"</Code>" or blur)."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Aria"</Code></TableCell>
                        <TableCell>
                            "As the user edits. The form gets "<Code inline=true>"novalidate"</Code>", so the browser submits "
                            "it regardless; check the fields yourself before sending the data."
                        </TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "Field atoms outside a "<Code inline=true>"Form"</Code>" without their own "
                    <Code inline=true>"validation_behavior"</Code>" use "<Code inline=true>"Native"</Code>"."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The form renders no data attributes. Give it classes, and style its fields through their own "
                    "attributes (see "<Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>")."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    "Fields you build from hooks take part in the form like the field atoms: their "
                    <Link href=routes::doc::form::Hook.materialize()>"Form Hooks"</Link>" read the server errors, and they "
                    "can follow the form\u{2019}s validation behavior by reading its "<Code inline=true>"FormContext"</Code>"."
                </p>

                <Section title="FormContext">
                    <p>
                        "Read it in your own fields with "<Code inline=true>"use_context::<FormContext>()"</Code>
                        " (from "<Code inline=true>"leptonic::atoms::form"</Code>")."
                    </p>
                    <ApiTable kind=ApiKind::Fields of="FormContext">
                        <ApiRow name="validation_behavior" ty="ValidationBehavior">"The form\u{2019}s validation behavior."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Form.materialize()>"Form overview"</Link></li>
                <li><Link href=routes::doc::form::Hook.materialize()>"Form Hooks"</Link></li>
                <li><Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link></li>
                <li><Link href=routes::doc::text_field::Atom.materialize()>"Text Field Atoms"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
