use leptonic::{
    components::prelude::*,
    hooks::*,
    prelude::icondata,
    utils::time::{Day, InMonth},
};
use leptos::{html, prelude::*};
use time::{
    Date, OffsetDateTime, Weekday,
    macros::{date, datetime, format_description},
};

// A stopgap until the calendar cells move the browser focus themselves.
use super::calendar_focus::{follow_focused_date, week_key};

/// Nights that are already booked.
const BOOKED: [(Date, Date); 3] = [
    (date!(2026 - 03 - 13), date!(2026 - 03 - 15)),
    (date!(2026 - 03 - 24), date!(2026 - 03 - 24)),
    (date!(2026 - 04 - 07), date!(2026 - 04 - 09)),
];

#[component]
pub fn CalendarUnavailableDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    let state = use_range_calendar_state(UseRangeCalendarStateInput {
        min: Some(datetime!(2026-03-03 0:00 UTC)),
        max: Some(datetime!(2026-04-26 0:00 UTC)),
        // `min`, `max` and the days of the grid are compared as timestamps: keep them all at midnight UTC.
        default_focused_value: Some(datetime!(2026-03-03 0:00 UTC)),
        is_date_unavailable: Some(Callback::new(|date: OffsetDateTime| {
            BOOKED
                .iter()
                .any(|(from, to)| (*from..=*to).contains(&date.date()))
        })),
        first_day_of_week: Weekday::Sunday,
        is_disabled: disabled.into(),
        ..Default::default()
    });
    let calendar = state.calendar;

    let grid = use_calendar_grid(UseCalendarGridInput {
        aria_label: "Stay".into(),
        // Must match `first_day_of_week`: 0 = Monday, 6 = Sunday.
        start_of_week: 6,
        // Enter / Space on a booked day does nothing (instead of selecting the closest earlier available day).
        on_select_focused_date: Some(Callback::new(move |()| {
            if !calendar
                .is_cell_unavailable
                .run(calendar.focused_date.get_untracked())
            {
                state.select_focused_date.run(());
            }
        })),
        ..UseCalendarGridInput::from_range_calendar_state(state)
    });

    let grid_ref = NodeRef::<html::Table>::new();
    follow_focused_date(calendar.focused_date, grid_ref, false);

    let status = move || match (state.anchor_date.get(), state.value.get()) {
        (Some(anchor), _) => format!("Check-in {} \u{2026}", format_date(anchor)),
        (
            None,
            DateRange {
                start: Some(start),
                end: Some(end),
            },
        ) => {
            format!(
                "Check-in {}, check-out {}",
                format_date(start),
                format_date(end)
            )
        }
        (None, _) => "Select your check-in date".to_owned(),
    };

    view! {
        <div class="demo-calendar demo-calendar-range">
            <div class="demo-calendar-header">
                <Button
                    on_press=move |_| calendar.focus_previous_page.run(())
                    variant=ButtonVariant::Flat
                    is_disabled=Signal::derive(move || disabled.get() || calendar.is_previous_visible_range_invalid.get())
                    attr:aria-label="Previous month"
                >
                    <Icon icon=icondata::BsChevronLeft/>
                </Button>
                <span class="demo-calendar-title">
                    {move || format!("{} {}", calendar.focused_month_name.get(), calendar.focused_year.get())}
                </span>
                <Button
                    on_press=move |_| calendar.focus_next_page.run(())
                    variant=ButtonVariant::Flat
                    is_disabled=Signal::derive(move || disabled.get() || calendar.is_next_visible_range_invalid.get())
                    attr:aria-label="Next month"
                >
                    <Icon icon=icondata::BsChevronRight/>
                </Button>
            </div>

            <table node_ref=grid_ref class="demo-calendar-grid" {..grid.grid_props.into_attrs()}>
                <thead>
                    <tr {..grid.header_props.into_attrs()}>
                        {grid.weekday_labels.into_iter().map(|label| view! { <th>{label}</th> }).collect_view()}
                    </tr>
                </thead>
                <tbody>
                    <For each=move || calendar.weeks.get() key=week_key let(week)>
                        <tr>
                            {week.days.into_iter().map(|day| view! { <BookingDayCell state day/> }).collect_view()}
                        </tr>
                    </For>
                </tbody>
            </table>
        </div>

        <p class="demo-status">{status}</p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}

/// One day. Days outside `min`/`max` are disabled (`aria-disabled`); booked days are unavailable: they stay
/// focusable, but clicking them does nothing.
#[component]
fn BookingDayCell(state: UseRangeCalendarStateReturn, day: Day) -> impl IntoView {
    let date = day.date_time;
    let calendar = state.calendar;
    let selectable = !day.disabled && !day.unavailable;
    let UseCalendarCellReturn {
        cell_props,
        button_props,
        is_selected,
        is_outside_month,
        formatted_date,
        ..
    } = use_calendar_cell(UseCalendarCellInput {
        day,
        is_focused: Signal::derive(move || calendar.is_cell_focused.run(date)),
        is_selected: Signal::derive(move || state.is_selected.run(date)),
        is_disabled: calendar.is_disabled,
        on_select: Some(Callback::new(move |day: Day| {
            if selectable {
                calendar.set_focused_date.run(day.date_time);
                state.select_date.run(day.date_time);
            }
        })),
        // Moving the cursor to a day of another month on mousedown would switch the month before the click
        // lands. Those days are selected (and focused) by `on_select` instead.
        on_focus: Some(Callback::new(move |day: Day| {
            if day.in_month == InMonth::Current {
                calendar.set_focused_date.run(day.date_time);
            }
        })),
    });

    let range_edge = move |edge: fn(&DateRange) -> Option<OffsetDateTime>| {
        is_selected.get()
            && edge(&state.highlighted_range.get()).is_some_and(|it| it.date() == date.date())
    };

    view! {
        <td
            {..cell_props.into_attrs()}
            data-range-start=move || range_edge(|range| range.start).then_some("")
            data-range-end=move || range_edge(|range| range.end).then_some("")
        >
            <button
                class="demo-calendar-day"
                data-unavailable=day.unavailable.then_some("")
                data-outside-month=is_outside_month.then_some("")
                on:pointerenter=move |_| {
                    if selectable && !is_outside_month {
                        state.highlight_date.run(date);
                    }
                }
                {..button_props.into_attrs()}
            >
                {formatted_date}
            </button>
        </td>
    }
}

fn format_date(date: OffsetDateTime) -> String {
    date.format(format_description!("[month repr:short] [day padding:none]"))
        .unwrap_or_default()
}
