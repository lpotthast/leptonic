use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::date_time_input::DateTimeInputDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageDateTimeInput() -> impl IntoView {
    view! {
        <DocPage title="Date Picker Component">
            <p>
                "The themed "<Code inline=true>"DateTimeInput"</Code>" component is leptonic\u{2019}s styled date picker: it "
                "shows the selected date in an input and opens a "
                <Link href=routes::doc::calendar::Component.materialize()><Code inline=true>"DateSelector"</Code></Link>
                " below it when you click the input or press "<Keys keys="ArrowDown"/>", "<Keys keys="Enter"/>" or "
                <Keys keys="Space"/>". See the "<Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link>
                " overview for concept guidance."
            </p>
            <p>
                "It is unfinished: the time selector is not implemented yet, the input needs a value before you open "
                "the dropdown, and the input can\u{2019}t be typed into. Build accessible date pickers from the "
                <Link href=routes::doc::date_picker::Hook.materialize()>"Date Picker Hooks"</Link>"."
            </p>

            <Demo description="Date input opening a date selector, with a disabled toggle" source=include_str!("demos/date_time_input.rs")>
                <DateTimeInputDemo/>
            </Demo>

            <Section title="DateTimeInput">
                <Section title="Props" id="date-time-input-props">
                    <ApiTable kind=ApiKind::Props of="DateTimeInput">
                        <ApiRow name="get" ty="Signal<Option<time::OffsetDateTime>>">
                            "Required. The value, shown in RFC 3339 format. Must be "<Code inline=true>"Some"</Code>
                            " when the dropdown opens."
                        </ApiRow>
                        <ApiRow name="set" ty="Out<Option<time::OffsetDateTime>>">"Required. Receives the date the user selects."</ApiRow>
                        <ApiRow name="input_type" ty="Type" default="Type::DateTime">
                            "What the dropdown offers: "<Code inline=true>"Date"</Code>" (the date selector), "
                            <Code inline=true>"Time"</Code>" or "<Code inline=true>"DateTime"</Code>". The time selector is a placeholder."
                        </ApiRow>
                        <ApiRow name="min, max" ty="Option<time::OffsetDateTime>" default="None">"The earliest and latest selectable date."</ApiRow>
                        <ApiRow name="guide_mode" ty="GuideMode" default="GuideMode::CalendarFirst">
                            "Which view the date selector starts with, see "
                            <Link href=format!("{}#guide-mode", routes::doc::calendar::Component.materialize())>"Guide Mode"</Link>"."
                        </ApiRow>
                        <ApiRow name="label" ty="Signal<Oco<'static, str>>" default="\"\"">"The input\u{2019}s placeholder."</ApiRow>
                        <ApiRow name="prepend" ty="ViewFn" default="empty">"Content rendered before the input."</ApiRow>
                        <ApiRow name="id" ty="Option<Oco<'static, str>>" default="None">"The input\u{2019}s id."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the input is disabled."</ApiRow>
                        <ApiRow name="margin" ty="Option<Margin>" default="None">"The margin around the field."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The dropdown is the "<Code inline=true>"DateSelector"</Code>", which works with the mouse only (see "
                    <Link href=format!("{}#accessibility", routes::doc::calendar::Component.materialize())>"Calendar Component"</Link>
                    "), and the input has no label of its own: "<Code inline=true>"label"</Code>" only sets its placeholder."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The input is styled with the theme\u{2019}s input variables; the dropdown with the "
                    <Link href=format!("{}#styling", routes::doc::calendar::Component.materialize())>"CSS variables of the Calendar Component"</Link>"."
                </p>
                <CssVariables prefix="--input-" scss=theme_scss!("input")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::DatePicker.materialize()>"Date Picker"</Link></li>
                <li><Link href=routes::doc::date_picker::Hook.materialize()>"Date Picker Hooks"</Link></li>
                <li><Link href=routes::doc::calendar::Component.materialize()>"Calendar Component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
