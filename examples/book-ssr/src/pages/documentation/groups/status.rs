use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageStatus() -> impl IntoView {
    view! {
        <DocPage title="Status">
            <p>
                "Status concepts tell users about the state of the app or of a task: a message that something succeeded or "
                "needs attention, a value within a known range, the progress of an operation, or a notification that "
                "comes and goes. They inform; none of them asks the user for input."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Status.materialize()/>
            </Section>

            <Section title="Relationships">
                <ul>
                    <li>
                        <Link href=routes::doc::Meter.materialize()>"Meter"</Link>" and "
                        <Link href=routes::doc::ProgressBar.materialize()>"Progress Bar"</Link>" both show a value within a "
                        "range. A progress bar tracks a task toward its end and can be indeterminate when the end is "
                        "unknown; a meter shows a measurement, such as disk usage, that is not a task."
                    </li>
                    <li>
                        <Link href=routes::doc::Alert.materialize()>"Alert"</Link>" and "
                        <Link href=routes::doc::Toast.materialize()>"Toast"</Link>" both show a message in a variant such as "
                        "success or warning. An alert sits in the flow of the page and stays; a toast appears above the app "
                        "and usually disappears after a timeout."
                    </li>
                    <li>
                        "Screen readers announce an alert as soon as it appears, interrupting the user. They don\u{2019}t "
                        "notice a toast: announce its message with "
                        <Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live_announcer"</Link>
                        " when it matters to the user."
                    </li>
                </ul>
            </Section>

            <Section title="Decision Guide">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Show a lasting message in the page, such as a warning above a form"</TableCell>
                        <TableCell><Link href=routes::doc::Alert.materialize()>"Alert"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Confirm briefly that something happened, without interrupting the user"</TableCell>
                        <TableCell><Link href=routes::doc::Toast.materialize()>"Toast"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show how far a task has advanced, or that it is still running"</TableCell>
                        <TableCell><Link href=routes::doc::ProgressBar.materialize()>"Progress Bar"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a value within a known range, such as disk usage or password strength"</TableCell>
                        <TableCell><Link href=routes::doc::Meter.materialize()>"Meter"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Hold the place of content that is still loading"</TableCell>
                        <TableCell><Link href=routes::doc::Skeleton.materialize()>"Skeleton"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Require a decision before the user continues"</TableCell>
                        <TableCell><Link href=routes::doc::Modal.materialize()>"Modal"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Label an item with a short status, such as \u{201c}Draft\u{201d}"</TableCell>
                        <TableCell><Link href=routes::doc::Chip.materialize()>"Chip"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>
        </DocPage>
    }
}
