use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{text_field_basic::TextFieldBasicDemo, text_field_search::TextFieldSearchDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageUseTextField() -> impl IntoView {
    view! {
        <DocPage title="Text Field Hooks">
            <p>
                <Code inline=true>"use_text_field"</Code>" and "<Code inline=true>"use_search_field"</Code>
                " add labelling, validation and keyboard handling to native text inputs. See the "
                <Link href=routes::doc::TextField.materialize()>"Text Field overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useTextField"/>

            <Section title="use_text_field">
                <p>
                    "The hook owns the field\u{2019}s value. Create its state with "<Code inline=true>"use_text_field_state"</Code>
                    " ("<Code inline=true>"default_value"</Code>", optional "<Code inline=true>"on_change"</Code>"), read it from "
                    <Code inline=true>"state.value"</Code>" and change it with "<Code inline=true>"state.set_value(..)"</Code>
                    ". Components that keep the value elsewhere build one with "
                    <Code inline=true>"TextFieldState::new(value, set_value)"</Code>"."
                </p>

                <Section title="Input" id="use-text-field-input">
                    <p>
                        "Create the input with "<Code inline=true>"UseTextFieldInput::new(state)"</Code>" and set further fields "
                        "with struct update syntax."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseTextFieldInput">
                        <ApiRow name="state" ty="TextFieldState">"The value, from "<Code inline=true>"use_text_field_state"</Code>"."</ApiRow>
                        <ApiRow name="element" ty="TextFieldElement" default="Input">
                            <Code inline=true>"Input"</Code>" or "<Code inline=true>"TextArea"</Code>": the element you render."
                        </ApiRow>
                        <ApiRow name="input_type" ty="Signal<InputType>" default="Text">
                            <Code inline=true>"Text"</Code>", "<Code inline=true>"Search"</Code>", "<Code inline=true>"Url"</Code>", "
                            <Code inline=true>"Tel"</Code>", "<Code inline=true>"Email"</Code>" or "<Code inline=true>"Password"</Code>
                            ". Only for "<Code inline=true>"<input>"</Code>"."
                        </ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The input\u{2019}s id, generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="label_id" ty="Option<String>" default="None">"The label\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="has_label" ty="bool" default="false">
                            "Whether you render a visible label with "<Code inline=true>"label_props"</Code>
                            ". The input is then labelled by it."
                        </ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the field or makes it read-only."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">
                            "Sets "<Code inline=true>"required"</Code>" and "<Code inline=true>"aria-required"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the value invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation; "
                            <Code inline=true>"false"</Code>" leaves validation to the other sources."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<String>>" default="None">
                            "Custom validation, returning "<Code inline=true>"Ok(())"</Code>" or "<Code inline=true>"Err(messages)"</Code>"."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="Aria">
                            <Code inline=true>"Aria"</Code>" shows errors as the user types, "<Code inline=true>"Native"</Code>
                            " defers them to form submission."
                        </ApiRow>
                        <ApiRow name="validation" ty="Option<UseFormValidationStateReturn>" default="None">
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
                        <ApiRow name="input_mode" ty="Option<InputMode>" default="None">"Which virtual keyboard to show."</ApiRow>
                        <ApiRow name="enter_key_hint" ty="Option<EnterKeyHint>" default="None">"The label of the virtual keyboard\u{2019}s Enter key."</ApiRow>
                        <ApiRow name="auto_capitalize" ty="Option<AutoCapitalize>" default="None">"Automatic capitalization."</ApiRow>
                        <ApiRow name="auto_correct, spell_check" ty="Option<bool>" default="None">"Automatic correction and spell checking."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focus the input when it mounts."</ApiRow>
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
                        <ApiRow name="on_focus, on_blur" ty="Option<Callback<FocusEvent>>" default="None">"Focus callbacks."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the field gains or loses focus."</ApiRow>
                        <ApiRow name="on_key_down, on_key_up" ty="Option<Callback<KeyboardEventWrapper>>" default="None">"Keyboard callbacks."</ApiRow>
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

                <Section title="Example" id="use-text-field-example">
                    <p>
                        "A required username with a description, a length limit and custom validation. The error message "
                        "appears as soon as the value is invalid."
                    </p>

                    <Demo
                        description="Required username field with description, validation error, a clear button and a disabled toggle"
                        source=include_str!("demos/text_field_basic.rs")
                        source_open=true
                    >
                        <TextFieldBasicDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="use_search_field">
                <p>
                    "A text field for search queries: the input gets "<Code inline=true>"type=\"search\""</Code>", "
                    <Keys keys="Enter"/>" calls "<Code inline=true>"on_submit"</Code>" with the value (without "
                    <Code inline=true>"on_submit"</Code>", it submits the form), and "<Keys keys="Escape"/>" empties the field "
                    "and calls "<Code inline=true>"on_clear"</Code>". In an empty field, "<Keys keys="Escape"/>" is left to "
                    "surrounding elements, so it can close a dialog. The value state is a "<Code inline=true>"TextFieldState"</Code>
                    ", as for "<Code inline=true>"use_text_field"</Code>"."
                </p>

                <Section title="Input" id="use-search-field-input">
                    <p>
                        "Create the input with "<Code inline=true>"UseSearchFieldInput::new(UseTextFieldInput { .. })"</Code>
                        ", which sets the text field\u{2019}s "<Code inline=true>"input_type"</Code>" to "
                        <Code inline=true>"Search"</Code>"."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseSearchFieldInput">
                        <ApiRow name="text_field" ty="UseTextFieldInput">
                            "The text field: value state, labelling, validation, \u{2026} See "
                            <a href="#use-text-field-input">"use_text_field"</a>"."
                        </ApiRow>
                        <ApiRow name="on_submit" ty="Option<Callback<String>>" default="None">
                            "Called with the value when the user presses "<Keys keys="Enter"/>". Without it, "<Keys keys="Enter"/>
                            " submits the form."
                        </ApiRow>
                        <ApiRow name="on_clear" ty="Option<Callback<()>>" default="None">
                            "Called when "<Keys keys="Escape"/>" or the clear button empties the field."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-search-field-return">
                    <ApiTable kind=ApiKind::Return of="UseSearchFieldReturn">
                        <ApiRow name="text_field" ty="UseTextFieldReturn">
                            "The text field\u{2019}s return: input, label, description and error message props, and validation."
                        </ApiRow>
                        <ApiRow name="clear_button" ty="UseButtonInput">
                            "The clear button\u{2019}s configuration. Pass it to "
                            <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>": the button is labelled "
                            "\u{201c}Clear search\u{201d}, isn\u{2019}t in the tab order, keeps focus in the input, and is "
                            "disabled while the field is disabled or read-only. Hide it while "<Code inline=true>"state.value"</Code>
                            " is empty."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-search-field-example">
                    <Demo
                        description="Search field with clear button showing the last submitted query"
                        source=include_str!("demos/text_field_search.rs")
                        source_open=true
                    >
                        <TextFieldSearchDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="use_number_field">
                <p>
                    "Numeric input with increment and decrement buttons, locale-aware formatting and floating-point precision "
                    "handling. See the "<Link href=routes::doc::text_field::NumberFieldHook.materialize()>"use_number_field"</Link>
                    " page."
                </p>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Enter">"Search field: calls "<Code inline=true>"on_submit"</Code>", or submits the form without it."</KeyRow>
                    <KeyRow keys="Escape">
                        "Search field: empties a non-empty field and calls "<Code inline=true>"on_clear"</Code>"; in an empty "
                        "field, the key propagates."
                    </KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::TextField.materialize()>"Text Field overview"</Link></li>
                <li><Link href=routes::doc::text_field::Atom.materialize()>"Text Field atoms"</Link></li>
                <li><Link href=routes::doc::text_field::Component.materialize()>"Text Field component"</Link></li>
                <li><Link href=routes::doc::text_field::NumberFieldHook.materialize()>"use_number_field"</Link></li>
                <li><Link href=routes::doc::hooks::UseLabel.materialize()>"use_label"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
