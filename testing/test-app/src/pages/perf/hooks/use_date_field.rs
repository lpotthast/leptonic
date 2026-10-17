//! `use_date_field` on plain elements, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_date_field.rs`): a date field (`use_date_field_state`,
//! empty, its value app state; its segments start from June 12, 2024 when edited: never today)
//! with its label ("Birthday"), its group of segments (`#perf-field`, a segment per part:
//! `use_date_segment`) and its hidden input (`#perf-input`, named `birthday`), between two plain
//! buttons (`#perf-before`, `#perf-after`), mounted at first. Its controls ([`Control`]) mount
//! and unmount it, set and clear its value, change its inputs (disabled, a minimum) and read its
//! value (`value`: `yyyymmdd`, 0 without).
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

/// The field: its state, `use_date_field` and the elements it returns props for.
#[component]
fn HookDateField(
    value: RwSignal<Option<Date>>,
    disabled: RwSignal<bool>,
    min_value: RwSignal<Option<Date>>,
) -> impl IntoView {
    let state = use_date_field_state(UseDateFieldStateInput::<Date> {
        value: Some(value.into()),
        placeholder_value: Signal::stored(Some(date(2024, 6, 12))),
        is_disabled: disabled.into(),
        min_value: min_value.into(),
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
                    view! { <Segment segment=segment data=data /> }
                }
            />
        </div>
        <input {..input_props.attrs()} id="perf-input" />
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(None::<Date>);
    let disabled = RwSignal::new(false);
    let min_value = RwSignal::new(None::<Date>);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-value", move || value.set(Some(date(2024, 6, 5)))),
        Control::action("clear-value", move || value.set(None)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("set-min", move || min_value.set(Some(date(2025, 1, 1)))),
        Control::action("clear-min", move || min_value.set(None)),
        Control::read("value", move || number(value.get_untracked())),
    ];
    view! {
        <h1>"use_date_field performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <HookDateField value=value disabled=disabled min_value=min_value />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
