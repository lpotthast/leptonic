use indoc::indoc;
use leptos::prelude::*;

use super::demos::text_field::TextFieldAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomTextField() -> impl IntoView {
    view! {
        <DocPage title="Text Field Atoms">
            <p>
                "The text field atoms render unstyled text fields: "<Code inline=true>"TextField"</Code>" holds the value and "
                "its validation, "<Code inline=true>"Input"</Code>" or "<Code inline=true>"TextArea"</Code>" renders the "
                "input, and the "<Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>" add a label, a "
                "description and an error message. See the "<Link href=routes::doc::TextField.materialize()>"Text Field overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"TextField"</Code>" calls "
                    <Link href=format!("{}#use-text-field-state", routes::doc::text_field::Hook.materialize())>"use_text_field_state"</Link>
                    " and "<Link href=format!("{}#use-text-field", routes::doc::text_field::Hook.materialize())>"use_text_field"</Link>". "
                    <Code inline=true>"Input"</Code>" and "<Code inline=true>"TextArea"</Code>" call "
                    <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>"."
                </p>
            </Section>

            <Section title="Example">
                <p>
                    "The input\u{2019}s attributes ("<Code inline=true>"placeholder"</Code>", "<Code inline=true>"input_type"</Code>
                    ", "<Code inline=true>"max_length"</Code>", \u{2026}) are props of the field; the "<Code inline=true>"Input"</Code>
                    " only renders them. Control the value with "<Code inline=true>"value"</Code>" and "
                    <Code inline=true>"set_value"</Code>" (e.g. both an "<Code inline=true>"RwSignal"</Code>" of your app), or "
                    "start from "<Code inline=true>"default_value"</Code>" and listen to "<Code inline=true>"on_change"</Code>":"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::{field::{Description, FieldError, Label}, input::Input, text_field::TextField};

                        let name = RwSignal::new(String::new());

                        view! {
                            <TextField value=name set_value=name is_required=true placeholder="Ferris">
                                <Label>"Name"</Label>
                                <Input/>
                                <Description>"Shown on your profile."</Description>
                                <FieldError/>
                            </TextField>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "A required display name showing its error as you type, and a bio in a "<Code inline=true>"TextArea"</Code>
                    " with a length limit. Both values live in signals of the demo."
                </p>
                <Demo
                    description="Display name input and bio text area controlled by signals, with validation, a character count and a disabled toggle"
                    source=include_str!("demos/text_field.rs")
                >
                    <TextFieldAtomDemo/>
                </Demo>
            </Section>

            <Section title="TextField">
                <p>
                    "Creates the field and renders a "<Code inline=true>"<div>"</Code>" around its children, which may contain "
                    "any markup besides the parts. Its "<Code inline=true>"Label"</Code>" is a "<Code inline=true>"<label>"</Code>
                    " for the input."
                </p>
                <Section title="Props" id="text-field-props">
                    <ApiTable kind=ApiKind::Props of="atoms::text_field::TextField">
                        <ApiRow name="default_value" ty="String" default="empty">
                            "The initial value, unless "<Code inline=true>"value"</Code>" is set. A form reset restores the value "
                            "the field started with."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<String>>" default="None">"Called with the value when it changes."</ApiRow>
                        <ApiRow name="value" ty="Option<Signal<String>>" default="None">
                            "The value (controlled): a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<String>>" default="None">
                            "Receives the new value: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="input_type" ty="Signal<InputType>" default="Text">
                            "The "<Code inline=true>"<input>"</Code>"\u{2019}s type: "<Code inline=true>"Text"</Code>", "
                            <Code inline=true>"Email"</Code>", "<Code inline=true>"Password"</Code>", \u{2026} Ignored by a "
                            <Code inline=true>"TextArea"</Code>"."
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
                        <ApiRow name="validate" ty="Option<ValidateFn<String>>" default="None">
                            "Validates the value, returning "<Code inline=true>"Ok(())"</Code>" or "<Code inline=true>"Err(messages)"</Code>"."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown. "<Code inline=true>"None"</Code>": the behavior of the surrounding "
                            <Link href=routes::doc::Form.materialize()>"Form"</Link>", else "<Code inline=true>"Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="name, form" ty="Option<String>" default="None">
                            "The form field name (also matching server errors) and the id of the form, if the field isn\u{2019}t inside it."
                        </ApiRow>
                        <ApiRow name="placeholder" ty="MaybeProp<String>" default="None">"The input\u{2019}s placeholder."</ApiRow>
                        <ApiRow name="pattern" ty="Option<String>" default="None">"A validation pattern (not for a "<Code inline=true>"TextArea"</Code>")."</ApiRow>
                        <ApiRow name="min_length, max_length" ty="Option<u32>" default="None">"Length constraints."</ApiRow>
                        <ApiRow name="auto_complete" ty="Option<String>" default="None">
                            "The "<Code inline=true>"autocomplete"</Code>" hint, e.g. "<Code inline=true>"\"email\""</Code>" or "
                            <Code inline=true>"\"off\""</Code>"."
                        </ApiRow>
                        <ApiRow name="auto_capitalize" ty="Option<AutoCapitalize>" default="None">"Automatic capitalization."</ApiRow>
                        <ApiRow name="auto_correct, spell_check" ty="Option<bool>" default="None">"Automatic correction and spell checking."</ApiRow>
                        <ApiRow name="input_mode" ty="Option<InputMode>" default="None">"Which virtual keyboard to show."</ApiRow>
                        <ApiRow name="enter_key_hint" ty="Option<EnterKeyHint>" default="None">"The label of the virtual keyboard\u{2019}s Enter key."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the input when it mounts."</ApiRow>
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
                </Section>
            </Section>

            <Section title="Input, TextArea">
                <p>
                    "The "<Code inline=true>"<input>"</Code>" or "<Code inline=true>"<textarea>"</Code>" of the field around it "
                    "(a "<Code inline=true>"TextField"</Code>", a "<Link href=routes::doc::search_field::Atom.materialize()>"SearchField"</Link>
                    ", a "<Link href=routes::doc::number_field::Atom.materialize()>"NumberField"</Link>" or a "
                    <Link href=routes::doc::combobox::Atom.materialize()>"ComboBox"</Link>"; a "<Code inline=true>"TextArea"</Code>
                    " needs a text field). They take only a node ref, classes and styles; everything else comes from the field."
                </p>
                <Section title="Props" id="input-props">
                    <ApiTable kind=ApiKind::Props of="atoms::input::Input">
                        <ApiRow name="node_ref" ty="NodeRef<Input>" default="unset">"The "<Code inline=true>"<input>"</Code>" element, e.g. to focus it from a shortcut."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<input>"</Code>"."</ApiRow>
                    </ApiTable>
                    <ApiTable kind=ApiKind::Props of="atoms::input::TextArea">
                        <ApiRow name="node_ref" ty="NodeRef<Textarea>" default="unset">"The "<Code inline=true>"<textarea>"</Code>" element, e.g. to focus it."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<textarea>"</Code>"."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>"Set to "<Code inline=true>"true"</Code>" while the state applies."</p>
                <Section title="TextField" id="text-field-data-attributes">
                    <p>"On the field\u{2019}s "<Code inline=true>"<div>"</Code>":"</p>
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-disabled" ty="true">"The field is disabled."</ApiRow>
                        <ApiRow name="data-invalid" ty="true">"The value is invalid."</ApiRow>
                        <ApiRow name="data-readonly" ty="true">"The field is read-only."</ApiRow>
                        <ApiRow name="data-required" ty="true">"The field is required."</ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Input, TextArea" id="input-data-attributes">
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-focused" ty="true">"The input has focus."</ApiRow>
                        <ApiRow name="data-focus-visible" ty="true">"The input has keyboard focus: show a focus ring."</ApiRow>
                        <ApiRow name="data-hovered" ty="true">"A mouse or pen is over the input."</ApiRow>
                        <ApiRow name="data-disabled" ty="true">"The field is disabled."</ApiRow>
                        <ApiRow name="data-invalid" ty="true">"The value is invalid."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Styling">
                <p>"Select the states with attribute selectors on your classes:"</p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-input { border: 1px solid var(--border); border-radius: 4px; }
                        .my-input[data-hovered] { border-color: var(--accent); }
                        .my-input[data-invalid] { border-color: var(--danger); }
                        .my-input[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 1px; }
                        .my-input[data-disabled] { opacity: 0.5; }
                    ")}
                </Code>
                <p>"The demo shows its complete styles under \u{201c}View styles\u{201d}."</p>
            </Section>

            <Section title="Composition">
                <p>
                    "A "<Code inline=true>"TextField"</Code>" accepts any markup between its parts, e.g. a row with the input "
                    "and a button, or a character count in the "<Code inline=true>"Description"</Code>" (as in the demo)."
                </p>
                <Section title="InputContext">
                    <p>
                        "The parts find the field\u{2019}s input through an "<Code inline=true>"InputContext"</Code>". A field you "
                        "build from hooks can provide one to use "<Code inline=true>"Input"</Code>" (and "
                        <Code inline=true>"TextArea"</Code>"), together with a "
                        <Link href=format!("{}#fieldcontext", routes::doc::field::Atom.materialize())>"FieldContext"</Link>
                        " and a "<Link href=format!("{}#labelcontext", routes::doc::field::Atom.materialize())>"LabelContext"</Link>
                        " for the label, description and error message:"
                    </p>
                    <DocTable headers=&["Constructor", "For"]>
                        <TableRow>
                            <TableCell><Code inline=true>"InputContext::text_field(input_props, state)"</Code></TableCell>
                            <TableCell>
                                "A field built with "<Code inline=true>"use_text_field"</Code>", from its "
                                <Code inline=true>"input_props"</Code>". It can render an "<Code inline=true>"Input"</Code>" or a "
                                <Code inline=true>"TextArea"</Code>"."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"InputContext::new(move || attrs, state)"</Code></TableCell>
                            <TableCell>"Any other input attributes, created for each rendered input."</TableCell>
                        </TableRow>
                    </DocTable>
                    <ApiTable kind=ApiKind::Fields of="InputContext">
                        <ApiRow name="state" ty="InputState">"The state the input shows in its data attributes."</ApiRow>
                    </ApiTable>
                    <Section title="InputState">
                        <ApiTable kind=ApiKind::Fields of="InputState">
                            <ApiRow name="is_disabled, is_invalid, is_focused, is_focus_visible" ty="Signal<bool>">
                                "From the field\u{2019}s hook; rendered as "<Code inline=true>"data-disabled"</Code>", "
                                <Code inline=true>"data-invalid"</Code>", "<Code inline=true>"data-focused"</Code>" and "
                                <Code inline=true>"data-focus-visible"</Code>"."
                            </ApiRow>
                        </ApiTable>
                    </Section>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::TextField.materialize()>"Text Field overview"</Link></li>
                <li><Link href=routes::doc::text_field::Hook.materialize()>"Text Field Hooks"</Link></li>
                <li><Link href=routes::doc::search_field::Atom.materialize()>"Search Field Atoms"</Link></li>
                <li><Link href=routes::doc::number_field::Atom.materialize()>"Number Field Atoms"</Link></li>
                <li><Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link></li>
                <li><Link href=routes::doc::Form.materialize()>"Form"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
