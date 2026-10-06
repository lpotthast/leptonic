use leptonic::{components::prelude::*, utils::time::GuideMode};
use leptos::prelude::*;
use time::OffsetDateTime;

#[component]
pub fn DateSelectorYearFirstDemo() -> impl IntoView {
    let (date, set_date) = signal(OffsetDateTime::now_utc());

    view! {
        <DateSelector value=date.get_untracked() on_change=set_date guide_mode=GuideMode::YearFirst/>
        <p class="demo-status">"Selected date: "{move || date.get().date().to_string()}</p>
    }
}
