use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PagePickers() -> impl IntoView {
    view! {
        <DocPage title="Pickers">
            <p>
                "Pickers let users choose from a list of options that opens on demand, so the options take no space until "
                "they are needed. A select opens its list from a button; a combobox opens it from a text input, and what "
                "the user types filters the options."
            </p>
            <p>
                "Both are fields: they have a label, a description and an error message, and they validate like the "
                "other "<Link href=routes::doc::Fields.materialize()>"fields"</Link>" of a form. Both allow a single "
                "option or several to be selected."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Pickers.materialize()/>
            </Section>

            <Section title="Relationships">
                <p>
                    "Select and combobox show a "<Link href=routes::doc::Listbox.materialize()>"listbox"</Link>" in a "
                    <Link href=routes::doc::Popover.materialize()>"popover"</Link>". The listbox brings the keyboard "
                    "navigation and selection of its options, the popover the positioning next to the trigger and "
                    "closing on outside interaction. A combobox keeps the focus in its input while the user moves through "
                    "the options."
                </p>
            </Section>

            <Section title="Decision Guide">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Pick from a short or medium list, without typing"</TableCell>
                        <TableCell><Link href=routes::doc::Select.materialize()>"Select"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pick from a long list, such as countries, by typing part of an option\u{2019}s name"</TableCell>
                        <TableCell><Link href=routes::doc::Combobox.materialize()>"Combobox"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Accept text that matches no option, with the options as suggestions"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::Combobox.materialize()>"Combobox"</Link>" with "
                            <Code inline=true>"allows_custom_value"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Choose one of two to five options that should stay visible"</TableCell>
                        <TableCell><Link href=routes::doc::Radio.materialize()>"Radio"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Keep all options visible, without a popover"</TableCell>
                        <TableCell><Link href=routes::doc::Listbox.materialize()>"Listbox"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Run an action chosen from a list, rather than set a value"</TableCell>
                        <TableCell><Link href=routes::doc::Menu.materialize()>"Menu"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>
        </DocPage>
    }
}
