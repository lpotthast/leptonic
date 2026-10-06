use leptonic::{
    components::prelude::*,
    hooks::*,
    prelude::icondata,
    utils::time::{Day, InMonth},
};
use leptos::{html, prelude::*};
use time::macros::{datetime, format_description};

// A stopgap until the calendar cells move the browser focus themselves.
use super::calendar_focus::{follow_focused_date, week_key};

#[component]
pub fn CalendarSingleDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    let state = use_calendar_state(UseCalendarStateInput {
        default_focused_value: Some(datetime!(2026-03-12 0:00 UTC)),
        is_disabled: disabled.into(),
        ..Default::default()
    });

    let grid = use_calendar_grid(UseCalendarGridInput {
        aria_label: "Appointment date".into(),
        ..UseCalendarGridInput::from_calendar_state(state)
    });

    let grid_ref = NodeRef::<html::Table>::new();
    follow_focused_date(state.focused_date, grid_ref, false);

    let selected = move || {
        state.value.get().map_or_else(
            || "No date selected".to_owned(),
            |date| {
                let format = format_description!("[weekday], [month repr:long] [day padding:none], [year]");
                format!("Selected: {}", date.format(format).unwrap_or_default())
            },
        )
    };

    view! {
        <div class="demo-calendar">
            <div class="demo-calendar-header">
                <Button
                    on_press=move |_| state.focus_previous_page.run(())
                    variant=ButtonVariant::Flat
                    is_disabled=Signal::derive(move || disabled.get() || state.is_previous_visible_range_invalid.get())
                    attr:aria-label="Previous month"
                >
                    <Icon icon=icondata::BsChevronLeft/>
                </Button>
                <span class="demo-calendar-title">
                    {move || format!("{} {}", state.focused_month_name.get(), state.focused_year.get())}
                </span>
                <Button
                    on_press=move |_| state.focus_next_page.run(())
                    variant=ButtonVariant::Flat
                    is_disabled=Signal::derive(move || disabled.get() || state.is_next_visible_range_invalid.get())
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
                    <For each=move || state.weeks.get() key=week_key let(week)>
                        <tr>
                            {week.days.into_iter().map(|day| view! { <DayCell state day/> }).collect_view()}
                        </tr>
                    </For>
                </tbody>
            </table>
        </div>

        <p class="demo-status">{selected}</p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}

/// One day: a grid cell with a button inside.
#[component]
fn DayCell(state: UseCalendarStateReturn, day: Day) -> impl IntoView {
    let date = day.date_time;
    let UseCalendarCellReturn {
        cell_props,
        button_props,
        is_today,
        is_outside_month,
        formatted_date,
        ..
    } = use_calendar_cell(UseCalendarCellInput {
        day,
        is_focused: Signal::derive(move || state.is_cell_focused.run(date)),
        is_selected: Signal::derive(move || state.is_selected.run(date)),
        is_disabled: state.is_disabled,
        on_select: Some(Callback::new(move |day: Day| state.select_date.run(day.date_time))),
        // Moving the cursor to a day of another month on mousedown would switch the month before the click
        // lands. Those days are selected (and focused) by `on_select` instead.
        on_focus: Some(Callback::new(move |day: Day| {
            if day.in_month == InMonth::Current {
                state.set_focused_date.run(day.date_time);
            }
        })),
    });

    view! {
        <td {..cell_props.into_attrs()}>
            <button
                class="demo-calendar-day"
                data-today=is_today.then_some("")
                data-outside-month=is_outside_month.then_some("")
                {..button_props.into_attrs()}
            >
                {formatted_date}
            </button>
        </td>
    }
}
