use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::search_field::SearchFieldDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageSearchField() -> impl IntoView {
    view! {
        <DocPage title="Search Field Component">
            <p>
                "The themed "<Code inline=true>"SearchField"</Code>" component: a search input with its label, description, clear "
                "button and validation errors, styled by leptonic\u{2019}s theme. See the "
                <Link href=routes::doc::SearchField.materialize()>"Search Field overview"</Link>" for concept guidance."
            </p>
            <p>
                <Keys keys="Enter"/>" calls "<Code inline=true>"on_submit"</Code>", and "<Keys keys="Escape"/>" or the clear "
                "button, shown while there is a value, empty the field and call "<Code inline=true>"on_clear"</Code>"."
            </p>

            <Demo
                description="Recipe search listing the recipes that match the submitted query, with disabled and read-only toggles"
                source=include_str!("demos/search_field.rs")
            >
                <SearchFieldDemo/>
            </Demo>

            <Section title="Props">
                <Section title="SearchField">
                    <ApiTable kind=ApiKind::Props of="components::text_field::SearchField">
                        <ApiRow name="label" ty="MaybeProp<String>" default="None">
                            "The visible label. Without it, set "<Code inline=true>"aria_label"</Code>"."
                        </ApiRow>
                        <ApiRow name="description" ty="MaybeProp<String>" default="None">"Help text below the input."</ApiRow>
                        <ApiRow name="default_value" ty="String" default="empty">
                            "The initial value, unless "<Code inline=true>"value"</Code>" is set. A form reset restores the value "
                            "the field started with."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<String>>" default="None">"Called with the value when it changes."</ApiRow>
                        <ApiRow name="value" ty="Option<Signal<String>>" default="None">
                            "The value (controlled): a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<String>>" default="None">
                            "Receives the new state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_submit" ty="Option<Callback<String>>" default="None">
                            "Called with the value when "<Keys keys="Enter"/>" is pressed. Without it, "<Keys keys="Enter"/>
                            " submits the form."
                        </ApiRow>
                        <ApiRow name="on_clear" ty="Option<Callback<()>>" default="None">
                            "Called when "<Keys keys="Escape"/>" or the clear button empties the field."
                        </ApiRow>
                        <ApiRow name="placeholder" ty="MaybeProp<String>" default="None">"Shown while the field is empty."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the field or makes it read-only."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">
                            "Marks the field as required (the native "<Code inline=true>"required"</Code>" with "
                            <Code inline=true>"Native"</Code>" validation, "<Code inline=true>"aria-required"</Code>" with "
                            <Code inline=true>"Aria"</Code>")."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the field invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<String>>" default="None">
                            "Validates the value, returning "<Code inline=true>"Ok(())"</Code>" or "<Code inline=true>"Err(messages)"</Code>"."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown. "<Code inline=true>"None"</Code>": the behavior of the surrounding "
                            <Link href=routes::doc::Form.materialize()>"Form"</Link>", else "<Code inline=true>"Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The input\u{2019}s "<Code inline=true>"name"</Code>", for form submission."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="generated">"The input\u{2019}s id."</ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">
                            "The id of the "<Code inline=true>"<form>"</Code>" the input belongs to, when it isn\u{2019}t inside it."
                        </ApiRow>
                        <ApiRow name="pattern" ty="Option<String>" default="None">"A regular expression the value must match (native validation)."</ApiRow>
                        <ApiRow name="input_mode" ty="Option<InputMode>" default="None">"Which virtual keyboard to show."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">
                            "Ids of further elements describing the field, in addition to its description and errors."
                        </ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the input when it mounts."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"The accessible name, for a field without label."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the input gains or loses focus."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the field\u{2019}s "<Code inline=true>"<div>"</Code>"."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Labels, State and Validation">
                <p>
                    "Labels, descriptions, the value state and validation work as for the "
                    <Link href=routes::doc::text_field::Component.materialize()>"Text Field Component"</Link>
                    ": see its sections on "
                    <Link href=format!("{}#labels-and-descriptions", routes::doc::text_field::Component.materialize())>"labels and descriptions"</Link>", "
                    <Link href=format!("{}#state", routes::doc::text_field::Component.materialize())>"state"</Link>" and "
                    <Link href=format!("{}#validation", routes::doc::text_field::Component.materialize())>"validation"</Link>"."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The field shares the input styling of leptonic\u{2019}s theme (see the "
                    <Link href=format!("{}#styling", routes::doc::text_field::Component.materialize())>"Text Field Component\u{2019}s CSS variables"</Link>
                    "). Its "<Code inline=true>"<div>"</Code>" has the classes "<Code inline=true>"leptonic-text-field"</Code>" and "
                    <Code inline=true>"leptonic-search-field"</Code>" and the data attributes of the "
                    <Link href=routes::doc::search_field::Atom.materialize()>"Search Field Atoms"</Link>", e.g. "
                    <Code inline=true>"data-empty"</Code>". For full control over the markup, build the field from the atoms."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::SearchField.materialize()>"Search Field overview"</Link></li>
                <li><Link href=routes::doc::search_field::Hook.materialize()>"use_search_field"</Link></li>
                <li><Link href=routes::doc::search_field::Atom.materialize()>"Search Field Atoms"</Link></li>
                <li><Link href=routes::doc::text_field::Component.materialize()>"Text Field Component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
