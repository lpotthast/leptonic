use indoc::indoc;
use leptonic::{components::prelude::*, utils::time::GuideMode};
use leptos::prelude::*;
use time::OffsetDateTime;

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

            <Code>
                {indoc!(r"
                    <DateSelector value=OffsetDateTime::now_utc() on_change=move |_v| {}/>
                ")}
            </Code>

            <DateSelector value=OffsetDateTime::now_utc() on_change=move |_v| {}/>

            <p>"The date selector can also start with the year selection."</p>

            <Code>
                {indoc!(r"
                    <DateSelector value=OffsetDateTime::now_utc() on_change=move |_v| {} guide_mode=GuideMode::YearFirst/>
                ")}
            </Code>

            <DateSelector value=OffsetDateTime::now_utc() on_change=move |_v| {} guide_mode=GuideMode::YearFirst/>

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
