use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    calendar_range::CalendarRangeDemo, calendar_single::CalendarSingleDemo,
    calendar_unavailable::CalendarUnavailableDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageCalendarHooks() -> impl IntoView {
    view! {
        <DocPage title="Calendar Hooks">
            <p>
                "The calendar hooks build month calendars for picking a single date or a date range, with a keyboard "
                "accessible grid of days. See the "<Link href=routes::doc::DateTime.materialize()>"Date & Time overview"</Link>
                " for how they relate to the date fields and date pickers."
            </p>

            <ReactAria hook="useRangeCalendar"/>

            <Section title="Architecture">
                <p>"A calendar is composed of a state hook, one grid and one cell per day:"</p>

                <DocTable headers=&["Hook", "Responsibility"]>
                    <TableRow>
                        <TableCell><Code inline=true>"use_calendar_state"</Code></TableCell>
                        <TableCell>
                            "Owns the selected date and the focused date (the keyboard cursor), computes the weeks of "
                            "the focused month and provides all navigation and query callbacks."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"use_range_calendar_state"</Code></TableCell>
                        <TableCell>
                            "Wraps "<Code inline=true>"use_calendar_state"</Code>" and adds range selection: the anchor "
                            "date, the highlighted range and the rules for unavailable dates."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"use_range_calendar"</Code></TableCell>
                        <TableCell>"Creates the range state and the attributes of the calendar container."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"use_calendar_grid"</Code></TableCell>
                        <TableCell>"The grid of days: ARIA attributes, weekday labels and all keyboard handling."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"use_calendar_cell"</Code></TableCell>
                        <TableCell>"One day: the grid cell, the button inside it, its label, roving tabindex and click selection."</TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "There is no "<Code inline=true>"use_calendar"</Code>" hook for single dates: compose "
                    <Code inline=true>"use_calendar_state"</Code>" with the grid and cell hooks directly, as the demo "
                    "below does. The hooks never render anything; the month header, the navigation buttons and the "
                    "markup of the grid are yours."
                </p>

                <p>
                    "Dates are "<Code inline=true>"time::OffsetDateTime"</Code>" values from the "
                    <LinkExt href="https://docs.rs/time" target=LinkTarget::_Blank>"time"</LinkExt>" crate. The grid data uses "
                    <Code inline=true>"Week"</Code>", "<Code inline=true>"Day"</Code>" and "<Code inline=true>"InMonth"</Code>
                    " from "<Code inline=true>"leptonic::utils::time"</Code>"; ranges are "
                    <Code inline=true>"DateRange"</Code>" values."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let state = use_calendar_state(UseCalendarStateInput {
                            on_change: Some(Callback::new(|date| leptos::logging::log!("{date:?}"))),
                            ..Default::default()
                        });

                        // Wires the grid's keyboard handling to the state's navigation callbacks.
                        let grid = use_calendar_grid(UseCalendarGridInput {
                            aria_label: "Appointment date".into(),
                            ..UseCalendarGridInput::from_calendar_state(state)
                        });

                        view! {
                            <table {..grid.grid_props.into_attrs()}>
                                <thead>
                                    <tr {..grid.header_props.into_attrs()}>
                                        {grid.weekday_labels.into_iter().map(|label| view! { <th>{label}</th> }).collect_view()}
                                    </tr>
                                </thead>
                                <tbody>
                                    <For each=move || state.weeks.get() key=|week| (week.days[0].date_time.date(), week.days[0].in_month as u8) let(week)>
                                        <tr>{week.days.into_iter().map(|day| view! { <DayCell state day/> }).collect_view()}</tr>
                                    </For>
                                </tbody>
                            </table>
                        }

                        #[component]
                        fn DayCell(state: UseCalendarStateReturn, day: Day) -> impl IntoView {
                            let date = day.date_time;
                            let cell = use_calendar_cell(UseCalendarCellInput {
                                day,
                                is_focused: Signal::derive(move || state.is_cell_focused.run(date)),
                                is_selected: Signal::derive(move || state.is_selected.run(date)),
                                is_disabled: state.is_disabled,
                                on_select: Some(Callback::new(move |day: Day| state.select_date.run(day.date_time))),
                                on_focus: Some(Callback::new(move |day: Day| state.set_focused_date.run(day.date_time))),
                            });
                            view! {
                                <td {..cell.cell_props.into_attrs()}>
                                    <button {..cell.button_props.into_attrs()}>{cell.formatted_date}</button>
                                </td>
                            }
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "A single-date calendar. Click a day or move with the arrow keys and press "<Keys keys="Enter"/>
                    ". Days of the neighboring months are dimmed but selectable."
                </p>

                <Demo description="Single-date calendar built from use_calendar_state, use_calendar_grid and use_calendar_cell" source=include_str!("demos/calendar_single.rs")>
                    <CalendarSingleDemo/>
                </Demo>
            </Section>

            <Section title="use_calendar_state">
                <Section title="Input" id="use-calendar-state-input">
                    <p><Code inline=true>"UseCalendarStateInput"</Code>" implements "<Code inline=true>"Default"</Code>"."</p>

                    <ApiTable kind=ApiKind::Input of="UseCalendarStateInput">
                        <ApiRow name="default_value" ty="Option<OffsetDateTime>" default="None">"The initially selected date."</ApiRow>
                        <ApiRow name="default_focused_value" ty="Option<OffsetDateTime>" default="None">
                            "The initially focused date, which also decides the month shown first. Falls back to "
                            <Code inline=true>"default_value"</Code>", then to the current time (local, UTC if the offset "
                            "is unknown, e.g. on the server). Clamped to "<Code inline=true>"min"</Code>" and "
                            <Code inline=true>"max"</Code>"."
                        </ApiRow>
                        <ApiRow name="min, max" ty="Option<OffsetDateTime>" default="None">
                            "The selectable range. Days outside it are disabled, and focus can\u{2019}t leave it."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                            "Disables selection and the grid\u{2019}s keyboard handling."
                        </ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">"Focus still moves, but nothing can be selected."</ApiRow>
                        <ApiRow name="is_date_unavailable" ty="Option<Callback<OffsetDateTime, bool>>" default="None">
                            "Marks dates as unavailable, e.g. booked days. Unavailable days stay focusable."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the value invalid while "<Code inline=true>"true"</Code>", e.g. from form validation. Feeds "
                            <Code inline=true>"is_value_invalid"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<OffsetDateTime>>>" default="None">"Called when the selected date changes."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<OffsetDateTime>>" default="None">"Called when the focused date changes."</ApiRow>
                        <ApiRow name="first_day_of_week" ty="time::Weekday" default="Monday">
                            "The first column of "<Code inline=true>"weeks"</Code>". Pass the matching "
                            <Code inline=true>"start_of_week"</Code>" to "<Code inline=true>"use_calendar_grid"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-calendar-state-return">
                    <p>
                        <Code inline=true>"UseCalendarStateReturn"</Code>" is "<Code inline=true>"Copy"</Code>
                        "; pass it to your cell components."
                    </p>

                    <ApiTable kind=ApiKind::Return of="UseCalendarStateReturn">
                        <ApiRow name="value" ty="Signal<Option<OffsetDateTime>>">"The selected date."</ApiRow>
                        <ApiRow name="focused_date" ty="Signal<OffsetDateTime>">
                            "The keyboard cursor. Always set; the grid shows its month."
                        </ApiRow>
                        <ApiRow name="weeks" ty="Signal<Vec<Week>>">
                            "Six weeks of seven "<Code inline=true>"Day"</Code>"s for the focused month, including days of "
                            "the previous and next month. Recomputed whenever the focused date or the value changes."
                        </ApiRow>
                        <ApiRow name="focused_year, focused_month_name" ty="Memo<i32>, Memo<String>">
                            "The year and the English month name of the focused date, for a heading."
                        </ApiRow>
                        <ApiRow name="is_previous_visible_range_invalid, is_next_visible_range_invalid" ty="Signal<bool>">
                            "Whether the previous or next month lies entirely outside "<Code inline=true>"min"</Code>
                            " / "<Code inline=true>"max"</Code>". Use them to disable the month buttons."
                        </ApiRow>
                        <ApiRow name="is_disabled, is_read_only, is_value_invalid" ty="Signal<bool>">
                            "The inputs, and whether the value is outside "<Code inline=true>"min"</Code>" / "
                            <Code inline=true>"max"</Code>", unavailable or externally invalid."
                        </ApiRow>
                        <ApiRow name="is_focused" ty="Signal<bool>">
                            "Whether the grid has focus, as reported through "<Code inline=true>"set_focused"</Code>"."
                        </ApiRow>
                        <ApiRow name="select_date" ty="Callback<OffsetDateTime>">
                            "Focuses and selects a date. Dates outside "<Code inline=true>"min"</Code>" / "
                            <Code inline=true>"max"</Code>" are ignored; an unavailable date selects the closest "
                            "earlier available date."
                        </ApiRow>
                        <ApiRow name="select_focused_date" ty="Callback<()>">"Selects the focused date with the same rules."</ApiRow>
                        <ApiRow name="set_value" ty="Callback<Option<OffsetDateTime>>">
                            "Sets or clears the value without these checks (ignored while disabled or read-only)."
                        </ApiRow>
                        <ApiRow name="set_focused_date, set_focused" ty="Callback<OffsetDateTime>, Callback<bool>">
                            "Move the cursor (clamped to "<Code inline=true>"min"</Code>" / "<Code inline=true>"max"</Code>
                            ") and record grid focus."
                        </ApiRow>
                        <ApiRow name="focus_next_day, focus_previous_day, focus_next_row, focus_previous_row" ty="Callback<()>">
                            "Move the cursor by one day or one week."
                        </ApiRow>
                        <ApiRow name="focus_next_page, focus_previous_page" ty="Callback<()>">
                            "Move the cursor to the first day of the next or previous month."
                        </ApiRow>
                        <ApiRow name="focus_section_start, focus_section_end" ty="Callback<()>">
                            "Move the cursor to the first or last day of its month."
                        </ApiRow>
                        <ApiRow name="focus_next_section, focus_previous_section" ty="Callback<bool>">
                            <Code inline=true>"false"</Code>": like the page callbacks. "<Code inline=true>"true"</Code>
                            ": the same day one year later or earlier."
                        </ApiRow>
                        <ApiRow name="is_selected, is_cell_focused" ty="Callback<OffsetDateTime, bool>">
                            "Whether a day is the selected or the focused date (compared by calendar day)."
                        </ApiRow>
                        <ApiRow name="is_cell_disabled, is_cell_unavailable" ty="Callback<OffsetDateTime, bool>">
                            "Whether a day is disabled (calendar disabled, or outside "<Code inline=true>"min"</Code>
                            " / "<Code inline=true>"max"</Code>") or unavailable."
                        </ApiRow>
                        <ApiRow name="months, years, years_range" ty="Signal<Vec<Month>>, Signal<Vec<Year>>, Signal<String>">
                            "Grids for month and year pickers: the twelve months of the focused year and a page of twelve "
                            "years, with a label such as \u{201c}2022 - 2033\u{201d}."
                        </ApiRow>
                        <ApiRow name="focus_month, focus_year, navigate_years_backward, navigate_years_forward" ty="Callback<..>">
                            "Pick a month (1\u{2013}12) or year for the cursor, and page through the years grid."
                        </ApiRow>
                    </ApiTable>

                    <p>"Each day of "<Code inline=true>"weeks"</Code>" is a "<Code inline=true>"Day"</Code>":"</p>
                    <ApiTable kind=ApiKind::Fields of="Day">
                        <ApiRow name="date_time" ty="OffsetDateTime">"The day, at the time of the focused date."</ApiRow>
                        <ApiRow name="index" ty="u8">"The day of the month, starting at 1."</ApiRow>
                        <ApiRow name="in_month" ty="InMonth">
                            <Code inline=true>"Previous"</Code>", "<Code inline=true>"Current"</Code>" or "
                            <Code inline=true>"Next"</Code>": the month the day belongs to, relative to the shown one."
                        </ApiRow>
                        <ApiRow name="disabled" ty="bool">"Outside "<Code inline=true>"min"</Code>" / "<Code inline=true>"max"</Code>"."</ApiRow>
                        <ApiRow name="unavailable" ty="bool">"Rejected by "<Code inline=true>"is_date_unavailable"</Code>"."</ApiRow>
                        <ApiRow name="is_focused" ty="bool">"The focused day of the shown month."</ApiRow>
                        <ApiRow name="is_selected" ty="bool">"The selected date."</ApiRow>
                        <ApiRow name="is_now" ty="bool">"Today."</ApiRow>
                        <ApiRow name="highlighted" ty="bool">"Always "<Code inline=true>"false"</Code>"; range highlighting comes from the range state."</ApiRow>
                    </ApiTable>
                    <p>
                        "Key "<Code inline=true>"For"</Code>" loops over days by "<Code inline=true>"date_time"</Code>" and "
                        <Code inline=true>"in_month"</Code>": a date can appear twice in the grid (as a day of the previous or "
                        "next month)."
                    </p>
                </Section>
            </Section>

            <Section title="use_range_calendar_state">
                <Section title="Input" id="use-range-calendar-state-input">
                    <p>
                        <Code inline=true>"UseRangeCalendarStateInput"</Code>" implements "<Code inline=true>"Default"</Code>
                        ". Most fields work as in "<Code inline=true>"UseCalendarStateInput"</Code>"."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseRangeCalendarStateInput">
                        <ApiRow name="default_value" ty="Option<DateRange>" default="None">"The initially selected range."</ApiRow>
                        <ApiRow name="default_focused_value" ty="Option<OffsetDateTime>" default="None">
                            "Falls back to the start of "<Code inline=true>"default_value"</Code>", then to now."
                        </ApiRow>
                        <ApiRow name="min, max" ty="Option<OffsetDateTime>" default="None">
                            "The selectable range. Days outside it are disabled, and focus can\u{2019}t leave it."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                            "Disables selection and the grid\u{2019}s keyboard handling."
                        </ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">"Focus still moves, but nothing can be selected."</ApiRow>
                        <ApiRow name="is_date_unavailable" ty="Option<Callback<OffsetDateTime, bool>>" default="None">
                            "Marks dates as unavailable, e.g. booked days. Unavailable days stay focusable."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the value invalid while "<Code inline=true>"true"</Code>", e.g. from form validation. Feeds "
                            <Code inline=true>"is_value_invalid"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<DateRange>>" default="None">
                            "Called when a range is completed or set with "<Code inline=true>"set_value"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<OffsetDateTime>>" default="None">"Called when the focused date changes."</ApiRow>
                        <ApiRow name="first_day_of_week" ty="time::Weekday" default="Monday">
                            "The first column of "<Code inline=true>"weeks"</Code>". Pass the matching "
                            <Code inline=true>"start_of_week"</Code>" to "<Code inline=true>"use_calendar_grid"</Code>"."
                        </ApiRow>
                        <ApiRow name="allows_non_contiguous_ranges" ty="bool" default="false">
                            "Whether a range may span unavailable dates. By default, the end of a range stops at the last "
                            "available date before the next unavailable one."
                        </ApiRow>
                    </ApiTable>

                    <p>
                        <Code inline=true>"DateRange"</Code>" has public "<Code inline=true>"start"</Code>" and "
                        <Code inline=true>"end"</Code>" fields ("<Code inline=true>"Option<OffsetDateTime>"</Code>"), the "
                        "constructors "<Code inline=true>"new(start, end)"</Code>" and "<Code inline=true>"empty()"</Code>
                        " (its "<Code inline=true>"Default"</Code>"), "<Code inline=true>"contains(&date)"</Code>
                        " (inclusive) and "<Code inline=true>"is_complete()"</Code>"."
                    </p>
                </Section>

                <Section title="Return" id="use-range-calendar-state-return">
                    <ApiTable kind=ApiKind::Return of="UseRangeCalendarStateReturn">
                        <ApiRow name="calendar" ty="UseCalendarStateReturn">
                            "The inner calendar state: "<Code inline=true>"focused_date"</Code>", "<Code inline=true>"weeks"</Code>
                            ", the navigation callbacks and the heading. Its "<Code inline=true>"value"</Code>" and "
                            <Code inline=true>"is_selected"</Code>" only know the initial start date; use the range\u{2019}s fields below."
                        </ApiRow>
                        <ApiRow name="value" ty="Signal<DateRange>">"The selected range."</ApiRow>
                        <ApiRow name="anchor_date" ty="Signal<Option<OffsetDateTime>>">
                            "The first date of a selection in progress. "<Code inline=true>"None"</Code>" when no selection is in progress."
                        </ApiRow>
                        <ApiRow name="highlighted_range" ty="Signal<DateRange>">
                            "While selecting: the range from the anchor to the focused date, in either order. Otherwise the value."
                        </ApiRow>
                        <ApiRow name="is_selected" ty="Callback<OffsetDateTime, bool>">
                            "Whether a day lies in the highlighted range and is neither disabled nor unavailable."
                        </ApiRow>
                        <ApiRow name="select_date, select_focused_date" ty="Callback<OffsetDateTime>, Callback<()>">
                            "The first call sets the anchor, the second completes the range and calls "
                            <Code inline=true>"on_change"</Code>". The date is clamped to "<Code inline=true>"min"</Code>
                            " / "<Code inline=true>"max"</Code>" and to the available stretch around the anchor; an "
                            "unavailable date is replaced by the closest earlier available one."
                        </ApiRow>
                        <ApiRow name="highlight_date" ty="Callback<OffsetDateTime>">
                            "Moves the focused date while a selection is in progress, e.g. on hover. Does nothing otherwise."
                        </ApiRow>
                        <ApiRow name="set_anchor_date" ty="Callback<Option<OffsetDateTime>>">
                            "Starts a selection, or cancels it with "<Code inline=true>"None"</Code>"."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Callback<DateRange>">
                            "Sets the range and calls "<Code inline=true>"on_change"</Code>" (ignored while disabled or read-only)."
                        </ApiRow>
                        <ApiRow name="clear" ty="Callback<()>">
                            "Clears the value and any selection in progress, without calling "<Code inline=true>"on_change"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_value_invalid" ty="Signal<bool>">
                            "Whether an end of the range is outside "<Code inline=true>"min"</Code>" / "<Code inline=true>"max"</Code>
                            " or unavailable, or "<Code inline=true>"is_invalid"</Code>" is set. Always "
                            <Code inline=true>"false"</Code>" (unless externally invalid) while a selection is in progress."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_range_calendar">
                <p>
                    "Creates the range state from the same input and returns the attributes of the calendar container."
                </p>

                <Section title="Input" id="use-range-calendar-input">
                    <p>
                        <Code inline=true>"UseRangeCalendarInput"</Code>" has exactly the fields of "
                        <a href="#use-range-calendar-state-input"><Code inline=true>"UseRangeCalendarStateInput"</Code></a>
                        " and implements "<Code inline=true>"Default"</Code>"."
                    </p>
                </Section>

                <Section title="Return" id="use-range-calendar-return">
                    <ApiTable kind=ApiKind::Return of="UseRangeCalendarReturn">
                        <ApiRow name="calendar_props" ty="UseRangeCalendarProps">
                            "An id, "<Code inline=true>"role=\"application\""</Code>", "
                            <Code inline=true>"aria-label=\"Date range picker\""</Code>", "<Code inline=true>"aria-disabled"</Code>
                            " and a keydown handler that cancels a selection in progress on "<Keys keys="Escape"/>". Spread with "
                            <Code inline=true>"{..calendar_props.into_attrs()}"</Code>"."
                        </ApiRow>
                        <ApiRow name="state" ty="UseRangeCalendarStateReturn">"The range state."</ApiRow>
                        <ApiRow name="calendar_id" ty="String">"The id of the container."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_calendar_grid">
                <p>
                    "All keyboard handling lives on the grid: it listens for "<Code inline=true>"keydown"</Code>
                    " events bubbling up from the day buttons and calls the navigation callbacks you pass. "
                    <Code inline=true>"UseCalendarGridInput::from_calendar_state(state)"</Code>" and "
                    <Code inline=true>"from_range_calendar_state(state)"</Code>" wire all of them; override single "
                    "fields with struct update syntax."
                </p>

                <Section title="Input" id="use-calendar-grid-input">
                    <ApiTable kind=ApiKind::Input of="UseCalendarGridInput">
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Accessible name of the grid. The constructors leave it empty."
                        </ApiRow>
                        <ApiRow name="weekday_labels" ty="Vec<String>" default="Mon \u{2026} Sun">
                            "Column labels, Monday first. Rotated by "<Code inline=true>"start_of_week"</Code>"."
                        </ApiRow>
                        <ApiRow name="start_of_week" ty="u8" default="0">
                            "The first column: 0 is Monday, 6 is Sunday. Must match the state\u{2019}s "
                            <Code inline=true>"first_day_of_week"</Code>"; the constructors don\u{2019}t set it."
                        </ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">
                            "Set "<Code inline=true>"aria-disabled"</Code>" / "<Code inline=true>"aria-readonly"</Code>
                            ". A disabled grid ignores all keys."
                        </ApiRow>
                        <ApiRow name="is_range" ty="bool" default="false">"Sets "<Code inline=true>"aria-multiselectable"</Code>"."</ApiRow>
                        <ApiRow name="on_select_focused_date" ty="Option<Callback<()>>" default="None">"Called on "<Keys keys="Enter"/>" and "<Keys keys="Space"/>"."</ApiRow>
                        <ApiRow name="on_focus_previous_day, on_focus_next_day" ty="Option<Callback<()>>" default="None">
                            "Called on "<Keys keys="ArrowLeft"/>" / "<Keys keys="ArrowRight"/>"."
                        </ApiRow>
                        <ApiRow name="on_focus_previous_week, on_focus_next_week" ty="Option<Callback<()>>" default="None">
                            "Called on "<Keys keys="ArrowUp"/>" / "<Keys keys="ArrowDown"/>"."
                        </ApiRow>
                        <ApiRow name="on_focus_previous_section, on_focus_next_section" ty="Option<Callback<bool>>" default="None">
                            "Called on "<Keys keys="PageUp"/>" / "<Keys keys="PageDown"/>", with "<Code inline=true>"true"</Code>
                            " while "<Keys keys="Shift"/>" is held."
                        </ApiRow>
                        <ApiRow name="on_focus_section_start, on_focus_section_end" ty="Option<Callback<()>>" default="None">
                            "Called on "<Keys keys="Home"/>" / "<Keys keys="End"/>"."
                        </ApiRow>
                        <ApiRow name="on_cancel_selection" ty="Option<Callback<()>>" default="None">
                            "Called on "<Keys keys="Escape"/>". Range calendars clear the anchor."
                        </ApiRow>
                        <ApiRow name="on_focus, on_blur" ty="Option<Callback<()>>" default="None">
                            "Called on "<Code inline=true>"focus"</Code>" / "<Code inline=true>"blur"</Code>" of the grid element itself."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-calendar-grid-return">
                    <ApiTable kind=ApiKind::Return of="UseCalendarGridReturn">
                        <ApiRow name="grid_props" ty="UseCalendarGridProps">
                            "Id, "<Code inline=true>"role=\"grid\""</Code>", "<Code inline=true>"aria-label"</Code>", "
                            <Code inline=true>"aria-disabled"</Code>", "<Code inline=true>"aria-readonly"</Code>", "
                            <Code inline=true>"aria-multiselectable"</Code>" and the key and focus handlers, for a "
                            <Code inline=true>"<table>"</Code>"."
                        </ApiRow>
                        <ApiRow name="header_props" ty="UseCalendarGridHeaderProps">
                            <Code inline=true>"role=\"row\""</Code>" and "<Code inline=true>"aria-hidden=\"true\""</Code>
                            " for the header row: each day\u{2019}s label already names it, so screen readers skip the column headers."
                        </ApiRow>
                        <ApiRow name="weekday_labels" ty="Vec<String>">"The labels in column order."</ApiRow>
                        <ApiRow name="grid_id" ty="String">"The id of the grid."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_calendar_cell">
                <Section title="Input" id="use-calendar-cell-input">
                    <ApiTable kind=ApiKind::Input of="UseCalendarCellInput">
                        <ApiRow name="day" ty="Day">"The day from the state\u{2019}s "<Code inline=true>"weeks"</Code>"."</ApiRow>
                        <ApiRow name="is_focused" ty="Signal<bool>">
                            "Whether this day is the focused date; typically "<Code inline=true>"state.is_cell_focused"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_selected" ty="Signal<bool>">
                            "Whether this day is selected: "<Code inline=true>"is_selected"</Code>" of the calendar or the range state."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the calendar is disabled. Days outside "<Code inline=true>"min"</Code>" / "<Code inline=true>"max"</Code>" are disabled anyway."</ApiRow>
                        <ApiRow name="on_select" ty="Option<Callback<Day>>">
                            "Called when the day is clicked, unless it is outside "<Code inline=true>"min"</Code>" / "
                            <Code inline=true>"max"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_focus" ty="Option<Callback<Day>>">
                            "Called when the button receives focus (by click or "<Keys keys="Tab"/>"). Move the state\u{2019}s "
                            "focused date there."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-calendar-cell-return">
                    <ApiTable kind=ApiKind::Return of="UseCalendarCellReturn">
                        <ApiRow name="cell_props" ty="UseCalendarCellProps">
                            <Code inline=true>"role=\"gridcell\""</Code>", "<Code inline=true>"aria-selected"</Code>" and "
                            <Code inline=true>"aria-disabled"</Code>" for the "<Code inline=true>"<td>"</Code>"."
                        </ApiRow>
                        <ApiRow name="button_props" ty="UseCalendarCellButtonProps">
                            <Code inline=true>"role=\"button\""</Code>", a roving "<Code inline=true>"tabindex"</Code>
                            " (0 on the focused date), an "<Code inline=true>"aria-label"</Code>" such as "
                            "\u{201c}12 March 2026, today, selected\u{201d}, "<Code inline=true>"aria-disabled"</Code>
                            ", the click handler and "<Code inline=true>"data-focus-visible"</Code>" for keyboard focus."
                        </ApiRow>
                        <ApiRow name="formatted_date" ty="String">"The day of the month, the button\u{2019}s text."</ApiRow>
                        <ApiRow name="is_today, is_outside_month" ty="bool">
                            "Whether the day is today, and whether it belongs to the previous or next month."
                        </ApiRow>
                        <ApiRow name="is_selected, is_focused" ty="Signal<bool>">"The inputs, passed through."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"The input, or the day is outside "<Code inline=true>"min"</Code>" / "<Code inline=true>"max"</Code>"."</ApiRow>
                        <ApiRow name="is_focus_visible" ty="Signal<bool>">"Whether the button has keyboard focus."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Focus Management">
                <p>
                    "Keyboard navigation changes the state\u{2019}s "<Code inline=true>"focused_date"</Code>
                    ", and the cells move their "<Code inline=true>"tabindex"</Code>" accordingly. The cells don\u{2019}t "
                    "move DOM focus, though: focus the button with "<Code inline=true>"tabindex=\"0\""</Code>
                    " yourself after each change, as the demos do in "<Code inline=true>"follow_focused_date"</Code>
                    ". Skip it when focus has moved elsewhere, e.g. to the month buttons, so that paging with a mouse "
                    "doesn\u{2019}t pull focus into the grid."
                </p>
                <p>
                    "Wire "<Code inline=true>"on_focus"</Code>" of each cell to "<Code inline=true>"set_focused_date"</Code>
                    ", so that clicking or tabbing to a day moves the cursor there. Skip days of the neighboring "
                    "months: the button receives focus on "<Code inline=true>"mousedown"</Code>", and moving the cursor "
                    "there switches the month and removes the button before the click arrives. "
                    <Code inline=true>"select_date"</Code>" moves the cursor for them instead."
                </p>
            </Section>

            <Section title="Dates and Times">
                <p>
                    "The hooks work with "<Code inline=true>"time::OffsetDateTime"</Code>" values: each "
                    <Code inline=true>"Day"</Code>" of the grid has the time of day and offset of the focused date. "
                    <Code inline=true>"min"</Code>", "<Code inline=true>"max"</Code>" and "
                    <Code inline=true>"DateRange::contains"</Code>" compare full timestamps, so give them the same time "
                    "of day and offset as the grid\u{2019}s days. The demos keep everything at midnight UTC by passing "
                    <Code inline=true>"default_focused_value"</Code>" (or "<Code inline=true>"default_value"</Code>"). "
                    "Without it, the grid starts at the current time, and a "<Code inline=true>"max"</Code>
                    " at midnight disables its own day."
                </p>
                <p>"Selected values carry the time of day of the grid\u{2019}s days, not that of a previous value."</p>
            </Section>

            <Section title="Range Selection">
                <p>
                    "A range is selected in two steps: the first click (or "<Keys keys="Enter"/>") sets the "
                    <Code inline=true>"anchor_date"</Code>", the second completes the range. In between, "
                    <Code inline=true>"highlighted_range"</Code>" spans from the anchor to the focused date, so moving "
                    "with the arrow keys previews the range. Hovering previews it as well when you call "
                    <Code inline=true>"highlight_date"</Code>" from "<Code inline=true>"pointerenter"</Code>
                    ". The second date may lie before the anchor; the range is ordered for you. "<Keys keys="Escape"/>
                    " cancels the selection and keeps the previous value. A selection in progress stays open when "
                    "focus leaves the calendar, and ranges can\u{2019}t be selected by dragging."
                </p>
                <p>
                    "Style the range from the cells: "<Code inline=true>"aria-selected"</Code>" marks every day in the "
                    "highlighted range, and comparing a day with "<Code inline=true>"highlighted_range"</Code>
                    " gives you its first and last day."
                </p>

                <Demo description="Range calendar built from use_range_calendar, with a hover preview" source=include_str!("demos/calendar_range.rs")>
                    <CalendarRangeDemo/>
                </Demo>
            </Section>

            <Section title="Unavailable Dates">
                <p>
                    "Two mechanisms restrict selection. "<Code inline=true>"min"</Code>" and "<Code inline=true>"max"</Code>
                    " disable the days outside them: they get "<Code inline=true>"aria-disabled"</Code>" and focus "
                    "can\u{2019}t reach them. "<Code inline=true>"is_date_unavailable"</Code>" marks single days, such as "
                    "booked nights, that stay focusable but can\u{2019}t be selected. In a range calendar, a range "
                    "can\u{2019}t span an unavailable day unless "<Code inline=true>"allows_non_contiguous_ranges"</Code>
                    " is set: once the anchor is set, the end stops at the last available day before the next booked one."
                </p>
                <p>
                    "Selecting an unavailable date, by click, "<Code inline=true>"select_date"</Code>" or "<Keys keys="Enter"/>
                    ", selects the closest earlier available date instead. To ignore such attempts, as this demo does, "
                    "call "<Code inline=true>"select_date"</Code>" only for available days and replace the grid\u{2019}s "
                    <Code inline=true>"on_select_focused_date"</Code>" with a callback that skips unavailable dates. While "
                    "a range selection is in progress, the cursor can still move past the next unavailable day; the "
                    "highlighted range then shows that day, but the completed range stops before the unavailable one."
                </p>
                <p>
                    "The demo uses "<Code inline=true>"use_range_calendar_state"</Code>" without the container hook and "
                    "starts weeks on Sunday."
                </p>

                <Demo description="Booking calendar with min, max, booked days and Sunday as the first day of the week" source=include_str!("demos/calendar_unavailable.rs")>
                    <CalendarUnavailableDemo/>
                </Demo>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus into the grid, to the focused date, and out again."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowRight">"Previous / next day, across month boundaries."</KeyRow>
                    <KeyRow keys="ArrowUp / ArrowDown">"Same weekday in the previous / next week."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"First day of the previous / next month."</KeyRow>
                    <KeyRow keys="Shift + PageUp / Shift + PageDown">"Same day in the previous / next year."</KeyRow>
                    <KeyRow keys="Home / End">"First / last day of the month."</KeyRow>
                    <KeyRow keys="Enter / Space">
                        "Selects the focused date. In a range calendar, the first press sets the anchor and the second completes the range."
                    </KeyRow>
                    <KeyRow keys="Escape">"Range calendars: cancels the selection in progress."</KeyRow>
                </KeyboardTable>

                <p>
                    "Focus never leaves "<Code inline=true>"min"</Code>" / "<Code inline=true>"max"</Code>
                    ". The state announces the new month and year through the live announcer when the cursor enters "
                    "another month."
                </p>
            </Section>

            <Section title="Internationalization">
                <p>"The calendar hooks are not localized yet:"</p>
                <ul>
                    <li>
                        "Month names ("<Code inline=true>"focused_month_name"</Code>", the months grid), the cells\u{2019} "
                        <Code inline=true>"aria-label"</Code>"s and the month announcements are English."
                    </li>
                    <li>
                        "The weekday labels default to English abbreviations. Pass your own "
                        <Code inline=true>"weekday_labels"</Code>", Monday first."
                    </li>
                    <li>
                        "The first day of the week is not derived from a locale. Set "<Code inline=true>"first_day_of_week"</Code>
                        " on the state and the matching "<Code inline=true>"start_of_week"</Code>" on the grid."
                    </li>
                    <li>
                        "Right-to-left layouts are not supported: "<Keys keys="ArrowLeft"/>" always moves to the previous day."
                    </li>
                    <li>"Only the Gregorian calendar is supported."</li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::DateTime.materialize()>"Date & Time overview"</Link></li>
                <li><Link href=routes::doc::date_time::DatePickerHooks.materialize()>"Date picker hooks"</Link></li>
                <li><Link href=routes::doc::date_time::DateFieldHooks.materialize()>"Date field hooks"</Link></li>
                <li><Link href=routes::doc::date_time::Component.materialize()>"Date & Time component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
