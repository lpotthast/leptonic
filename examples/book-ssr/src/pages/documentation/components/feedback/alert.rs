use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{alert_custom::AlertCustomDemo, alert_variants::AlertVariantsDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageAlert() -> impl IntoView {
    view! {
        <DocPage title="Alert">
            <p>
                "An alert is a highlighted box that tells your users about something important: an operation succeeded, "
                "something needs their attention, or an error occurred. The "<Code inline=true>"Alert"</Code>
                " component shows a title, content and an icon, colored by its variant."
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
                    " also accept an inline "<Code inline=true>"style"</Code>" string. "
                    "The "<Code inline=true>"AlertIcon"</Code>" component renders the icon of a variant on its own."
                </p>
            </Section>

            <Section title="Customization">
                <p>
                    "Title and content are both optional. Set "<Code inline=true>"default_icon_slot"</Code>" to "
                    <Code inline=true>"AlertIconSlot::None"</Code>" and fill the "<Code inline=true>"AlertPrepend"</Code>
                    " and "<Code inline=true>"AlertAppend"</Code>" slots to replace the icon, or place an "
                    <Code inline=true>"AlertIcon"</Code>" wherever you like. To restyle the slots, pass "
                    <Code inline=true>"classes"</Code>" to the alert and target its "
                    <Code inline=true>".leptonic-alert-*"</Code>
                    " elements."
                </p>

                <Demo
                    description="Alerts with custom prepend and append slots and an icon in the title"
                    source=include_str!("demos/alert_custom.rs")
                >
                    <AlertCustomDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt alerts to your design:"</p>
                <CssVariables prefix="--alert-" scss=theme_scss!("alert")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::components::Toast.materialize()>"Toast"</Link></li>
                <li><Link href=routes::doc::components::Icon.materialize()>"Icon"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
