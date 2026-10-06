use leptonic::{components::prelude::*, utils::time::Type};
use leptos::prelude::*;
use time::OffsetDateTime;

#[component]
pub fn DateTimeInputDemo() -> impl IntoView {
    // The input needs a value before its dropdown is opened.
    let (date, set_date) = signal(Some(OffsetDateTime::now_utc()));
    let disabled = RwSignal::new(false);

    view! {
        <DateTimeInput get=date set=set_date input_type=Type::Date is_disabled=disabled/>

        <p class="demo-status">
            {move || date.get().map_or_else(|| "No date selected".to_owned(), |date| format!("Selected: {}", date.date()))}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
