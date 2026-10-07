use leptonic::{
    atoms::{
        datepicker::{DateField, DateInput, DateSegment},
        field::Label,
    },
    jiff::civil::Date,
};
use leptos::prelude::*;

#[component]
pub fn DateFieldConceptDemo() -> impl IntoView {
    let birthday = RwSignal::new(None::<Date>);

    view! {
        <DateField<Date> value=birthday set_value=birthday classes="demo-date-field">
            <Label classes="demo-field-label">"Birthday"</Label>
            <DateInput
                classes="demo-date-input"
                children=|segment| view! { <DateSegment segment classes="demo-date-segment"/> }
            />
        </DateField<Date>>

        <p class="demo-status">
            {move || {
                birthday.get().map_or_else(
                    || "No date entered".to_owned(),
                    |date| format!("Born on {}", date.strftime("%B %-d, %Y")),
                )
            }}
        </p>
    }
}
