use leptonic::{
    atoms::checkbox::{CheckboxButton, CheckboxField},
    hooks::{
        IntoAttrs,
        datepicker::{
            DateFieldData, DateFieldOptions, DateSegment, DateSegmentType, UseDateFieldReturn,
            UseDateSegmentInput, UseDateSegmentReturn, UseTimeFieldInput, UseTimeFieldStateInput,
            use_date_segment, use_time_field, use_time_field_state,
        },
    },
    jiff::civil::{DateTime, Time, time},
    utils::CapturedElement,
};
use leptos::prelude::*;

#[component]
pub fn TimeFieldHookDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let state = use_time_field_state(UseTimeFieldStateInput::<Time> {
        default_value: Some(time(10, 0, 0, 0)),
        min_value: Signal::stored(Some(time(8, 0, 0, 0))),
        max_value: Signal::stored(Some(time(18, 0, 0, 0))),
        is_disabled: disabled.into(),
        ..UseTimeFieldStateInput::default()
    });
    // A time field is a date field of hours and minutes: `state.field` edits the time on a date.
    let UseDateFieldReturn {
        label_props,
        field_props,
        input_props,
        error_message_props,
        data,
        ..
    } = use_time_field(UseTimeFieldInput {
        state,
        element: CapturedElement::new(),
        input_element: CapturedElement::new(),
        options: DateFieldOptions {
            has_label: true.into(),
            ..DateFieldOptions::default()
        },
    });
    let (field_attrs, field_styles) = field_props.into_parts();
    let segments = state.field.segments;
    let errors = state.field.validation.validation_errors;

    view! {
        <div class="demo-date-field">
            <span {..label_props.into_attrs()} class="demo-field-label">"Pickup time"</span>
            <div {..field_attrs} style=field_styles class="demo-date-input">
                <For
                    each=move || {
                        segments.with(|segments| {
                            segments.iter().enumerate().map(|(index, segment)| (index, segment.kind)).collect::<Vec<_>>()
                        })
                    }
                    key=|key| *key
                    children=move |(index, kind)| {
                        let initial = segments.with_untracked(|segments| segments[index].clone());
                        let segment = Signal::derive(move || {
                            segments.with(|segments| {
                                segments.get(index).filter(|segment| segment.kind == kind).cloned()
                            })
                            .unwrap_or_else(|| initial.clone())
                        });
                        view! { <Segment segment data/> }
                    }
                />
            </div>
            <input {..input_props.into_attrs()}/>
            <span {..error_message_props.into_attrs()} class="demo-field-error">{move || errors.get().join(" ")}</span>
        </div>

        <p class="demo-status">
            {move || {
                let invalid = if errors.with(Vec::is_empty) { "" } else { " (invalid)" };
                state.value.get().map_or_else(
                    || "No time entered.".to_owned(),
                    |time| format!("Pickup at {}{invalid}.", time.strftime("%H:%M")),
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

/// An editable segment, or a literal between them. The field edits a `DateTime`.
#[component]
fn Segment(segment: Signal<DateSegment>, data: DateFieldData<DateTime>) -> impl IntoView {
    let kind = segment.with_untracked(|segment| segment.kind);
    let text = move || segment.with(|segment| segment.text.clone());
    if kind == DateSegmentType::Literal {
        return view! {
            <span aria-hidden="true" class="demo-date-segment" data-type=kind.as_str()>{text}</span>
        }
        .into_any();
    }
    let UseDateSegmentReturn { segment_props } = use_date_segment(UseDateSegmentInput {
        segment,
        data,
        element: CapturedElement::new(),
    });
    let (attrs, styles) = segment_props.into_parts();
    view! {
        <span {..attrs} style=styles class="demo-date-segment" data-type=kind.as_str()>{text}</span>
    }
    .into_any()
}
