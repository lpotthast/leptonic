use std::sync::Arc;

use leptonic::{
    atoms::{
        datepicker::{DateField, DateInput, DateSegment},
        field::{Description, FieldError, Label},
    },
    components::prelude::Checkbox,
    hooks::ValidationBehavior,
    jiff::civil::{Date, Weekday},
};
use leptos::prelude::*;

#[component]
pub fn DateFieldAtomDemo() -> impl IntoView {
    let delivery = RwSignal::new(None::<Date>);
    let disabled = RwSignal::new(false);

    view! {
        <DateField<Date>
            value=delivery
            set_value=delivery
            validate=Arc::new(|date: &Option<Date>| match date {
                Some(date) if matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday) => {
                    Err(vec!["We deliver on weekdays only.".to_owned()])
                }
                _ => Ok(()),
            })
            validation_behavior=ValidationBehavior::Aria
            is_disabled=disabled
            classes="demo-date-field"
        >
            <Label classes="demo-field-label">"Delivery date"</Label>
            <DateInput
                classes="demo-date-input"
                children=|segment| view! { <DateSegment segment classes="demo-date-segment"/> }
            />
            <Description classes="demo-field-description">"Monday to Friday."</Description>
            <FieldError classes="demo-field-error"/>
        </DateField<Date>>

        <p class="demo-status">
            {move || {
                delivery.get().map_or_else(
                    || "No date entered.".to_owned(),
                    |date| {
                        let weekend = matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday);
                        let invalid = if weekend { " (invalid)" } else { "" };
                        format!("Delivery on {}{invalid}.", date.strftime("%A, %B %-d, %Y"))
                    },
                )
            }}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
