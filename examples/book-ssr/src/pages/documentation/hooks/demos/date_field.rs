use std::sync::Arc;

use leptonic::{
    atoms::checkbox::Checkbox,
    hooks::{
        IntoAttrs,
        datepicker::{
            DateFieldData, DateFieldOptions, DateSegment, DateSegmentType, UseDateFieldInput,
            UseDateFieldReturn, UseDateFieldStateInput, UseDateSegmentInput, UseDateSegmentReturn,
            use_date_field, use_date_field_state, use_date_segment,
        },
    },
    jiff::civil::{Date, Weekday},
    utils::CapturedElement,
};
use leptos::prelude::*;

#[component]
pub fn DateFieldHookDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let state = use_date_field_state(UseDateFieldStateInput::<Date> {
        is_disabled: disabled.into(),
        validate: Some(Arc::new(|date: &Option<Date>| match date {
            Some(date) if matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday) => {
                Err(vec!["We deliver on weekdays only.".to_owned()])
            }
            _ => Ok(()),
        })),
        ..UseDateFieldStateInput::default()
    });
    let UseDateFieldReturn {
        label_props,
        field_props,
        input_props,
        description_props,
        error_message_props,
        data,
    } = use_date_field(UseDateFieldInput {
        state,
        element: CapturedElement::new(),
        input_element: CapturedElement::new(),
        options: DateFieldOptions {
            has_label: true.into(),
            ..DateFieldOptions::default()
        },
    });
    let (field_attrs, field_styles) = field_props.into_parts();
    let segments = state.segments;
    let errors = state.validation.validation_errors;

    view! {
        <div class="demo-date-field">
            <span {..label_props.into_attrs()} class="demo-field-label">"Delivery date"</span>
            <div {..field_attrs} style=field_styles class="demo-date-input">
                // A segment's element stays while its text changes: keyed by position and kind.
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
            // Carries the value (ISO 8601) in forms.
            <input {..input_props.into_attrs()}/>
            <span {..description_props.into_attrs()} class="demo-field-description">"Monday to Friday."</span>
            <span {..error_message_props.into_attrs()} class="demo-field-error">{move || errors.get().join(" ")}</span>
        </div>

        <p class="demo-status">
            {move || {
                let invalid = if errors.with(Vec::is_empty) { "" } else { " (invalid)" };
                state.value.get().map_or_else(
                    || "No date entered.".to_owned(),
                    |date| format!("Delivery on {}{invalid}.", date.strftime("%A, %B %-d, %Y")),
                )
            }}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}

/// An editable segment (a spin button), or a literal between them, hidden from screen readers.
#[component]
fn Segment(segment: Signal<DateSegment>, data: DateFieldData<Date>) -> impl IntoView {
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
