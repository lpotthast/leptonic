//! `use_range_calendar_state` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_range_calendar_state.rs`): a range calendar's state on
//! June 2024 (June 12 focused, no range; fixed dates, never today), rendered as plain text: its
//! range (`#perf-value`), the range being selected's anchor (`#perf-anchor`), the highlighted
//! range (`#perf-highlighted`), the focused date (`#perf-focused`) and the visible range
//! (`#perf-range`), mounted at first. Its controls ([`Control`]) mount and unmount it, call the
//! state's methods (selecting, highlighting, committing, clearing, dragging) and change its
//! reactive inputs (limits, disabled, read-only).

use leptonic::{
    DateRange,
    hooks::calendar::{RangeCalendarState, UseRangeCalendarStateInput, use_range_calendar_state},
    jiff::civil::{Date, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// `range` as text ("none" without).
fn show(range: Option<DateRange>) -> String {
    range.map_or_else(
        || "none".to_owned(),
        |range| format!("{} - {}", range.start, range.end),
    )
}

/// The state's inputs, app state the controls change.
#[derive(Clone, Copy)]
struct Inputs {
    min_value: RwSignal<Option<Date>>,
    max_value: RwSignal<Option<Date>>,
    disabled: RwSignal<bool>,
    read_only: RwSignal<bool>,
}

/// Creates the state (handing it to the page's controls) and renders it.
#[component]
fn State(inputs: Inputs, state: StoredValue<Option<RangeCalendarState>>) -> impl IntoView {
    let range = use_range_calendar_state(UseRangeCalendarStateInput {
        default_focused_value: Some(date(2024, 6, 12)),
        min_value: inputs.min_value.into(),
        max_value: inputs.max_value.into(),
        is_disabled: inputs.disabled.into(),
        is_read_only: inputs.read_only.into(),
        ..UseRangeCalendarStateInput::default()
    });
    state.set_value(Some(range));
    on_cleanup(move || state.set_value(None));
    let calendar = range.calendar;
    view! {
        <p>"Value: " <span id="perf-value">{move || show(range.value.get())}</span></p>
        <p>
            "Anchor: "
            <span id="perf-anchor">
                {move || range.anchor_date.get().map_or_else(|| "none".to_owned(), |date| date.to_string())}
            </span>
        </p>
        <p>
            "Highlighted: "
            <span id="perf-highlighted">{move || show(range.highlighted_range.get())}</span>
        </p>
        <p>"Focused: " <span id="perf-focused">{move || calendar.focused_date.get().to_string()}</span></p>
        <p>
            "Visible: "
            <span id="perf-range">
                {move || {
                    let visible = calendar.visible_range.get();
                    format!("{} - {}", visible.start, visible.end)
                }}
            </span>
        </p>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<RangeCalendarState>);
    let inputs = Inputs {
        min_value: RwSignal::new(None),
        max_value: RwSignal::new(None),
        disabled: RwSignal::new(false),
        read_only: RwSignal::new(false),
    };
    // Calls `method` on the state while it is mounted.
    let call = move |method: fn(&RangeCalendarState)| {
        move || {
            if let Some(state) = state.get_value() {
                method(&state);
            }
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action(
            "select-june-10",
            call(|state| state.select_date(date(2024, 6, 10))),
        ),
        Control::action(
            "select-june-14",
            call(|state| state.select_date(date(2024, 6, 14))),
        ),
        Control::action(
            "highlight-june-13",
            call(|state| state.highlight_date(date(2024, 6, 13))),
        ),
        Control::action(
            "select-focused-date",
            call(RangeCalendarState::select_focused_date),
        ),
        Control::action("commit", call(RangeCalendarState::commit_selection)),
        Control::action("clear", call(RangeCalendarState::clear_selection)),
        Control::action("drop-anchor", call(|state| state.set_anchor_date(None))),
        Control::action(
            "set-value",
            call(|state| {
                state.set_value(Some(DateRange {
                    start: date(2024, 6, 17),
                    end: date(2024, 6, 19),
                }));
            }),
        ),
        Control::action("start-dragging", call(|state| state.set_dragging(true))),
        Control::action("stop-dragging", call(|state| state.set_dragging(false))),
        Control::action(
            "focus-after-june-14",
            call(|state| state.focus_nearest_available_date(date(2024, 6, 14))),
        ),
        Control::action("next-page", call(|state| state.calendar.focus_next_page())),
        Control::action("set-min", move || {
            inputs.min_value.set(Some(date(2024, 6, 10)));
        }),
        Control::action("set-max", move || {
            inputs.max_value.set(Some(date(2024, 6, 25)));
        }),
        Control::action("clear-limits", move || {
            inputs.min_value.set(None);
            inputs.max_value.set(None);
        }),
        Control::action("disable", move || inputs.disabled.set(true)),
        Control::action("enable", move || inputs.disabled.set(false)),
        Control::action("make-read-only", move || inputs.read_only.set(true)),
        Control::action("make-editable", move || inputs.read_only.set(false)),
        Control::read("is-dragging", move || {
            f64::from(u8::from(
                state
                    .get_value()
                    .is_some_and(|state| state.is_dragging.get_untracked()),
            ))
        }),
    ];
    view! {
        <h1>"use_range_calendar_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <State inputs=inputs state=state />
        </Show>
    }
}
