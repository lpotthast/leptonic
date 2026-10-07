use leptonic::{
    atoms::calendar::{
        Calendar, CalendarCell, CalendarCellButton, CalendarErrorMessage, CalendarGrid,
        CalendarGridBody, CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow,
        CalendarHeading, CalendarNextButton, CalendarPreviousButton, CalendarWeek, RangeCalendar,
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
                nostrip:first_day_of_week=first_day_of_week
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
) -> impl IntoView {
    let value = RwSignal::new(default_value);
    let is_date_unavailable = if unavailable_weekends {
        Some(Callback::new(|(date, _): (Date, Option<Date>)| {
            matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday)
        }))
    } else if max_week {
        Some(Callback::new(|(date, anchor): (Date, Option<Date>)| {
            anchor.is_some_and(|anchor| (date - anchor).get_days().abs() > 7)
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

/// Calendars (June 2019, as react-aria's tests) in different configurations: basic, min/max,
/// unavailable weekends, disabled, read-only, invalid, two months, a week, three days, a range
/// calendar and one with unavailable weekends.
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
        </div>
    }
}
