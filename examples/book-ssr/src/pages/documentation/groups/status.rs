use leptos::prelude::*;

use super::demos::status_alert::StatusAlertDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageStatus() -> impl IntoView {
    view! {
        <DocPage title="Status">
            <p>
                "Status concepts tell users about the state of the app or of a task: a value within a known range, the "
                "progress of an operation, or a notification that comes and goes. They inform; none of them asks the user "
                "for input. A message in the page, such as an error above a form, needs no leptonic piece: see "
                <AnchorLink href="#alert">"Alert"</AnchorLink>" below."
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
                        "An "<AnchorLink href="#alert">"alert"</AnchorLink>" and a "
                        <Link href=routes::doc::Toast.materialize()>"toast"</Link>" both show a message in a variant such as "
                        "success or warning. An alert sits in the flow of the page and stays; a toast appears above the app "
                        "and usually disappears after a timeout."
                    </li>
                    <li>
                        "Screen readers announce both when they appear. Keyboard users reach the toasts with "
                        <Keys keys="F6"/>", as the toast region is a "
                        <Link href=routes::doc::focus::UseLandmark.materialize()>"landmark"</Link>"; an alert is part of "
                        "the page and reached like the rest of it."
                    </li>
                </ul>
            </Section>

            <Section title="Decision Guide">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Show a lasting message in the page, such as a warning above a form"</TableCell>
                        <TableCell>"An "<AnchorLink href="#alert">"alert"</AnchorLink></TableCell>
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
                        <TableCell>"A "<Link href=format!("{}#skeleton", routes::doc::Layout.materialize())>"skeleton"</Link>" drawn with CSS"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Require a decision before the user continues"</TableCell>
                        <TableCell><Link href=routes::doc::Modal.materialize()>"Modal"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Alert">
                <p>
                    "An alert is a message in the flow of the page that tells users about something important: an "
                    "operation succeeded, something needs their attention, or an error occurred. It needs no leptonic "
                    "piece: render the message in an element with the role that fits its urgency, and style it with CSS."
                </p>
                <ul>
                    <li>
                        <Code inline=true>"role=\"alert\""</Code>" for urgent messages, such as an error: screen readers "
                        "announce it at once, interrupting what they are reading, also when you add the element to the page."
                    </li>
                    <li>
                        <Code inline=true>"role=\"status\""</Code>" for messages that can wait, such as \u{201c}Saved\u{201d}: "
                        "screen readers announce changes of its content after they finished reading. Keep the element in the "
                        "page and change its content, as a status added together with its message may not be announced."
                    </li>
                    <li>
                        "Tell the variant by the icon and the text, not by color alone, and hide decorative icons from "
                        "assistive technology ("<Code inline=true>"aria-hidden=\"true\""</Code>")."
                    </li>
                    <li>
                        "An alert doesn\u{2019}t take the focus. For a message that requires a response, use a "
                        <Link href=routes::doc::Dialog.materialize()>"dialog"</Link>" with "
                        <Code inline=true>"DialogRole::AlertDialog"</Code>"; to only announce something to screen reader "
                        "users, use the "<Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live announcer"</Link>"."
                    </li>
                </ul>
                <Demo description="A status message after saving and an alert after a failed upload" source=include_str!("demos/status_alert.rs")>
                    <StatusAlertDemo/>
                </Demo>
            </Section>
        </DocPage>
    }
}
