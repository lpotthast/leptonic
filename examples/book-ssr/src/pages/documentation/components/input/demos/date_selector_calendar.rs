use leptonic::components::prelude::*;
use leptos::prelude::*;
use time::OffsetDateTime;

#[component]
pub fn DateSelectorCalendarDemo() -> impl IntoView {
    view! {
        <DateSelector value=OffsetDateTime::now_utc() on_change=move |_v| {}/>
    }
}
