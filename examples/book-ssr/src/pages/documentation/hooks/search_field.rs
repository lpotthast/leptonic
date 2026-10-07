use leptos::prelude::*;

use super::demos::search_field::SearchFieldDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseSearchField() -> impl IntoView {
    view! {
        <DocPage title="use_search_field">
            <p>
                "The "<Code inline=true>"use_search_field"</Code>" hook turns a "
                <Link href=format!("{}#use-text-field", routes::doc::text_field::Hook.materialize())>"use_text_field"</Link>" input into a search field "
                "that submits on "<Keys keys="Enter"/>" and clears on "<Keys keys="Escape"/>". See the "
                <Link href=routes::doc::SearchField.materialize()>"Search Field overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useSearchField"/>

            <Section title="Input">
                <p>
                    "Pass a "<Code inline=true>"UseSearchFieldInput"</Code>" with every field named, and set the text field\u{2019}s "
                    <Code inline=true>"input_type"</Code>" to "<Code inline=true>"Search"</Code>". The value state is a "
                    <Code inline=true>"TextFieldState"</Code>

                    " from "<Link href=format!("{}#use-text-field-state", routes::doc::text_field::Hook.materialize())>"use_text_field_state"</Link>"."
                </p>

                <ApiTable kind=ApiKind::Input of="UseSearchFieldInput">
                    <ApiRow name="text_field" ty="UseTextFieldInput">
                        "Required. The text field: value state, labelling, validation, \u{2026} See "
                        <Link href=format!("{}#use-text-field-input", routes::doc::text_field::Hook.materialize())>"use_text_field"</Link>"."
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

            <Section title="Return">
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

            <Section title="Example">
                <p>
                    "A search field with a clear button, showing the last submitted query. The input gets its attributes "
                    "from "<Code inline=true>"text_field.input_props"</Code>", the button from "
                    <Code inline=true>"use_button(clear_button)"</Code>":"
                </p>

                <Demo
                    description="Search field with a clear button showing the last submitted query, with a disabled toggle"
                    source=include_str!("demos/search_field.rs")
                    source_open=true
                >
                    <SearchFieldDemo/>
                </Demo>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Enter">"Calls "<Code inline=true>"on_submit"</Code>" with the value, or submits the form without it."</KeyRow>
                    <KeyRow keys="Escape">
                        "Empties a non-empty field and calls "<Code inline=true>"on_clear"</Code>". In an empty field, the key "
                        "is left to surrounding elements, so it can close a dialog."
                    </KeyRow>
                </KeyboardTable>
                <p>"While the field is read-only, both keys are left to the browser and surrounding elements."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::SearchField.materialize()>"Search Field overview"</Link></li>
                <li><Link href=routes::doc::search_field::Atom.materialize()>"Search Field Atoms"</Link></li>
                <li><Link href=routes::doc::text_field::Hook.materialize()>"Text Field Hooks"</Link></li>
                <li><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
