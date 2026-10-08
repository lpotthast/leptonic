use indoc::indoc;
use leptos::prelude::*;

use super::demos::search_field::SearchFieldAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomSearchField() -> impl IntoView {
    view! {
        <DocPage title="Search Field Atoms">
            <p>
                "The search field atoms render an unstyled search field: "<Code inline=true>"SearchField"</Code>" holds the "
                "query, submits and clears it, "<Code inline=true>"SearchFieldClearButton"</Code>" empties it, and the "
                <Link href=format!("{}#input-textarea", routes::doc::text_field::Atom.materialize())>"Input"</Link>" and the "
                <Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>" render the input, a label, a description "
                "and an error message. See the "<Link href=routes::doc::SearchField.materialize()>"Search Field overview"</Link>
                " for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"SearchField"</Code>" calls "
                    <Link href=format!("{}#use-text-field-state", routes::doc::text_field::Hook.materialize())>"use_text_field_state"</Link>" and "
                    <Link href=routes::doc::search_field::Hook.materialize()>"use_search_field"</Link>"; "
                    <Code inline=true>"SearchFieldClearButton"</Code>" calls "
                    <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" and "
                    <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>"."
                </p>
            </Section>

            <Section title="Example">
                <p>
                    "Import the atoms from their modules or from "<Code inline=true>"leptonic::atoms::prelude"</Code>":"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::{
                            field::Label,
                            input::Input,
                            search_field::{SearchField, SearchFieldClearButton},
                        };

                        let (query, set_query) = signal(String::new());

                        view! {
                            <SearchField on_submit=move |submitted: String| set_query.set(submitted) classes="my-search">
                                <Label>"Search"</Label>
                                <Input/>
                                <SearchFieldClearButton classes="my-clear">
                                    <span aria-hidden="true">"\u{2715}"</span>
                                </SearchFieldClearButton>
                            </SearchField>
                            <p>"Results for \u{201c}" {query} "\u{201d}"</p>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo
                    description="Recipe search field with a clear button hidden while empty, showing submitted queries and clears, with a disabled toggle"
                    source=include_str!("demos/search_field.rs")
                >
                    <SearchFieldAtomDemo/>
                </Demo>
            </Section>

            <Section title="SearchField">
                <p>
                    "Creates the field and renders a "<Code inline=true>"<div>"</Code>" around its children, like "
                    <Link href=format!("{}#textfield", routes::doc::text_field::Atom.materialize())>"TextField"</Link>". "
                    <Keys keys="Enter"/>" calls "<Code inline=true>"on_submit"</Code>" with the value (without "
                    <Code inline=true>"on_submit"</Code>", it submits the form), and "<Keys keys="Escape"/>" or the "
                    <Code inline=true>"SearchFieldClearButton"</Code>" empty it and call "<Code inline=true>"on_clear"</Code>
                    ". The input has "<Code inline=true>"type=\"search\""</Code>"."
                </p>
                <Section title="Props" id="search-field-props">
                    <ApiTable kind=ApiKind::Props of="atoms::search_field::SearchField">
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
                        <ApiRow name="pattern" ty="Option<String>" default="None">"A validation pattern."</ApiRow>
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

            <Section title="SearchFieldClearButton">
                <p>
                    "The "<Code inline=true>"<button>"</Code>" emptying the search field around it, labelled \u{201c}Clear "
                    "search\u{201d}. It isn\u{2019}t in the tab order ("<Keys keys="Escape"/>" clears from the keyboard), "
                    "keeps focus in the input, and is disabled while the field is disabled or read-only."
                </p>
                <Section title="Props" id="search-field-clear-button-props">
                    <ApiTable kind=ApiKind::Props of="SearchFieldClearButton">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<button>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">
                            "Required. The button\u{2019}s content, typically an icon (hidden from screen readers, the label "
                            "names the button)."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>"Set to "<Code inline=true>"true"</Code>" while the state applies."</p>
                <Section title="SearchField" id="search-field-data-attributes">
                    <p>"On the field\u{2019}s "<Code inline=true>"<div>"</Code>":"</p>
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-empty" ty="true">"The value is empty. Hide the clear button with it."</ApiRow>
                        <ApiRow name="data-disabled" ty="true">"The field is disabled."</ApiRow>
                        <ApiRow name="data-invalid" ty="true">"The value is invalid."</ApiRow>
                        <ApiRow name="data-readonly" ty="true">"The field is read-only."</ApiRow>
                        <ApiRow name="data-required" ty="true">"The field is required."</ApiRow>
                    </ApiTable>
                    <p>
                        "The input renders the data attributes of the "
                        <Link href=format!("{}#input-data-attributes", routes::doc::text_field::Atom.materialize())>"Input"</Link>"."
                    </p>
                </Section>
                <Section title="SearchFieldClearButton" id="search-field-clear-button-data-attributes">
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-pressed" ty="true">"The button is pressed."</ApiRow>
                        <ApiRow name="data-hovered" ty="true">"A mouse or pen is over the button."</ApiRow>
                        <ApiRow name="data-focused" ty="true">"The button has focus. It is no tab stop, and pressing it keeps the focus in the input, so this is rare."</ApiRow>
                        <ApiRow name="data-focus-visible" ty="true">"The button has keyboard focus: show a focus ring."</ApiRow>
                        <ApiRow name="data-disabled" ty="true">"The button is disabled (with the field, or while it is read-only)."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. "<Code inline=true>"SearchField"</Code>" renders a "<Code inline=true>"<div>"</Code>
                    " (default class "<Code inline=true>"leptonic-SearchField"</Code>") around its children: a "
                    <Code inline=true>"Label"</Code>", an "<Code inline=true>"Input"</Code>" ("<Code inline=true>"leptonic-Input"</Code>
                    ") and a "<Code inline=true>"SearchFieldClearButton"</Code>" ("<Code inline=true>"leptonic-SearchFieldClearButton"</Code>
                    "), with any markup between them. The clear button is named \u{201c}Clear search\u{201d}; its content is "
                    "yours, e.g. an "<Code inline=true>"aria-hidden"</Code>" \u{2715}."
                </p>
                <p>
                    "Hide the clear button while the field has "<Code inline=true>"data-empty"</Code>", and style the input "
                    "and the button through their own data attributes. The demo above uses this CSS (its button and input "
                    "classes are styled like the "<Link href=format!("{}#styling", routes::doc::button::Atom.materialize())>"Button Atom"</Link>
                    " and the "<Link href=format!("{}#styling", routes::doc::text_field::Atom.materialize())>"Input"</Link>"):"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .demo-field { display: flex; flex-direction: column; gap: 0.25rem; }
                        .demo-input-row { display: flex; align-items: center; gap: 0.5rem; }
                        .demo-search-field[data-empty] .demo-search-field-clear { visibility: hidden; }
                    ")}
                </Code>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::SearchField.materialize()>"Search Field overview"</Link></li>
                <li><Link href=routes::doc::search_field::Hook.materialize()>"use_search_field"</Link></li>
                <li><Link href=routes::doc::text_field::Atom.materialize()>"Text Field Atoms"</Link></li>
                <li><Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
