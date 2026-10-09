use leptonic::{
    DateDuration, DateRange, DateTimeFormat, I18nProvider, Locale,
    atoms::calendar::{
        Calendar, CalendarCell, CalendarCellButton, CalendarErrorMessage, CalendarGrid,
        CalendarGridBody, CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow,
        CalendarHeading, CalendarMonthPicker, CalendarNextButton, CalendarPreviousButton,
        CalendarWeek, CalendarYearPicker, RangeCalendar, use_calendar_states,
    },
    hooks::calendar::{
        CommitBehavior, DateAvailabilityQuery, PageBehavior, SelectionAlignment,
        UseCalendarPickerReturn,
    },
    jiff::civil::{Date, Weekday, date},
};
use leptos::prelude::*;

use crate::pages::Section;

/// Unavailable dates of December 2021 (react-spectrum's `CalendarBase.test.js`): the 6th to the
/// 10th and the 22nd to the 26th.
const DECEMBER_2021: &[(Date, Date)] = &[
    (date(2021, 12, 6), date(2021, 12, 10)),
    (date(2021, 12, 22), date(2021, 12, 26)),
];

/// Unavailable dates of react-spectrum's `RangeCalendar.test.js` button cases.
const APRIL_25_TO_30_2022: &[(Date, Date)] = &[(date(2022, 4, 25), date(2022, 4, 30))];
const MAY_1_TO_4_2022: &[(Date, Date)] = &[(date(2022, 5, 1), date(2022, 5, 4))];
const MAY_2_TO_4_2022: &[(Date, Date)] = &[(date(2022, 5, 2), date(2022, 5, 4))];

/// Whether `date` lies in one of `intervals` (ends included).
fn in_intervals(intervals: &[(Date, Date)], date: Date) -> bool {
    intervals
        .iter()
        .any(|(start, end)| *start <= date && date <= *end)
}

fn is_weekend(date: Date) -> bool {
    matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday)
}

/// The parts inside a calendar: heading, buttons and `months` grids.
#[component]
fn Parts(
    #[prop(default = 1)] months: i32,
    #[prop(optional)] weekday_style: Option<DateTimeFormat>,
) -> impl IntoView {
    view! {
        <header>
            <CalendarPreviousButton>"<"</CalendarPreviousButton>
            <CalendarHeading />
            <CalendarNextButton>">"</CalendarNextButton>
        </header>
        {(0..months)
            .map(|month| {
                view! {
                    <CalendarGrid
                        offset=DateDuration::months(month)
                        nostrip:weekday_style=weekday_style
                    >
                        <CalendarGridHeader>
                            <CalendarHeaderRow children=|day| {
                                view! { <CalendarHeaderCell>{day}</CalendarHeaderCell> }
                            } />
                        </CalendarGridHeader>
                        <CalendarGridBody children=|week| {
                            view! {
                                <CalendarWeek
                                    week=week
                                    children=|date| {
                                        view! {
                                            <CalendarCell date=date>
                                                <CalendarCellButton />
                                            </CalendarCell>
                                        }
                                    }
                                />
                            }
                        } />
                    </CalendarGrid>
                }
            })
            .collect_view()}
        <CalendarErrorMessage>"Invalid date"</CalendarErrorMessage>
    }
}

fn format(value: Option<Date>) -> String {
    value.map_or_else(|| "none".to_owned(), |date| date.to_string())
}

fn format_range(value: Option<DateRange>) -> String {
    value.map_or_else(
        || "none".to_owned(),
        |range| format!("{} - {}", range.start, range.end),
    )
}

/// The values a controlled calendar reported (`none` before any), comma-separated.
fn format_changes(changes: &[String]) -> String {
    if changes.is_empty() {
        "none".to_owned()
    } else {
        changes.join(", ")
    }
}

/// A button inside a calendar clearing its value through its state (RAC `Calendar.test.js`,
/// "should support setting "null" for method setValue").
#[component]
fn ResetValue(name: &'static str) -> impl IntoView {
    let states = use_calendar_states();
    view! {
        <button
            id=format!("test-calendar-{name}-reset")
            on:click=move |_| {
                if let Some(states) = states {
                    states.calendar().set_value(None);
                }
            }
        >
            "Reset value"
        </button>
    }
}

