use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::text_field::TextFieldConceptDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageTextFieldOverview() -> impl IntoView {
    view! {
        <DocPage title="Text Field">
            <p>
                "Text fields let people type text: a name, an email address, a password or a message, on one line or "
                "several. The input type makes a text field an email, URL, phone number or password field. A field "
                "connects its input with a label, a description and validation errors, so that screen readers announce "
                "them together and pressing the label focuses the input."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Capture free-form text, on one line or several"</TableCell>
                        <TableCell><b>"Text Field"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Enter a password (masked)"</TableCell>
                        <TableCell><b>"Text Field"</b>" with "<Code inline=true>"input_type=InputType::Password"</Code></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Search with submit on Enter and a clear button"</TableCell>
                        <TableCell><Link href=routes::doc::SearchField.materialize()>"Search Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Enter a precise number"</TableCell>
                        <TableCell><Link href=routes::doc::NumberField.materialize()>"Number Field"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick a number from a range by dragging"</TableCell>
                        <TableCell><Link href=routes::doc::Slider.materialize()>"Slider"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Choose from predefined options"</TableCell>
                        <TableCell><Link href=routes::doc::Select.materialize()>"Select"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Search and filter a list of options"</TableCell>
                        <TableCell><Link href=routes::doc::Combobox.materialize()>"Combobox"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::text_field::Hook.materialize()>"Text Field Hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_text_field_state"</Code>" and "<Code inline=true>"use_text_field"</Code>
                            ": the value, label, description and error message wiring and validation for an input or text "
                            "area you render."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::text_field::Atom.materialize()>"Text Field Atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"TextField"</Code>", composed from an "<Code inline=true>"Input"</Code>
                            " (or "<Code inline=true>"TextArea"</Code>") and the "
                            <Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>" "
                            <Code inline=true>"Label"</Code>", "<Code inline=true>"Description"</Code>" and "
                            <Code inline=true>"FieldError"</Code>", styled through data attributes. Put them in a "
                            <Link href=routes::doc::Form.materialize()>"Form"</Link>" to validate on submission."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::text_field::Component.materialize()>"Text Field Component"</Link></TableCell>
                        <TableCell>"A themed text field taking its label, description and validation as props."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The themed "<Code inline=true>"TextField"</Code>" takes its label and description as props. Pass it a "
                    "signal as "<Code inline=true>"value"</Code>" and its setter as "<Code inline=true>"set_value"</Code>":"
                </p>

                <Demo description="Labelled name field greeting the entered name" source=include_str!("demos/text_field.rs") source_open=true>
                    <TextFieldConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "Text fields render native "<Code inline=true>"<input>"</Code>" (or "<Code inline=true>"<textarea>"</Code>
                        ") elements with a "<Code inline=true>"<label>"</Code>", so they need no extra keyboard handling."
                    </li>
                    <li>
                        "The hook connects label, description and error message with "<Code inline=true>"aria-labelledby"</Code>
                        " and "<Code inline=true>"aria-describedby"</Code>", sets "<Code inline=true>"aria-invalid"</Code>
                        " while the value is invalid, and marks a required field with "<Code inline=true>"aria-required"</Code>
                        " (or the native "<Code inline=true>"required"</Code>" when the browser validates the form)."
                    </li>
                    <li>"A field without a visible label needs an "<Code inline=true>"aria_label"</Code>"."</li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::text_field::Hook.materialize()>"Text Field Hooks"</Link></li>
                <li><Link href=routes::doc::text_field::Atom.materialize()>"Text Field Atoms"</Link></li>
                <li><Link href=routes::doc::text_field::Component.materialize()>"Text Field Component"</Link></li>
                <li><Link href=routes::doc::SearchField.materialize()>"Search Field"</Link></li>
                <li><Link href=routes::doc::NumberField.materialize()>"Number Field"</Link></li>
                <li><Link href=routes::doc::Field.materialize()>"Field"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
