use leptonic::{
    atoms::{
        checkbox::Checkbox,
        datepicker::{DateField, DateInput, DateSegment},
        field::Label,
    },
    hooks::datepicker::HourCycle,
    jiff::{Zoned, civil::date},
};
use leptos::prelude::*;

#[component]
pub fn DateFieldZonedDemo() -> impl IntoView {
    // A zoned value: the field edits the date and time in New York and shows the time zone.
    let call = RwSignal::new(
        date(2026, 3, 12)
            .at(9, 30, 0, 0)
            .in_tz("America/New_York")
            .ok(),
    );
    let read_only = RwSignal::new(false);
    let disabled = RwSignal::new(false);

    view! {
        <DateField<Zoned>
            value=call
            set_value=call
            hour_cycle=HourCycle::H24
            is_read_only=read_only
            is_disabled=disabled
            classes="demo-date-field"
        >
            <Label classes="demo-field-label">"Call with the New York office"</Label>
            <DateInput
                classes="demo-date-input"
                children=|segment| view! { <DateSegment segment classes="demo-date-segment"/> }
            />
        </DateField<Zoned>>

        <p class="demo-status">
            {move || {
                call.get().map_or_else(
                    || "No time entered".to_owned(),
                    |call| format!("Call at {}", call.strftime("%Y-%m-%d %H:%M %Z")),
                )
            }}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=read_only set_selected=read_only classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Read-only"
            </Checkbox>
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}