/// A calendar `name` (its label) showing its value in `#test-calendar-{name}-value`, with a
/// button before it. A `controlled` calendar keeps its value and lists the dates it reported in
/// `#test-calendar-{name}-changes`; every calendar shows the last focused date it reported in
/// `#test-calendar-{name}-focus-change` (`none` before any).
#[component]
fn TestCalendar(
    name: &'static str,
    #[prop(optional)] default_value: Option<Date>,
    #[prop(optional)] min_value: Option<Date>,
    #[prop(optional)] max_value: Option<Date>,
    #[prop(optional)] unavailable_weekends: bool,
    /// Unavailable date intervals (instead of weekends).
    #[prop(optional)]
    unavailable: Option<&'static [(Date, Date)]>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_read_only: bool,
    #[prop(optional)] is_invalid: bool,
    #[prop(optional)] auto_focus: bool,
    #[prop(optional)] visible_duration: Option<DateDuration>,
    #[prop(optional)] first_day_of_week: Option<Weekday>,
    #[prop(optional)] page_behavior: PageBehavior,
    #[prop(optional)] selection_alignment: SelectionAlignment,
    #[prop(optional)] default_focused_value: Option<Date>,
    /// A fixed focused date (it moves only in what the calendar reports).
    #[prop(optional)]
    focused_value: Option<Date>,
    #[prop(optional)] weeks_in_month: Option<u8>,
    #[prop(optional)] weekday_style: Option<DateTimeFormat>,
    #[prop(default = 1)] months: i32,
    /// Also labelled by a heading (`#test-calendar-{name}-label`).
    #[prop(optional)]
    labelled: bool,
    /// Labelled only by the heading (no `aria-label`).
    #[prop(optional)]
    labelled_only_by_heading: bool,
    /// Without any label (named by its month).
    #[prop(optional)]
    unlabelled: bool,
    #[prop(optional)] id: Option<&'static str>,
    /// Described and detailed by paragraphs (`#test-calendar-{name}-description`, `-details`).
    #[prop(optional)]
    described: bool,
    /// The value stays; changes are only reported.
    #[prop(optional)]
    controlled: bool,
    /// With a button inside clearing the value through the calendar's state.
    #[prop(optional)]
    reset_button: bool,
) -> impl IntoView {
    let value = RwSignal::new(default_value);
    let changes = RwSignal::new(Vec::<String>::new());
    let focus_change = RwSignal::new(None::<Date>);
    let label_id = format!("test-calendar-{name}-label");
    let description_id = format!("test-calendar-{name}-description");
    let details_id = format!("test-calendar-{name}-details");
    let aria_labelledby = (labelled || labelled_only_by_heading).then(|| label_id.clone());
    let aria_label = (!unlabelled && !labelled_only_by_heading).then(|| name.to_owned());
    let is_date_unavailable = if unavailable_weekends {
        Some(Callback::new(is_weekend))
    } else {
        unavailable.map(|intervals| Callback::new(move |date: Date| in_intervals(intervals, date)))
    };
    let on_change = Callback::new(move |date: Option<Date>| {
        changes.update(|changes| changes.push(format(date)));
    });
    let calendar = if controlled {
        view! {
            <Calendar
                aria_label=aria_label
                nostrip:aria_labelledby=aria_labelledby
                value=Signal::stored(default_value)
                on_change=on_change
                is_read_only=is_read_only
                auto_focus=auto_focus
            >
                <Parts />
            </Calendar>
        }
        .into_any()
    } else {
        view! {
            <Calendar
                nostrip:id=id.map(str::to_owned)
                aria_label=aria_label
                nostrip:aria_labelledby=aria_labelledby
                nostrip:aria_describedby=described.then(|| description_id.clone())
                nostrip:aria_details=described.then(|| details_id.clone())
                value=value
                set_value=value
                min_value=min_value
                max_value=max_value
                nostrip:is_date_unavailable=is_date_unavailable
                is_disabled=is_disabled
                is_read_only=is_read_only
                is_invalid=is_invalid
                auto_focus=auto_focus
                visible_duration=visible_duration.unwrap_or(DateDuration::months(months))
                first_day_of_week=first_day_of_week
                page_behavior=page_behavior
                selection_alignment=selection_alignment
                nostrip:default_focused_value=default_focused_value
                nostrip:focused_value=focused_value.map(Signal::stored)
                on_focused_value_change=Callback::new(move |date: Date| {
                    focus_change.set(Some(date))
                })
                weeks_in_month=weeks_in_month
            >
                <Parts
                    months=if visible_duration.is_some() { 1 } else { months }
                    nostrip:weekday_style=weekday_style
                />
                {reset_button.then(|| view! { <ResetValue name=name /> })}
            </Calendar>
        }
        .into_any()
    };
    view! {
        <Section name=name>
            <section id=format!("test-calendar-{name}")>
                <button id=format!("test-calendar-{name}-before")>"Before"</button>
                {(labelled || labelled_only_by_heading)
                    .then(|| view! { <h3 id=label_id>"Booking"</h3> })}
                {described
                    .then(|| {
                        view! {
                            <p id=description_id.clone()>"Pick a date for the booking."</p>
                            <p id=details_id.clone()>"Bookings start at noon."</p>
                        }
                    })}
                {calendar}
                <div>
                    "Value: "
                    <span id=format!(
                        "test-calendar-{name}-value",
                    )>{move || format(value.get())}</span>
                </div>
                <div>
                    "Changes: "
                    <span id=format!(
                        "test-calendar-{name}-changes",
                    )>{move || changes.with(|changes| format_changes(changes))}</span>
                </div>
                <div>
                    "Focus change: "
                    <span id=format!(
                        "test-calendar-{name}-focus-change",
                    )>{move || format(focus_change.get())}</span>
                </div>
            </section>
        </Section>
    }
}

