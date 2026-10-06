use leptonic::hooks::*;
use leptos::prelude::*;
use time::OffsetDateTime;

// leptonic has no segment atom yet: the demos share this component built from `use_date_segment`.
use super::date_segments::{DateSegments, SegmentControls};

#[component]
pub fn BirthdayFieldDemo() -> impl IntoView {
    let (birthday, set_birthday) = signal(None::<OffsetDateTime>);

    let field = use_date_field(UseDateFieldInput {
        value: birthday.into(),
        label: Some("Birthday".to_owned()),
        on_change: Some(Callback::new(move |date| set_birthday.set(date))),
        ..Default::default()
    });
    let controls = SegmentControls::from(&field);

    view! {
        <div class="demo-date-field">
            <span id=field.label_props.id class="demo-date-field-label">"Birthday"</span>
            <div {..field.field_props.into_attrs()} class="demo-date-field-input">
                <DateSegments controls is_disabled=false is_read_only=false is_invalid=field.is_invalid/>
            </div>
        </div>

        <p class="demo-status">
            {move || birthday.get().map_or_else(|| "No date entered".to_owned(), |date| format!("Born on {}", date.date()))}
        </p>
    }
}
