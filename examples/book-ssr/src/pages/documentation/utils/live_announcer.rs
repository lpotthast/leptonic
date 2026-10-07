use indoc::indoc;
use leptos::prelude::*;

use super::demos::live_announcer::LiveAnnouncerDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageLiveAnnouncer() -> impl IntoView {
    view! {
        <DocPage title="live_announcer">
            <p>
                "Screen readers announce what has focus and what changes in the accessibility tree. Some changes have neither: "
                "\u{201c}3 results available\u{201d} in a combobox, the new value of a spin button, a background save that "
                "failed. The functions of "<Code inline=true>"leptonic::utils::live_announcer"</Code>
                " tell screen readers about them through ARIA live regions. They are one of the "
                "building blocks for screen readers, next to "
                <Link href=routes::doc::screen_readers::VisuallyHidden.materialize()>"VisuallyHidden"</Link>"."
            </p>

            <ReactAriaSource path="live-announcer/LiveAnnouncer.tsx"/>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::utils::live_announcer::{announce_assertive, announce_polite};

                        announce_polite("Sorted by name, ascending.");
                        announce_assertive("Connection lost.");
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Turn on a screen reader (VoiceOver, NVDA, TalkBack, \u{2026}) and press the buttons. The polite message is "
                    "read after the current speech; the assertive one interrupts it."
                </p>
                <Demo description="Buttons announcing polite and assertive messages" source=include_str!("demos/live_announcer.rs")>
                    <LiveAnnouncerDemo/>
                </Demo>
            </Section>

            <Section title="announce_polite">
                <p>
                    <Code inline=true>"announce_polite(message)"</Code>" reads the message at the next graceful opportunity, "
                    "e.g. after the current sentence. Use it for most messages."
                </p>
            </Section>

            <Section title="announce_assertive">
                <p>
                    <Code inline=true>"announce_assertive(message)"</Code>" reads the message immediately, interrupting the "
                    "current speech. Reserve it for urgent messages, such as errors that need attention now."
                </p>
            </Section>

            <Section title="announce">
                <p>
                    <Code inline=true>"announce(message, assertiveness)"</Code>" is either of the above, chosen by an "
                    <AnchorLink href="#assertiveness"><Code inline=true>"Assertiveness"</Code></AnchorLink>
                    ", for code that decides the urgency at runtime."
                </p>
            </Section>

            <Section title="announce_with_timeout">
                <p>
                    <Code inline=true>"announce_with_timeout(message, assertiveness, timeout)"</Code>" keeps the message in "
                    "its live region for "<Code inline=true>"timeout"</Code>" (a "<Code inline=true>"Duration"</Code>
                    ") instead of "<Code inline=true>"DEFAULT_ANNOUNCEMENT_TIMEOUT"</Code>" (7 seconds). The other functions "
                    "use the default."
                </p>
            </Section>

            <Section title="clear_announcer">
                <p>
                    <Code inline=true>"clear_announcer(assertiveness)"</Code>" removes the pending messages of one kind ("
                    <Code inline=true>"Some(Assertiveness::Polite)"</Code>"), or of both with "<Code inline=true>"None"</Code>
                    ", e.g. when the state they describe is gone."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::utils::live_announcer::{Assertiveness, clear_announcer};

                        clear_announcer(Some(Assertiveness::Polite));
                    ")}
                </Code>
            </Section>

            <Section title="destroy_announcer">
                <p>
                    <Code inline=true>"destroy_announcer()"</Code>" removes the live regions from the document; the next "
                    "announcement creates them again."
                </p>
            </Section>

            <Section title="Announcement">
                <p>
                    "The message of every function is an "<Code inline=true>"impl Into<Announcement>"</Code>":"
                </p>
                <DocTable headers=&["Variant", "Announces"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Announcement::Text(String)"</Code></TableCell>
                        <TableCell>
                            "Plain text. "<Code inline=true>"&str"</Code>" and "<Code inline=true>"String"</Code>
                            " convert into it, so you rarely write it out."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Announcement::LabelledBy(Vec<String>)"</Code></TableCell>
                        <TableCell>
                            "The accessible name of elements already in the page, given by their ids in order, like "
                            <Code inline=true>"aria-labelledby"</Code>". Use it for rich content you render anyway."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Assertiveness">
                <p>
                    <Code inline=true>"Assertiveness::Polite"</Code>" (the default) or "<Code inline=true>"Assertiveness::Assertive"</Code>
                    ", the urgency of an announcement. "<Code inline=true>"as_aria_live()"</Code>" returns the matching "
                    <Code inline=true>"aria-live"</Code>" value."
                </p>
            </Section>

            <Section title="How It Works">
                <p>
                    "On the first announcement, a single "
                    <Link href=routes::doc::screen_readers::VisuallyHidden.materialize()>"visually hidden"</Link>
                    " element with one "<Code inline=true>"assertive"</Code>" and one "<Code inline=true>"polite"</Code>
                    " log region is added to "<Code inline=true>"<body>"</Code>". The first message waits 100 ms for the "
                    "regions to be registered, so that Safari reads it too. Each message becomes a child of its region and "
                    "is removed after its timeout. Announcing does nothing during server-side rendering. Leptonic\u{2019}s "
                    "own hooks use the announcer too, e.g. drag and drop, spin buttons and calendars."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::screen_readers::VisuallyHidden.materialize()>"VisuallyHidden"</Link></li>
                <li><Link href=routes::doc::screen_readers::UseVisuallyHidden.materialize()>"use_visually_hidden"</Link></li>
                <li><Link href=routes::doc::screen_readers::UseDescription.materialize()>"use_description"</Link></li>
                <li><Link href=routes::doc::Accessibility.materialize()>"Accessibility"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