/// A range calendar `name` showing its value in `#test-calendar-{name}-value`. A `controlled`
/// range calendar keeps its value and lists the ranges it reported in
/// `#test-calendar-{name}-changes`.
#[component]
fn TestRangeCalendar(
    name: &'static str,
    #[prop(optional)] default_value: Option<DateRange>,
    #[prop(optional)] default_focused_value: Option<Date>,
    #[prop(optional)] min_value: Option<Date>,
    #[prop(optional)] max_value: Option<Date>,
    #[prop(optional)] unavailable_weekends: bool,
    /// Unavailable date intervals.
    #[prop(optional)]
    unavailable: Option<&'static [(Date, Date)]>,
    /// Dates more than a week from the range's anchor are unavailable.
    #[prop(optional)]
    max_week: bool,
    #[prop(optional)] allows_non_contiguous_ranges: bool,
    #[prop(optional)] commit_behavior: CommitBehavior,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_read_only: bool,
    #[prop(optional)] is_invalid: bool,
    #[prop(optional)] auto_focus: bool,
    #[prop(default = 1)] months: i32,
    #[prop(optional)] selection_alignment: Option<SelectionAlignment>,
    /// The value stays; changes are only reported.
    #[prop(optional)]
    controlled: bool,
) -> impl IntoView {
    let value = RwSignal::new(default_value);
    let changes = RwSignal::new(Vec::<String>::new());
    let is_date_unavailable = if unavailable_weekends {
        Some(Callback::new(|query: DateAvailabilityQuery| {
            is_weekend(query.date)
        }))
    } else if let Some(intervals) = unavailable {
        Some(Callback::new(move |query: DateAvailabilityQuery| {
            in_intervals(intervals, query.date)
        }))
    } else if max_week {
        Some(Callback::new(|query: DateAvailabilityQuery| {
            query
                .anchor_date
                .is_some_and(|anchor| (query.date - anchor).get_days().abs() > 7)
        }))
    } else {
        None
    };
    let on_change = Callback::new(move |range: Option<DateRange>| {
        changes.update(|changes| changes.push(format_range(range)));
    });
    let calendar = if controlled {
        view! {
            <RangeCalendar
                aria_label=name
                value=Signal::stored(default_value)
                on_change=on_change
                auto_focus=auto_focus
            >
                <Parts />
            </RangeCalendar>
        }
        .into_any()
    } else {
        view! {
            <RangeCalendar
                aria_label=name
                value=value
                set_value=value
                on_change=on_change
                min_value=min_value
                max_value=max_value
                nostrip:is_date_unavailable=is_date_unavailable
                allows_non_contiguous_ranges=allows_non_contiguous_ranges
                nostrip:default_focused_value=default_focused_value
                commit_behavior=commit_behavior
                is_disabled=is_disabled
                is_read_only=is_read_only
                is_invalid=is_invalid
                auto_focus=auto_focus
                visible_duration=DateDuration::months(months)
                selection_alignment=selection_alignment
            >
                <Parts months=months />
            </RangeCalendar>
        }
        .into_any()
    };
    view! {
        <Section name=name>
            <section id=format!("test-calendar-{name}")>
                <button id=format!("test-calendar-{name}-before")>"Before"</button>
                {calendar}
                <div>
                    "Value: "
                    <span id=format!(
                        "test-calendar-{name}-value",
                    )>{move || format_range(value.get())}</span>
                </div>
                <div>
                    "Changes: "
                    <span id=format!(
                        "test-calendar-{name}-changes",
                    )>{move || changes.with(|changes| format_changes(changes))}</span>
                </div>
                <button id=format!("test-calendar-{name}-after")>"After"</button>
                <button id=format!("test-calendar-{name}-clear") on:click=move |_| value.set(None)>
                    "Clear"
                </button>
            </section>
        </Section>
    }
}

