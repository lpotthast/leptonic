use leptonic::{components::prelude::*, jiff::civil::Date};
use leptos::prelude::*;

#[component]
pub fn DatePickerConceptDemo() -> impl IntoView {
    let departure = RwSignal::new(None::<Date>);

    view! {
        <DatePicker label="Departure" value=departure set_value=departure/>

        <p class="demo-status">
            {move || {
                departure.get().map_or_else(
                    || "No departure date.".to_owned(),
                    |date| format!("Departure on {}.", date.strftime("%A, %B %-d, %Y")),
                )
            }}
        </p>
    }
}
