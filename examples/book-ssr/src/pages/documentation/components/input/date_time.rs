use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    date_selector_calendar::DateSelectorCalendarDemo,
    date_selector_year_first::DateSelectorYearFirstDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageDateTime() -> impl IntoView {
    view! {
        <DocPage title="Date & Time Components">
            <p>
                "The themed "<Code inline=true>"DateSelector"</Code>" component lets you pick a date from a calendar. "
                "You navigate between months, switch to a month or year overview, and choose a day. See the "
                <Link href=routes::doc::DateTime.materialize()>"Date & Time overview"</Link>" for concept guidance."
            </p>
            <p>
                "These components predate the "<Link href=routes::doc::date_time::CalendarHooks.materialize()>"calendar hooks"</Link>
                ": "<Code inline=true>"DateSelector"</Code>" only uses "<Code inline=true>"use_calendar_state"</Code>
                ". Its days and navigation arrows are plain elements that react to clicks, so it can\u{2019}t be used with "
                "the keyboard and screen readers don\u{2019}t see a calendar grid. Build accessible date pickers from the hooks."
            </p>

            <Demo description="Date selector starting with the calendar" source=include_str!("demos/date_selector_calendar.rs")>
                <DateSelectorCalendarDemo/>
            </Demo>

            <Section title="DateSelector">
                <Section title="Props">
                    <ApiTable kind=ApiKind::Props of="DateSelector">
                        <ApiRow name="value" ty="time::OffsetDateTime">
                            "The initially selected date. Required. The selector does not follow later changes of your value."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Out<time::OffsetDateTime>">
                            "Receives the date the user selects. Required."
                        </ApiRow>
                        <ApiRow name="min, max" ty="Option<time::OffsetDateTime>" default="None">
                            "The earliest and latest selectable date."
                        </ApiRow>
                        <ApiRow name="guide_mode" ty="Signal<GuideMode>" default="GuideMode::CalendarFirst">
                            "Which view the selector starts with, see "<a href="#guide-mode">"Guide mode"</a>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Guide mode">
                    <p>
                        "With "<Code inline=true>"GuideMode::YearFirst"</Code>" (from "
                        <Code inline=true>"leptonic::utils::time"</Code>"), the selector starts with the year overview "
                        "and guides you through year, month and day. This is quicker for dates far from today, such as "
                        "birthdays."
                    </p>

                    <Demo
                        description="Date selector starting with the year selection"
                        source=include_str!("demos/date_selector_year_first.rs")
                    >
                        <DateSelectorYearFirstDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="DateTimeInput">
                <p>
                    "The "<Code inline=true>"DateTimeInput"</Code>" component shows the selected date in a text input "
                    "and opens a "<Code inline=true>"DateSelector"</Code>" in a dropdown. It is unfinished: the time "
                    "selector is not implemented yet, and the input requires a value to be set before you open the dropdown."
                </p>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt the date selector to your design:"</p>
                <CssVariables prefix="--datetime-" scss=theme_scss!("datetime")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::DateTime.materialize()>"Date & Time overview"</Link></li>
                <li><Link href=routes::doc::date_time::CalendarHooks.materialize()>"Calendar hooks"</Link></li>
                <li><Link href=routes::doc::InputCategory.materialize()>"Input components"</Link></li>
                <li><Link href=routes::doc::text_field::Component.materialize()>"Input component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
