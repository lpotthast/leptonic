use indoc::indoc;
use leptonic::{utils::time::GuideMode, prelude::*};
use leptos::*;
use time::OffsetDateTime;

#[component]
pub fn PageDateTime() -> impl IntoView {
    view! {
        <H1>"Date & Time"</H1>

        <P>"Select dates using the calendar-like "<Code inline=true>"<DateSelector>"</Code>" component."</P>

        <Code>
            {indoc!(r"
                <DateSelector value=OffsetDateTime::now_utc() on_change=move |_v| {}/>
            ")}
        </Code>

        <DateSelector value=OffsetDateTime::now_utc() on_change=move |_v| {}/>

        <P>"The date selector can also start with the year selection."</P>

        <Code>
            {indoc!(r"
                <DateSelector value=OffsetDateTime::now_utc() on_change=move |_v| {} guide_mode=GuideMode::YearFirst/>
            ")}
        </Code>

        <DateSelector value=OffsetDateTime::now_utc() on_change=move |_v| {} guide_mode=GuideMode::YearFirst/>

        <H2>"Input fields"</H2>

        <P>"Selecting dates and times through input fields will be supported soon. Stay tuned."</P>
    }
}
