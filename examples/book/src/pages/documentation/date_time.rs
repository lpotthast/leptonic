use leptonic::{prelude::*, datetime::GuideMode};
use leptos::*;
use time::OffsetDateTime;

#[component]
pub fn PageDateTime(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"Date & Time"</H1>

        <DateSelector value=OffsetDateTime::now_utc() on_change=move |_v| {}/>

        <DateSelector value=OffsetDateTime::now_utc() on_change=move |_v| {} guide_mode=GuideMode::YearFirst/>
    }
}
