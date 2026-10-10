use leptonic::{
    atoms::{
        checkbox::{CheckboxButton, CheckboxField},
        datepicker::{DateInput, DateSegment, TimeField},
        field::{Description, FieldError, Label},
    },
    hooks::{datepicker::TimeBound, form::ValidationBehavior},
    jiff::civil::{Date, DateTime, date},
};
use leptos::prelude::*;

/// The day of the delivery: the order day, or the day after.
fn delivery_day(next_day: bool) -> Date {
    let order_day = date(2026, 6, 5);
    if next_day {
        order_day.tomorrow().unwrap_or(order_day)
    } else {
        order_day
    }
}

#[component]
pub fn TimeFieldBoundDemo() -> impl IntoView {
    // Ordered on June 5, the parcel can arrive from 2 PM that day: a bound on one date and time,
    // not a time of every day.
    let earliest = delivery_day(false).at(14, 0, 0, 0);
    let next_day = RwSignal::new(false);
    let delivery = RwSignal::new(Some(delivery_day(false).at(11, 0, 0, 0)));

    // Moves the delivery to the chosen day, keeping its time.
    let choose_day = move |next: bool| {
        next_day.set(next);
        delivery.update(|delivery| {
            if let Some(delivery) = delivery {
                *delivery = delivery_day(next).to_datetime(delivery.time());
            }
        });
    };

    view! {
        <TimeField<DateTime>
            value=delivery
            set_value=delivery
            // A time typed into the empty field lands on the chosen day.
            placeholder_value=Signal::derive(move || Some(delivery_day(next_day.get()).at(0, 0, 0, 0)))
            min_value=TimeBound::Absolute(earliest)
            validation_behavior=ValidationBehavior::Aria
            classes="demo-date-field"
        >
            <Label classes="demo-field-label">"Delivery time"</Label>
            <DateInput
                classes="demo-date-input"
                children=|segment| view! { <DateSegment segment classes="demo-date-segment"/> }
            />
            <Description classes="demo-field-description">"Delivery starts at 2 PM on the order day."</Description>
            <FieldError classes="demo-field-error"/>
        </TimeField<DateTime>>

        <p class="demo-status">
            {move || {
                delivery.get().map_or_else(
                    || "No time entered.".to_owned(),
                    |delivery| {
                        let invalid = if delivery < earliest { " (too early)" } else { "" };
                        format!("Delivery on {}{invalid}.", delivery.strftime("%B %-d at %H:%M"))
                    },
                )
            }}
        </p>

        <div class="demo-controls">
            <CheckboxField is_selected=next_day set_selected=choose_day>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Deliver the next day"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
