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
                "Text fields let people type a value: a name, an email address, a password, a search query or a number. "
                "A field connects its input with a label, a description and validation errors, so that screen readers "
                "announce them together and pressing the label focuses the input."
            </p>

            <p>
                "There are three kinds of field. A "<b>"text field"</b>" takes any text, on one line or several; its input "
                "type makes it an email, URL, phone number or password field. A "<b>"search field"</b>" submits its query "
                "on Enter and clears on Escape. A "<b>"number field"</b>" parses and formats numbers for the user\u{2019}s "
                "locale, keeps them in a range and steps them with buttons or the arrow keys. Each kind exists as a hook, "
                "as atoms and as a themed component."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Capture free-form text, on one line or several"</TableCell>
                        <TableCell><b>"TextField"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Enter a password (masked)"</TableCell>
                        <TableCell><b>"TextField"</b>" with "<Code inline=true>"input_type=InputType::Password"</Code></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Search with submit on Enter and a clear button"</TableCell>
                        <TableCell><b>"SearchField"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Enter a precise number"</TableCell>
                        <TableCell><b>"NumberField"</b></TableCell>
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
                        <TableCell><Link href=routes::doc::text_field::Hook.materialize()>"use_text_field & use_search_field"</Link></TableCell>
                        <TableCell>
                            "Label, description and error message wiring, validation and the search keys for inputs you "
                            "render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::text_field::NumberFieldHook.materialize()>"use_number_field"</Link></TableCell>
                        <TableCell>
                            "A locale-aware number input with stepper buttons, arrow-key and scroll wheel stepping, built on "
                            <Link href=routes::doc::hooks::UseSpinButton.materialize()>"use_spin_button"</Link>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::text_field::Atom.materialize()>"Text Field atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"TextField"</Code>" and "<Code inline=true>"SearchField"</Code>", composed "
                            "from an "<Code inline=true>"Input"</Code>" (or "<Code inline=true>"TextArea"</Code>") and the "
                            <Link href=routes::doc::atoms::Field.materialize()>"field atoms"</Link>" "
                            <Code inline=true>"Label"</Code>", "<Code inline=true>"Description"</Code>" and "
                            <Code inline=true>"FieldError"</Code>", styled through data attributes. Put them in a "
                            <Link href=routes::doc::atoms::Form.materialize()>"Form"</Link>" to validate on submission."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::text_field::NumberFieldAtom.materialize()>"Number Field atoms"</Link></TableCell>
                        <TableCell>
                            "An unstyled "<Code inline=true>"NumberField"</Code>" for any integer or float type, with stepper "
                            "buttons, an "<Code inline=true>"Input"</Code>" and the field atoms."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::text_field::Component.materialize()>"Text Field components"</Link></TableCell>
                        <TableCell>
                            "Themed "<Code inline=true>"TextField"</Code>", "<Code inline=true>"SearchField"</Code>" and "
                            <Code inline=true>"NumberField"</Code>", taking label, description and validation as props."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The themed "<Code inline=true>"TextField"</Code>" takes its label and description as props. Bind its "
                    "value to a signal with "<Code inline=true>"state"</Code>":"
                </p>

                <Demo description="Labelled name field greeting the entered name" source=include_str!("demos/text_field.rs") source_open=true>
                    <TextFieldConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "All fields render native "<Code inline=true>"<input>"</Code>" (or "<Code inline=true>"<textarea>"</Code>
                        ") elements with a "<Code inline=true>"<label>"</Code>". A search field\u{2019}s input has "
                        <Code inline=true>"type=\"search\""</Code>"; a number field\u{2019}s is a text input, so that it can "
                        "show formatted numbers, with a virtual keyboard for numbers."
                    </li>
                    <li>
                        "The hooks connect label, description and error message with "<Code inline=true>"aria-labelledby"</Code>
                        " and "<Code inline=true>"aria-describedby"</Code>", and set "<Code inline=true>"aria-invalid"</Code>
                        " and "<Code inline=true>"aria-required"</Code>"."
                    </li>
                    <li>
                        "A number field sets "<Code inline=true>"aria-roledescription=\"Number field\""</Code>
                        " and announces value changes to screen readers."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Enter">"Search field: submit the query. Number field: commit the typed value."</KeyRow>
                    <KeyRow keys="Escape">"Search field: clear the query."</KeyRow>
                    <KeyRow keys="ArrowUp / ArrowDown">"Number field: increment or decrement by one step."</KeyRow>
                    <KeyRow keys="Home / End">"Number field: set to the minimum or maximum."</KeyRow>
                </KeyboardTable>
            </Section>
        </DocPage>
    }
}
