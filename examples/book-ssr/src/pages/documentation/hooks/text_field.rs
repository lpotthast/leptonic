use indoc::indoc;
use leptos::prelude::*;

use super::demos::text_field_basic::TextFieldBasicDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseTextField() -> impl IntoView {
    view! {
        <DocPage title="Text Field Hooks">
            <p>
                "The text field hooks add labelling and validation to a native text input or text area: "
                <AnchorLink href="#use-text-field-state"><Code inline=true>"use_text_field_state"</Code></AnchorLink>
                " holds the value, "<AnchorLink href="#use-text-field"><Code inline=true>"use_text_field"</Code></AnchorLink>
                " returns the attributes of the input, its label, description and error message, and "
                <AnchorLink href="#use-formatted-text-field"><Code inline=true>"use_formatted_text_field"</Code></AnchorLink>
                " keeps the text of a formatted field valid while typing. See the "
                <Link href=routes::doc::TextField.materialize()>"Text Field overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useTextField"/>

            <Section title="Example">
                <p>"Create the state, pass it to the hook and spread the returned props onto your elements:"</p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::form::{InputType, TextFieldElement, UseTextFieldInput, UseTextFieldStateInput, ValidationBehavior, use_text_field, use_text_field_state};

                        let name = use_text_field_state(UseTextFieldStateInput::default());
                        let field = use_text_field(UseTextFieldInput {
                            state: name,
                            has_label: true.into(),
                            // Everything else off:
                            id: None,
                            element: TextFieldElement::Input,
                            input_type: Signal::stored(InputType::Text),
                            is_disabled: false.into(),
                            is_read_only: false.into(),
                            is_required: false.into(),
                            is_invalid: false.into(),
                            validate: None,
                            validation_behavior: ValidationBehavior::Aria,
                            validation: None,
                            name: None,
                            form: None,
                            placeholder: MaybeProp::default(),
                            pattern: None,
                            min_length: None,
                            max_length: None,
                            auto_complete: None,
                            auto_capitalize: None,
                            auto_correct: None,
                            spell_check: None,
                            input_mode: Signal::stored(None),
                            enter_key_hint: None,
                            auto_focus: false,
                            exclude_from_tab_order: false,
                            label_id: None,
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

                        view! {
                            <label {..field.label_props.into_attrs()}>"Name"</label>
                            <input {..field.input_props.into_attrs()}/>
                            <p>{move || format!("Hello, {}!", name.value.get())}</p>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "A required username with a description, a length limit and custom validation. The error message "
                    "appears as soon as the value is invalid."
                </p>
                <Demo
                    description="Required username field with description, validation error, a clear button and a disabled toggle"
                    source=include_str!("demos/text_field_basic.rs")
                >
                    <TextFieldBasicDemo/>
                </Demo>
            </Section>

            <Section title="use_text_field_state">
                <p>
                    "Creates the field\u{2019}s "<Code inline=true>"TextFieldState"</Code>". The hook owns the value, starting "
                    "at "<Code inline=true>"default_value"</Code>", unless you bind it to your app state with "
                    <Code inline=true>"value"</Code>". Either way, every change goes through "
                    <Code inline=true>"state.set_value(..)"</Code>"."
                </p>
                <Section title="Input" id="use-text-field-state-input">
                    <ApiTable kind=ApiKind::Input of="UseTextFieldStateInput">
                        <ApiRow name="default_value" ty="String" default="empty">
                            "The initial value. Ignored when "<Code inline=true>"value"</Code>" is bound."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<String>>" default="None">
                            "The value as app state, replacing "<Code inline=true>"default_value"</Code>": "
                            <Code inline=true>"Some(signal.into())"</Code>" for an "<Code inline=true>"RwSignal<String>"</Code>
                            " or a "<Code inline=true>"(ReadSignal, WriteSignal)"</Code>" pair."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<String>>" default="None">"Called with the value when it changes."</ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Return" id="use-text-field-state-return">
                    <p>"The hook returns a "<Code inline=true>"TextFieldState"</Code>" ("<Code inline=true>"Copy"</Code>")."</p>
                    <ApiTable kind=ApiKind::Return of="TextFieldState">
                        <ApiRow name="value" ty="Signal<String>">"The current value."</ApiRow>
                    </ApiTable>
                    <DocTable headers=&["Method", "Purpose"]>
                        <TableRow>
                            <TableCell><Code inline=true>"set_value(value)"</Code></TableCell>
                            <TableCell>"Changes the value."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"with_on_change(callback)"</Code></TableCell>
                            <TableCell>"The same state, also calling "<Code inline=true>"callback"</Code>" when the value changes."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"TextFieldState::new(value, set_value)"</Code></TableCell>
                            <TableCell>
                                "A state whose value lives elsewhere, such as the input text of a "
                                <Link href=routes::doc::combobox::Hook.materialize()>"combobox"</Link>"."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"TextFieldState::from(signal)"</Code></TableCell>
                            <TableCell>
                                "A state bound to an "<Code inline=true>"RwSignal<String>"</Code>" or a "
                                <Code inline=true>"(ReadSignal, WriteSignal)"</Code>" pair, like Leptos\u{2019} "
                                <Code inline=true>"bind:value"</Code>"."
                            </TableCell>
                        </TableRow>
                    </DocTable>
                </Section>
            </Section>

            <Section title="use_text_field">
                <p>"Wires the input to its label, description and error message, and validates the value."</p>
                <Section title="Input" id="use-text-field-input">
                    <p>
                        "Pass a "<Code inline=true>"UseTextFieldInput"</Code>" with every field named; the Default column "
                        "gives the value for fields you don\u{2019}t need."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseTextFieldInput">
                        <ApiRow name="state" ty="TextFieldState">
                            "Required. The value, from "<AnchorLink href="#use-text-field-state"><Code inline=true>"use_text_field_state"</Code></AnchorLink>"."
                        </ApiRow>
                        <ApiRow name="element" ty="TextFieldElement" default="Input">
                            <Code inline=true>"Input"</Code>" or "<Code inline=true>"TextArea"</Code>": the element you render."
                        </ApiRow>
                        <ApiRow name="input_type" ty="Signal<InputType>" default="Text">
                            <Code inline=true>"Text"</Code>", "<Code inline=true>"Search"</Code>", "<Code inline=true>"Url"</Code>", "
                            <Code inline=true>"Tel"</Code>", "<Code inline=true>"Email"</Code>" or "<Code inline=true>"Password"</Code>
                            ". Only for "<Code inline=true>"<input>"</Code>"; reactive, e.g. for a \u{201c}show password\u{201d} toggle."
                        </ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The input\u{2019}s id, generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="label_id" ty="Option<String>" default="None">"The label\u{2019}s id, generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="has_label" ty="Signal<bool>" default="false">
                            "Whether you render a visible label with "<Code inline=true>"label_props"</Code>
                            ". The input is then labelled by it."
                        </ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the field or makes it read-only."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">
                            "Marks the field as required: with "<Code inline=true>"aria-required"</Code>" under "
                            <Code inline=true>"ValidationBehavior::Aria"</Code>", with the native "<Code inline=true>"required"</Code>
                            " under "<Code inline=true>"Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the value invalid while "<Code inline=true>"true"</Code>", taking precedence over all other "
                            "validation; "<Code inline=true>"false"</Code>" leaves validation to the other sources."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<String>>" default="None">
                            "Custom validation, returning "<Code inline=true>"Ok(())"</Code>" or "<Code inline=true>"Err(messages)"</Code>"."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Aria">
                            <Code inline=true>"Aria"</Code>" shows errors as the user types, "<Code inline=true>"Native"</Code>
                            " defers them to form submission."
                        </ApiRow>
                        <ApiRow name="validation" ty="Option<FormValidationState>" default="None">
                            "The validation state of a field built on the text field whose value isn\u{2019}t the text (e.g. a "
                            "number field). Replaces the text field\u{2019}s own; "<Code inline=true>"is_invalid"</Code>" and "
                            <Code inline=true>"validate"</Code>" are then up to that field."
                        </ApiRow>
                        <ApiRow name="name, form, auto_complete" ty="Option<String>" default="None">
                            "The corresponding native attributes."
                        </ApiRow>
                        <ApiRow name="placeholder" ty="MaybeProp<String>" default="None">"The placeholder text."</ApiRow>
                        <ApiRow name="pattern" ty="Option<String>" default="None">"A validation pattern. Only for "<Code inline=true>"<input>"</Code>"."</ApiRow>
                        <ApiRow name="min_length, max_length" ty="Option<u32>" default="None">"Length constraints, rendered as native attributes."</ApiRow>
                        <ApiRow name="input_mode" ty="Signal<Option<InputMode>>" default="None">"Which virtual keyboard to show."</ApiRow>
                        <ApiRow name="enter_key_hint" ty="Option<EnterKeyHint>" default="None">"The label of the virtual keyboard\u{2019}s Enter key."</ApiRow>
                        <ApiRow name="auto_capitalize" ty="Option<AutoCapitalize>" default="None">"Automatic capitalization."</ApiRow>
                        <ApiRow name="auto_correct, spell_check" ty="Option<bool>" default="None">"Automatic correction and spell checking."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the input when it mounts."</ApiRow>
                        <ApiRow name="exclude_from_tab_order" ty="bool" default="false">"Sets "<Code inline=true>"tabindex=\"-1\""</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the field when there is no visible label."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Further elements labelling the field."</ApiRow>
                        <ApiRow name="aria_describedby, aria_errormessage" ty="Option<String>" default="None">
                            "Further description or error message ids, merged with the generated ones."
                        </ApiRow>
                        <ApiRow name="aria_activedescendant, aria_controls" ty="Signal<Option<String>>" default="None">
                            "For inputs that control a popup, like a combobox."
                        </ApiRow>
                        <ApiRow name="aria_autocomplete" ty="Option<AriaAutocomplete>" default="None">"The "<Code inline=true>"aria-autocomplete"</Code>" attribute."</ApiRow>
                        <ApiRow name="aria_haspopup" ty="Option<AriaHasPopup>" default="None">"The "<Code inline=true>"aria-haspopup"</Code>" attribute."</ApiRow>
                        <ApiRow name="on_focus, on_blur" ty="Option<Callback<FocusEvent>>" default="None">"Called when the input gains or loses focus, with the event."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the input gains or loses focus."</ApiRow>
                        <ApiRow name="on_key_down, on_key_up" ty="Option<Callback<KeyboardEventWrapper>>" default="None">"Called on key presses in the input."</ApiRow>
                        <ApiRow name="shortcuts" ty="Option<KeyboardShortcuts>" default="None">"Keyboard shortcuts handled while the field has focus."</ApiRow>
                    </ApiTable>

                    <p>
                        "Other DOM events, like "<Code inline=true>"copy"</Code>" or "<Code inline=true>"paste"</Code>
                        ", are attached directly to the element ("<Code inline=true>"on:paste=.."</Code>")."
                    </p>
                </Section>

                <Section title="Return" id="use-text-field-return">
                    <ApiTable kind=ApiKind::Return of="UseTextFieldReturn">
                        <ApiRow name="input_props" ty="UseTextFieldInputProps">
                            "Attributes and event handlers for the "<Code inline=true>"<input>"</Code>" or "
                            <Code inline=true>"<textarea>"</Code>". Spread with "<Code inline=true>"{..input_props.into_attrs()}"</Code>
                            ". Keeps the element\u{2019}s value in sync with the state."
                        </ApiRow>
                        <ApiRow name="label_props" ty="UseLabelProps">"Id and "<Code inline=true>"for"</Code>" of the label."</ApiRow>
                        <ApiRow name="description_props" ty="SlotProps">
                            "For the description. The input references it only while it is rendered."
                        </ApiRow>
                        <ApiRow name="error_message_props" ty="SlotProps">
                            "For the error message. Render it only while "<Code inline=true>"is_invalid"</Code>
                            "; the input references it while it is rendered."
                        </ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The "<Code inline=true>"<input>"</Code>" or "<Code inline=true>"<textarea>"</Code>", once rendered."</ApiRow>
                        <ApiRow name="is_focused" ty="Signal<bool>">"Whether the input has focus."</ApiRow>
                        <ApiRow name="is_focus_visible" ty="Signal<bool>">"Whether a focus ring should be shown (keyboard focus)."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the displayed validation result is invalid."</ApiRow>
                        <ApiRow name="validation_errors" ty="Signal<Vec<String>>">"The displayed error messages."</ApiRow>
                        <ApiRow name="validation_details" ty="Signal<ValidityStateSnapshot>">"Detailed validity state."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_formatted_text_field">
                <p>
                    "Keeps the text of a field with a format, such as a number or a hex color, valid while the user types: "
                    "it checks every edit with the state\u{2019}s "<Code inline=true>"is_valid_text"</Code>" before the browser "
                    "applies it, and reverts composed text (from an input method or autocorrect) that ends up invalid. The "
                    <Link href=routes::doc::number_field::Hook.materialize()>"Number Field Hooks"</Link>" and "
                    <Link href=routes::doc::color_field::Hook.materialize()>"Color Field Hooks"</Link>
                    " use it. Attach the returned handlers to the input next to "<Code inline=true>"use_text_field"</Code>
                    "\u{2019}s props."
                </p>
                <Section title="Input" id="use-formatted-text-field-input">
                    <p>"The input has no defaults: set every field."</p>
                    <ApiTable kind=ApiKind::Input of="UseFormattedTextFieldInput">
                        <ApiRow name="element" ty="CapturedElement">
                            "The input, e.g. "<Code inline=true>"use_text_field"</Code>"\u{2019}s "<Code inline=true>"element"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="state" ty="S: FormattedTextState">"The field\u{2019}s text state. Required."</ApiRow>
                    </ApiTable>
                    <p>
                        "A "<Code inline=true>"FormattedTextState"</Code>" is "<Code inline=true>"Copy"</Code>" and has two methods: "
                        <Code inline=true>"is_valid_text(&self, &str) -> bool"</Code>" (whether a text may be typed: a valid "
                        "value, or the beginning of one) and "<Code inline=true>"set_text(&self, String)"</Code>" (sets the text "
                        "without committing it, to revert an invalid composition). "<Code inline=true>"NumberFieldState"</Code>
                        " and "<Code inline=true>"ColorFieldState"</Code>" implement it; implement it for the state of a field of "
                        "your own."
                    </p>
                </Section>
                <Section title="Return" id="use-formatted-text-field-return">
                    <ApiTable kind=ApiKind::Return of="FormattedTextFieldHandlers">
                        <ApiRow name="on_beforeinput" ty="EventHandler<InputEvent>">
                            "Rejects edits that would make the text invalid, before the browser applies them."
                        </ApiRow>
                        <ApiRow name="on_compositionstart, on_compositionend" ty="EventHandler<CompositionEvent>">
                            "Remember the text before a composition and restore it when the composed text is invalid."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::TextField.materialize()>"Text Field overview"</Link></li>
                <li><Link href=routes::doc::text_field::Atom.materialize()>"Text Field Atoms"</Link></li>
                <li><Link href=routes::doc::search_field::Hook.materialize()>"use_search_field"</Link></li>
                <li><Link href=routes::doc::number_field::Hook.materialize()>"Number Field Hooks"</Link></li>
                <li><Link href=routes::doc::field::Hook.materialize()>"Field Hooks"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