/// A calendar with the classes and DOM attributes of its parts set (RAC `Calendar.test.js`,
/// "should render with custom classes", "should support DOM props").
#[component]
fn ClassesCalendar() -> impl IntoView {
    view! {
        <Section name="classes">
            <section id="test-calendar-classes">
                <Calendar
                    aria_label="classes"
                    default_value=date(2019, 6, 5)
                    classes="calendar"
                    attr:data-foo="bar"
                >
                    <CalendarGrid classes="grid" attr:data-bar="baz">
                        <CalendarGridBody children=|week| {
                            view! {
                                <CalendarWeek
                                    week=week
                                    children=|date| {
                                        view! {
                                            <CalendarCell date=date>
                                                <CalendarCellButton classes="cell" attr:data-baz="foo" />
                                            </CalendarCell>
                                        }
                                    }
                                />
                            }
                        } />
                    </CalendarGrid>
                </Calendar>
            </section>
        </Section>
    }
}

/// A calendar with its focused date as app state, moved by `#test-calendar-focus-set` (to June 20).
#[component]
fn FocusCalendar() -> impl IntoView {
    let focused = RwSignal::new(date(2019, 6, 5));
    view! {
        <Section name="focus">
            <section id="test-calendar-focus">
                <button
                    id="test-calendar-focus-set"
                    on:click=move |_| focused.set(date(2019, 6, 20))
                >
                    "Focus June 20"
                </button>
                <Calendar aria_label="focus" focused_value=focused set_focused_value=focused>
                    <Parts />
                </Calendar>
            </section>
        </Section>
    }
}

/// A `<select>` of a month or year picker, labelled by it.
fn picker_select(picker: UseCalendarPickerReturn) -> impl IntoView {
    view! {
        <select
            aria-label=picker.aria_label
            prop:value=move || picker.value.get().to_string()
            on:change=move |e| {
                if let Ok(id) = event_target_value(&e).parse::<i16>() {
                    picker.on_change.run(id);
                }
            }
        >
            <For
                each=move || picker.items.get()
                key=|item| (item.id, item.formatted.clone())
                children=move |item| {
                    let id = item.id;
                    view! {
                        <option value=id.to_string() selected=move || picker.value.get() == id>
                            {item.formatted}
                        </option>
                    }
                }
            />
        </select>
    }
}

