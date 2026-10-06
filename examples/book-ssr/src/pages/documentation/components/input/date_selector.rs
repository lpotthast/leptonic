use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    date_selector_calendar::DateSelectorCalendarDemo,
    date_selector_year_first::DateSelectorYearFirstDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageDateSelector() -> impl IntoView {
    view! {
        <DocPage title="Calendar Component">
            <p>
                "The themed "<Code inline=true>"DateSelector"</Code>" component is leptonic\u{2019}s styled calendar: you "
                "navigate between months, switch to a month or year overview, and choose a day. See the "
                <Link href=routes::doc::Calendar.materialize()>"Calendar"</Link>" overview for concept guidance."
            </p>

            <Demo description="Date selector starting with the calendar" source=include_str!("demos/date_selector_calendar.rs")>
                <DateSelectorCalendarDemo/>
            </Demo>

            <Section title="DateSelector">
                <Section title="Props" id="date-selector-props">
                    <ApiTable kind=ApiKind::Props of="DateSelector">
                        <ApiRow name="value" ty="time::OffsetDateTime">
                            "Required. The initially selected date. The selector does not follow later changes of your value."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Out<time::OffsetDateTime>">
                            "Required. Receives the date the user selects."
                        </ApiRow>
                        <ApiRow name="min, max" ty="Option<time::OffsetDateTime>" default="None">
                            "The earliest and latest selectable date."
                        </ApiRow>
                        <ApiRow name="guide_mode" ty="Signal<GuideMode>" default="GuideMode::CalendarFirst">
                            "Which view the selector starts with, see "<AnchorLink href="#guide-mode">"Guide Mode"</AnchorLink>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Guide Mode">
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

            <Section title="Accessibility">
                <p>
                    "The selector predates the "<Link href=routes::doc::calendar::Hook.materialize()>"Calendar Hooks"</Link>
                    " and only uses "<Code inline=true>"use_calendar_state"</Code>". Its days and navigation arrows are plain "
                    "elements that react to clicks, so it can\u{2019}t be used with the keyboard and screen readers "
                    "don\u{2019}t see a calendar grid. Build accessible calendars from the hooks."
                </p>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt the date selector to your design:"</p>
                <CssVariables prefix="--datetime-" scss=theme_scss!("datetime")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Calendar.materialize()>"Calendar"</Link></li>
                <li><Link href=routes::doc::calendar::Hook.materialize()>"Calendar Hooks"</Link></li>
                <li><Link href=routes::doc::date_picker::Component.materialize()>"Date Picker Component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
