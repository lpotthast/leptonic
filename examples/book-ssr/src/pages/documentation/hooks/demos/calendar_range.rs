use leptonic::{
    components::prelude::*,
    hooks::*,
    prelude::icondata,
    utils::time::{Day, InMonth},
};
use leptos::{html, prelude::*};
use leptos_use::use_document;
use time::{
    OffsetDateTime,
    macros::{datetime, format_description},
};
use wasm_bindgen::JsCast;

#[component]
pub fn CalendarRangeDemo() -> impl IntoView {
    let UseRangeCalendarReturn {
        calendar_props,
        state,
        ..
    } = use_range_calendar(UseRangeCalendarInput {
        default_value: Some(DateRange::new(
            datetime!(2026-03-10 0:00 UTC),
            datetime!(2026-03-14 0:00 UTC),
        )),
        ..Default::default()
    });
    let calendar = state.calendar;

    let grid = use_calendar_grid(UseCalendarGridInput {
        aria_label: "Trip dates".into(),
        ..UseCalendarGridInput::from_range_calendar_state(state)
    });

    let grid_ref = NodeRef::<html::Table>::new();
    follow_focused_date(calendar.focused_date, grid_ref);

    let status = move || match (state.anchor_date.get(), state.value.get()) {
        (Some(anchor), _) => format!("Selecting from {} \u{2026}", format_date(anchor)),
        (
            None,
            DateRange {
                start: Some(start),
                end: Some(end),
            },
        ) => {
            let nights = (end.date() - start.date()).whole_days();
            format!(
                "{} \u{2013} {} ({nights} nights)",
                format_date(start),
                format_date(end)
            )
        }
        (None, _) => "No range selected".to_owned(),
    };

    view! {
        <div class="demo-calendar demo-calendar-range" {..calendar_props.into_attrs()}>
            <div class="demo-calendar-header">
                <Button
                    on_press=move |_| calendar.focus_previous_page.run(())
                    variant=ButtonVariant::Flat
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
                    <For each=move || calendar.weeks.get() key=|week| day_key(&week.days[0]) let(week)>
                        <tr>
                            {week.days.into_iter().map(|day| view! { <RangeDayCell state day/> }).collect_view()}
                        </tr>
                    </For>
                </tbody>
            </table>
        </div>

        <p class="demo-state-display">{status}</p>
    }
}

/// One day of the range calendar. The cell is selected while it lies in the highlighted range, which follows the
/// focused (or hovered) date while a selection is in progress.
#[component]
fn RangeDayCell(state: UseRangeCalendarStateReturn, day: Day) -> impl IntoView {
    let date = day.date_time;
    let calendar = state.calendar;
    let UseCalendarCellReturn {
        cell_props,
        button_props,
        is_selected,
        is_today,
        is_outside_month,
        formatted_date,
        ..
    } = use_calendar_cell(UseCalendarCellInput {
        day,
        is_focused: Signal::derive(move || calendar.is_cell_focused.run(date)),
        is_selected: Signal::derive(move || state.is_selected.run(date)),
        is_disabled: calendar.is_disabled,
        on_select: Some(Callback::new(move |day: Day| {
            calendar.set_focused_date.run(day.date_time);
            state.select_date.run(day.date_time);
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
                data-today=is_today.then_some("")
                data-outside-month=is_outside_month.then_some("")
                on:pointerenter=move |_| {
                    if !is_outside_month {
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

/// Weeks are recomputed whenever the focused date changes. Keying them by date keeps the DOM, and the focused
/// button, while you move within a month. A date appears in three months (e.g. as a day of the next month), hence
/// `in_month`.
fn day_key(day: &Day) -> (time::Date, u8) {
    (day.date_time.date(), day.in_month as u8)
}

/// The cells only update their `tabindex` when the focused date changes; they don't move DOM focus. Focus the
/// tabbable cell after each change, unless focus is elsewhere on the page (e.g. on the month buttons).
fn follow_focused_date(focused_date: Signal<OffsetDateTime>, grid: NodeRef<html::Table>) {
    Effect::watch(
        move || focused_date.get(),
        move |_, _, _| {
            // Wait for the new month to render.
            request_animation_frame(move || {
                let Some(grid) = grid.get_untracked() else {
                    return;
                };
                // Focus is on the body when the previously focused cell was removed by a month change.
                let focus_in_grid = use_document().active_element().is_none_or(|active| {
                    active.tag_name() == "BODY" || grid.contains(Some(&active))
                });
                if focus_in_grid
                    && let Ok(Some(cell)) = grid.query_selector("button[tabindex='0']")
                    && let Ok(cell) = cell.dyn_into::<web_sys::HtmlElement>()
                {
                    let _ = cell.focus();
                }
            });
        },
        false,
    );
}