/// A range calendar with month and year pickers (RAC `RangeCalendar.test.tsx`, "should support
/// month and year dropdowns"), focused on April 1, 2026.
#[component]
fn PickersCalendar() -> impl IntoView {
    view! {
        <Section name="pickers">
            <section id="test-calendar-pickers">
                <RangeCalendar aria_label="Appointment date" default_focused_value=date(2026, 4, 1)>
                    <header>
                        <CalendarPreviousButton>"<"</CalendarPreviousButton>
                        <CalendarMonthPicker children=picker_select />
                        <CalendarYearPicker children=picker_select />
                        <CalendarNextButton>">"</CalendarNextButton>
                    </header>
                    <Grid />
                </RangeCalendar>
            </section>
        </Section>
    }
}

/// A grid of a calendar's first month.
#[component]
fn Grid() -> impl IntoView {
    view! {
        <CalendarGrid>
            <CalendarGridHeader>
                <CalendarHeaderRow children=|day| {
                    view! { <CalendarHeaderCell>{day}</CalendarHeaderCell> }
                } />
            </CalendarGridHeader>
            <CalendarGridBody children=|week| {
                view! {
                    <CalendarWeek
                        week=week
                        children=|date| {
                            view! {
                                <CalendarCell date=date>
                                    <CalendarCellButton />
                                </CalendarCell>
                            }
                        }
                    />
                }
            } />
        </CalendarGrid>
    }
}

/// A calendar whose visible duration changes (RAC `Calendar.test.js`, "should handle changing the
/// visible duration"): a week of April 7, 2026, a month after `#test-calendar-duration-month`.
#[component]
fn DurationCalendar() -> impl IntoView {
    let duration = RwSignal::new(DateDuration::weeks(1));
    view! {
        <Section name="duration">
            <section id="test-calendar-duration">
                <button
                    id="test-calendar-duration-month"
                    on:click=move |_| duration.set(DateDuration::months(1))
                >
                    "Month"
                </button>
                <Calendar
                    aria_label="duration"
                    default_focused_value=date(2026, 4, 7)
                    visible_duration=duration
                >
                    <Parts />
                </Calendar>
            </section>
        </Section>
    }
}

