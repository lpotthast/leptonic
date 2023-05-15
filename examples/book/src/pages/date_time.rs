use leptonic::{prelude::*, datetime::GuideMode};
use leptos::*;
use time::OffsetDateTime;

#[component]
pub fn PageDateTime(cx: Scope) -> impl IntoView {
    view! { cx,
        <H2>"Date & Time"</H2>

        <DateSelector value=OffsetDateTime::now_utc()/>

        <DateSelector value=OffsetDateTime::now_utc() guide_mode=GuideMode::YearFirst/>
    }
}
