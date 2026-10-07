use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageFields() -> impl IntoView {
    view! {
        <DocPage title="Fields">
            <p>
                "Fields collect the values of a form: text, numbers, on/off settings and choices among a few visible "
                "options. They belong together because they share a frame: a label, an optional description and an error "
                "message, connected to the control so that screen readers announce them with it, and the validation "
                "behavior of the form around them."
            </p>
            <p>
                "Choices from a list that opens on demand are in "<Link href=routes::doc::Pickers.materialize()>"Pickers"</Link>
                "; dates, times and colors have groups of their own ("<Link href=routes::doc::DateTime.materialize()>"Date & Time"</Link>
                ", "<Link href=routes::doc::Color.materialize()>"Color"</Link>"). How fields validate and show errors is "
                "explained in "<Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link>"."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Fields.materialize()/>
            </Section>

            <Section title="Relationships">
                <ul>
                    <li>
                        <Link href=routes::doc::Field.materialize()>"Field"</Link>" is the frame the other fields are built "
                        "with: it connects a label, a description and an error message to a control. Use it directly to "
                        "give a control of your own the same frame."
                    </li>
                    <li>
                        <Link href=routes::doc::Form.materialize()>"Form"</Link>" holds the other fields: it decides when they "
                        "show their errors and passes the errors your server returns on to them. Its hooks give a field you "
                        "build yourself the same validation and reset behavior."
                    </li>
                    <li>
                        <Link href=routes::doc::Checkbox.materialize()>"Checkbox"</Link>", "
                        <Link href=routes::doc::Radio.materialize()>"Radio"</Link>" and "
                        <Link href=routes::doc::Switch.materialize()>"Switch"</Link>" are native inputs (inside a "
                        <Code inline=true>"<label>"</Code>" in the atoms), so they submit and reset with their form."
                    </li>
                    <li>
                        <Link href=routes::doc::SearchField.materialize()>"Search Field"</Link>" is a "
                        <Link href=routes::doc::TextField.materialize()>"Text Field"</Link>" for search queries: "
                        <Keys keys="Enter"/>" submits the query, "<Keys keys="Escape"/>" or a clear button empties the field."
                    </li>
                    <li>
                        <Link href=routes::doc::NumberField.materialize()>"Number Field"</Link>" and "
                        <Link href=routes::doc::Slider.materialize()>"Slider"</Link>" both edit a number in a range: one "
                        "by typing and stepping, the other by dragging."
                    </li>
                    <li>
                        "For rich text, use the "<Link href=format!("{}#rich-content", routes::doc::Layout.materialize())>"leptos-tiptap"</Link>
                        " crate; it is not built on the field frame and needs a label of your own."
                    </li>
                </ul>
            </Section>

            <Section title="Decision Guide">
                <DocTable headers=&["Choice", "Pick the first when\u{2026}", "Pick the second when\u{2026}"]>
                    <TableRow>
                        <TableCell><strong>"Checkbox vs Switch"</strong></TableCell>
                        <TableCell>"The option is submitted with a form later"</TableCell>
                        <TableCell>"The setting takes effect immediately"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><strong>"Checkbox vs Radio"</strong></TableCell>
                        <TableCell>"Any number of independent options can be on"</TableCell>
                        <TableCell>"Exactly one of a small, visible set is chosen"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><strong>"Radio vs "<Link href=routes::doc::Select.materialize()>"Select"</Link></strong></TableCell>
                        <TableCell>"Two to five options that users should compare at a glance"</TableCell>
                        <TableCell>"A longer list, or little space"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><strong>"Text Field vs Number Field"</strong></TableCell>
                        <TableCell>"Any text: names, email addresses, passwords"</TableCell>
                        <TableCell>"A number, formatted for the user\u{2019}s locale, kept in a range and stepped with buttons or keys"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><strong>"Text Field vs Search Field"</strong></TableCell>
                        <TableCell>"Text that is part of a form"</TableCell>
                        <TableCell>"A search query that runs on its own, submitted with "<Keys keys="Enter"/></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><strong>"Number Field vs Slider"</strong></TableCell>
                        <TableCell>"The exact value matters"</TableCell>
                        <TableCell>"The position within the range matters more than the exact number, or a range has two bounds"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><strong>"Text Field vs Rich Text Editor"</strong></TableCell>
                        <TableCell>"Plain text, on one line or several"</TableCell>
                        <TableCell>"Formatted text, with headings, emphasis and block quotes"</TableCell>
                    </TableRow>
                </DocTable>
            </Section>
        </DocPage>
    }
}
