//! `use_date_range_picker` on plain elements, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_date_range_picker.rs`): a date range picker
//! (`use_date_range_picker_state`, empty and closed, its range app state; its placeholder June 12,
//! 2024: never today) labelled "Trip", its group (`#perf-group`) with the segments of its start
//! and end fields (`#perf-start`, `#perf-end`: `use_date_field` with the picker,
//! `use_date_segment`) and its button (`use_button` of the input it returns), and while open a
//! plain dialog (`#perf-dialog`) with a button selecting June 10 to 14 (`#perf-pick`), between two
//! plain buttons (`#perf-before`, `#perf-after`), mounted at first. Its controls ([`Control`])
//! mount and unmount it, close it, set and clear its range, disable and enable it, and read its
//! range (`start`, `end`: `yyyymmdd`, 0 without).
//!
//! The page creates the live announcer on load (`announce("")`): a focused segment announces its
//! changes, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{
    Assertiveness, CapturedElement, DateRange, ValueBinding, announce,
    hooks::{
        button::use_button,
        datepicker::{
            DateFieldData, DateFieldOptions, DateFieldPicker, DatePickerOptions,
            DateRangePickerState, DateSegment, DateSegmentType, RangePart, RangeValue,
            UseDateFieldInput, UseDateFieldReturn, UseDateFieldStateInput, UseDatePickerReturn,
            UseDateRangePickerInput, UseDateRangePickerStateInput, UseDateSegmentInput,
            UseDateSegmentReturn, use_date_field, use_date_field_state, use_date_range_picker,
            use_date_range_picker_state, use_date_segment,
        },
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

/// The segments of a field, keyed by position and kind.
#[component]
fn Segments(segments: Signal<Vec<DateSegment>>, data: DateFieldData<Date>) -> impl IntoView {
    view! {
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
    }
}

/// The picker: its state, `use_date_range_picker`, its two fields and the elements they return
/// props for.
#[component]
fn HookDateRangePicker(
    value: RwSignal<Option<RangeValue<Date>>>,
    disabled: RwSignal<bool>,
    state: StoredValue<Option<DateRangePickerState<Date>>>,
) -> impl IntoView {
    let picker = use_date_range_picker_state(UseDateRangePickerStateInput::<Date> {
        value: Some(value.into()),
        placeholder_value: Signal::stored(Some(date(2024, 6, 12))),
        ..UseDateRangePickerStateInput::default()
    });
    state.set_value(Some(picker));
    on_cleanup(move || state.set_value(None));
    let group = CapturedElement::new();
    let UseDatePickerReturn {
        label: label_props,
        group: group_props,
        field_describedby,
        labelledby,
        button,
        dialog_labelledby,
        focus_manager,
        ..
    } = use_date_range_picker(UseDateRangePickerInput {
        state: picker,
        group,
        options: DatePickerOptions {
            id: Some("perf-group".into()),
            has_label: true.into(),
            is_disabled: disabled.into(),
            dialog_id: Signal::stored(Some("perf-dialog".into())),
            ..DatePickerOptions::default()
        },
    });
    // A field of one end, editing the picker's.
    // (In a picker, the field's element has no id: `id` is the wrapper's.)
    let field = move |part: RangePart, id: &'static str| {
        let value = match part {
            RangePart::Start => picker.start,
            RangePart::End => picker.end,
        };
        let field_state = use_date_field_state(UseDateFieldStateInput {
            value: Some(ValueBinding::new(
                value,
                Callback::new(move |value| picker.set_date_time(part, value)),
            )),
            placeholder_value: Signal::stored(Some(date(2024, 6, 12))),
            is_disabled: disabled.into(),
            validation: Some(picker.validation),
            ..UseDateFieldStateInput::default()
        });
        let UseDateFieldReturn {
            field: field_props,
            data,
            ..
        } = use_date_field(UseDateFieldInput {
            state: field_state,
            element: CapturedElement::new(),
            input_element: CapturedElement::new(),
            options: DateFieldOptions {
                aria_label: match part {
                    RangePart::Start => "Start date",
                    RangePart::End => "End date",
                }
                .into(),
                picker: Some(DateFieldPicker {
                    overlay: picker.overlay,
                    focus_manager: Some(focus_manager),
                    labelledby,
                    describedby: field_describedby,
                }),
                ..DateFieldOptions::default()
            },
        });
        let (attrs, styles) = field_props.attrs();
        view! {
            <div id=id>
                <div {..attrs} style=styles>
                    <Segments segments=field_state.segments data=data />
                </div>
            </div>
        }
    };
    let start = field(RangePart::Start, "perf-start");
    let end = field(RangePart::End, "perf-end");
    let (button_attrs, button_styles) = use_button(button).button.attrs();
    let (group_attrs, group_styles) = group_props.attrs();
    let open = picker.overlay.is_open;
    view! {
        <span {..label_props.attrs()}>"Trip"</span>
        <div {..group_attrs} style=group_styles>
            {start}
            <span aria-hidden="true">" – "</span>
            {end}
            <button {..button_attrs} style=button_styles>
                "▼"
            </button>
        </div>
        <Show when=move || open.get()>
            <div id="perf-dialog" role="dialog" aria-labelledby=dialog_labelledby>
                <button
                    id="perf-pick"
                    on:click=move |_| {
                        picker
                            .select_range(DateRange {
                                start: date(2024, 6, 10),
                                end: date(2024, 6, 14),
                            });
                    }
                >
                    "June 10 to 14"
                </button>
            </div>
        </Show>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(None::<RangeValue<Date>>);
    let disabled = RwSignal::new(false);
    let state = StoredValue::new(None::<DateRangePickerState<Date>>);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("close", move || {
            if let Some(state) = state.get_value() {
                state.set_open(false);
            }
        }),
        Control::action("set-value", move || {
            value.set(Some(RangeValue {
                start: date(2024, 6, 5),
                end: date(2024, 6, 9),
            }));
        }),
        Control::action("clear-value", move || value.set(None)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("start", move || {
            number(value.get_untracked().map(|range| range.start))
        }),
        Control::read("end", move || {
            number(value.get_untracked().map(|range| range.end))
        }),
    ];
    view! {
        <h1>"use_date_range_picker performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <HookDateRangePicker value=value disabled=disabled state=state />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
