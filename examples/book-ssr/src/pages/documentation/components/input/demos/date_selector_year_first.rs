use leptonic::{components::prelude::*, utils::time::GuideMode};
use leptos::prelude::*;
use time::OffsetDateTime;

#[component]
pub fn DateSelectorYearFirstDemo() -> impl IntoView {
    view! {
        <DateSelector value=OffsetDateTime::now_utc() on_change=move |_v| {} guide_mode=GuideMode::YearFirst/>
    }
}
