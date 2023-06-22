use leptonic::{prelude::*, datetime::GuideMode};
use leptos::*;
use time::OffsetDateTime;

#[component]
pub fn PageDateTime(cx: Scope) -> impl IntoView {
    view! { cx,
        <H2>"Date & Time"</H2>

        <DateSelector value=OffsetDateTime::now_utc() on_change=move |_v| {}/>

        <DateSelector value=OffsetDateTime::now_utc() on_change=move |_v| {} guide_mode=GuideMode::YearFirst/>
    }
}
