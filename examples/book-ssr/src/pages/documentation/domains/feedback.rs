use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageFeedback() -> impl IntoView {
    view! {
        <DocPage title="Feedback">
            <p>
                "Components that communicate status, results, or contextual information to the user \u{2014} from inline chips and tooltips to modal dialogs and toast notifications."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Feedback.materialize()/>
            </Section>

            <Section title="Decision Guide">
                <p>"Choosing between feedback mechanisms:"</p>

                <DocTable headers=&["Choice", "When to pick the first", "When to pick the second"]>
                    <TableRow>
                        <TableCell><strong>"Tooltip vs Popover"</strong></TableCell>
                        <TableCell>"Brief, text-only hint (no interaction needed)"</TableCell>
                        <TableCell>"Rich content with links, buttons, or forms"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><strong>"Modal vs Toast"</strong></TableCell>
                        <TableCell>"Blocking action that requires user decision"</TableCell>
                        <TableCell>"Transient notification that auto-dismisses"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><strong>"Modal vs Alert"</strong></TableCell>
                        <TableCell>"Interactive dialog with buttons/forms"</TableCell>
                        <TableCell>"Static, persistent status message in the page flow"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><strong>"Chip vs Progress"</strong></TableCell>
                        <TableCell>"Static status label or tag"</TableCell>
                        <TableCell>"Dynamic progress of an ongoing operation"</TableCell>
                    </TableRow>
                </DocTable>
            </Section>
        </DocPage>
    }
}
