use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::date_selector_calendar::DateSelectorCalendarDemo;
use super::demos::date_selector_year_first::DateSelectorYearFirstDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageDateTime() -> impl IntoView {
    view! {
        <Article>
            <h1 id="date-and-time" class="anchor">
                "Date & Time"
                <AnchorLink href="#date-and-time" description="Direct link to article header"/>
            </h1>

            <p>"Select dates using the calendar-like "<Code inline=true>"<DateSelector>"</Code>" component."</p>

            <DemoShell source=include_str!("demos/date_selector_calendar.rs")>
                <DateSelectorCalendarDemo />
            </DemoShell>

            <p>"The date selector can also start with the year selection."</p>

            <DemoShell source=include_str!("demos/date_selector_year_first.rs")>
                <DateSelectorYearFirstDemo />
            </DemoShell>

            <h2 id="input-fields" class="anchor">
                "Input fields"
                <AnchorLink href="#input-fields" description="Direct link to section: Input fields"/>
            </h2>

            <p>"Selecting dates and times through input fields will be supported soon. Stay tuned."</p>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Date & Time", link: "#date-and-time" },
                Toc::Leaf { title: "Input fields", link: "#input-fields" },
            ]
        }/>
    }
}
