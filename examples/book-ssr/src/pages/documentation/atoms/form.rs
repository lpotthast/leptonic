use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::form::FormAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomForm() -> impl IntoView {
    view! {
        <DocPage title="Form Atom">
            <p>
                <Code inline=true>"Form"</Code>" renders a "<Code inline=true>"<form>"</Code>" whose field atoms share one "
                "validation behavior and show the validation errors your server returns. It is unstyled. See "
                <Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link>" for how fields validate."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"Form"</Code>" provides a "<Code inline=true>"FormValidationContext"</Code>" with the "
                    "server errors, which the field hooks read (see "
                    <Link href=format!("{}#server-errors", routes::doc::Forms.materialize())>"Server errors"</Link>
                    "), and a "<Code inline=true>"FormContext"</Code>" with the validation behavior, which the field atoms read."
                </p>
            </Section>

            <Section title="Example">
                <p>
                    "Attach listeners and attributes to the component; they land on the "<Code inline=true>"<form>"</Code>":"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::{
                            button::Button,
                            field::{FieldError, Label},
                            form::Form,
                            input::Input,
                            text_field::TextField,
                        };

                        view! {
                            <Form on:submit=move |e: SubmitEvent| { e.prevent_default(); save(); }>
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
                    " field; it disappears when you edit the value."
                </p>
                <Demo
                    description="Newsletter form with a required email text field, a description, an error message, submit, reset and a simulated server error"
                    source=include_str!("demos/form.rs")
                >
                    <FormAtomDemo/>
                </Demo>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="atoms::form::Form">
                    <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Native">
                        "When the fields inside show their errors. A field\u{2019}s own "<Code inline=true>"validation_behavior"</Code>
                        " takes precedence."
                    </ApiRow>
                    <ApiRow name="validation_errors" ty="Option<Signal<HashMap<String, Vec<String>>>>" default="None">
                        "Errors from the server, by field "<Code inline=true>"name"</Code>". A field shows its errors until its "
                        "value changes."
                    </ApiRow>
                    <ApiRow name="id" ty="Option<String>" default="None">
                        "The form\u{2019}s id, for fields outside of it (their "<Code inline=true>"form"</Code>" prop)."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<form>"</Code>"."</ApiRow>
                    <ApiRow name="children" ty="Children">"The fields, buttons and any other content."</ApiRow>
                </ApiTable>
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

            <Section title="FormContext">
                <p>
                    "The context the field atoms read their default validation behavior from. Read it in your own fields with "
                    <Code inline=true>"use_context::<FormContext>()"</Code>"."
                </p>
                <ApiTable kind=ApiKind::Fields of="FormContext">
                    <ApiRow name="validation_behavior" ty="ValidationBehavior">"The form\u{2019}s validation behavior."</ApiRow>
                </ApiTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
                <li><Link href=routes::doc::text_field::Atom.materialize()>"Text field atoms"</Link></li>
                <li><Link href=routes::doc::atoms::Field.materialize()>"Field atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
