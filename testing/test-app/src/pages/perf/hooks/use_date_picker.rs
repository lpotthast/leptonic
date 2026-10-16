//! `use_date_picker` on plain elements, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_date_picker.rs`): a date picker (`use_date_picker_state`,
//! empty and closed, its value app state; its placeholder June 12, 2024: never today) labelled
//! "Departure", its group (`#perf-group`) with its field's segments (`use_date_field` with the
//! picker, `use_date_segment`) and its button (`use_button` of the input it returns), and while
//! open a plain dialog (`#perf-dialog`) with a button selecting June 14 (`#perf-pick`), between two
//! plain buttons (`#perf-before`, `#perf-after`), mounted at first. Its controls ([`Control`])
//! mount and unmount it, close it, set and clear its value, disable and enable it, and read its
//! value (`value`: `yyyymmdd`, 0 without).
//!
//! The page creates the live announcer on load (`announce("")`): a focused segment announces its
//! changes, and the first announcement of a page waits 100 ms for the live regions.

use leptonic::{
    Assertiveness, CapturedElement, IntoAttrs, ValueBinding, announce,
    hooks::{
        button::use_button,
        datepicker::{
            DateFieldData, DateFieldOptions, DateFieldPicker, DatePickerOptions, DatePickerState,
            DateSegment, DateSegmentType, UseDateFieldInput, UseDateFieldReturn,
            UseDateFieldStateInput, UseDatePickerInput, UseDatePickerReturn,
            UseDatePickerStateInput, UseDateSegmentInput, UseDateSegmentReturn, use_date_field,
            use_date_field_state, use_date_picker, use_date_picker_state, use_date_segment,
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
    let UseDateSegmentReturn { segment_props } = use_date_segment(UseDateSegmentInput {
        segment,
        data,
        element: CapturedElement::new(),
    });
    let (attrs, styles) = segment_props.into_parts();
    view! {
        <span {..attrs} style=styles data-type=kind.as_str()>
            {text}
        </span>
    }
    .into_any()
}

/// The picker: its state, `use_date_picker`, its field and the elements they return props for.
#[component]
fn HookDatePicker(
    value: RwSignal<Option<Date>>,
    disabled: RwSignal<bool>,
    state: StoredValue<Option<DatePickerState<Date>>>,
) -> impl IntoView {
    let picker = use_date_picker_state(UseDatePickerStateInput::<Date> {
        value: Some(value.into()),
        placeholder_value: Signal::stored(Some(date(2024, 6, 12))),
        ..UseDatePickerStateInput::default()
    });
    state.set_value(Some(picker));
    on_cleanup(move || state.set_value(None));
    let group = CapturedElement::new();
    let UseDatePickerReturn {
        label_props,
        group_props,
        field_describedby,
        labelledby,
        button,
        dialog_labelledby,
        ..
    } = use_date_picker(UseDatePickerInput {
        state: picker,
        group,
        options: DatePickerOptions {
            id: Some("perf-group".to_owned()),
            has_label: true.into(),
            is_disabled: disabled.into(),
            dialog_id: Signal::stored(Some("perf-dialog".to_owned())),
            ..DatePickerOptions::default()
        },
    });
    let field_state = use_date_field_state(UseDateFieldStateInput {
        value: Some(ValueBinding::new(
            picker.value,
            Callback::new(move |value| picker.set_value(value)),
        )),
        placeholder_value: Signal::stored(Some(date(2024, 6, 12))),
        is_disabled: disabled.into(),
        validation: Some(picker.validation),
        ..UseDateFieldStateInput::default()
    });
    let UseDateFieldReturn {
        field_props,
        input_props,
        data,
        ..
    } = use_date_field(UseDateFieldInput {
        state: field_state,
        element: CapturedElement::new(),
        input_element: CapturedElement::new(),
        options: DateFieldOptions {
            picker: Some(DateFieldPicker {
                overlay: picker.overlay,
                focus_manager: None,
                labelledby,
                describedby: field_describedby,
            }),
            ..DateFieldOptions::default()
        },
    });
    let (button_attrs, button_styles) = use_button(button).props.into_parts();
    let (group_attrs, group_styles) = group_props.into_parts();
    let (field_attrs, field_styles) = field_props.into_parts();
    let segments = field_state.segments;
    let open = picker.overlay.is_open;
    view! {
        <span {..label_props.into_attrs()}>"Departure"</span>
        <div {..group_attrs} style=group_styles>
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
            <button {..button_attrs} style=button_styles>
                "▼"
            </button>
        </div>
        <input {..input_props.into_attrs()} />
        <Show when=move || open.get()>
            <div id="perf-dialog" role="dialog" aria-labelledby=dialog_labelledby>
                <button id="perf-pick" on:click=move |_| picker.select_date(date(2024, 6, 14))>
                    "June 14"
                </button>
            </div>
        </Show>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(None::<Date>);
    let disabled = RwSignal::new(false);
    let state = StoredValue::new(None::<DatePickerState<Date>>);
    Effect::new(|| announce("", Assertiveness::Polite));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("close", move || {
            if let Some(state) = state.get_value() {
                state.set_open(false);
            }
        }),
        Control::action("set-value", move || value.set(Some(date(2024, 6, 5)))),
        Control::action("clear-value", move || value.set(None)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("value", move || number(value.get_untracked())),
    ];
    view! {
        <h1>"use_date_picker performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <HookDatePicker value=value disabled=disabled state=state />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
