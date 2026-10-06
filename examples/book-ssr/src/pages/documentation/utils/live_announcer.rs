use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::live_announcer::LiveAnnouncerDemo;
use crate::kit::*;

#[component]
pub fn PageLiveAnnouncer() -> impl IntoView {
    view! {
        <DocPage title="Live Announcer">
            <p>
                "Screen readers announce what has focus and what changes in the accessibility tree. Some changes have neither: "
                "\u{201c}3 results available\u{201d} in a combo box, the new value of a spin button, a background save that "
                "failed. The live announcer (in "<Code inline=true>"leptonic::utils::live_announcer"</Code>
                ") tells screen readers about them through ARIA live regions."
            </p>

            <Section title="Demo">
                <p>
                    "Turn on a screen reader (VoiceOver, NVDA, TalkBack, \u{2026}) and press the buttons. The polite message is "
                    "read after the current speech; the assertive one interrupts it."
                </p>
                <Demo description="Buttons announcing polite and assertive messages" source=include_str!("demos/live_announcer.rs")>
                    <LiveAnnouncerDemo/>
                </Demo>
            </Section>

            <Section title="Announcing">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::utils::live_announcer::{announce_polite, announce_assertive, Assertiveness, clear_announcer};

                        announce_polite("Sorted by name, ascending.");
                        announce_assertive("Connection lost.");
                        clear_announcer(Some(Assertiveness::Polite));
                    "#)}
                </Code>

                <DocTable headers=&["Function", "Description"]>
                    <TableRow>
                        <TableCell><Code inline=true>"announce_polite(message)"</Code></TableCell>
                        <TableCell>"Read at the next graceful opportunity, e.g. after the current sentence. Use this for most messages."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"announce_assertive(message)"</Code></TableCell>
                        <TableCell>"Read immediately, interrupting current speech. Reserve it for urgent messages."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"announce(message, assertiveness)"</Code></TableCell>
                        <TableCell>"Either of the above, by "<Code inline=true>"Assertiveness::Polite"</Code>" or "<Code inline=true>"Assertive"</Code>"."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"announce_with_timeout(message, assertiveness, timeout)"</Code></TableCell>
                        <TableCell>
                            "Keep the message in its live region for "<Code inline=true>"timeout"</Code>" instead of "
                            <Code inline=true>"DEFAULT_ANNOUNCEMENT_TIMEOUT"</Code>" (7 seconds)."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"clear_announcer(assertiveness)"</Code></TableCell>
                        <TableCell>"Remove pending messages of one kind, or of both with "<Code inline=true>"None"</Code>"."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"destroy_announcer()"</Code></TableCell>
                        <TableCell>"Remove the live regions from the document; the next announcement creates them again."</TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "A message is an "<Code inline=true>"Announcement"</Code>": plain text ("<Code inline=true>"&str"</Code>
                    " and "<Code inline=true>"String"</Code>" convert into it), or "<Code inline=true>"Announcement::LabelledBy(ids)"</Code>
                    " to announce the accessible name of elements already in the page, for richer content."
                </p>
            </Section>

            <Section title="How it works">
                <p>
                    "On the first announcement, a single visually hidden element with one "<Code inline=true>"assertive"</Code>
                    " and one "<Code inline=true>"polite"</Code>" log region is added to "<Code inline=true>"<body>"</Code>
                    ". Each message becomes a child of its region and is removed after its timeout. Announcing does nothing "
                    "during server-side rendering. Leptonic\u{2019}s own hooks use the announcer too, e.g. drag and drop, "
                    "spin buttons and calendars."
                </p>
            </Section>
        </DocPage>
    }
}
