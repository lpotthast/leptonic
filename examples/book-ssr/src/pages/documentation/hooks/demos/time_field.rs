use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

use super::date_segments::{DateSegments, SegmentControls};

#[component]
pub fn TimeFieldDemo() -> impl IntoView {
    let (value, set_value) = signal(Some(TimeValue::hm(14, 30)));
    let hour_cycle_24 = RwSignal::new(true);
    let show_seconds = RwSignal::new(false);

    view! {
        // The hour cycle and the seconds segment are fixed when the field is created: re-create it when they change.
        // The value lives outside of the field and carries over.
        {move || {
            let hour_cycle_24 = hour_cycle_24.get();
            let show_seconds = show_seconds.get();
            view! { <TimeField value set_value hour_cycle_24 show_seconds/> }
        }}

        <Checkbox state=hour_cycle_24>"24-hour clock"</Checkbox>
        <Checkbox state=show_seconds>"Show seconds"</Checkbox>

        <p class="demo-mt-half">
            "Value: "
            <code>
                {move || value.get().map_or_else(|| "None".to_owned(), |time| time.format_hms())}
            </code>
        </p>
    }
}

#[component]
fn TimeField(
    value: ReadSignal<Option<TimeValue>>,
    set_value: WriteSignal<Option<TimeValue>>,
    hour_cycle_24: bool,
    show_seconds: bool,
) -> impl IntoView {
    let field = use_time_field(UseTimeFieldInput {
        value: value.into(),
        label: Some("Meeting time".to_owned()),
        hour_cycle_24,
        show_seconds,
        on_change: Some(Callback::new(move |new_value| set_value.set(new_value))),
        ..Default::default()
    });

    let controls = SegmentControls::from(&field);

    view! {
        <div class="demo-date-field">
            <span id=field.label_props.id class="demo-date-field-label">"Meeting time"</span>
            <div {..field.field_props.into_attrs()} class="demo-date-field-input">
                <DateSegments controls is_disabled=false is_read_only=false is_invalid=false/>
            </div>
        </div>
    }
}
