//! `use_time_field` on plain elements, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_time_field.rs`): a time field (`use_time_field_state` of
//! a time of day, empty, its value app state; 12-hour in the test-app's en-US) with its label
//! ("Pickup"), its group of segments (`#perf-field`, a segment per part: `use_date_segment`) and
//! its hidden input (`#perf-input`, named `pickup`), between two plain buttons (`#perf-before`,
//! `#perf-after`), mounted at first. Its controls ([`Control`]) mount and unmount it, set and
//! clear its value, change its inputs (disabled, a minimum) and read its value (`value`:
//! `hhmmss`, -1 without).
//!
//! The page creates the live announcer on load (`announce("")`): a focused segment announces its
//! changes, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{
    Assertiveness, CapturedElement, announce,
    hooks::datepicker::{
        DateFieldData, DateFieldOptions, DateSegment, DateSegmentType, TimeBound,
        UseDateFieldReturn, UseDateSegmentInput, UseDateSegmentReturn, UseTimeFieldInput,
        UseTimeFieldStateInput, use_date_segment, use_time_field, use_time_field_state,
    },
    jiff::civil::{DateTime, Time, time},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// `time` as `hhmmss`, for the read controls (-1: none).
fn number(time: Option<Time>) -> f64 {
    time.map_or(-1.0, |time| {
        f64::from(time.hour()) * 10_000.0
            + f64::from(time.minute()) * 100.0
            + f64::from(time.second())
    })
}

/// An editable segment (a spin button), or a literal between them.
#[component]
fn Segment(segment: Signal<DateSegment>, data: DateFieldData<DateTime>) -> impl IntoView {
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

/// The field: its state, `use_time_field` and the elements it returns props for.
#[component]
fn HookTimeField(
    value: RwSignal<Option<Time>>,
    disabled: RwSignal<bool>,
    min_value: RwSignal<Option<TimeBound<Time>>>,
) -> impl IntoView {
    let state = use_time_field_state(UseTimeFieldStateInput::<Time> {
        value: Some(value.into()),
        is_disabled: disabled.into(),
        min_value: min_value.into(),
        name: Some("pickup".to_owned()),
        ..UseTimeFieldStateInput::default()
    });
    let UseDateFieldReturn {
        label: label_props,
        field: field_props,
        input: input_props,
        data,
        ..
    } = use_time_field(UseTimeFieldInput {
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
    let segments = state.field.segments;
    view! {
        <span {..label_props.attrs()}>"Pickup"</span>
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
    let value = RwSignal::new(None::<Time>);
    let disabled = RwSignal::new(false);
    let min_value = RwSignal::new(None::<TimeBound<Time>>);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-value", move || value.set(Some(time(8, 30, 0, 0)))),
        Control::action("clear-value", move || value.set(None)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("set-min", move || {
            min_value.set(Some(TimeBound::TimeOfDay(time(9, 0, 0, 0))));
        }),
        Control::action("clear-min", move || min_value.set(None)),
        Control::read("value", move || number(value.get_untracked())),
    ];
    view! {
        <h1>"use_time_field performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <HookTimeField value=value disabled=disabled min_value=min_value />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
