use leptonic::{
    atoms::{
        checkbox::{CheckboxButton, CheckboxField},
        datepicker::{DateInput, DateSegment, TimeField},
        field::{Description, FieldError, Label},
    },
    hooks::ValidationBehavior,
    jiff::civil::{Time, time},
};
use leptos::prelude::*;

#[component]
pub fn TimeFieldAtomDemo() -> impl IntoView {
    let pickup = RwSignal::new(Some(time(10, 0, 0, 0)));
    let disabled = RwSignal::new(false);

    view! {
        <TimeField<Time>
            value=pickup
            set_value=pickup
            min_value=time(8, 0, 0, 0)
            max_value=time(18, 0, 0, 0)
            validation_behavior=ValidationBehavior::Aria
            is_disabled=disabled
            classes="demo-date-field"
        >
            <Label classes="demo-field-label">"Pickup time"</Label>
            <DateInput
                classes="demo-date-input"
                children=|segment| view! { <DateSegment segment classes="demo-date-segment"/> }
            />
            <Description classes="demo-field-description">"Between 8 AM and 6 PM."</Description>
            <FieldError classes="demo-field-error"/>
        </TimeField<Time>>

        <p class="demo-status">
            {move || {
                pickup.get().map_or_else(
                    || "No time entered.".to_owned(),
                    |pickup| {
                        let outside = pickup < time(8, 0, 0, 0) || pickup > time(18, 0, 0, 0);
                        let invalid = if outside { " (invalid)" } else { "" };
                        format!("Pickup at {}{invalid}.", pickup.strftime("%H:%M"))
                    },
                )
            }}
        </p>

        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
