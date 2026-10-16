//! `use_calendar_state` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_calendar_state.rs`): a calendar's state on June 2024
//! (June 12 focused, no date selected; fixed dates, never today), rendered as plain text: its
//! value (`#perf-value`), focused date (`#perf-focused`) and visible range (`#perf-range`),
//! mounted at first. Its controls ([`Control`]) mount and unmount it, call the state's methods
//! (moving the focused date, selecting, focusing) and change its reactive inputs (limits,
//! disabled, read-only, visible duration, first day of the week, page behavior).

use leptonic::{
    DateDuration,
    hooks::calendar::{CalendarState, PageBehavior, UseCalendarStateInput, use_calendar_state},
    jiff::civil::{Date, Weekday, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The state's inputs, app state the controls change.
#[derive(Clone, Copy)]
struct Inputs {
    min_value: RwSignal<Option<Date>>,
    max_value: RwSignal<Option<Date>>,
    disabled: RwSignal<bool>,
    read_only: RwSignal<bool>,
    months: RwSignal<i32>,
    first_day: RwSignal<Option<Weekday>>,
    page_behavior: RwSignal<PageBehavior>,
}

/// Creates the state (handing it to the page's controls) and renders it.
#[component]
fn State(inputs: Inputs, state: StoredValue<Option<CalendarState>>) -> impl IntoView {
    let calendar = use_calendar_state(UseCalendarStateInput {
        default_focused_value: Some(date(2024, 6, 12)),
        min_value: inputs.min_value.into(),
        max_value: inputs.max_value.into(),
        is_disabled: inputs.disabled.into(),
        is_read_only: inputs.read_only.into(),
        visible_duration: Signal::derive(move || DateDuration::months(inputs.months.get())),
        first_day_of_week: inputs.first_day.into(),
        page_behavior: inputs.page_behavior.into(),
        ..UseCalendarStateInput::default()
    });
    state.set_value(Some(calendar));
    on_cleanup(move || state.set_value(None));
    view! {
        <p>
            "Value: "
            <span id="perf-value">
                {move || calendar.value.get().map_or_else(|| "none".to_owned(), |date| date.to_string())}
            </span>
        </p>
        <p>"Focused: " <span id="perf-focused">{move || calendar.focused_date.get().to_string()}</span></p>
        <p>
            "Visible: "
            <span id="perf-range">
                {move || {
                    let range = calendar.visible_range.get();
                    format!("{} - {}", range.start, range.end)
                }}
            </span>
        </p>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let state = StoredValue::new(None::<CalendarState>);
    let inputs = Inputs {
        min_value: RwSignal::new(None),
        max_value: RwSignal::new(None),
        disabled: RwSignal::new(false),
        read_only: RwSignal::new(false),
        months: RwSignal::new(1),
        first_day: RwSignal::new(None),
        page_behavior: RwSignal::new(PageBehavior::Visible),
    };
    // Calls `method` on the state while it is mounted.
    let call = move |method: fn(&CalendarState)| {
        move || {
            if let Some(state) = state.get_value() {
                method(&state);
            }
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("focus-next-day", call(CalendarState::focus_next_day)),
        Control::action(
            "focus-previous-day",
            call(CalendarState::focus_previous_day),
        ),
        Control::action("focus-next-row", call(CalendarState::focus_next_row)),
        Control::action(
            "focus-previous-row",
            call(CalendarState::focus_previous_row),
        ),
        Control::action("focus-next-page", call(CalendarState::focus_next_page)),
        Control::action(
            "focus-previous-page",
            call(CalendarState::focus_previous_page),
        ),
        Control::action(
            "focus-section-start",
            call(CalendarState::focus_section_start),
        ),
        Control::action("focus-section-end", call(CalendarState::focus_section_end)),
        Control::action(
            "focus-next-year",
            call(|state| state.focus_next_section(true)),
        ),
        Control::action(
            "focus-previous-year",
            call(|state| state.focus_previous_section(true)),
        ),
        Control::action(
            "select-focused-date",
            call(CalendarState::select_focused_date),
        ),
        Control::action(
            "select-june-14",
            call(|state| state.select_date(date(2024, 6, 14))),
        ),
        Control::action("clear-value", call(|state| state.set_value(None))),
        Control::action(
            "focus-october",
            call(|state| state.set_focused_date(date(2024, 10, 15))),
        ),
        Control::action("set-focused", call(|state| state.set_focused(true))),
        Control::action("set-unfocused", call(|state| state.set_focused(false))),
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
        Control::action("show-three-months", move || inputs.months.set(3)),
        Control::action("show-one-month", move || inputs.months.set(1)),
        Control::action("start-on-monday", move || {
            inputs.first_day.set(Some(Weekday::Monday));
        }),
        Control::action("start-on-locale-day", move || inputs.first_day.set(None)),
        Control::action("page-single", move || {
            inputs.page_behavior.set(PageBehavior::Single);
        }),
        Control::action("page-visible", move || {
            inputs.page_behavior.set(PageBehavior::Visible);
        }),
        Control::read("is-focused", move || {
            f64::from(u8::from(
                state
                    .get_value()
                    .is_some_and(|state| state.is_focused.get_untracked()),
            ))
        }),
    ];
    view! {
        <h1>"use_calendar_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <State inputs=inputs state=state />
        </Show>
    }
}