/// Calendars (June 2019, as react-aria's tests) in different configurations: basic, min/max,
/// unavailable weekends, disabled, read-only, invalid, two months, a week, three days, a range
/// calendar (also for touch input) and one with unavailable weekends, one without a value
/// (showing today), paging by single units (two months, three weeks, five days, from January
/// 1, 2019), two weeks, six week rows, month and year pickers, a changing visible duration, and
/// range calendars with each commit behavior (November 13 to 15, 2025). Each calendar is a
/// [`Section`] named like the calendar (`focus`, `pickers`, `duration` and `classes` for the
/// components of their own).
///
/// The cases of react-spectrum's `CalendarBase.test.js`, `Calendar.test.js` and
/// `RangeCalendar.test.js` have sections of their own: several months (`three-months`,
/// `range-three-months`), the alignment of the initial value (`align-*`, `range-align-*`), limits
/// (`min-visible`, `limits-*`, `buttons-focus`, `week-limits`, `max-date`, `range-min-max`), BC
/// dates (`bc`, `range-bc`), unavailable intervals (`intervals`, `range-intervals`,
/// `range-*-unavailable`), controlled values and focused dates (`controlled`, `focus-*`,
/// `auto-*`), the first day of the week (`saturday`, `thursday`, `fr*`, `de`), labels
/// (`unlabelled`, `labelledby-only`, `custom-id`, `described`), and range selection
/// (`range-mid`, `range-*`).
#[component]
pub fn PageAtomCalendar() -> impl IntoView {
    let june5 = date(2019, 6, 5);
    let june_5_10 = DateRange {
        start: june5,
        end: date(2019, 6, 10),
    };
    let june_10_20 = DateRange {
        start: date(2019, 6, 10),
        end: date(2019, 6, 20),
    };
    let fr = || "fr-FR".parse::<Locale>().expect("a locale");
    let de = || "de-DE".parse::<Locale>().expect("a locale");
    let he = || "he".parse::<Locale>().expect("a locale");
    view! {
        <div id="test-page-atom-calendar">
            <h1>"Calendar"</h1>
            <TestCalendar name="basic" default_value=june5 />
            <TestCalendar
                name="min-max"
                default_value=date(2019, 6, 15)
                min_value=date(2019, 6, 10)
                max_value=date(2019, 6, 20)
            />
            <TestCalendar name="unavailable" default_value=june5 unavailable_weekends=true />
            <TestCalendar name="disabled" default_value=june5 is_disabled=true />
            <TestCalendar name="read-only" default_value=june5 is_read_only=true />
            <TestCalendar name="invalid" default_value=june5 is_invalid=true />
            <TestCalendar name="two-months" default_value=june5 months=2 />
            <TestCalendar name="week" default_value=june5 visible_duration=DateDuration::weeks(1) />
            <TestCalendar name="days" default_value=june5 visible_duration=DateDuration::days(3) />
            <TestCalendar name="monday" default_value=june5 first_day_of_week=Weekday::Monday />
            <TestCalendar name="labelled" default_value=june5 labelled=true />
            <I18nProvider locale=he()>
                <TestCalendar name="rtl" default_value=june5 />
            </I18nProvider>
            <FocusCalendar />
            <TestRangeCalendar name="range" default_value=june_5_10 />
            <TestRangeCalendar name="range-week" default_focused_value=june5 max_week=true />
            <TestRangeCalendar
                name="range-unavailable"
                default_focused_value=june5
                unavailable_weekends=true
            />
            <TestCalendar name="today" />
            <TestCalendar
                name="single-page"
                default_value=date(2019, 1, 1)
                months=2
                page_behavior=PageBehavior::Single
            />
            <TestCalendar
                name="weeks-single"
                default_value=date(2019, 1, 1)
                visible_duration=DateDuration::weeks(3)
                page_behavior=PageBehavior::Single
            />
            <TestCalendar
                name="days-single"
                default_value=date(2019, 1, 1)
                visible_duration=DateDuration::days(5)
                page_behavior=PageBehavior::Single
            />
            <TestCalendar
                name="two-weeks"
                default_value=june5
                visible_duration=DateDuration::weeks(2)
            />
            <TestCalendar
                name="six-weeks"
                default_focused_value=date(2026, 4, 1)
                weeks_in_month=6
            />
            <PickersCalendar />
            <DurationCalendar />
            <ClassesCalendar />
            <TestCalendar name="three-months" default_value=june5 months=3 />
            <TestRangeCalendar name="range-three-months" default_value=june_5_10 months=3 />
            {[
                ("align-start", SelectionAlignment::Start),
                ("align-center", SelectionAlignment::Center),
                ("align-end", SelectionAlignment::End),
            ]
                .into_iter()
                .map(|(name, alignment)| {
                    view! {
                        <TestCalendar
                            name=name
                            default_value=date(2020, 2, 3)
                            months=3
                            selection_alignment=alignment
                        />
                    }
                })
                .collect_view()}
            {[
                ("range-align-start", SelectionAlignment::Start),
                ("range-align-center", SelectionAlignment::Center),
                ("range-align-end", SelectionAlignment::End),
            ]
                .into_iter()
                .map(|(name, alignment)| {
                    view! {
                        <TestRangeCalendar
                            name=name
                            default_value=DateRange {
                                start: date(2020, 2, 3),
                                end: date(2020, 2, 10),
                            }
                            months=3
                            selection_alignment=alignment
                        />
                    }
                })
                .collect_view()}
            <TestCalendar
                name="min-visible"
                default_value=date(2019, 2, 3)
                min_value=date(2019, 2, 1)
                months=3
            />
            <TestRangeCalendar
                name="range-min-visible"
                default_value=DateRange {
                    start: date(2019, 2, 3),
                    end: date(2019, 2, 10),
                }
                min_value=date(2019, 2, 1)
                months=3
            />
            <TestRangeCalendar
                name="range-wide"
                default_value=DateRange {
                    start: date(2019, 1, 3),
                    end: date(2019, 3, 10),
                }
                months=3
            />
            <TestCalendar
                name="limits-previous"
                default_value=date(2019, 2, 10)
                min_value=date(2019, 2, 3)
                max_value=date(2019, 3, 20)
            />
            <TestCalendar
                name="limits-next"
                default_value=date(2019, 3, 10)
                min_value=date(2019, 2, 3)
                max_value=date(2019, 3, 20)
            />
            <TestCalendar
                name="buttons-focus"
                default_value=date(2019, 3, 10)
                min_value=date(2019, 2, 3)
                max_value=date(2019, 4, 20)
            />
            <TestRangeCalendar
                name="range-buttons-focus"
                default_value=DateRange {
                    start: date(2019, 3, 10),
                    end: date(2019, 3, 15),
                }
                min_value=date(2019, 2, 3)
                max_value=date(2019, 4, 20)
            />
            <TestCalendar
                name="week-limits"
                default_value=june5
                min_value=date(2019, 6, 2)
                max_value=date(2019, 6, 8)
            />
            <TestCalendar name="max-date" default_focused_value=date(9999, 12, 12) />
            <TestCalendar name="bc" default_value=date(-4, 2, 3) />
            <TestRangeCalendar
                name="range-bc"
                default_value=DateRange {
                    start: date(0, 12, 14),
                    end: date(1, 1, 22),
                }
            />
            <TestRangeCalendar
                name="range-bc-days"
                default_value=DateRange {
                    start: date(-4, 2, 3),
                    end: date(-4, 2, 18),
                }
            />
            <TestCalendar
                name="intervals"
                default_value=date(2021, 12, 15)
                unavailable=DECEMBER_2021
            />
            <TestRangeCalendar
                name="range-intervals"
                default_value=DateRange {
                    start: date(2021, 12, 15),
                    end: date(2021, 12, 15),
                }
                unavailable=DECEMBER_2021
            />
            <TestRangeCalendar
                name="range-previous-unavailable"
                default_value=DateRange {
                    start: date(2022, 5, 10),
                    end: date(2022, 5, 12),
                }
                unavailable=APRIL_25_TO_30_2022
            />
            <TestRangeCalendar
                name="range-next-unavailable"
                default_value=DateRange {
                    start: date(2022, 4, 10),
                    end: date(2022, 4, 12),
                }
                unavailable=MAY_1_TO_4_2022
            />
            <TestRangeCalendar
                name="range-navigation-unavailable"
                default_value=DateRange {
                    start: date(2022, 4, 10),
                    end: date(2022, 4, 12),
                }
                unavailable=MAY_2_TO_4_2022
            />
            <TestRangeCalendar
                name="range-non-contiguous"
                default_value=DateRange {
                    start: date(2021, 12, 15),
                    end: date(2021, 12, 15),
                }
                unavailable_weekends=true
                allows_non_contiguous_ranges=true
            />
            <TestRangeCalendar
                name="range-blur-nearest"
                default_value=DateRange {
                    start: date(2022, 3, 1),
                    end: date(2022, 3, 5),
                }
                unavailable_weekends=true
                allows_non_contiguous_ranges=true
            />
            <TestCalendar
                name="invalid-unavailable"
                default_value=date(2022, 3, 5)
                unavailable_weekends=true
            />
            <TestRangeCalendar
                name="range-invalid"
                default_value=DateRange {
                    start: date(2022, 3, 10),
                    end: date(2022, 3, 12),
                }
                is_invalid=true
            />
            <TestRangeCalendar
                name="range-valid"
                default_value=DateRange {
                    start: date(2022, 3, 10),
                    end: date(2022, 3, 12),
                }
            />
            <TestRangeCalendar name="range-mid" default_value=june_10_20 />
            <TestRangeCalendar name="range-read-only" default_value=june_10_20 is_read_only=true />
            <TestRangeCalendar name="range-disabled" default_focused_value=june5 is_disabled=true />
            <TestRangeCalendar name="range-invalid-drag" default_value=june_10_20 is_invalid=true />
            <TestRangeCalendar
                name="range-min-max"
                default_value=DateRange {
                    start: date(2019, 2, 8),
                    end: date(2019, 2, 15),
                }
                min_value=date(2019, 2, 5)
                max_value=date(2019, 2, 15)
            />
            <TestRangeCalendar name="range-controlled" default_value=june_5_10 controlled=true />
            <TestRangeCalendar
                name="range-auto"
                default_value=DateRange {
                    start: date(2019, 2, 3),
                    end: date(2019, 2, 18),
                }
                auto_focus=true
            />
            <TestRangeCalendar name="range-today" />
            <TestRangeCalendar name="range-today-read-only" is_read_only=true auto_focus=true />
            <TestRangeCalendar
                name="range-across-months"
                default_value=DateRange {
                    start: date(2019, 6, 20),
                    end: date(2019, 7, 10),
                }
            />
            <TestRangeCalendar
                name="commit-clear-empty"
                default_focused_value=date(2025, 11, 1)
                commit_behavior=CommitBehavior::Clear
            />
            <TestCalendar name="controlled" default_value=june5 controlled=true auto_focus=true />
            <TestCalendar
                name="controlled-read-only"
                default_value=june5
                controlled=true
                auto_focus=true
                is_read_only=true
            />
            <TestCalendar name="focus-default" default_focused_value=june5 auto_focus=true />
            <TestCalendar name="focus-controlled" focused_value=june5 auto_focus=true />
            <TestCalendar
                name="focus-constrained"
                default_focused_value=june5
                min_value=date(2019, 7, 5)
                auto_focus=true
            />
            <TestCalendar
                name="focus-controlled-constrained"
                focused_value=june5
                min_value=date(2019, 7, 5)
                auto_focus=true
            />
            <TestCalendar name="auto-today" auto_focus=true />
            <TestCalendar name="auto-selected" default_value=date(2019, 2, 3) auto_focus=true />
            <TestCalendar
                name="saturday"
                default_value=date(2024, 1, 1)
                first_day_of_week=Weekday::Saturday
            />
            <TestCalendar
                name="thursday"
                default_value=date(2025, 1, 1)
                first_day_of_week=Weekday::Thursday
            />
            <I18nProvider locale=fr()>
                <TestCalendar name="fr" default_value=date(2024, 1, 1) />
                <TestCalendar
                    name="fr-sunday"
                    default_value=date(2024, 1, 1)
                    first_day_of_week=Weekday::Sunday
                />
                <TestCalendar
                    name="fr-saturday"
                    default_value=date(2024, 1, 1)
                    first_day_of_week=Weekday::Saturday
                />
            </I18nProvider>
            <I18nProvider locale=de()>
                <TestCalendar name="de" default_value=june5 />
            </I18nProvider>
            <TestCalendar
                name="weekday-short"
                default_value=june5
                weekday_style=DateTimeFormat::Short
            />
            <TestCalendar name="unlabelled" default_value=june5 unlabelled=true />
            <TestCalendar
                name="labelledby-only"
                default_value=june5
                labelled_only_by_heading=true
            />
            <TestCalendar name="custom-id" default_value=june5 labelled=true id="hi" />
            <TestCalendar name="described" default_value=june5 described=true />
            <TestCalendar name="reset" default_value=date(2020, 3, 3) reset_button=true />
            <TestCalendar
                name="weeks-visible"
                default_value=date(2019, 1, 1)
                visible_duration=DateDuration::weeks(3)
            />
            <TestCalendar
                name="days-visible"
                default_value=date(2019, 1, 1)
                visible_duration=DateDuration::days(5)
            />
            {[
                ("commit-select", CommitBehavior::Select),
                ("commit-clear", CommitBehavior::Clear),
                ("commit-reset", CommitBehavior::Reset),
            ]
                .into_iter()
                .map(|(name, commit_behavior)| {
                    view! {
                        <TestRangeCalendar
                            name=name
                            default_value=DateRange {
                                start: date(2025, 11, 13),
                                end: date(2025, 11, 15),
                            }
                            commit_behavior=commit_behavior
                        />
                    }
                })
                .collect_view()}
        </div>
    }
}
