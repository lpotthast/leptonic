use leptonic::utils::CapturedElement;
use leptonic::utils::date_time_formatter::DateTimeFormat;
use leptonic::{
    components::prelude::*,
    hooks::{
        IntoAttrs,
        calendar::{
            CalendarData, CommitBehavior, UseCalendarCellInput, UseCalendarCellReturn,
            UseCalendarGridInput, UseCalendarInput, UseCalendarReturn, UseRangeCalendarStateInput,
            use_calendar_cell, use_calendar_grid, use_range_calendar, use_range_calendar_state,
        },
        use_button,
    },
    jiff::civil::{Date, date},
    prelude::icondata,
    utils::{
        data_attributes::flag,
        date::{DateExt, DateRange},
    },
};
use leptos::prelude::*;

#[component]
pub fn CalendarRangeDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    let state = use_range_calendar_state(UseRangeCalendarStateInput {
        default_value: Some(DateRange {
            start: date(2026, 3, 10),
            end: date(2026, 3, 14),
        }),
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
    } = use_range_calendar(
        UseCalendarInput {
            aria_label: "Trip dates".into(),
            ..Default::default()
        },
        state,
        CommitBehavior::Select,
    );
    let (previous_attrs, previous_styles) = use_button(previous_button).props.into_parts();
    let (next_attrs, next_styles) = use_button(next_button).props.into_parts();

    let status = move || match (state.anchor_date.get(), state.value.get()) {
        (Some(anchor), _) => format!("Selecting from {} \u{2026}", anchor.strftime("%b %-d")),
        (None, Some(DateRange { start, end })) => {
            let nights = match (end - start).get_days() {
                1 => "1 night".to_owned(),
                nights => format!("{nights} nights"),
            };
            format!(
                "{} \u{2013} {} ({nights})",
                start.strftime("%b %-d"),
                end.strftime("%b %-d")
            )
        }
        (None, None) => "No range selected".to_owned(),
    };

    view! {
        <div {..calendar_props.into_attrs()} class="demo-calendar demo-calendar-range">
            <header class="demo-calendar-header">
                <button {..previous_attrs} style=previous_styles class="demo-calendar-nav">
                    <Icon icon=icondata::BsChevronLeft/>
                </button>
                <h2 class="demo-calendar-title" aria-hidden="true">{title}</h2>
                <button {..next_attrs} style=next_styles class="demo-calendar-nav">
                    <Icon icon=icondata::BsChevronRight/>
                </button>
            </header>
            <MonthGrid data/>
        </div>

        <p class="demo-status">{status}</p>

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

/// A day. It is selected while it lies in the highlighted range: the selected range, or, while selecting, the range
/// from the first selected day to the focused (or hovered) one.
#[component]
fn DayCell(data: CalendarData, date: Date, is_outside_month: Signal<bool>) -> impl IntoView {
    let range = data.state.range();
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
                data-today=flag(is_today)
                data-outside-month=flag(is_outside_month)
            >
                {formatted_date}
            </div>
        </td>
    }
}
