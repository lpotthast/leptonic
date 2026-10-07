use leptos::prelude::*;

use super::demos::search_field::SearchFieldConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageSearchFieldOverview() -> impl IntoView {
    view! {
        <DocPage title="Search Field">
            <p>
                "A search field lets people type a query and run it. It is a text field with search behavior: "
                <Keys keys="Enter"/>" submits the query, "<Keys keys="Escape"/>" clears it, and a clear button empties it "
                "with the pointer. Like every field, it connects its input with a label, a description and validation "
                "errors."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Run a search when the user submits a query"</TableCell><TableCell><b>"Search Field"</b></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Filter a list of options while typing and pick one"</TableCell>
                        <TableCell><Link href=routes::doc::Combobox.materialize()>"Combobox"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Capture other free-form text"</TableCell>
                        <TableCell><Link href=routes::doc::TextField.materialize()>"Text Field"</Link></TableCell>
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
                        <TableCell><Link href=routes::doc::search_field::Hook.materialize()>"use_search_field"</Link></TableCell>
                        <TableCell>
                            "The search keys and the clear button\u{2019}s configuration, on top of "
                            <Link href=format!("{}#use-text-field", routes::doc::text_field::Hook.materialize())>"use_text_field"</Link>
                            ", for an input you render."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::search_field::Atom.materialize()>"Search Field Atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"SearchField"</Code>" and "<Code inline=true>"SearchFieldClearButton"</Code>
                            ", composed with an "<Code inline=true>"Input"</Code>" and the "
                            <Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>", styled through data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The "<Code inline=true>"SearchField"</Code>" atom reports submitted queries through "
                    <Code inline=true>"on_submit"</Code>" and an emptied field through "<Code inline=true>"on_clear"</Code>
                    ". Compose it from a "<Code inline=true>"Label"</Code>", an "<Code inline=true>"Input"</Code>" and a "
                    <Code inline=true>"SearchFieldClearButton"</Code>" (the CSS is on the "
                    <Link href=format!("{}#styling", routes::doc::search_field::Atom.materialize())>"Search Field Atoms"</Link>
                    " page). Type a query and press "<Keys keys="Enter"/>":"
                </p>

                <Demo
                    description="Recipe search field showing the submitted query"
                    source=include_str!("demos/search_field.rs")
                    source_open=true
                >
                    <SearchFieldConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "The input is a native "<Code inline=true>"<input type=\"search\">"</Code>", which screen readers "
                        "announce as a search field, with a "<Code inline=true>"<label>"</Code>"."
                    </li>
                    <li>
                        "The clear button is labelled \u{201c}Clear search\u{201d} and isn\u{2019}t in the tab order: "
                        <Keys keys="Escape"/>" clears the field from the keyboard."
                    </li>
                    <li>
                        "Label, description and error message are connected with "<Code inline=true>"aria-labelledby"</Code>
                        " and "<Code inline=true>"aria-describedby"</Code>", as for every "
                        <Link href=routes::doc::Field.materialize()>"field"</Link>"."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Enter">"Submits the query, or the form when there is no submit handler."</KeyRow>
                    <KeyRow keys="Escape">
                        "Clears the query. In an empty field, the key is left to surrounding elements, e.g. to close a dialog."
                    </KeyRow>
                </KeyboardTable>
                <p>"While the field is read-only, it leaves both keys to the browser and to surrounding elements."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::search_field::Hook.materialize()>"use_search_field"</Link></li>
                <li><Link href=routes::doc::search_field::Atom.materialize()>"Search Field Atoms"</Link></li>
                <li><Link href=routes::doc::TextField.materialize()>"Text Field"</Link></li>
                <li><Link href=routes::doc::Combobox.materialize()>"Combobox"</Link></li>
                <li><Link href=routes::doc::Field.materialize()>"Field"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
