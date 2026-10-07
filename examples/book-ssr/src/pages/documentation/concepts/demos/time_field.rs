use leptonic::{
    atoms::{
        datepicker::{DateInput, DateSegment, TimeField},
        field::Label,
    },
    jiff::civil::Time,
};
use leptos::prelude::*;

#[component]
pub fn TimeFieldConceptDemo() -> impl IntoView {
    let alarm = RwSignal::new(None::<Time>);

    view! {
        <TimeField<Time> value=alarm set_value=alarm classes="demo-date-field">
            <Label classes="demo-field-label">"Alarm"</Label>
            <DateInput
                classes="demo-date-input"
                children=|segment| view! { <DateSegment segment classes="demo-date-segment"/> }
            />
        </TimeField<Time>>

        <p class="demo-status">
            {move || {
                alarm.get().map_or_else(
                    || "No alarm set".to_owned(),
                    |time| format!("Alarm at {}", time.strftime("%H:%M")),
                )
            }}
        </p>
    }
}
