//! `use_date_segment` in a date field of hooks, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_date_segment.rs`): a date field (`use_date_field_state`
//! on June 5, 2024, its value app state; empty segments start from June 12, 2024 when edited:
//! never today, and `use_date_field`) labelled "Birthday", its group (`#perf-field`) with a
//! segment per part (`use_date_segment`, literals as plain text), between two plain buttons
//! (`#perf-before`, `#perf-after`), mounted at first. Its controls ([`Control`]) mount and
//! unmount the segments (the field stays), clear and set the value, disable and enable the field,
//! and read its value (`value`: `yyyymmdd`, 0 without).
//!
//! The page creates the live announcer on load (`announce("")`): a focused segment announces its
//! changes, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{
    Assertiveness, CapturedElement, announce,
    hooks::datepicker::{
        DateFieldData, DateFieldOptions, DateSegment, DateSegmentType, UseDateFieldInput,
        UseDateFieldReturn, UseDateFieldStateInput, UseDateSegmentInput, UseDateSegmentReturn,
        use_date_field, use_date_field_state, use_date_segment,
    },
    jiff::civil::{Date, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// `date` as `yyyymmdd`, for the read controls (0: none).
fn number(date: Option<Date>) -> f64 {
    date.map_or(0.0, |date| {
        f64::from(date.year()) * 10_000.0 + f64::from(date.month()) * 100.0 + f64::from(date.day())
    })
}

/// An editable segment (a spin button), or a literal between them.
#[component]
fn Segment(segment: Signal<DateSegment>, data: DateFieldData<Date>) -> impl IntoView {
    let kind = segment.with_untracked(|segment| segment.kind);
    let text = move || segment.with(|segment| segment.text.clone());
    if kind == DateSegmentType::Literal {
        return view! {
            <span aria-hidden="true" data-type=kind.as_str()>
                {text}
            </span>
        }
        .into_any();
    }
    let UseDateSegmentReturn {
        segment: segment_props,
    } = use_date_segment(UseDateSegmentInput {
        segment,
        data,
        element: CapturedElement::new(),
    });
    let (attrs, styles) = segment_props.attrs();
    view! {
        <span {..attrs} style=styles data-type=kind.as_str()>
            {text}
        </span>
    }
    .into_any()
}

/// The field: its state, `use_date_field` and the elements it returns props for; its segments
/// while `segments_mounted`.
#[component]
fn HookDateField(
    value: RwSignal<Option<Date>>,
    disabled: RwSignal<bool>,
    segments_mounted: RwSignal<bool>,
) -> impl IntoView {
    let state = use_date_field_state(UseDateFieldStateInput::<Date> {
        value: Some(value.into()),
        placeholder_value: Signal::stored(Some(date(2024, 6, 12))),
        is_disabled: disabled.into(),
        name: Some("birthday".to_owned()),
        ..UseDateFieldStateInput::default()
    });
    let UseDateFieldReturn {
        label: label_props,
        field: field_props,
        input: input_props,
        data,
        ..
    } = use_date_field(UseDateFieldInput {
        state,
        element: CapturedElement::new(),
        input_element: CapturedElement::new(),
        options: DateFieldOptions {
            id: Some("perf-field".into()),
            has_label: true.into(),
            ..DateFieldOptions::default()
        },
    });
    let (field_attrs, field_styles) = field_props.attrs();
    let segments = state.segments;
    view! {
        <span {..label_props.attrs()}>"Birthday"</span>
        <div {..field_attrs} style=field_styles>
            <For
                each=move || {
                    segments
                        .with(|segments| {
                            segments
                                .iter()
                                .enumerate()
                                .map(|(index, segment)| (index, segment.kind))
                                .collect::<Vec<_>>()
                        })
                }
                key=|key| *key
                children=move |(index, kind)| {
                    let initial = segments.with_untracked(|segments| segments[index].clone());
                    let segment = Signal::derive(move || {
                        segments
                            .with(|segments| {
                                segments.get(index).filter(|segment| segment.kind == kind).cloned()
                            })
                            .unwrap_or_else(|| initial.clone())
                    });
                    view! {
                        <Show when=move || segments_mounted.get()>
                            <Segment segment=segment data=data />
                        </Show>
                    }
                }
            />
        </div>
        <input {..input_props.attrs()} id="perf-input" />
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(Some(date(2024, 6, 5)));
    let disabled = RwSignal::new(false);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-value", move || value.set(Some(date(2024, 6, 5)))),
        Control::action("clear-value", move || value.set(None)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("value", move || number(value.get_untracked())),
    ];
    view! {
        <h1>"use_date_segment performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <HookDateField value=value disabled=disabled segments_mounted=mounted />
        <button id="perf-after">"After"</button>
    }
}
