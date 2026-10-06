use std::sync::Arc;

use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;
use time::{OffsetDateTime, Weekday};

// leptonic has no segment atom yet: the demos share this component built from `use_date_segment`.
use super::date_segments::{DateSegments, SegmentControls};

#[component]
pub fn DateFieldDemo() -> impl IntoView {
    let (value, set_value) = signal(None::<OffsetDateTime>);
    let disabled = RwSignal::new(false);

    let field = use_date_field(UseDateFieldInput {
        value: value.into(),
        is_disabled: disabled.into(),
        label: Some("Delivery date".to_owned()),
        description: Some("We deliver on weekdays.".to_owned()),
        validate: Some(Arc::new(|value: &Option<OffsetDateTime>| match value {
            Some(date) if matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday) => {
                Err(vec!["Pick a weekday.".to_owned()])
            }
            _ => Ok(()),
        })),
        on_change: Some(Callback::new(move |new_value| set_value.set(new_value))),
        ..Default::default()
    });

    let controls = SegmentControls::from(&field);
    let is_invalid = field.is_invalid;
    let validation_errors = field.validation_errors;

    view! {
        <div class="demo-date-field">
            <span id=field.label_props.id class="demo-date-field-label">"Delivery date"</span>
            <div {..field.field_props.into_attrs()} class="demo-date-field-input">
                <DateSegments controls is_disabled=disabled is_read_only=false is_invalid/>
            </div>
            <span id=field.description_props.id class="demo-date-field-description">"We deliver on weekdays."</span>
            <span
                id=field.error_props.id
                role=field.error_props.role
                aria-live=field.error_props.aria_live
                class="demo-date-field-error"
            >
                {move || validation_errors.get().join(" ")}
            </span>
        </div>

        <p class="demo-status">
            {move || value.get().map_or_else(|| "No date entered".to_owned(), |date| format!("Delivery on {}", date.date()))}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
