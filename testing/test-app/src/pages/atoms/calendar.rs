use leptonic::{
    atoms::calendar::{
        Calendar, CalendarCell, CalendarCellButton, CalendarErrorMessage, CalendarGrid,
        CalendarGridBody, CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow,
        CalendarHeading, CalendarMonthPicker, CalendarNextButton, CalendarPreviousButton,
        CalendarWeek, CalendarYearPicker, RangeCalendar,
    },
    hooks::calendar::{
        CommitBehavior, DateAvailabilityQuery, PageBehavior, UseCalendarPickerReturn,
    },
    jiff::civil::{Date, Weekday, date},
    utils::{
        date::{DateDuration, DateRange},
        i18n::{I18nProvider, Locale},
    },
};
use leptos::prelude::*;

/// The parts inside a calendar: heading, buttons and `months` grids.
#[component]
fn Parts(#[prop(default = 1)] months: i32) -> impl IntoView {
    view! {
        <header>
            <CalendarPreviousButton>"<"</CalendarPreviousButton>
            <CalendarHeading />
            <CalendarNextButton>">"</CalendarNextButton>
        </header>
        {(0..months)
            .map(|month| {
                view! {
                    <CalendarGrid offset=DateDuration::months(month)>
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

/// A calendar `name` (its label) showing its value in `#test-calendar-{name}-value`, with a
/// button before it.
#[component]
fn TestCalendar(
    name: &'static str,
    #[prop(optional)] default_value: Option<Date>,
    #[prop(optional)] min_value: Option<Date>,
    #[prop(optional)] max_value: Option<Date>,
    #[prop(optional)] unavailable_weekends: bool,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_read_only: bool,
    #[prop(optional)] is_invalid: bool,
    #[prop(optional)] visible_duration: Option<DateDuration>,
    #[prop(optional)] first_day_of_week: Option<Weekday>,
    #[prop(optional)] page_behavior: PageBehavior,
    #[prop(optional)] default_focused_value: Option<Date>,
    #[prop(optional)] weeks_in_month: Option<u8>,
    #[prop(default = 1)] months: i32,
    /// Also labelled by a heading (`#test-calendar-{name}-label`).
    #[prop(optional)]
    labelled: bool,
) -> impl IntoView {
    let value = RwSignal::new(default_value);
    let label_id = format!("test-calendar-{name}-label");
    let aria_labelledby = labelled.then(|| label_id.clone());
    let is_date_unavailable = unavailable_weekends.then(|| {
        Callback::new(|date: Date| matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday))
    });
    view! {
        <section id=format!("test-calendar-{name}")>
            <button id=format!("test-calendar-{name}-before")>"Before"</button>
            {labelled.then(|| view! { <h3 id=label_id>"Booking"</h3> })}
            <Calendar
                aria_label=name
                nostrip:aria_labelledby=aria_labelledby
                value=value
                set_value=value
                min_value=min_value
                max_value=max_value
                nostrip:is_date_unavailable=is_date_unavailable
                is_disabled=is_disabled
                is_read_only=is_read_only
                is_invalid=is_invalid
                visible_duration=visible_duration.unwrap_or(DateDuration::months(months))
                first_day_of_week=first_day_of_week
                page_behavior=page_behavior
                nostrip:default_focused_value=default_focused_value
                weeks_in_month=weeks_in_month
            >
                <Parts months=if visible_duration.is_some() { 1 } else { months } />
            </Calendar>
            <div>
                "Value: " <span id=format!("test-calendar-{name}-value")>{move || format(value.get())}</span>
            </div>
        </section>
    }
}

/// A range calendar `name` showing its value in `#test-calendar-{name}-value`.
#[component]
fn TestRangeCalendar(
    name: &'static str,
    #[prop(optional)] default_value: Option<DateRange>,
    #[prop(optional)] default_focused_value: Option<Date>,
    #[prop(optional)] unavailable_weekends: bool,
    /// Dates more than a week from the range's anchor are unavailable.
    #[prop(optional)]
    max_week: bool,
    #[prop(optional)] allows_non_contiguous_ranges: bool,
    #[prop(optional)] commit_behavior: CommitBehavior,
) -> impl IntoView {
    let value = RwSignal::new(default_value);
    let is_date_unavailable = if unavailable_weekends {
        Some(Callback::new(|query: DateAvailabilityQuery| {
            matches!(query.date.weekday(), Weekday::Saturday | Weekday::Sunday)
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
    view! {
        <section id=format!("test-calendar-{name}")>
            <button id=format!("test-calendar-{name}-before")>"Before"</button>
            <RangeCalendar
                aria_label=name
                value=value
                set_value=value
                nostrip:is_date_unavailable=is_date_unavailable
                allows_non_contiguous_ranges=allows_non_contiguous_ranges
                nostrip:default_focused_value=default_focused_value
                commit_behavior=commit_behavior
            >
                <Parts />
            </RangeCalendar>
            <div>
                "Value: "
                <span id=format!("test-calendar-{name}-value")>
                    {move || format_range(value.get())}
                </span>
            </div>
            <button id=format!("test-calendar-{name}-after")>"After"</button>
            <button id=format!("test-calendar-{name}-clear") on:click=move |_| value.set(None)>
                "Clear"
            </button>
        </section>
    }
}

/// A calendar with its focused date as app state, moved by `#test-calendar-focus-set` (to June 20).
#[component]
fn FocusCalendar() -> impl IntoView {
    let focused = RwSignal::new(date(2019, 6, 5));
    view! {
        <section id="test-calendar-focus">
            <button id="test-calendar-focus-set" on:click=move |_| focused.set(date(2019, 6, 20))>
                "Focus June 20"
            </button>
            <Calendar aria_label="focus" focused_value=focused set_focused_value=focused>
                <Parts />
            </Calendar>
        </section>
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
    }
}

/// Calendars (June 2019, as react-aria's tests) in different configurations: basic, min/max,
/// unavailable weekends, disabled, read-only, invalid, two months, a week, three days, a range
/// calendar and one with unavailable weekends, a range calendar for touch input, one without a
/// value (showing today), paging by single units (two months, three weeks, five days, from January
/// 1, 2019), two weeks, six week rows, month and year pickers, a changing visible duration, and
/// range calendars with each commit behavior (November 13 to 15, 2025).
#[component]
pub fn PageAtomCalendar() -> impl IntoView {
    let june5 = date(2019, 6, 5);
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
            <TestCalendar
                name="week"
                default_value=june5
                visible_duration=DateDuration::weeks(1)
            />
            <TestCalendar name="days" default_value=june5 visible_duration=DateDuration::days(3) />
            <TestCalendar name="monday" default_value=june5 first_day_of_week=Weekday::Monday />
            <TestCalendar name="labelled" default_value=june5 labelled=true />
            <I18nProvider locale={"he".parse::<Locale>().expect("a locale")}>
                <TestCalendar name="rtl" default_value=june5 />
            </I18nProvider>
            <FocusCalendar />
            <TestRangeCalendar
                name="range"
                default_value=DateRange {
                    start: june5,
                    end: date(2019, 6, 10),
                }
            />
            <TestRangeCalendar name="range-week" default_focused_value=june5 max_week=true />
            <TestRangeCalendar
                name="range-unavailable"
                default_focused_value=june5
                unavailable_weekends=true
            />
            <TestRangeCalendar
                name="range-touch"
                default_value=DateRange {
                    start: june5,
                    end: date(2019, 6, 10),
                }
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
            <TestCalendar name="held" default_focused_value=date(2020, 3, 3) />
            <PickersCalendar />
            <DurationCalendar />
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
