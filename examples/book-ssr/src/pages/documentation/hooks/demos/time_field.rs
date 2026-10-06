use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

// leptonic has no segment atom yet: the demos share this component built from `use_date_segment`.
use super::date_segments::{DateSegments, SegmentControls};

#[component]
pub fn TimeFieldDemo() -> impl IntoView {
    let (value, set_value) = signal(Some(TimeValue::hm(14, 30)));
    let hour_cycle_24 = RwSignal::new(true);
    let show_seconds = RwSignal::new(false);
    let disabled = RwSignal::new(false);

    view! {
        // The hour cycle and the seconds segment are fixed when the field is created: re-create it when they change.
        // The value lives outside of the field and carries over.
        {move || {
            let hour_cycle_24 = hour_cycle_24.get();
            let show_seconds = show_seconds.get();
            view! { <TimeField value set_value hour_cycle_24 show_seconds disabled/> }
        }}

        <p class="demo-status">
            {move || match value.get() {
                None => "No time entered".to_owned(),
                Some(time) if show_seconds.get() => format!("Meeting at {}", time.format_hms()),
                Some(time) => format!("Meeting at {}", time.format_hm()),
            }}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=hour_cycle_24 set_selected=hour_cycle_24>"24-hour clock"</Checkbox>
            <Checkbox is_selected=show_seconds set_selected=show_seconds>"Show seconds"</Checkbox>
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}

#[component]
fn TimeField(
    value: ReadSignal<Option<TimeValue>>,
    set_value: WriteSignal<Option<TimeValue>>,
    hour_cycle_24: bool,
    show_seconds: bool,
    disabled: RwSignal<bool>,
) -> impl IntoView {
    let field = use_time_field(UseTimeFieldInput {
        value: value.into(),
        is_disabled: disabled.into(),
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
                <DateSegments controls is_disabled=disabled is_read_only=false is_invalid=false/>
            </div>
        </div>
    }
}
