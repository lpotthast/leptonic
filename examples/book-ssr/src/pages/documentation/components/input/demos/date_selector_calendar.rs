use leptonic::components::prelude::*;
use leptos::prelude::*;
use time::OffsetDateTime;

#[component]
pub fn DateSelectorCalendarDemo() -> impl IntoView {
    let (date, set_date) = signal(OffsetDateTime::now_utc());

    view! {
        <DateSelector value=date.get_untracked() on_change=set_date/>
        <p class="demo-status">"Selected date: "{move || date.get().date().to_string()}</p>
    }
}
