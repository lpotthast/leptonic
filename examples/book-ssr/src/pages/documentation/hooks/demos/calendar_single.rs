use leptonic::utils::CapturedElement;
use leptonic::utils::date_time_formatter::DateTimeFormat;
use leptonic::{
    components::prelude::*,
    hooks::{
        IntoAttrs,
        calendar::{
            CalendarData, UseCalendarCellInput, UseCalendarCellReturn, UseCalendarGridInput,
            UseCalendarInput, UseCalendarReturn, UseCalendarStateInput, use_calendar,
            use_calendar_cell, use_calendar_grid, use_calendar_state,
        },
        use_button,
    },
    jiff::civil::{Date, date},
    prelude::icondata,
    utils::{data_attributes::flag, date::DateExt},
};
use leptos::prelude::*;

#[component]
pub fn CalendarSingleDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    let state = use_calendar_state(UseCalendarStateInput {
        default_value: Some(date(2026, 3, 12)),
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
    } = use_calendar(
        UseCalendarInput {
            aria_label: "Appointment date".into(),
            ..Default::default()
        },
        state,
    );
    let (previous_attrs, previous_styles) = use_button(previous_button).props.into_parts();
    let (next_attrs, next_styles) = use_button(next_button).props.into_parts();

    let selected = move || {
        state.value.get().map_or_else(
            || "No date selected".to_owned(),
            |date| format!("Selected: {}", date.strftime("%A, %B %-d, %Y")),
        )
    };

    view! {
        <div {..calendar_props.into_attrs()} class="demo-calendar">
            <header class="demo-calendar-header">
                <button {..previous_attrs} style=previous_styles class="demo-calendar-nav">
                    <Icon icon=icondata::BsChevronLeft/>
                </button>
                // The calendar's label names the month already.
                <h2 class="demo-calendar-title" aria-hidden="true">{title}</h2>
                <button {..next_attrs} style=next_styles class="demo-calendar-nav">
                    <Icon icon=icondata::BsChevronRight/>
                </button>
            </header>
            <MonthGrid data/>
        </div>

        <p class="demo-status">{selected}</p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
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
            // Each day's label names its weekday: the header is for sighted users only.
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

/// A day: a grid cell with a focusable button inside.
#[component]
fn DayCell(data: CalendarData, date: Date, is_outside_month: Signal<bool>) -> impl IntoView {
    let UseCalendarCellReturn {
        cell_props,
        button_props,
        is_selected,
        is_today,
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

    view! {
        <td {..cell_props.into_attrs()}>
            <div
                {..button_attrs}
                style=button_styles
                class="demo-calendar-day"
                data-selected=flag(is_selected)
                data-today=flag(is_today)
                data-outside-month=flag(is_outside_month)
            >
                {formatted_date}
            </div>
        </td>
    }
}
