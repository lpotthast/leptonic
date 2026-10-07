use indoc::indoc;
use leptos::prelude::*;

use super::demos::{calendar::AtomCalendarDemo, calendar_range::AtomRangeCalendarDemo};
use crate::{kit::*, routes};

/// A link to a section of the Calendar Hooks page.
fn hooks_section(anchor: &str) -> String {
    format!("{}#{anchor}", routes::doc::calendar::Hook.materialize())
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomCalendar() -> impl IntoView {
    view! {
        <DocPage title="Calendar Atoms">
            <p>
                "The unstyled "<Code inline=true>"Calendar"</Code>" and "<Code inline=true>"RangeCalendar"</Code>
                " hold a month of days to pick a date or a range of dates from; you compose them from atoms for the "
                "heading, the page buttons, the grid and the days. See the "
                <Link href=routes::doc::Calendar.materialize()>"Calendar overview"</Link>" for when to use a calendar."
            </p>

            <ReactAria hook="Calendar"/>
            <ReactAria hook="RangeCalendar"/>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Built on"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Calendar"</Code>", "<Code inline=true>"RangeCalendar"</Code></TableCell>
                        <TableCell>
                            <Link href=hooks_section("use-calendar-state")>"use_calendar_state"</Link>" and "
                            <Link href=hooks_section("use-calendar")>"use_calendar"</Link>", or "
                            <Link href=hooks_section("use-range-calendar-state")>"use_range_calendar_state"</Link>" and "
                            <Link href=hooks_section("use-range-calendar")>"use_range_calendar"</Link>
                            ". They hand the state to the atoms inside through a context."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"CalendarHeading"</Code></TableCell>
                        <TableCell><Link href=hooks_section("use-calendar-heading")>"use_calendar_heading"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"CalendarMonthPicker"</Code>", "<Code inline=true>"CalendarYearPicker"</Code></TableCell>
                        <TableCell>
                            <Link href=hooks_section("use-calendar-month-picker")>"use_calendar_month_picker"</Link>", "
                            <Link href=hooks_section("use-calendar-year-picker")>"use_calendar_year_picker"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"CalendarPreviousButton"</Code>", "<Code inline=true>"CalendarNextButton"</Code></TableCell>
                        <TableCell>
                            <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" with the calendar\u{2019}s "
                            <Code inline=true>"previous_button"</Code>" and "<Code inline=true>"next_button"</Code>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"CalendarGrid"</Code>" and its parts"</TableCell>
                        <TableCell><Link href=hooks_section("use-calendar-grid")>"use_calendar_grid"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"CalendarCell"</Code>", "<Code inline=true>"CalendarCellButton"</Code></TableCell>
                        <TableCell>
                            <Link href=hooks_section("use-calendar-cell")>"use_calendar_cell"</Link>", "
                            <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>" and "
                            <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>"."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <p>
                    "The atoms are in "<Code inline=true>"leptonic::atoms::calendar"</Code>". The header row renders "
                    "its children per weekday name, the body per week, a week per day."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{atoms::calendar::*, jiff::civil::Date};
                        use leptos::prelude::*;

                        let value = RwSignal::new(None::<Date>);

                        view! {
                            <Calendar value set_value=value aria_label="Appointment date">
                                <header>
                                    <CalendarPreviousButton>"<"</CalendarPreviousButton>
                                    <CalendarHeading/>
                                    <CalendarNextButton>">"</CalendarNextButton>
                                </header>
                                <CalendarGrid>
                                    <CalendarGridHeader>
                                        <CalendarHeaderRow children=|day| view! { <CalendarHeaderCell>{day}</CalendarHeaderCell> }/>
                                    </CalendarGridHeader>
                                    <CalendarGridBody children=|week| view! {
                                        <CalendarWeek week children=|date| view! {
                                            <CalendarCell date>
                                                <CalendarCellButton/>
                                            </CalendarCell>
                                        }/>
                                    }/>
                                </CalendarGrid>
                            </Calendar>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Click a day, or tab into the grid, move with the arrow keys and press "<Keys keys="Enter"/>
                    ". Weekends are unavailable: they take the focus, but can\u{2019}t be selected."
                </p>

                <Demo description="Calendar of the atoms with unavailable weekends, with disabled and read-only toggles" source=include_str!("demos/calendar.rs")>
                    <AtomCalendarDemo/>
                </Demo>
            </Section>

            <Section title="Calendar">
                <p>
                    "A calendar for a single date: a "<Code inline=true>"<div>"</Code>" with "
                    <Code inline=true>"role=\"application\""</Code>", named by "<Code inline=true>"aria_label"</Code>
                    " and the visible month. Put the other atoms inside."
                </p>

                <Section title="Props" id="calendar-props">
                    <ApiTable kind=ApiKind::Props of="Calendar">
                        <ApiRow name="default_value" ty="Option<Date>" default="None">"The initially selected date."</ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Option<Date>>>" default="None">
                            "The selected date (controlled): a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Option<Date>>>" default="None">
                            "Receives the selected date: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                            ", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<Date>>>" default="None">"Called with the selected date."</ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<Option<Date>>" default="None">
                            "The first and last selectable date. Days outside are disabled."
                        </ApiRow>
                        <ApiRow name="is_date_unavailable" ty="Option<Callback<Date, bool>>" default="None">
                            "Whether a date can\u{2019}t be selected, e.g. because it is booked."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Nothing can be focused or selected."</ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">"The focus moves, but nothing can be selected."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the value invalid, e.g. after form validation."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Whether the focused date takes the focus when the calendar is rendered."</ApiRow>
                        <ApiRow name="default_focused_value" ty="Option<Date>" default="None">
                            "The initially focused date, which decides the month shown first. Default: the value, else today."
                        </ApiRow>
                        <ApiRow name="focused_value" ty="Option<Signal<Date>>" default="None">"The focused date (controlled)."</ApiRow>
                        <ApiRow name="set_focused_value" ty="Option<Out<Date>>" default="None">"Receives the focused date."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<Date>>" default="None">"Called with the focused date."</ApiRow>
                        <ApiRow name="visible_duration" ty="Signal<DateDuration>" default="DateDuration::months(1)">
                            "How much is visible at once. See "<AnchorLink href="#calendargrid">"CalendarGrid"</AnchorLink>
                            " for several months. A change aligns the visible range around the focused date again."
                        </ApiRow>
                        <ApiRow name="page_behavior" ty="Signal<PageBehavior>" default="Visible">
                            "Whether the page buttons move by the visible duration or by one unit of it ("<Code inline=true>"Single"</Code>
                            ", e.g. one month of a two-month calendar)."
                        </ApiRow>
                        <ApiRow name="first_day_of_week" ty="MaybeProp<Weekday>" default="None">"The first day of the week. Default: the locale\u{2019}s."</ApiRow>
                        <ApiRow name="selection_alignment" ty="Signal<SelectionAlignment>" default="Center">
                            "Where the focused date sits in several visible months, initially and when the duration changes."
                        </ApiRow>
                        <ApiRow name="weeks_in_month" ty="MaybeProp<u8>" default="None">
                            "A fixed number of week rows, e.g. 6, so that the calendar keeps its height."
                        </ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The calendar\u{2019}s id. Generated when not given."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the calendar, together with the visible month."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby, aria_details" ty="Option<String>" default="None">
                            "Ids of elements labelling, describing or detailing the calendar."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the calendar element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The heading, the buttons and the grids. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="RangeCalendar">
                <p>
                    "A calendar for a range of dates, picked by two presses or by dragging. Its parts are those of "
                    <Code inline=true>"Calendar"</Code>". This one shows two months: one "<Code inline=true>"CalendarGrid"</Code>
                    " and one "<Code inline=true>"CalendarHeading"</Code>" per month, the second with an "
                    <Code inline=true>"offset"</Code>" of a month."
                </p>

                <Demo description="Range calendar of the atoms showing two months, with a disabled toggle" source=include_str!("demos/calendar_range.rs")>
                    <AtomRangeCalendarDemo/>
                </Demo>

                <Section title="Props" id="rangecalendar-props">
                    <p>"The props shared with "<AnchorLink href="#calendar-props">"Calendar"</AnchorLink>" work the same; the values are ranges."</p>

                    <ApiTable kind=ApiKind::Props of="RangeCalendar">
                        <ApiRow name="default_value" ty="Option<DateRange>" default="None">"The initially selected range."</ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Option<DateRange>>>" default="None">"The selected range (controlled)."</ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Option<DateRange>>>" default="None">"Receives the selected range."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<DateRange>>>" default="None">"Called with the selected range."</ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<Option<Date>>" default="None">"The first and last selectable date."</ApiRow>
                        <ApiRow name="is_date_unavailable" ty="Option<Callback<DateAvailabilityQuery, bool>>" default="None">
                            "Whether a date can\u{2019}t be selected. The query holds the "<Code inline=true>"date"</Code>" and the "
                            <Code inline=true>"anchor_date"</Code>", the first selected day of a range in progress."
                        </ApiRow>
                        <ApiRow name="allows_non_contiguous_ranges" ty="bool" default="false">"Whether a range may span unavailable dates."</ApiRow>
                        <ApiRow name="commit_behavior" ty="CommitBehavior" default="Select">
                            "What a press outside the days, or the focus leaving, does with a range in progress: finish it at "
                            "the focused date ("<Code inline=true>"Select"</Code>"), drop it ("<Code inline=true>"Reset"</Code>
                            ") or clear the value too ("<Code inline=true>"Clear"</Code>")."
                        </ApiRow>
                        <ApiRow name="is_disabled, is_read_only, is_invalid" ty="Signal<bool>" default="false">"As in "<Code inline=true>"Calendar"</Code>"."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Whether the focused date takes the focus when the calendar is rendered."</ApiRow>
                        <ApiRow name="default_focused_value" ty="Option<Date>" default="None">"The initially focused date. Default: the range\u{2019}s start, else today."</ApiRow>
                        <ApiRow name="focused_value" ty="Option<Signal<Date>>" default="None">"The focused date (controlled)."</ApiRow>
                        <ApiRow name="set_focused_value" ty="Option<Out<Date>>" default="None">"Receives the focused date."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<Date>>" default="None">"Called with the focused date."</ApiRow>
                        <ApiRow name="visible_duration" ty="Signal<DateDuration>" default="DateDuration::months(1)">"How much is visible at once."</ApiRow>
                        <ApiRow name="page_behavior" ty="Signal<PageBehavior>" default="Visible">"How the page buttons move."</ApiRow>
                        <ApiRow name="first_day_of_week" ty="MaybeProp<Weekday>" default="None">"The first day of the week. Default: the locale\u{2019}s."</ApiRow>
                        <ApiRow name="selection_alignment" ty="MaybeProp<SelectionAlignment>" default="None">
                            "Where the initially focused date sits in several visible months. Default: centered, or at the "
                            "start if the range wouldn\u{2019}t fit then."
                        </ApiRow>
                        <ApiRow name="weeks_in_month" ty="MaybeProp<u8>" default="None">"A fixed number of week rows."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The calendar\u{2019}s id. Generated when not given."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the calendar, together with the visible months."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby, aria_details" ty="Option<String>" default="None">
                            "Ids of elements labelling, describing or detailing the calendar."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the calendar element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The heading, the buttons and the grids. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="CalendarHeading">
                <p>
                    "An "<Code inline=true>"<h2>"</Code>" with the visible month: \u{201c}March 2026\u{201d}. It is hidden "
                    "from assistive technology, as the calendar\u{2019}s name says the month already."
                </p>

                <Section title="Props" id="calendarheading-props">
                    <ApiTable kind=ApiKind::Props of="CalendarHeading">
                        <ApiRow name="offset" ty="DateDuration" default="zero">"Shows a later month, for calendars showing several."</ApiRow>
                        <ApiRow name="format" ty="CalendarHeadingFormat" default="Default">
                            "How the month is formatted, see "
                            <Link href=hooks_section("use-calendar-heading")>"use_calendar_heading"</Link>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the heading."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="CalendarMonthPicker">
                <p>
                    "Lets the user jump to a month of the focused date\u{2019}s year. It renders no element of its own: its "
                    <Code inline=true>"children"</Code>" receive the "
                    <Link href=hooks_section("use-calendar-month-picker-return")>"UseCalendarPickerReturn"</Link>
                    " (the months, the focused month and "<Code inline=true>"on_change"</Code>", which moves the focused "
                    "date) and render the picker, e.g. a "<Code inline=true>"<select>"</Code>"."
                </p>

                <Section title="Props" id="calendarmonthpicker-props">
                    <ApiTable kind=ApiKind::Props of="CalendarMonthPicker">
                        <ApiRow name="children" ty="FnOnce(UseCalendarPickerReturn) -> impl IntoView">"Renders the picker. Required."</ApiRow>
                        <ApiRow name="format" ty="Option<MonthFormat>" default="None">
                            "How the months are formatted. "<Code inline=true>"None"</Code>": "<Code inline=true>"Short"</Code>
                            " (\u{201c}Mar\u{201d})."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="calendarmonthpicker-example">
                    <p>"A "<Code inline=true>"<select>"</Code>" for either picker, placed in the calendar\u{2019}s header:"</p>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{atoms::calendar::*, hooks::calendar::UseCalendarPickerReturn};
                            use leptos::prelude::*;

                            #[component]
                            fn PickerSelect(picker: UseCalendarPickerReturn) -> impl IntoView {
                                let UseCalendarPickerReturn { aria_label, value, items, on_change } = picker;
                                view! {
                                    <select
                                        aria-label=aria_label
                                        on:change=move |e| {
                                            if let Ok(id) = event_target_value(&e).parse() {
                                                on_change.run(id);
                                            }
                                        }
                                    >
                                        <For each=move || items.get() key=|item| item.id let:item>
                                            <option value=item.id.to_string() selected=move || value.get() == item.id>
                                                {item.formatted}
                                            </option>
                                        </For>
                                    </select>
                                }
                            }

                            view! {
                                <Calendar aria_label="Birthday">
                                    <header>
                                        <CalendarMonthPicker children=|picker| view! { <PickerSelect picker/> }/>
                                        <CalendarYearPicker visible_years=100 children=|picker| view! { <PickerSelect picker/> }/>
                                    </header>
                                    // The grid, as above.
                                </Calendar>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="CalendarYearPicker">
                <p>
                    "Lets the user jump to a year: "<Code inline=true>"visible_years"</Code>" years around the focused "
                    "date\u{2019}s year, within the minimum and maximum. Like "
                    <AnchorLink href="#calendarmonthpicker">"CalendarMonthPicker"</AnchorLink>", it renders no element: its "
                    <Code inline=true>"children"</Code>" render the picker from the "<Code inline=true>"UseCalendarPickerReturn"</Code>
                    ", whose item ids are the years."
                </p>

                <Section title="Props" id="calendaryearpicker-props">
                    <ApiTable kind=ApiKind::Props of="CalendarYearPicker">
                        <ApiRow name="children" ty="FnOnce(UseCalendarPickerReturn) -> impl IntoView">"Renders the picker. Required."</ApiRow>
                        <ApiRow name="visible_years" ty="Option<u8>" default="None">
                            "How many years to offer. "<Code inline=true>"None"</Code>": 20."
                        </ApiRow>
                        <ApiRow name="format" ty="CalendarYearPickerFormat" default="CalendarYearPickerFormat::default()">
                            "How the years are formatted, see "
                            <Link href=hooks_section("use-calendar-year-picker-input")>"use_calendar_year_picker"</Link>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="CalendarPreviousButton">
                <p>
                    "A "<Code inline=true>"<button>"</Code>" named \u{201c}Previous\u{201d} that shows the previous page. It is "
                    "disabled where the minimum ends the dates; disabled while focused, it hands the focus to the grid."
                </p>

                <Section title="Props" id="calendarpreviousbutton-props">
                    <ApiTable kind=ApiKind::Props of="CalendarPreviousButton">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the button."</ApiRow>
                        <ApiRow name="children" ty="Children">"The button\u{2019}s content, e.g. an icon. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="CalendarNextButton">
                <p>"A "<Code inline=true>"<button>"</Code>" named \u{201c}Next\u{201d} that shows the next page, disabled where the maximum ends the dates."</p>

                <Section title="Props" id="calendarnextbutton-props">
                    <ApiTable kind=ApiKind::Props of="CalendarNextButton">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the button."</ApiRow>
                        <ApiRow name="children" ty="Children">"The button\u{2019}s content, e.g. an icon. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="CalendarErrorMessage">
                <p>
                    "A "<Code inline=true>"<div>"</Code>" for the reason the selection is invalid. Invalid selected days refer "
                    "to it with "<Code inline=true>"aria-describedby"</Code>"."
                </p>

                <Section title="Props" id="calendarerrormessage-props">
                    <ApiTable kind=ApiKind::Props of="CalendarErrorMessage">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the message."</ApiRow>
                        <ApiRow name="children" ty="Children">"The message. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="CalendarGrid">
                <p>
                    "A month of days: a "<Code inline=true>"<table>"</Code>" with "<Code inline=true>"role=\"grid\""</Code>
                    ". It handles the keys for the days inside. For a calendar showing several months (a "
                    <Code inline=true>"visible_duration"</Code>" of "<Code inline=true>"DateDuration::months(2)"</Code>
                    "), render a grid per month and give the later ones an "<Code inline=true>"offset"</Code>"."
                </p>

                <Section title="Props" id="calendargrid-props">
                    <ApiTable kind=ApiKind::Props of="CalendarGrid">
                        <ApiRow name="offset" ty="DateDuration" default="zero">"Shows a later month: "<Code inline=true>"DateDuration::months(1)"</Code>" for the second."</ApiRow>
                        <ApiRow name="weekday_style" ty="Option<DateTimeFormat>" default="None">
                            "How the weekday names are formatted: "<Code inline=true>"Narrow"</Code>" (\u{201c}M\u{201d}, the "
                            "default), "<Code inline=true>"Short"</Code>" or "<Code inline=true>"Long"</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the table."</ApiRow>
                        <ApiRow name="children" ty="Children">"A "<Code inline=true>"CalendarGridHeader"</Code>" and a "<Code inline=true>"CalendarGridBody"</Code>". Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="CalendarGridHeader">
                <p>"The grid\u{2019}s "<Code inline=true>"<thead>"</Code>", hidden from assistive technology: each day\u{2019}s label names its weekday."</p>

                <Section title="Props" id="calendargridheader-props">
                    <ApiTable kind=ApiKind::Props of="CalendarGridHeader">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the header."</ApiRow>
                        <ApiRow name="children" ty="Children">"A "<Code inline=true>"CalendarHeaderRow"</Code>". Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="CalendarHeaderRow">
                <p>"The "<Code inline=true>"<tr>"</Code>" of weekday names, in the locale\u{2019}s language and order."</p>

                <Section title="Props" id="calendarheaderrow-props">
                    <ApiTable kind=ApiKind::Props of="CalendarHeaderRow">
                        <ApiRow name="children" ty="Fn(String) -> impl IntoView">"Renders a weekday name, e.g. as a "<Code inline=true>"CalendarHeaderCell"</Code>". Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the row."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="CalendarHeaderCell">
                <p>"A "<Code inline=true>"<th>"</Code>" with a weekday name."</p>

                <Section title="Props" id="calendarheadercell-props">
                    <ApiTable kind=ApiKind::Props of="CalendarHeaderCell">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the cell."</ApiRow>
                        <ApiRow name="children" ty="Children">"The weekday name. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="CalendarGridBody">
                <p>"The grid\u{2019}s "<Code inline=true>"<tbody>"</Code>", with as many rows as the month has weeks."</p>

                <Section title="Props" id="calendargridbody-props">
                    <ApiTable kind=ApiKind::Props of="CalendarGridBody">
                        <ApiRow name="children" ty="Fn(u8) -> impl IntoView">"Renders the week with an index, e.g. as a "<Code inline=true>"CalendarWeek"</Code>". Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the body."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="CalendarWeek">
                <p>
                    "A week\u{2019}s "<Code inline=true>"<tr>"</Code>". Its cells stay when the calendar pages, only their "
                    "dates change, so that the focus stays in the grid."
                </p>

                <Section title="Props" id="calendarweek-props">
                    <ApiTable kind=ApiKind::Props of="CalendarWeek">
                        <ApiRow name="week" ty="u8">"The week\u{2019}s index in the month, from "<Code inline=true>"CalendarGridBody"</Code>". Required."</ApiRow>
                        <ApiRow name="children" ty="Fn(Signal<Date>) -> impl IntoView">"Renders a day, e.g. as a "<Code inline=true>"CalendarCell"</Code>". Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the row."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="CalendarCell">
                <p>
                    "A day\u{2019}s "<Code inline=true>"<td>"</Code>" with "<Code inline=true>"role=\"gridcell\""</Code>
                    ", "<Code inline=true>"aria-selected"</Code>", "<Code inline=true>"aria-disabled"</Code>" and "
                    <Code inline=true>"aria-invalid"</Code>"."
                </p>

                <Section title="Props" id="calendarcell-props">
                    <ApiTable kind=ApiKind::Props of="CalendarCell">
                        <ApiRow name="date" ty="Signal<Date>">"The day, from "<Code inline=true>"CalendarWeek"</Code>". Required."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables the day regardless of the calendar."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the cell."</ApiRow>
                        <ApiRow name="children" ty="Children">"A "<Code inline=true>"CalendarCellButton"</Code>". Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="CalendarCellButton">
                <p>
                    "The day\u{2019}s focusable "<Code inline=true>"<div>"</Code>" with "<Code inline=true>"role=\"button\""</Code>
                    ", labelled with the full date (\u{201c}Today, Thursday, March 12, 2026 selected\u{201d}). It takes the focus "
                    "when its day becomes the focused date, and carries the day\u{2019}s state as data attributes."
                </p>

                <Section title="Props" id="calendarcellbutton-props">
                    <ApiTable kind=ApiKind::Props of="CalendarCellButton">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the button."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">"The content. Default: the day of the month."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>"On the day buttons ("<Code inline=true>"CalendarCellButton"</Code>"):"</p>
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-selected" ty="true">"The day is selected, or lies in the highlighted range of a range calendar."</ApiRow>
                    <ApiRow name="data-selection-start, data-selection-end" ty="true">"The first or last day of the highlighted range (range calendars)."</ApiRow>
                    <ApiRow name="data-today" ty="true">"The day is today. Set in the browser only."</ApiRow>
                    <ApiRow name="data-focused, data-focus-visible" ty="true">"The day has the focus; by keyboard."</ApiRow>
                    <ApiRow name="data-hovered, data-pressed" ty="true">"A pointer is over the day; it presses the day."</ApiRow>
                    <ApiRow name="data-disabled" ty="true">
                        "The day can\u{2019}t be focused or selected: the calendar or the cell ("<Code inline=true>"is_disabled"</Code>
                        ") is disabled, or the day lies outside the minimum and maximum, the grid\u{2019}s month or the "
                        "visible range."
                    </ApiRow>
                    <ApiRow name="data-unavailable" ty="true">"The day is unavailable: it takes the focus, but can\u{2019}t be selected."</ApiRow>
                    <ApiRow name="data-invalid" ty="true">"The day is part of an invalid selection."</ApiRow>
                    <ApiRow name="data-outside-month" ty="true">"The day belongs to the previous or next month."</ApiRow>
                    <ApiRow name="data-outside-visible-range" ty="true">"The day lies outside the visible range."</ApiRow>
                </ApiTable>

                <p>
                    "On "<Code inline=true>"Calendar"</Code>" and "<Code inline=true>"RangeCalendar"</Code>": "
                    <Code inline=true>"data-disabled"</Code>" and "<Code inline=true>"data-invalid"</Code>". On the page "
                    "buttons: "<Code inline=true>"data-pressed"</Code>", "<Code inline=true>"data-hovered"</Code>", "
                    <Code inline=true>"data-focused"</Code>", "<Code inline=true>"data-focus-visible"</Code>" and "
                    <Code inline=true>"data-disabled"</Code>"."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. Each renders its element with its default class, followed by the "<Code inline=true>"classes"</Code>" you "
                    "pass: "<Code inline=true>"Calendar"</Code>" and "<Code inline=true>"RangeCalendar"</Code>" a "<Code inline=true>"<div>"</Code>" ("<Code inline=true>"leptonic-Calendar"</Code>", "
                    <Code inline=true>"leptonic-RangeCalendar"</Code>"), "<Code inline=true>"CalendarHeading"</Code>" an "<Code inline=true>"<h2>"</Code>" ("<Code inline=true>"leptonic-CalendarHeading"</Code>"), "
                    "the page buttons "<Code inline=true>"<button>"</Code>"s ("<Code inline=true>"leptonic-CalendarPreviousButton"</Code>", "<Code inline=true>"leptonic-CalendarNextButton"</Code>"), "
                    <Code inline=true>"CalendarGrid"</Code>" a "<Code inline=true>"<table>"</Code>" ("<Code inline=true>"leptonic-CalendarGrid"</Code>") with header cells ("
                    <Code inline=true>"leptonic-CalendarHeaderCell"</Code>") and weeks ("<Code inline=true>"leptonic-CalendarWeek"</Code>"), "<Code inline=true>"CalendarCell"</Code>" a "
                    <Code inline=true>"<td>"</Code>" ("<Code inline=true>"leptonic-CalendarCell"</Code>") and "<Code inline=true>"CalendarCellButton"</Code>" the day ("
                    <Code inline=true>"leptonic-CalendarCellButton"</Code>"). The month and year pickers render elements of your own. The row around the heading and the page buttons\u{2019} arrows are your own "
                    "markup (the arrows "<Code inline=true>"aria-hidden"</Code>": the buttons are named \u{201c}Previous\u{201d} and \u{201c}Next\u{201d}). "
                    "For a range, make the selected days a band and fill its ends; collapse the grid\u{2019}s borders, so that the band "
                    "has no gaps. The demos above use this CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .demo-calendar { display: inline-flex; flex-direction: column; gap: 0.5rem; padding: 1rem; border: 1px solid var(--border); border-radius: 8px; }
                        .demo-calendar-header { display: flex; align-items: center; justify-content: space-between; }
                        .demo-calendar-title { margin: 0; font-size: 1rem; }
                        .demo-calendar-nav { width: 1.8em; height: 1.8em; border: none; border-radius: 50%; background: none; font-size: 1.25em; cursor: pointer; }
                        .demo-calendar-nav[data-hovered] { background: var(--surface); }
                        .demo-calendar-nav[data-disabled] { color: var(--muted); cursor: default; }
                        .demo-calendar-nav[data-focus-visible] { outline: 2px solid var(--focus); }
                        .demo-calendar-grid { border-collapse: collapse; }
                        .demo-calendar-grid :is(th, td) { padding: 0; text-align: center; }
                        .demo-calendar-day { display: flex; align-items: center; justify-content: center; width: 2.5em; height: 2.5em; border-radius: 50%; cursor: pointer; }
                        .demo-calendar-day[data-hovered] { background: var(--surface); }
                        .demo-calendar-day[data-today] { font-weight: 700; text-decoration: underline; }
                        .demo-calendar-day:is([data-outside-month], [data-disabled]) { color: var(--muted); cursor: default; }
                        .demo-calendar-day[data-unavailable] { color: var(--muted); text-decoration: line-through; cursor: not-allowed; }
                        .demo-calendar-day[data-selected] { background: var(--accent); color: var(--surface); }
                        .demo-calendar-day[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: -2px; }

                        .demo-calendar-range .demo-calendar-day[data-selected] { border-radius: 0; background: var(--surface); color: inherit; }
                        .demo-calendar-range .demo-calendar-day[data-selection-start] { border-radius: 999px 0 0 999px; }
                        .demo-calendar-range .demo-calendar-day[data-selection-end] { border-radius: 0 999px 999px 0; }
                        .demo-calendar-range .demo-calendar-day:is([data-selection-start], [data-selection-end]) { background: var(--accent); color: var(--surface); }
                    ")}
                </Code>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <Section title="Composition">
                <ul>
                    <li>
                        "Name the calendar with "<Code inline=true>"aria_label"</Code>" or "<Code inline=true>"aria_labelledby"</Code>
                        "; its name also says the visible month. The heading and the weekday names are hidden from "
                        "assistive technology, and the page buttons are named \u{201c}Previous\u{201d} and \u{201c}Next\u{201d}: "
                        "icons inside them need no label. The keys are listed on the "
                        <Link href=format!("{}#accessibility", routes::doc::Calendar.materialize())>"Calendar overview"</Link>"."
                    </li>
                    <li>
                        "Parts of your own get the calendar\u{2019}s state from "<Code inline=true>"use_calendar_states()"</Code>
                        " inside a calendar and pass it to the "<Link href=routes::doc::calendar::Hook.materialize()>"Calendar Hooks"</Link>
                        ". For month and year pickers, "<AnchorLink href="#calendarmonthpicker">"CalendarMonthPicker"</AnchorLink>" and "
                        <AnchorLink href="#calendaryearpicker">"CalendarYearPicker"</AnchorLink>" do this for you."
                    </li>
                    <li>
                        "Inside a "<Link href=routes::doc::date_picker::Atom.materialize()><Code inline=true>"DatePicker"</Code></Link>
                        " (or "<Code inline=true>"DateRangePicker"</Code>"), the calendar sits in a popover, takes the picker\u{2019}s "
                        "date, limits and state instead of its own, and takes the focus when it opens."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Calendar.materialize()>"Calendar overview"</Link></li>
                <li><Link href=routes::doc::calendar::Hook.materialize()>"Calendar Hooks"</Link></li>
                <li><Link href=routes::doc::date_picker::Atom.materialize()>"Date Picker Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
