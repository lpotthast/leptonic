use leptonic::{prelude::*, datetime::GuideMode};
use leptos::*;
use time::OffsetDateTime;

#[component]
pub fn PageDateTime(cx: Scope) -> impl IntoView {
    view! { cx,
        <Typography variant=TypographyVariant::H2>"Date & Time"</Typography>

        <DateSelector value=OffsetDateTime::now_utc()/>

        <DateSelector value=OffsetDateTime::now_utc() guide_mode=GuideMode::YearFirst/>
    }
}
