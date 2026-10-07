use leptos::prelude::*;

use super::demos::{alert_custom::AlertCustomDemo, alert_variants::AlertVariantsDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageAlert() -> impl IntoView {
    view! {
        <DocPage title="Alert Component">
            <p>
                "An alert is a highlighted box in the flow of the page that tells your users about something important: "
                "an operation succeeded, something needs their attention, or an error occurred. It stays until you remove "
                "it. The themed "<Code inline=true>"Alert"</Code>" component shows a title, content and an icon, colored "
                "by its variant. For a message that appears above the app and goes away on its own, use a "
                <Link href=routes::doc::Toast.materialize()>"Toast"</Link>"."
            </p>

            <Demo description="Alerts in all four variants" source=include_str!("demos/alert_variants.rs")>
                <AlertVariantsDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="Alert">
                    <ApiRow name="variant" ty="AlertVariant">
                        <Code inline=true>"Success"</Code>", "<Code inline=true>"Info"</Code>", "
                        <Code inline=true>"Warn"</Code>" or "<Code inline=true>"Danger"</Code>
                        ". Rendered as the "<Code inline=true>"data-variant"</Code>" attribute. Required."
                    </ApiRow>
                    <ApiRow name="default_icon_slot" ty="AlertIconSlot" default="Prepend">
                        "Where the variant\u{2019}s icon goes: "<Code inline=true>"Prepend"</Code>", "
                        <Code inline=true>"Append"</Code>" or "<Code inline=true>"None"</Code>" (no icon)."
                    </ApiRow>
                    <ApiRow name="assertiveness" ty="Assertiveness" default="Assertive">
                        "How urgently screen readers announce the alert when it appears: "<Code inline=true>"Assertive"</Code>
                        " interrupts them ("<Code inline=true>"role=\"alert\""</Code>"), "<Code inline=true>"Polite"</Code>
                        " waits until they are done ("<Code inline=true>"role=\"status\""</Code>", e.g. for \u{201c}Saved\u{201d})."
                    </ApiRow>
                    <ApiRow name="alert_prepend" ty="Option<AlertPrepend>" default="None">
                        "Slot before the text. Replaces the icon if it would go there."
                    </ApiRow>
                    <ApiRow name="alert_title" ty="Option<AlertTitle>" default="None">"The title slot."</ApiRow>
                    <ApiRow name="alert_content" ty="Option<AlertContent>" default="None">"The content slot."</ApiRow>
                    <ApiRow name="alert_append" ty="Option<AlertAppend>" default="None">
                        "Slot after the text. Replaces the icon if it would go there."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles."
                    </ApiRow>
                </ApiTable>
                <p>
                    "All slots take their content as children. "<Code inline=true>"AlertPrepend"</Code>", "
                    <Code inline=true>"AlertTitle"</Code>" and "<Code inline=true>"AlertAppend"</Code>
                    " also accept an inline "<Code inline=true>"style"</Code>" string."
                </p>
            </Section>

            <Section title="Customization">
                <p>
                    "Title and content are both optional. Set "<Code inline=true>"default_icon_slot"</Code>" to "
                    <Code inline=true>"AlertIconSlot::None"</Code>" and fill the "<Code inline=true>"AlertPrepend"</Code>
                    " and "<Code inline=true>"AlertAppend"</Code>" slots to replace the icon, or place an "
                    <AnchorLink href="#alerticon">"AlertIcon"</AnchorLink>" wherever you like. To restyle the slots, pass "
                    <Code inline=true>"classes"</Code>" to the alert and target its "
                    <Code inline=true>".leptonic-alert-*"</Code>" elements."
                </p>

                <Demo
                    description="Alerts with custom prepend and append slots and an icon in the title"
                    source=include_str!("demos/alert_custom.rs")
                >
                    <AlertCustomDemo/>
                </Demo>
            </Section>

            <Section title="AlertIcon">
                <p>
                    "The icon of a variant on its own, e.g. inside the title. It is decorative: hidden from screen readers, "
                    "like every "<Link href=routes::doc::Icon.materialize()>"Icon"</Link>"."
                </p>
                <ApiTable kind=ApiKind::Props of="AlertIcon">
                    <ApiRow name="variant" ty="AlertVariant">"The variant whose icon to show. Required."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "By default, an alert has "<Code inline=true>"role=\"alert\""</Code>": screen readers announce it right away "
                        "when it is added to the page or its content changes, interrupting what they are reading. An alert "
                        "rendered with the page is read as part of the page."
                    </li>
                    <li>
                        "For a message that shouldn\u{2019}t interrupt, such as \u{201c}Saved\u{201d}, set "
                        <Code inline=true>"assertiveness=Assertiveness::Polite"</Code>": the alert gets "
                        <Code inline=true>"role=\"status\""</Code>", and screen readers announce it once they are done "
                        "reading. For an announcement without a visible message, use "
                        <Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live_announcer"</Link>"."
                    </li>
                    <li>
                        "The variant shows only as a color and an icon, which screen readers don\u{2019}t announce. Say "
                        "what happened in the title or the content (\u{201c}Upload failed\u{201d}, not just "
                        "\u{201c}Danger\u{201d})."
                    </li>
                </ul>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt alerts to your design:"</p>
                <CssVariables prefix="--alert-" scss=theme_scss!("alert")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Status.materialize()>"Status"</Link></li>
                <li><Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live_announcer"</Link></li>
                <li><Link href=routes::doc::Icon.materialize()>"Icon Component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
