use leptonic::{
    atoms::prelude::Popover,
    components::prelude::*,
    hooks::*,
    prelude::icondata,
    utils::{
        CapturedElement,
        time::{Day, InMonth},
    },
};
use leptos::{html, prelude::*};
use time::OffsetDateTime;

// Stopgaps until leptonic has a segment atom and the calendar cells move the browser focus themselves.
use super::{
    calendar_focus::{follow_focused_date, week_key},
    date_segments::{DateSegments, SegmentControls},
};

#[component]
pub fn DatePickerDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    let picker = use_date_picker(UseDatePickerInput {
        label: Some("Departure".to_owned()),
        is_disabled: disabled.into(),
        ..Default::default()
    });
    let state = picker.state;

    // The field edits the picker's value.
    let field = use_date_field(UseDateFieldInput {
        value: state.value,
        on_change: Some(state.set_value),
        is_disabled: disabled.into(),
        is_date_picker: true,
        ..Default::default()
    });
    let controls = SegmentControls::from(&field);

    // The popover is positioned at the whole field. It keeps the focus inside while open and returns it when it
    // closes.
    let anchor = CapturedElement::new();
    let dialog = StoredValue::new(picker.dialog_props);
    let UseDatePickerCalendarProps {
        value: calendar_value,
        on_change: on_pick,
        min,
        max,
        default_focused_value,
        auto_focus,
        ..
    } = picker.calendar_props;

    let status = move || {
        let date = state.formatted_value.get();
        if date.is_empty() { "No departure date".to_owned() } else { format!("Departure: {date}") }
    };

    view! {
        <div {..anchor.attr()} class="demo-date-picker">
            <div {..picker.group_props.into_attrs()} class="demo-date-picker-group">
                <span {..picker.label_props.into_attrs()} class="demo-date-field-label">"Departure"</span>
                // Both prop sets carry an `id` and a `role`: spread them on separate elements.
                <div {..picker.field_props.into_attrs()} class="demo-date-field-input demo-date-picker-input">
                    <div {..field.field_props.into_attrs()} class="demo-date-picker-segments">
                        <DateSegments controls is_disabled=disabled is_read_only=false is_invalid=picker.is_invalid/>
                    </div>
                    <button {..picker.button_props.into_attrs()} class="demo-date-picker-button">
                        <Icon icon=icondata::BsCalendar3/>
                    </button>
                </div>
            </div>
        </div>

        <Popover
            is_open=picker.is_open
            set_open=picker.set_open
            trigger=anchor
            placement=Placement::BottomStart
            classes="demo-date-picker-popover"
        >
            <div
                id=dialog.with_value(|dialog| dialog.id.clone())
                role=dialog.with_value(|dialog| dialog.role)
                aria-modal=dialog.with_value(|dialog| dialog.aria_modal)
                aria-labelledby=dialog.with_value(|dialog| dialog.aria_labelledby.clone())
            >
                // Created on every opening, so that it starts at the current value.
                <PickerCalendar value=calendar_value on_pick min max default_focused_value auto_focus/>
            </div>
        </Popover>

        <p class="demo-status">{status}</p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}

/// A single-month calendar built from the calendar hooks, configured from the picker's `calendar_props`.
#[component]
fn PickerCalendar(
    value: Signal<Option<OffsetDateTime>>,
    on_pick: Callback<OffsetDateTime>,
    min: Option<OffsetDateTime>,
    max: Option<OffsetDateTime>,
    default_focused_value: Option<OffsetDateTime>,
    auto_focus: bool,
) -> impl IntoView {
    let calendar = use_calendar_state(UseCalendarStateInput {
        default_value: value.get_untracked(),
        min,
        max,
        default_focused_value,
        on_change: Some(Callback::new(move |date: Option<OffsetDateTime>| {
            if let Some(date) = date {
                on_pick.run(date);
            }
        })),
        ..Default::default()
    });
    let grid = use_calendar_grid(UseCalendarGridInput::from_calendar_state(calendar));

    let grid_ref = NodeRef::<html::Table>::new();
    follow_focused_date(calendar.focused_date, grid_ref, auto_focus);

    view! {
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
                <For each=move || calendar.weeks.get() key=week_key let(week)>
                    <tr>
                        {week.days.into_iter().map(|day| view! { <DayCell calendar day/> }).collect_view()}
                    </tr>
                </For>
            </tbody>
        </table>
    }
}

#[component]
fn DayCell(calendar: UseCalendarStateReturn, day: Day) -> impl IntoView {
    let date = day.date_time;
    let cell = use_calendar_cell(UseCalendarCellInput {
        day,
        is_focused: Signal::derive(move || calendar.is_cell_focused.run(date)),
        is_selected: Signal::derive(move || calendar.is_selected.run(date)),
        is_disabled: calendar.is_disabled,
        on_select: Some(Callback::new(move |day: Day| calendar.select_date.run(day.date_time))),
        // Moving the cursor to a day of another month on mousedown would switch the month before the click
        // lands. Those days are selected (and focused) by `on_select` instead.
        on_focus: Some(Callback::new(move |day: Day| {
            if day.in_month == InMonth::Current {
                calendar.set_focused_date.run(day.date_time);
            }
        })),
    });

    view! {
        <td {..cell.cell_props.into_attrs()}>
            <button
                class="demo-calendar-day"
                data-outside-month=cell.is_outside_month.then_some("")
                data-today=cell.is_today.then_some("")
                {..cell.button_props.into_attrs()}
            >
                {cell.formatted_date}
            </button>
        </td>
    }
}
