use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageInputCategory() -> impl IntoView {
    view! {
        <DocPage title="Input">
            <p>
                "Components that accept user input \u{2014} from simple buttons and checkboxes to rich text editors and date pickers."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::InputCategory.materialize()/>
            </Section>

            <Section title="Decision Guide">
                <p>"Choosing between similar input controls:"</p>

                <DocTable headers=&["Choice", "When to pick the first", "When to pick the second"]>
                    <TableRow>
                        <TableCell><strong>"Button vs Link"</strong></TableCell>
                        <TableCell>"Performs an action (submit, delete, toggle)"</TableCell>
                        <TableCell>"Navigates to a URL or anchor"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><strong>"Checkbox vs Toggle"</strong></TableCell>
                        <TableCell>"Multi-select from a group, or form-submitted boolean"</TableCell>
                        <TableCell>"Binary switch with immediate effect (settings)"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><strong>"Select vs Combobox"</strong></TableCell>
                        <TableCell>"Short list, user picks from fixed options"</TableCell>
                        <TableCell>"Long list, user benefits from typing to filter"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><strong>"Select vs Listbox"</strong></TableCell>
                        <TableCell>"Space-constrained, options hidden until opened"</TableCell>
                        <TableCell>"All options should be visible at once"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><strong>"Text Field vs Search Field"</strong></TableCell>
                        <TableCell>"General-purpose text input"</TableCell>
                        <TableCell>"Input specifically for search queries (has clear button)"</TableCell>
                    </TableRow>
                </DocTable>
            </Section>
        </DocPage>
    }
}
