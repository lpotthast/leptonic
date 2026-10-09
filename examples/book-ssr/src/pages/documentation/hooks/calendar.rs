use leptos::prelude::*;

use super::demos::{
    calendar_range::CalendarRangeDemo, calendar_single::CalendarSingleDemo,
    calendar_unavailable::CalendarUnavailableDemo,
};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageCalendarHooks() -> impl IntoView {
    view! {
        <DocPage title="Calendar Hooks">
            <p>
                "The calendar hooks build calendars for picking a date or a range of dates: a keyboard accessible grid of "
                "days with buttons to page through the months. See the "
                <Link href=routes::doc::Calendar.materialize()>"Calendar overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useCalendar"/>
            <ReactAria hook="useRangeCalendar"/>

            <p>"A calendar is a state hook, a calendar hook, a grid hook per month and a cell hook per day:"</p>

            <DocTable headers=&["Hook", "Responsibility"]>
                <TableRow>
                    <TableCell><AnchorLink href="#use-calendar-state"><Code inline=true>"use_calendar_state"</Code></AnchorLink></TableCell>
                    <TableCell>
                        "The selected date, the focused date (the keyboard cursor) and the visible range, which follows "
                        "the focused date; navigation and selection methods."
                    </TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><AnchorLink href="#use-range-calendar-state"><Code inline=true>"use_range_calendar_state"</Code></AnchorLink></TableCell>
                    <TableCell>"A calendar state whose selection is a range, chosen by two selections or by dragging."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>
                        <AnchorLink href="#use-calendar"><Code inline=true>"use_calendar"</Code></AnchorLink>", "
                        <AnchorLink href="#use-range-calendar"><Code inline=true>"use_range_calendar"</Code></AnchorLink>
                    </TableCell>
                    <TableCell>
                        "The calendar element, its previous and next buttons, its title and the announcements of the "
                        "month and the selection."
                    </TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><AnchorLink href="#use-calendar-grid"><Code inline=true>"use_calendar_grid"</Code></AnchorLink></TableCell>
                    <TableCell>"A month\u{2019}s grid of days: its label, the weekday names, the number of weeks and the keyboard."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><AnchorLink href="#use-calendar-cell"><Code inline=true>"use_calendar_cell"</Code></AnchorLink></TableCell>
                    <TableCell>"A day: its label, selection by press or drag, and the browser focus following the focused date."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>
                        <AnchorLink href="#use-calendar-heading"><Code inline=true>"use_calendar_heading"</Code></AnchorLink>", "
                        <AnchorLink href="#use-calendar-month-picker"><Code inline=true>"use_calendar_month_picker"</Code></AnchorLink>", "
                        <AnchorLink href="#use-calendar-year-picker"><Code inline=true>"use_calendar_year_picker"</Code></AnchorLink>
                    </TableCell>
                    <TableCell>"Headings of further months, and pickers to jump to a month or a year."</TableCell>
                </TableRow>
            </DocTable>

            <p>
                "The hooks render nothing: the header, the buttons and the markup of the grid are yours. The "
                <Link href=routes::doc::calendar::Atom.materialize()>"Calendar Atoms"</Link>" render them for you. Dates are "
                <Link href="https://docs.rs/jiff/latest/jiff/civil/struct.Date.html" target=LinkTarget::Blank>
                    <Code inline=true>"jiff::civil::Date"</Code>
                </Link>" values, re-exported as "<Code inline=true>"leptonic::jiff"</Code>"."
            </p>

            <Section title="Demo">
                <p>
                    "A calendar for a single date. Click a day, or tab into the grid, move with the arrow keys and press "
                    <Keys keys="Enter"/>". The days of the neighboring months fill the first and last week; they can\u{2019}t "
                    "be selected. The days carry the cell\u{2019}s state as data attributes for the stylesheet."
                </p>

                <Demo
                    description="Single-date calendar built from use_calendar_state, use_calendar, use_calendar_grid and use_calendar_cell"
                    source=include_str!("demos/calendar_single.rs")
                    source_open=true
                >
                    <CalendarSingleDemo/>
                </Demo>
            </Section>

            <Section title="use_calendar_state">
                <p>
                    "Creates the state of a calendar. Without a value or a focused date, the calendar shows the month of "
                    "today; on the server, that is the server\u{2019}s today."
                </p>

                <Section title="Input" id="use-calendar-state-input">
                    <p><Code inline=true>"UseCalendarStateInput"</Code>" implements "<Code inline=true>"Default"</Code>"."</p>

                    <ApiTable kind=ApiKind::Input of="calendar::use_calendar_state::UseCalendarStateInput">
                        <ApiRow name="default_value" ty="Option<Date>" default="None">"The initially selected date."</ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<Option<Date>>>" default="None">
                            "The selected date as app state (e.g. a "<Code inline=true>"ValueBinding"</Code>" from an "
                            <Code inline=true>"RwSignal"</Code>"), replacing "<Code inline=true>"default_value"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<Date>>>" default="None">"Called with the newly selected date."</ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<Option<Date>>" default="None">
                            "The first and last selectable date. Days outside are disabled, and the focused date stays within."
                        </ApiRow>
                        <ApiRow name="is_date_unavailable" ty="Option<Callback<Date, bool>>" default="None">
                            "Whether a date can\u{2019}t be selected, e.g. because it is booked. Unavailable days can still take the focus."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Nothing can be focused or selected."</ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">"The focus still moves, but nothing can be selected."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the value invalid, e.g. after form validation. Adds to "<Code inline=true>"is_value_invalid"</Code>"."
                        </ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Whether the focused date takes the browser focus when the calendar is rendered."</ApiRow>
                        <ApiRow name="default_focused_value" ty="Option<Date>" default="None">
                            "The initially focused date, which decides the month shown first. Default: the value, else today, "
                            "within the minimum and maximum."
                        </ApiRow>
                        <ApiRow name="focused_value" ty="Option<ValueBinding<Date>>" default="None">
                            "The focused date as app state, replacing "<Code inline=true>"default_focused_value"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<Date>>" default="None">"Called with the newly focused date."</ApiRow>
                        <ApiRow name="visible_duration" ty="Signal<DateDuration>" default="DateDuration::months(1)">
                            "How much is visible at once: months, or a number of weeks or days, see "
                            <AnchorLink href="#several-months">"Several Months"</AnchorLink>". A change aligns the visible "
                            "range around the focused date again."
                        </ApiRow>
                        <ApiRow name="page_behavior" ty="Signal<PageBehavior>" default="Visible">
                            "Whether the previous and next buttons page by the whole visible duration ("
                            <Code inline=true>"Visible"</Code>") or by one unit of it ("<Code inline=true>"Single"</Code>")."
                        </ApiRow>
                        <ApiRow name="selection_alignment" ty="Signal<SelectionAlignment>" default="Center">
                            "Where the focused date sits in a visible duration of several months, initially and when the "
                            "duration changes: "
                            <Code inline=true>"Start"</Code>", "<Code inline=true>"Center"</Code>" or "<Code inline=true>"End"</Code>"."
                        </ApiRow>
                        <ApiRow name="first_day_of_week" ty="Signal<Option<Weekday>>" default="None">
                            "The first day of the week. "<Code inline=true>"None"</Code>": the locale\u{2019}s (Sunday in the US, Monday in most of Europe)."
                        </ApiRow>
                        <ApiRow name="weeks_in_month" ty="Signal<Option<u8>>" default="None">
                            "A fixed number of week rows, e.g. 6, so that the calendar keeps its height from month to month."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-calendar-state-return">
                    <p>
                        <Code inline=true>"CalendarState"</Code>" is "<Code inline=true>"Copy"</Code>
                        ": pass it to the Leptos components rendering your parts."
                    </p>

                    <ApiTable kind=ApiKind::Return of="CalendarState">
                        <ApiRow name="value" ty="Signal<Option<Date>>">"The selected date."</ApiRow>
                        <ApiRow name="focused_date" ty="Signal<Date>">"The keyboard cursor. Always visible."</ApiRow>
                        <ApiRow name="visible_range" ty="Signal<DateRange>">"The visible dates, e.g. the first and last day of the month."</ApiRow>
                        <ApiRow name="is_focused" ty="Signal<bool>">"Whether a grid of the calendar has the focus."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>">"The inputs."</ApiRow>
                        <ApiRow name="is_value_invalid" ty="Signal<bool>">
                            "Whether the value is outside the minimum and maximum, unavailable, or marked invalid."
                        </ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<Option<Date>>">"The inputs."</ApiRow>
                        <ApiRow name="visible_duration" ty="Signal<DateDuration>">"The input."</ApiRow>
                        <ApiRow name="first_day_of_week" ty="Signal<Weekday>">"The first day of the week: the input, else the locale\u{2019}s."</ApiRow>
                    </ApiTable>

                    <p>"Its methods move the focused date and select:"</p>
                    <DocTable headers=&["Method", "Does"]>
                        <TableRow>
                            <TableCell><Code inline=true>"select_date(date)"</Code>", "<Code inline=true>"select_focused_date()"</Code></TableCell>
                            <TableCell>
                                "Select a date (not while disabled or read-only). "<Code inline=true>"select_date"</Code>" selects "
                                "the closest earlier available date in place of an unavailable one; "
                                <Code inline=true>"select_focused_date"</Code>" does nothing on an unavailable one."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_value(value)"</Code></TableCell>
                            <TableCell>
                                "Sets or clears the value, within the minimum and maximum; an unavailable date becomes the "
                                "previous available one. Does nothing while the calendar is disabled or read-only."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_focused_date(date)"</Code>", "<Code inline=true>"set_focused(bool)"</Code></TableCell>
                            <TableCell>
                                "Move the focused date (within the minimum and maximum; the visible range follows), and "
                                "record whether a grid has the focus. While it has, the focused date\u{2019}s cell takes the "
                                "browser focus."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>
                                <Code inline=true>"focus_next_day()"</Code>", "<Code inline=true>"focus_previous_day()"</Code>", "
                                <Code inline=true>"focus_next_row()"</Code>", "<Code inline=true>"focus_previous_row()"</Code>
                            </TableCell>
                            <TableCell>"Move the focused date by a day or a week."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"focus_next_page()"</Code>", "<Code inline=true>"focus_previous_page()"</Code></TableCell>
                            <TableCell>"Show the next or previous page and move the focused date by as much, keeping its day."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>
                                <Code inline=true>"focus_next_section(larger)"</Code>", "<Code inline=true>"focus_previous_section(larger)"</Code>
                            </TableCell>
                            <TableCell>
                                "In a month view, the same day a month later or earlier ("<Code inline=true>"larger"</Code>": a year); in a "
                                "week view a week ("<Code inline=true>"larger"</Code>": a month); in a day view a page."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"focus_section_start()"</Code>", "<Code inline=true>"focus_section_end()"</Code></TableCell>
                            <TableCell>"The first or last day of the focused date\u{2019}s month (or week, in a week view)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>
                                <Code inline=true>"is_selected(date)"</Code>", "<Code inline=true>"is_cell_focused(date)"</Code>", "
                                <Code inline=true>"is_cell_disabled(date)"</Code>", "<Code inline=true>"is_cell_unavailable(date)"</Code>", "
                                <Code inline=true>"is_invalid(date)"</Code>
                            </TableCell>
                            <TableCell>
                                "Queries per date (tracked): selected, focused, disabled (calendar disabled, not visible or "
                                "outside the minimum and maximum), unavailable, outside the minimum and maximum."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>
                                <Code inline=true>"is_previous_visible_range_invalid()"</Code>", "
                                <Code inline=true>"is_next_visible_range_invalid()"</Code>
                            </TableCell>
                            <TableCell>"Whether the previous or next page lies wholly outside the minimum and maximum."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"dates_in_week(week, from)"</Code>", "<Code inline=true>"weeks_in_month(from)"</Code></TableCell>
                            <TableCell>
                                "The dates of a week row of the month starting at "<Code inline=true>"from"</Code>" (default: "
                                "the visible range\u{2019}s start), and the number of week rows. "<Code inline=true>"None"</Code>
                                " marks days before the first representable date."
                            </TableCell>
                        </TableRow>
                    </DocTable>
                </Section>
            </Section>

            <Section title="use_range_calendar_state">
                <p>"Creates the state of a calendar whose selection is a range."</p>

                <Section title="Input" id="use-range-calendar-state-input">
                    <p>
                        <Code inline=true>"UseRangeCalendarStateInput"</Code>" implements "<Code inline=true>"Default"</Code>
                        ". The fields shared with "<AnchorLink href="#use-calendar-state-input"><Code inline=true>"UseCalendarStateInput"</Code></AnchorLink>
                        " work the same."
                    </p>

                    <ApiTable kind=ApiKind::Input of="calendar::use_range_calendar_state::UseRangeCalendarStateInput">
                        <ApiRow name="default_value" ty="Option<DateRange>" default="None">"The initially selected range."</ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<Option<DateRange>>>" default="None">
                            "The selected range as app state, replacing "<Code inline=true>"default_value"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<DateRange>>>" default="None">"Called with the newly selected range."</ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<Option<Date>>" default="None">"The first and last selectable date."</ApiRow>
                        <ApiRow name="is_date_unavailable" ty="Option<Callback<DateAvailabilityQuery, bool>>" default="None">
                            "Whether a date can\u{2019}t be selected. The query holds the "<Code inline=true>"date"</Code>" and the "
                            <Code inline=true>"anchor_date"</Code>", the first selected day of a range in progress, e.g. to "
                            "limit the length of a stay."
                        </ApiRow>
                        <ApiRow name="allows_non_contiguous_ranges" ty="bool" default="false">
                            "Whether a range may span unavailable dates. By default, a range in progress ends at the last "
                            "available date before the next unavailable one."
                        </ApiRow>
                        <ApiRow name="is_disabled, is_read_only, is_invalid" ty="Signal<bool>" default="false">"As in a single-date calendar."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Whether the focused date takes the browser focus when the calendar is rendered."</ApiRow>
                        <ApiRow name="default_focused_value" ty="Option<Date>" default="None">"The initially focused date. Default: the range\u{2019}s start, else today."</ApiRow>
                        <ApiRow name="focused_value" ty="Option<ValueBinding<Date>>" default="None">"The focused date as app state."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<Date>>" default="None">"Called with the newly focused date."</ApiRow>
                        <ApiRow name="visible_duration" ty="Signal<DateDuration>" default="DateDuration::months(1)">"How much is visible at once."</ApiRow>
                        <ApiRow name="page_behavior" ty="Signal<PageBehavior>" default="Visible">"How the previous and next buttons page."</ApiRow>
                        <ApiRow name="selection_alignment" ty="Signal<Option<SelectionAlignment>>" default="None">
                            "Where the initially focused date sits in several visible months. Default: centered, or at the "
                            "start if the range wouldn\u{2019}t fit then."
                        </ApiRow>
                        <ApiRow name="first_day_of_week" ty="Signal<Option<Weekday>>" default="None">"The first day of the week. "<Code inline=true>"None"</Code>": the locale\u{2019}s."</ApiRow>
                        <ApiRow name="weeks_in_month" ty="Signal<Option<u8>>" default="None">"A fixed number of week rows."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-range-calendar-state-return">
                    <ApiTable kind=ApiKind::Return of="RangeCalendarState">
                        <ApiRow name="calendar" ty="CalendarState">
                            "The calendar: focused date, visible range and navigation. While a range is being selected, its "
                            "minimum and maximum narrow to the available dates around the anchor."
                        </ApiRow>
                        <ApiRow name="value" ty="Signal<Option<DateRange>>">"The selected range."</ApiRow>
                        <ApiRow name="anchor_date" ty="Signal<Option<Date>>">"The first selected day of a range in progress."</ApiRow>
                        <ApiRow name="highlighted_range" ty="Signal<Option<DateRange>>">
                            "The range to show as selected: from the anchor to the focused date while selecting, else the value."
                        </ApiRow>
                        <ApiRow name="is_dragging" ty="Signal<bool>">"Whether the user drags to select a range."</ApiRow>
                        <ApiRow name="is_value_invalid" ty="Signal<bool>">
                            "Whether an end of the range is outside the minimum and maximum or unavailable, or the range is "
                            "marked invalid."
                        </ApiRow>
                    </ApiTable>

                    <p>
                        <Code inline=true>"select_date(date)"</Code>" and "<Code inline=true>"select_focused_date()"</Code>
                        " set the anchor first, then the other end, which sets the value. "
                        <Code inline=true>"commit_selection()"</Code>" finishes a range in progress at the focused date, "
                        <Code inline=true>"highlight_date(date)"</Code>" moves the focused date while selecting (hovering), "
                        <Code inline=true>"set_anchor_date(None)"</Code>" cancels a range in progress, "
                        <Code inline=true>"clear_selection()"</Code>" also clears the value, and "
                        <Code inline=true>"set_value(range)"</Code>" sets it. "<Code inline=true>"is_selected(date)"</Code>
                        " and "<Code inline=true>"is_invalid(date)"</Code>" answer for the highlighted range and the "
                        "available dates around the anchor."
                    </p>
                    <p>
                        "A "<Code inline=true>"DateRange"</Code>" ("<Code inline=true>"leptonic"</Code>") has "
                        "the fields "<Code inline=true>"start"</Code>" and "<Code inline=true>"end"</Code>", both included. "
                        <Code inline=true>"DateRange::between(a, b)"</Code>" orders two dates; "
                        <Code inline=true>"contains(date)"</Code>" checks a date."
                    </p>
                </Section>
            </Section>

            <Section title="use_calendar">
                <p>
                    <Code inline=true>"use_calendar"</Code>" connects a "<Code inline=true>"CalendarState"</Code>
                    " with the calendar\u{2019}s element, labelled by "<Code inline=true>"aria_label"</Code>" and the visible "
                    "month (\u{201c}Appointment date, March 2026\u{201d}). It announces the new month when the previous or next "
                    "button pages, and the new selection."
                </p>

                <Section title="Input" id="use-calendar-input">
                    <p>"Pass a "<Code inline=true>"UseCalendarInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>

                    <ApiTable kind=ApiKind::Input of="UseCalendarInput">
                        <ApiRow name="state" ty="CalendarState">"From "<Code inline=true>"use_calendar_state"</Code>". Required."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The calendar element\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the calendar, together with the visible month."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby, aria_details" ty="Option<String>" default="None">
                            "Ids of elements labelling, describing or detailing the calendar."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-calendar-return">
                    <ApiTable kind=ApiKind::Return of="UseCalendarReturn">
                        <ApiRow name="calendar_props" ty="UseCalendarProps">
                            "For the calendar element: its id, "<Code inline=true>"role=\"application\""</Code>" and label."
                        </ApiRow>
                        <ApiRow name="previous_button, next_button" ty="UseButtonInput">
                            "The page buttons, for "<Link href=routes::doc::button::Hook.materialize()><Code inline=true>"use_button"</Code></Link>
                            ": named \u{201c}Previous\u{201d} and \u{201c}Next\u{201d}, disabled where the minimum or maximum ends "
                            "the dates. A button disabled while focused hands the focus to the grid."
                        </ApiRow>
                        <ApiRow name="error_message_props" ty="SlotProps">
                            "For an error message: invalid selected days refer to it with "<Code inline=true>"aria-describedby"</Code>"."
                        </ApiRow>
                        <ApiRow name="title" ty="Signal<String>">"The visible range: \u{201c}March 2026\u{201d}."</ApiRow>
                        <ApiRow name="data" ty="CalendarData">"What the grids and cells need. Clone it for each."</ApiRow>
                    </ApiTable>

                    <p>
                        <Code inline=true>"CalendarData"</Code>" holds the "<Code inline=true>"state"</Code>", a "
                        <Code inline=true>"CalendarStates"</Code>": "<Code inline=true>"Single(CalendarState)"</Code>" or "
                        <Code inline=true>"Range(RangeCalendarState)"</Code>". Its "<Code inline=true>"calendar()"</Code>
                        " gives the calendar state of either, "<Code inline=true>"range()"</Code>" the range state of a range calendar."
                    </p>
                </Section>
            </Section>

            <Section title="use_range_calendar">
                <p>
                    <Code inline=true>"use_range_calendar"</Code>" is "
                    <AnchorLink href="#use-calendar"><Code inline=true>"use_calendar"</Code></AnchorLink>" for a "
                    <Code inline=true>"RangeCalendarState"</Code>", with the same return. When a pointer is released "
                    "outside the days, or the focus leaves the calendar, while a range is in progress, "
                    <Code inline=true>"commit_behavior"</Code>" decides what happens: "<Code inline=true>"Select"</Code>
                    " (the default) finishes the range at the focused date, "<Code inline=true>"Reset"</Code>" drops it and "
                    "keeps the previous value, "<Code inline=true>"Clear"</Code>" also clears the value."
                </p>

                <Section title="Input" id="use-range-calendar-input">
                    <ApiTable kind=ApiKind::Input of="UseRangeCalendarInput">
                        <ApiRow name="state" ty="RangeCalendarState">"From "<Code inline=true>"use_range_calendar_state"</Code>". Required."</ApiRow>
                        <ApiRow name="commit_behavior" ty="CommitBehavior" default="Select">
                            "What a press outside the dates or leaving the calendar does with a range being selected."
                        </ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The calendar element\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the calendar, together with the visible month."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby, aria_details" ty="Option<String>" default="None">
                            "Ids of elements labelling, describing or detailing the calendar."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_calendar_grid">
                <p>
                    "The grid of a month: a "<Code inline=true>"<table>"</Code>" with "<Code inline=true>"role=\"grid\""</Code>
                    ", labelled by the calendar\u{2019}s label and the month. It handles the keys (listed on the "
                    <Link href=format!("{}#accessibility", routes::doc::Calendar.materialize())>"Calendar overview"</Link>
                    ") for the cells inside."
                </p>

                <Section title="Input" id="use-calendar-grid-input">
                    <p>"Pass a "<Code inline=true>"UseCalendarGridInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>

                    <ApiTable kind=ApiKind::Input of="calendar::use_calendar_grid::UseCalendarGridInput">
                        <ApiRow name="data" ty="CalendarData">"The calendar\u{2019}s data. Required."</ApiRow>
                        <ApiRow name="start_date" ty="Option<Signal<Date>>" default="None">
                            "The first day of the grid\u{2019}s month. Default: the visible range\u{2019}s start; set it for "
                            "the further months of a calendar showing several."
                        </ApiRow>
                        <ApiRow name="end_date" ty="Option<Signal<Date>>" default="None">"The grid\u{2019}s last day. Default: the visible range\u{2019}s end."</ApiRow>
                        <ApiRow name="weekday_style" ty="DateTimeFormat" default="Narrow">
                            "How the weekday names are formatted: "<Code inline=true>"Narrow"</Code>" (\u{201c}M\u{201d}), "
                            <Code inline=true>"Short"</Code>" (\u{201c}Mon\u{201d}) or "<Code inline=true>"Long"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-calendar-grid-return">
                    <ApiTable kind=ApiKind::Return of="calendar::use_calendar_grid::UseCalendarGridReturn">
                        <ApiRow name="grid_props" ty="UseCalendarGridProps">
                            "For the "<Code inline=true>"<table>"</Code>": id, role, label, "<Code inline=true>"aria-readonly"</Code>", "
                            <Code inline=true>"aria-disabled"</Code>", "<Code inline=true>"aria-multiselectable"</Code>
                            " (range calendars) and the focus and key handlers."
                        </ApiRow>
                        <ApiRow name="start_date" ty="Signal<Date>">"The first day of the grid\u{2019}s month."</ApiRow>
                        <ApiRow name="week_days" ty="Signal<Vec<String>>">
                            "The weekday names in column order, in the locale\u{2019}s language. Hide the header row from "
                            "assistive technology: each day\u{2019}s label names its weekday."
                        </ApiRow>
                        <ApiRow name="weeks_in_month" ty="Signal<u8>">"The number of week rows."</ApiRow>
                    </ApiTable>

                    <p>
                        "Render a row per week and get its days from "<Code inline=true>"dates_in_week(week, Some(start_date))"</Code>
                        " of the calendar state, as the demo does."
                    </p>
                </Section>
            </Section>

            <Section title="use_calendar_cell">
                <p>
                    "A day: a "<Code inline=true>"<td>"</Code>" with "<Code inline=true>"role=\"gridcell\""</Code>" and a "
                    "button inside it, which takes the focus. Pressing the button selects the day. The button takes the "
                    "browser focus when its day becomes the focused date while the grid has the focus, and scrolls into view "
                    "when the keyboard moved it there."
                </p>

                <Section title="Input" id="use-calendar-cell-input">
                    <p>"Pass a "<Code inline=true>"UseCalendarCellInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>

                    <ApiTable kind=ApiKind::Input of="calendar::use_calendar_cell::UseCalendarCellInput">
                        <ApiRow name="data" ty="CalendarData">"The calendar\u{2019}s data. Required."</ApiRow>
                        <ApiRow name="date" ty="Signal<Date>">
                            "The day. Required. It may change: a calendar can keep its cells, and the focus in them, while it pages."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables the day regardless of the calendar."</ApiRow>
                        <ApiRow name="is_outside_month" ty="Signal<bool>" default="false">
                            "Whether the day belongs to another month than the grid\u{2019}s. Such days are shown but disabled."
                        </ApiRow>
                        <ApiRow name="element" ty="CapturedElement" default="CapturedElement::new()">"Captures the button element, which the hook focuses."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-calendar-cell-return">
                    <ApiTable kind=ApiKind::Return of="calendar::use_calendar_cell::UseCalendarCellReturn">
                        <ApiRow name="cell_props" ty="UseCalendarCellProps">
                            "For the "<Code inline=true>"<td>"</Code>": "<Code inline=true>"role=\"gridcell\""</Code>", "
                            <Code inline=true>"aria-selected"</Code>", "<Code inline=true>"aria-disabled"</Code>" and "
                            <Code inline=true>"aria-invalid"</Code>"."
                        </ApiRow>
                        <ApiRow name="button_props" ty="PropsWithStyles<UseCalendarCellButtonProps>">
                            "For the button: "<Code inline=true>"role=\"button\""</Code>", a roving "<Code inline=true>"tabindex"</Code>
                            " (0 on the focused date), a label such as \u{201c}Today, Thursday, March 12, 2026 selected\u{201d}, "
                            "the press handlers and the element capture. Spread it with "<Code inline=true>"into_parts()"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_pressed, is_focused, is_selected" ty="Signal<bool>">
                            "Whether the day is pressed, the focused date while the grid has the focus, and selected (in a "
                            "range calendar: in the highlighted range)."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">
                            "Whether the day can\u{2019}t be focused or selected: disabled calendar, outside the visible range, "
                            "the minimum and maximum or the grid\u{2019}s month."
                        </ApiRow>
                        <ApiRow name="is_unavailable" ty="Signal<bool>">"Whether the day is unavailable."</ApiRow>
                        <ApiRow name="is_outside_visible_range" ty="Signal<bool>">"Whether the day lies outside the visible range."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the day is part of an invalid selection."</ApiRow>
                        <ApiRow name="is_today" ty="Signal<bool>">
                            "Whether the day is today in the browser. Always "<Code inline=true>"false"</Code>" on the server, "
                            "whose today may be another day."
                        </ApiRow>
                        <ApiRow name="formatted_date" ty="Signal<String>">"The day of the month, formatted for the locale: the button\u{2019}s text."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_calendar_heading">
                <p>
                    <Code inline=true>"use_calendar_heading"</Code>" returns the heading of a month as a "
                    <Code inline=true>"Signal<String>"</Code>": \u{201c}March 2026\u{201d}. A calendar showing weeks or days "
                    "gets a range of dates. Hide the heading from assistive technology: the calendar\u{2019}s label names the "
                    "visible range already."
                </p>

                <Section title="Input" id="use-calendar-heading-input">
                    <p>"The input has no defaults: set every field."</p>
                    <ApiTable kind=ApiKind::Input of="UseCalendarHeadingInput">
                        <ApiRow name="state" ty="CalendarStates">
                            "The calendar\u{2019}s state: "<Code inline=true>"data.state"</Code>" of "
                            <Code inline=true>"use_calendar"</Code>"\u{2019}s data. Required."
                        </ApiRow>
                        <ApiRow name="offset" ty="DateDuration">
                            "How far after the first visible month the heading\u{2019}s month is, for calendars showing several "
                            "("<Code inline=true>"DateDuration::default()"</Code>" for the first). Required."
                        </ApiRow>
                        <ApiRow name="format" ty="CalendarHeadingFormat">
                            "How the heading is formatted ("<Code inline=true>"CalendarHeadingFormat::default()"</Code>": "
                            "\u{201c}March 2026\u{201d}). Required."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="CalendarHeadingFormat">
                    <ApiTable kind=ApiKind::Fields of="CalendarHeadingFormat">
                        <ApiRow name="day" ty="Option<NumericFormat>" default="None">"Includes the day; always for weeks and days."</ApiRow>
                        <ApiRow name="month" ty="MonthFormat" default="Long">"The month: \u{201c}March\u{201d}, \u{201c}Mar\u{201d}, \u{201c}3\u{201d}, \u{2026}"</ApiRow>
                        <ApiRow name="year" ty="NumericFormat" default="Numeric">"The year: \u{201c}2026\u{201d} or \u{201c}26\u{201d}."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_calendar_month_picker">
                <p>
                    <Code inline=true>"use_calendar_month_picker"</Code>" lists the months of the focused date\u{2019}s year, "
                    "for a select or a grid of buttons that jumps to a month. Picking one moves the focused date there. The "
                    <Link href=format!("{}#calendarmonthpicker", routes::doc::calendar::Atom.materialize())>"CalendarMonthPicker"</Link>
                    " atom renders it as a select."
                </p>

                <Section title="Input" id="use-calendar-month-picker-input">
                    <p>"The input has no defaults: set every field."</p>
                    <ApiTable kind=ApiKind::Input of="UseCalendarMonthPickerInput">
                        <ApiRow name="state" ty="CalendarStates">"The calendar\u{2019}s state. Required."</ApiRow>
                        <ApiRow name="format" ty="MonthFormat">
                            "How the months are formatted, e.g. "<Code inline=true>"MonthFormat::Short"</Code>" (\u{201c}Mar\u{201d}). Required."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-calendar-month-picker-return">
                    <ApiTable kind=ApiKind::Return of="UseCalendarPickerReturn">
                        <ApiRow name="aria_label" ty="Signal<String>">"Names the picker: \u{201c}month\u{201d}."</ApiRow>
                        <ApiRow name="value" ty="Signal<i16>">"The focused date\u{2019}s month (1 to 12)."</ApiRow>
                        <ApiRow name="items" ty="Signal<Vec<CalendarPickerItem>>">
                            "The months, each with its "<Code inline=true>"id"</Code>" (the month), the "<Code inline=true>"date"</Code>
                            " it focuses and its "<Code inline=true>"formatted"</Code>" name."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Callback<i16>">"Moves the focused date to a month (an item\u{2019}s id)."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_calendar_year_picker">
                <p>
                    <Code inline=true>"use_calendar_year_picker"</Code>" lists years around the focused date\u{2019}s year, "
                    "within the minimum and maximum. It returns the same "
                    <AnchorLink href="#use-calendar-month-picker-return"><Code inline=true>"UseCalendarPickerReturn"</Code></AnchorLink>
                    ", named \u{201c}year\u{201d}; the items\u{2019} ids are the years. The "
                    <Link href=format!("{}#calendaryearpicker", routes::doc::calendar::Atom.materialize())>"CalendarYearPicker"</Link>
                    " atom renders it as a select."
                </p>

                <Section title="Input" id="use-calendar-year-picker-input">
                    <p>"The input has no defaults: set every field."</p>
                    <ApiTable kind=ApiKind::Input of="UseCalendarYearPickerInput">
                        <ApiRow name="state" ty="CalendarStates">"The calendar\u{2019}s state. Required."</ApiRow>
                        <ApiRow name="visible_years" ty="u8">"How many years to list, e.g. 20. Required."</ApiRow>
                        <ApiRow name="format" ty="CalendarYearPickerFormat">
                            "How the years are formatted: "<Code inline=true>"year"</Code>" ("<Code inline=true>"NumericFormat"</Code>
                            ", default "<Code inline=true>"Numeric"</Code>") and "<Code inline=true>"era"</Code>" ("
                            <Code inline=true>"Option<DateTimeFormat>"</Code>"; "<Code inline=true>"None"</Code>" shows the short "
                            "era for years before Christ only). Required ("<Code inline=true>"CalendarYearPickerFormat::default()"</Code>")."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Range Selection">
                <p>
                    "A range is selected by two presses (or "<Keys keys="Enter"/>"): the first sets the anchor, the second "
                    "the other end. In between, the highlighted range runs from the anchor to the focused date, so the "
                    "arrow keys and hovering preview it. With a mouse or a finger, dragging from one day to another selects "
                    "the range in one go, and dragging an end of the selected range moves that end. "<Keys keys="Escape"/>
                    " cancels a range in progress."
                </p>
                <p>
                    "The cells don\u{2019}t tell you which day starts or ends the range: compare their day with "
                    <Code inline=true>"highlighted_range"</Code>", as the demo does to round the ends of the band."
                </p>

                <Demo description="Range calendar built from use_range_calendar_state and use_range_calendar" source=include_str!("demos/calendar_range.rs")>
                    <CalendarRangeDemo/>
                </Demo>
            </Section>

            <Section title="Unavailable Dates">
                <p>
                    "Two inputs restrict the selection. "<Code inline=true>"min_value"</Code>" and "<Code inline=true>"max_value"</Code>
                    " disable the days outside them: they can\u{2019}t take the focus, and the page buttons stop there. "
                    <Code inline=true>"is_date_unavailable"</Code>" marks single days, such as booked nights: they can take "
                    "the focus, but pressing them does nothing. A range can\u{2019}t span an unavailable day unless "
                    <Code inline=true>"allows_non_contiguous_ranges"</Code>" is set: once the anchor is set, the days beyond "
                    "the next booked one are disabled."
                </p>
                <p>"The demo also starts its weeks on Sunday."</p>

                <Demo description="Booking calendar with a minimum, a maximum and booked days, weeks starting on Sunday" source=include_str!("demos/calendar_unavailable.rs")>
                    <CalendarUnavailableDemo/>
                </Demo>
            </Section>

            <Section title="Several Months">
                <p>
                    "With "<Code inline=true>"visible_duration: DateDuration::months(2)"</Code>", the visible range spans two "
                    "months: render a grid per month and give the second one "<Code inline=true>"start_date"</Code>" and "
                    <Code inline=true>"end_date"</Code>" a month after the visible range\u{2019}s start (the "
                    <Link href=format!("{}#calendargrid", routes::doc::calendar::Atom.materialize())>"CalendarGrid"</Link>
                    " atom does this with its "<Code inline=true>"offset"</Code>"). The page buttons then move by two months, "
                    "or by one with "<Code inline=true>"PageBehavior::Single"</Code>". A duration in weeks or days shows "
                    "that many days in a single grid."
                </p>
            </Section>

            <Section title="Internationalization">
                <p>
                    "The weekday names, the month names, the day numbers and the first day of the week follow the locale of "
                    "the surrounding "<Link href=routes::doc::utilities::I18nProvider.materialize()>"I18nProvider"</Link>
                    ", and the arrow keys follow the writing direction. The labels and announcements (\u{201c}Previous\u{201d}, "
                    "\u{201c}Today\u{201d}, \u{201c}selected\u{201d}, \u{201c}Selected Range\u{201d}, the range selection "
                    "prompts) follow the locale too; only the Gregorian calendar is supported."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Calendar.materialize()>"Calendar overview"</Link></li>
                <li><Link href=routes::doc::calendar::Atom.materialize()>"Calendar Atoms"</Link></li>
                <li><Link href=routes::doc::date_picker::Hook.materialize()>"Date Picker Hooks"</Link></li>
                <li><Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live_announcer"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
