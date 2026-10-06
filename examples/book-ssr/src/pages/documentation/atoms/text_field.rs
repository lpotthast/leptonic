use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{search_field::SearchFieldAtomDemo, text_field::TextFieldAtomDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomTextField() -> impl IntoView {
    view! {
        <DocPage title="Text Field Atoms">
            <p>
                "The text field atoms render unstyled text fields: "<Code inline=true>"TextField"</Code>" holds the value and "
                "its validation, "<Code inline=true>"Input"</Code>" or "<Code inline=true>"TextArea"</Code>" renders the "
                "input, and the "<Link href=routes::doc::atoms::Field.materialize()>"field atoms"</Link>" add a label, a "
                "description and an error message. "<Code inline=true>"SearchField"</Code>" adds submitting and clearing. "
                "See the "<Link href=routes::doc::TextField.materialize()>"Text Field overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"TextField"</Code>" calls "<Code inline=true>"use_text_field_state"</Code>" and "
                    <Link href=routes::doc::text_field::Hook.materialize()>"use_text_field"</Link>", "
                    <Code inline=true>"SearchField"</Code>" calls "
                    <Link href=format!("{}#use-search-field", routes::doc::text_field::Hook.materialize())>"use_search_field"</Link>
                    ". "<Code inline=true>"Input"</Code>" and "<Code inline=true>"TextArea"</Code>" call "
                    <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>"; "
                    <Code inline=true>"SearchFieldClearButton"</Code>" calls "
                    <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" and "<Code inline=true>"use_hover"</Code>"."
                </p>
            </Section>

            <Section title="Example">
                <p>
                    "The input\u{2019}s attributes ("<Code inline=true>"placeholder"</Code>", "<Code inline=true>"input_type"</Code>
                    ", "<Code inline=true>"max_length"</Code>", \u{2026}) are props of the field; the "<Code inline=true>"Input"</Code>
                    " only renders them. Bind the value to a signal of your app with "
                    <Code inline=true>"state=TextFieldState::from(signal)"</Code>", or start from "
                    <Code inline=true>"default_value"</Code>" and listen to "<Code inline=true>"on_change"</Code>":"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::{field::{Description, FieldError, Label}, input::Input, text_field::TextField};

                        let name = RwSignal::new(String::new());

                        view! {
                            <TextField state=TextFieldState::from(name) is_required=true placeholder="Ferris">
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
                    " with a length limit. Both values are bound to signals."
                </p>
                <Demo
                    description="Display name input and bio text area bound to signals, with validation, a character count and a disabled toggle"
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
                        <ApiRow name="default_value" ty="String" default="empty">"The initial value, restored on form reset."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<String>>" default="None">"Called with the value when it changes."</ApiRow>
                        <ApiRow name="state" ty="Option<TextFieldState>" default="None">
                            "The value state, replacing "<Code inline=true>"default_value"</Code>". Bind a signal with "
                            <Code inline=true>"TextFieldState::from(rw_signal)"</Code>" or a "
                            <Code inline=true>"(ReadSignal, WriteSignal)"</Code>" pair."
                        </ApiRow>
                        <ApiRow name="input_type" ty="Signal<InputType>" default="Text">
                            "The "<Code inline=true>"<input>"</Code>"\u{2019}s type: "<Code inline=true>"Text"</Code>", "
                            <Code inline=true>"Email"</Code>", "<Code inline=true>"Password"</Code>", \u{2026} Ignored by a "
                            <Code inline=true>"TextArea"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the field or makes it read-only."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">
                            "Sets "<Code inline=true>"required"</Code>" and "<Code inline=true>"aria-required"</Code>" on the input."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the field invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<String>>" default="None">
                            "Validates the value, returning "<Code inline=true>"Ok(())"</Code>" or "<Code inline=true>"Err(messages)"</Code>"."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown. "<Code inline=true>"None"</Code>": the behavior of the surrounding "
                            <Link href=routes::doc::atoms::Form.materialize()>"Form"</Link>", else "<Code inline=true>"Native"</Code>"."
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
                        <ApiRow name="children" ty="Children">"The parts, and any other content."</ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Data Attributes" id="text-field-data-attributes">
                    <p>"Set to "<Code inline=true>"true"</Code>" on the field\u{2019}s "<Code inline=true>"<div>"</Code>" while the state applies:"</p>
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-disabled" ty="true">"The field is disabled."</ApiRow>
                        <ApiRow name="data-invalid" ty="true">"The value is invalid."</ApiRow>
                        <ApiRow name="data-readonly" ty="true">"The field is read-only."</ApiRow>
                        <ApiRow name="data-required" ty="true">"The field is required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Input, TextArea">
                <p>
                    "The "<Code inline=true>"<input>"</Code>" or "<Code inline=true>"<textarea>"</Code>" of the field around it "
                    "(a "<Code inline=true>"TextField"</Code>" or "<Code inline=true>"SearchField"</Code>"). They take only "
                    "classes and styles; everything else comes from the field."
                </p>
                <Section title="Props" id="input-props">
                    <ApiTable kind=ApiKind::Props of="atoms::input::Input">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<input>"</Code>"."</ApiRow>
                    </ApiTable>
                    <ApiTable kind=ApiKind::Props of="atoms::input::TextArea">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<textarea>"</Code>"."</ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Data Attributes" id="input-data-attributes">
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-focused" ty="true">"The input has focus."</ApiRow>
                        <ApiRow name="data-focus-visible" ty="true">"The input has keyboard focus: show a focus ring."</ApiRow>
                        <ApiRow name="data-hovered" ty="true">"A mouse or pen is over the input."</ApiRow>
                        <ApiRow name="data-disabled" ty="true">"The field is disabled."</ApiRow>
                        <ApiRow name="data-invalid" ty="true">"The value is invalid."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="SearchField">
                <p>
                    "A text field for search queries: "<Keys keys="Enter"/>" calls "<Code inline=true>"on_submit"</Code>
                    " with the value (without "<Code inline=true>"on_submit"</Code>", it submits the form), and "
                    <Keys keys="Escape"/>" or the "<Code inline=true>"SearchFieldClearButton"</Code>" empty it and call "
                    <Code inline=true>"on_clear"</Code>". The input has "<Code inline=true>"type=\"search\""</Code>"."
                </p>
                <Demo
                    description="Recipe search field with a clear button hidden while empty, showing submitted queries and clears, with a disabled toggle"
                    source=include_str!("demos/search_field.rs")
                >
                    <SearchFieldAtomDemo/>
                </Demo>
                <Section title="Props" id="search-field-props">
                    <ApiTable kind=ApiKind::Props of="atoms::search_field::SearchField">
                        <ApiRow name="default_value" ty="String" default="empty">"The initial value, restored on form reset."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<String>>" default="None">"Called with the value when it changes."</ApiRow>
                        <ApiRow name="state" ty="Option<TextFieldState>" default="None">
                            "The value state, replacing "<Code inline=true>"default_value"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_submit" ty="Option<Callback<String>>" default="None">
                            "Called with the value when "<Keys keys="Enter"/>" is pressed. Without it, "<Keys keys="Enter"/>
                            " submits the form."
                        </ApiRow>
                        <ApiRow name="on_clear" ty="Option<Callback<()>>" default="None">
                            "Called when "<Keys keys="Escape"/>" or the clear button empties the field."
                        </ApiRow>
                        <ApiRow name="input_type" ty="Signal<InputType>" default="Search">"The input\u{2019}s type."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the field or makes it read-only."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">
                            "Sets "<Code inline=true>"required"</Code>" and "<Code inline=true>"aria-required"</Code>" on the input."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the field invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<String>>" default="None">
                            "Validates the value, returning "<Code inline=true>"Ok(())"</Code>" or "<Code inline=true>"Err(messages)"</Code>"."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown. "<Code inline=true>"None"</Code>": the behavior of the surrounding "
                            <Link href=routes::doc::atoms::Form.materialize()>"Form"</Link>", else "<Code inline=true>"Native"</Code>"."
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
                        <ApiRow name="children" ty="Children">"The parts, and any other content."</ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Data Attributes" id="search-field-data-attributes">
                    <p>"As "<Code inline=true>"TextField"</Code>"\u{2019}s, plus:"</p>
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-empty" ty="true">"The value is empty. Hide the clear button with it."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="SearchFieldClearButton">
                <p>
                    "The "<Code inline=true>"<button>"</Code>" emptying the search field around it, labelled \u{201c}Clear "
                    "search\u{201d}. It isn\u{2019}t in the tab order ("<Keys keys="Escape"/>" clears from the keyboard), "
                    "keeps focus in the input, and is disabled while the field is disabled or read-only."
                </p>
                <Section title="Props" id="search-field-clear-button-props">
                    <ApiTable kind=ApiKind::Props of="SearchFieldClearButton">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<button>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"The button\u{2019}s content, typically an icon."</ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Data Attributes" id="search-field-clear-button-data-attributes">
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-pressed" ty="true">"The button is pressed."</ApiRow>
                        <ApiRow name="data-hovered" ty="true">"A mouse or pen is over the button."</ApiRow>
                        <ApiRow name="data-disabled" ty="true">"The button is disabled."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Enter">"Search field: calls "<Code inline=true>"on_submit"</Code>", or submits the form."</KeyRow>
                    <KeyRow keys="Escape">
                        "Search field: empties the field and calls "<Code inline=true>"on_clear"</Code>". In an empty field, "
                        "the key is left to surrounding elements, e.g. to close a dialog."
                    </KeyRow>
                </KeyboardTable>
            </Section>

            <Section title="Styling">
                <p>"Select the states with attribute selectors on your classes:"</p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-input { border: 1px solid gray; border-radius: 4px; }
                        .my-input[data-hovered] { border-color: royalblue; }
                        .my-input[data-invalid] { border-color: crimson; }
                        .my-input[data-focus-visible] { outline: 2px solid royalblue; outline-offset: 1px; }
                        .my-search[data-empty] .my-clear { visibility: hidden; }
                    ")}
                </Code>
                <p>"The demos show their complete styles under \u{201c}View styles\u{201d}."</p>
            </Section>

            <Section title="InputContext">
                <p>
                    "The parts find the field\u{2019}s input through an "<Code inline=true>"InputContext"</Code>". A field you "
                    "build from hooks can provide one to use "<Code inline=true>"Input"</Code>" (and "
                    <Code inline=true>"TextArea"</Code>"), together with a "
                    <Link href=format!("{}#fieldcontext", routes::doc::atoms::Field.materialize())>"FieldContext"</Link>
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

            <SeeAlso>
                <li><Link href=routes::doc::TextField.materialize()>"Text Field overview"</Link></li>
                <li><Link href=routes::doc::text_field::Hook.materialize()>"Text field hooks"</Link></li>
                <li><Link href=routes::doc::atoms::Field.materialize()>"Field atoms"</Link></li>
                <li><Link href=routes::doc::atoms::Form.materialize()>"Form atom"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
