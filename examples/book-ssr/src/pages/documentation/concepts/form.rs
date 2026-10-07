use leptos::prelude::*;

use super::demos::form::FormConceptDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageFormOverview() -> impl IntoView {
    view! {
        <DocPage title="Form">
            <p>
                "A form collects the values of several fields and submits them together: a sign-up, a checkout, the "
                "settings of an account. Before the values leave, each field checks its own, and an invalid form stays "
                "put with its errors shown next to the fields that need fixing."
            </p>
            <p>
                "leptonic\u{2019}s fields validate themselves; the form decides when they show their errors (as the user "
                "types, or once the form is submitted) and passes the errors your server returns on to the fields they "
                "belong to. It uses the browser\u{2019}s own form handling, so a form submits with "<Keys keys="Enter"/>
                " and resets with a reset button."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Submit several values together, validated first"</TableCell>
                        <TableCell><b>"Form"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Label a single control and show its errors"</TableCell>
                        <TableCell><Link href=routes::doc::Field.materialize()>"Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Run a search query as the user submits it"</TableCell>
                        <TableCell><Link href=routes::doc::SearchField.materialize()>"Search Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Change a setting with immediate effect, without submitting"</TableCell>
                        <TableCell><Link href=routes::doc::Switch.materialize()>"Switch"</Link></TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "See "<Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link>" for how fields validate, "
                    "which sources of errors they combine and how a form is reset."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link>
                    " for how the layers relate."
                </p>
                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::form::Hook.materialize()>"Form Hooks"</Link></TableCell>
                        <TableCell>
                            "The validation and reset behavior of leptonic\u{2019}s fields, for fields you build yourself, and "
                            "the context that passes server errors to them. The form is a plain "<Code inline=true>"<form>"</Code>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::form::Atom.materialize()>"Form Atom"</Link></TableCell>
                        <TableCell>
                            "An unstyled "<Code inline=true>"<form>"</Code>" that sets one validation behavior for the field "
                            "atoms inside it and shows the errors your server returns."
                        </TableCell>
                    </TableRow>
                </DocTable>
                <p>"A form has no look of its own: give the atom classes to lay out its fields."</p>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Put fields with a "<Code inline=true>"name"</Code>" and a submit button into the "
                    <Link href=routes::doc::form::Atom.materialize()>"Form Atom"</Link>". Submit the empty form to see the "
                    "errors:"
                </p>
                <Demo
                    description="Invitation form with two required text fields and a submit button"
                    source=include_str!("demos/form.rs")
                    source_open=true
                >
                    <FormConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "The form is a native "<Code inline=true>"<form>"</Code>". It becomes a landmark (role "
                        <Code inline=true>"form"</Code>") once it has a name: give forms that are a main part of a page an "
                        <Code inline=true>"attr:aria-label"</Code>" or "<Code inline=true>"attr:aria-labelledby"</Code>"."
                    </li>
                    <li>
                        "Each field marks itself invalid with "<Code inline=true>"aria-invalid"</Code>" and references its "
                        "error message with "<Code inline=true>"aria-describedby"</Code>", so screen readers announce the "
                        "error with the field (see "<Link href=routes::doc::Field.materialize()>"Field"</Link>")."
                    </li>
                    <li>
                        "With the default native validation, a submission that finds invalid fields moves focus to the "
                        "first of them and shows its focus ring. The browser\u{2019}s own error bubbles are suppressed: the "
                        "fields show the errors."
                    </li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="Tab / Shift + Tab">"Moves focus between the fields and buttons of the form."</KeyRow>
                    <KeyRow keys="Enter">"In a single-line text field: submits the form (when it has a submit button)."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::form::Hook.materialize()>"Form Hooks"</Link></li>
                <li><Link href=routes::doc::form::Atom.materialize()>"Form Atom"</Link></li>
                <li><Link href=routes::doc::Field.materialize()>"Field"</Link></li>
                <li><Link href=routes::doc::TextField.materialize()>"Text Field"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
