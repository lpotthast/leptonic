use leptonic::{
    atoms::checkbox::Checkbox,
    hooks::{
        IntoAttrs,
        calendar::{
            CalendarData, CommitBehavior, DateAvailabilityQuery, UseCalendarCellInput,
            UseCalendarCellReturn, UseCalendarGridInput, UseCalendarReturn, UseRangeCalendarInput,
            UseRangeCalendarStateInput, use_calendar_cell, use_calendar_grid, use_range_calendar,
            use_range_calendar_state,
        },
        use_button,
    },
    jiff::civil::{Date, Weekday, date},
    utils::{
        CapturedElement,
        data_attributes::flag,
        date::{DateExt, DateRange},
        date_time_formatter::DateTimeFormat,
    },
};
use leptos::prelude::*;

/// Nights that are already booked.
const BOOKED: [(Date, Date); 3] = [
    (date(2026, 3, 13), date(2026, 3, 15)),
    (date(2026, 3, 24), date(2026, 3, 24)),
    (date(2026, 4, 7), date(2026, 4, 9)),
];

#[component]
pub fn CalendarUnavailableDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    let state = use_range_calendar_state(UseRangeCalendarStateInput {
        min_value: Signal::stored(Some(date(2026, 3, 3))),
        max_value: Signal::stored(Some(date(2026, 4, 26))),
        // Also called with the first selected day of a range in progress, e.g. to limit the length of a stay.
        is_date_unavailable: Some(Callback::new(|query: DateAvailabilityQuery| {
            BOOKED
                .iter()
                .any(|(from, to)| (*from..=*to).contains(&query.date))
        })),
        first_day_of_week: Signal::stored(Some(Weekday::Sunday)),
        is_disabled: disabled.into(),
        ..Default::default()
    });
    let UseCalendarReturn {
        calendar_props,
        previous_button,
        next_button,
        title,
        data,
        ..
    } = use_range_calendar(UseRangeCalendarInput {
        state,
        commit_behavior: CommitBehavior::Select,
        id: None,
        aria_label: "Stay".into(),
        aria_labelledby: None,
        aria_describedby: None,
        aria_details: None,
    });
    let (previous_attrs, previous_styles) = use_button(previous_button).props.into_parts();
    let (next_attrs, next_styles) = use_button(next_button).props.into_parts();

    let status = move || match (state.anchor_date.get(), state.value.get()) {
        (Some(anchor), _) => format!("Check-in {} \u{2026}", anchor.strftime("%b %-d")),
        (None, Some(DateRange { start, end })) => format!(
            "Check-in {}, check-out {}",
            start.strftime("%b %-d"),
            end.strftime("%b %-d")
        ),
        (None, None) => "Select your check-in date".to_owned(),
    };

    view! {
        <div {..calendar_props.into_attrs()} class="demo-calendar demo-calendar-range">
            <header class="demo-calendar-header">
                <button {..previous_attrs} style=previous_styles class="demo-calendar-nav">
                    <span aria-hidden="true">"\u{2039}"</span>
                </button>
                <h2 class="demo-calendar-title" aria-hidden="true">{title}</h2>
                <button {..next_attrs} style=next_styles class="demo-calendar-nav">
                    <span aria-hidden="true">"\u{203a}"</span>
                </button>
            </header>
            <MonthGrid data/>
        </div>

        <p class="demo-status">{status}</p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}

/// The grid of the visible month: a row of weekday names, then a row per week.
#[component]
fn MonthGrid(data: CalendarData) -> impl IntoView {
    let calendar = data.state.calendar();
    let grid = use_calendar_grid(UseCalendarGridInput {
        data: data.clone(),
        start_date: None,
        end_date: None,
        weekday_style: DateTimeFormat::Narrow,
    });
    let month = grid.start_date;
    let data = StoredValue::new(data);

    view! {
        <table {..grid.grid_props.into_attrs()} class="demo-calendar-grid">
            <thead aria-hidden="true">
                <tr>{move || grid.week_days.get().into_iter().map(|day| view! { <th>{day}</th> }).collect_view()}</tr>
            </thead>
            <tbody>
                <For each=move || 0..grid.weeks_in_month.get() key=|week| *week let(week)>
                    <tr>
                        <For
                            each=move || calendar.dates_in_week(week, Some(month.get())).into_iter().flatten()
                            key=|date| *date
                            let(date)
                        >
                            <DayCell
                                data=data.get_value()
                                date
                                is_outside_month=Signal::derive(move || !date.is_same_month(month.get()))
                            />
                        </For>
                    </tr>
                </For>
            </tbody>
        </table>
    }
}

/// A day. Days outside the minimum and maximum are disabled: they can't take the focus. Booked days are unavailable:
/// they can take the focus, but not be selected.
#[component]
fn DayCell(data: CalendarData, date: Date, is_outside_month: Signal<bool>) -> impl IntoView {
    let range = data.state.range();
    let UseCalendarCellReturn {
        cell_props,
        button_props,
        is_selected,
        is_disabled,
        is_unavailable,
        formatted_date,
        ..
    } = use_calendar_cell(UseCalendarCellInput {
        is_outside_month,
        data,
        date: date.into(),
        is_disabled: Signal::stored(false),
        element: CapturedElement::new(),
    });
    let (button_attrs, button_styles) = button_props.into_parts();
    let highlighted = move || range.and_then(|range| range.highlighted_range.get());

    view! {
        <td {..cell_props.into_attrs()}>
            <div
                {..button_attrs}
                style=button_styles
                class="demo-calendar-day"
                data-selected=flag(is_selected)
                data-selection-start=move || highlighted().is_some_and(|range| range.start == date).then_some("true")
                data-selection-end=move || highlighted().is_some_and(|range| range.end == date).then_some("true")
                data-disabled=flag(is_disabled)
                data-unavailable=flag(is_unavailable)
                data-outside-month=flag(is_outside_month)
            >
                {formatted_date}
            </div>
        </td>
    }
}
