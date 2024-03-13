use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::utils::time::GuideMode;
use leptos::*;
use time::OffsetDateTime;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageDateTime() -> impl IntoView {
    view! {
        <Article>
            <H1 id="date-and-time" class="anchor">
                "Date & Time"
                <AnchorLink href="#date-and-time" description="Direct link to article header"/>
            </H1>

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

            <H2 id="input-fields" class="anchor">
                "Input fields"
                <AnchorLink href="#input-fields" description="Direct link to section: Input fields"/>
            </H2>

            <P>"Selecting dates and times through input fields will be supported soon. Stay tuned."</P>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Date & Time", link: "#date-and-time" },
                Toc::Leaf { title: "Input fields", link: "#input-fields" },
            ]
        }/>
    }
}
