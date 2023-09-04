use leptos::*;
use time::macros::format_description;
use uuid::Uuid;

use crate::{
    datetime::{
        is_in_range, start_of_next_month, start_of_previous_month, whole_days_in, Day, GuideMode,
        InMonth, Month, SaveReplaceYear, Week, Year,
    },
    prelude::{Callable, Callback},
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Selection {
    Year,
    Month,
    Day,
}

#[component]
pub fn DateSelector(
    value: time::OffsetDateTime,
    on_change: Callback<time::OffsetDateTime>,
    #[prop(optional)] min: Option<time::OffsetDateTime>,
    #[prop(optional)] max: Option<time::OffsetDateTime>,
    #[prop(into, optional, default = GuideMode::CalendarFirst.into())] guide_mode: MaybeSignal<
        GuideMode,
    >,
) -> impl IntoView {
    let (staging, set_staging) = create_signal(value);

    let staging_year = Signal::derive(move || staging.get().year());
    let staging_month = Signal::derive(move || staging.get().month().to_string());

    let selected: Memo<time::OffsetDateTime> = create_memo(move |_| staging.get());
    create_effect(move |_| on_change.call(selected.get()));

    let (show, set_show) = create_signal(match guide_mode.get() {
        GuideMode::CalendarFirst => Selection::Day,
        GuideMode::YearFirst => Selection::Year,
    });

    let (years_starting_at, set_years_starting_at) = create_signal(None);
    let years = Signal::derive(move || {
        staging.with(|staging| {
            create_years(
                *staging,
                years_starting_at.get(),
                min.as_ref(),
                max.as_ref(),
            )
        })
    });
    let years_range = Signal::derive(move || {
        years.with(|years| {
            if years.is_empty() {
                "ERR: no years".to_owned()
            } else {
                format!("{} - {}", years[0].number, years[years.len() - 1].number)
            }
        })
    });
    let months = Signal::derive(move || {
        staging.with(|staging| create_months(*staging, min.as_ref(), max.as_ref()))
    });
    // TODO: Make static. Make i18n.
    let (short_weekday_names, _) = create_signal(create_week_day_names());
    let weeks = Signal::derive(move || create_weeks(&staging.get(), min.as_ref(), max.as_ref()));

    let select_previous_month =
        move |_| set_staging.update(|staging| *staging = start_of_previous_month(*staging));
    let select_next_month =
        move |_| set_staging.update(|staging| *staging = start_of_next_month(*staging));
    let select_previous_year = move |_| {
        set_staging
            .update(|staging| *staging = staging.save_replace_year(staging.year() - 1).unwrap())
        // TODO: Set set_years_starting_at to Some(self.staging.year() - 4),
    };
    let select_next_year = move |_| {
        set_staging
            .update(|staging| *staging = staging.save_replace_year(staging.year() + 1).unwrap())
        // TODO: Set set_years_starting_at to Some(self.staging.year() - 4),
    };
    let select_previous_years = move |_| {
        years.with(|years| {
            set_years_starting_at.update(|starting| {
                *starting = match years.len() {
                    0 => None,
                    _ => Some(years[0].number - (3 * 7)),
                }
            })
        })
    };
    let select_next_years = move |_| {
        years.with(|years| {
            set_years_starting_at.update(|starting| {
                *starting = match years.len() {
                    0 => None,
                    _ => Some(years[years.len() - 1].number + 1),
                }
            })
        })
    };
    let select_year = move |year: Year| {
        if !year.disabled {
            set_staging
                .update(|staging| *staging = staging.save_replace_year(year.number).unwrap());
            set_show.update(|show| *show = Selection::Month);
        }
    };
    let select_month = move |month: Month| {
        if !month.disabled {
            set_staging.update(|staging| {
                *staging = staging
                    .save_replace_month(time::Month::try_from(month.index).unwrap())
                    .unwrap()
            });
            set_show.update(|show| *show = Selection::Day);
        }
    };
    let select_day = move |day: Day| {
        if !day.disabled {
            set_staging.update(|staging| *staging = day.date_time);
        }
    };

    view! {
        <leptonic-datetime>
        <leptonic-date-selector>
            <leptonic-calender-month>
                <div class={"actions"}>
                    {move || match show.get() {
                        Selection::Year => view! {
                            <div on:click=select_previous_years
                                class="previous arrow-left">
                            </div>
                            <div on:click=move |_| set_show.update(|show| *show = Selection::Month)
                                class="current-date">
                                {years_range}
                            </div>
                            <div on:click=select_next_years
                                class="next arrow-right">
                            </div>
                        },
                        Selection::Month => view! {
                            <div on:click=select_previous_year
                                class="previous arrow-left">
                            </div>
                            <div on:click=move |_| set_show.update(|show| *show = Selection::Year)
                                class="current-date">
                                {staging_year}
                            </div>
                            <div on:click=select_next_year
                                class="next arrow-right">
                            </div>
                        },
                        Selection::Day => view! {
                            <div on:click=select_previous_month
                                class="previous arrow-left">
                            </div>
                            <div on:click=move |_| set_show.update(|show| *show = Selection::Year)
                                class="current-date">
                                {staging_month} " " {staging_year}
                            </div>
                            <div on:click=select_next_month
                                class="next arrow-right">
                            </div>
                        },
                    }}
                </div>

                <Show when=move || show.get() == Selection::Year fallback=|| ()>
                    <div class="years">
                        <For
                            each=move || years.get()
                            key=|year| year.number
                            view=move |year| {
                                view! {
                                    <div on:click=move |_e| select_year(year)
                                        class="year"
                                        class:is-staging=year.is_staging
                                        class:is-now=year.is_now
                                        class:disabled=year.disabled
                                    >
                                        {year.number}
                                    </div>
                                }
                            }
                        />
                    </div>
                </Show>

                <Show when=move || show.get() == Selection::Month fallback=|| ()>
                    <div class="months">
                        <For
                            each=move || months.get()
                            key=|month| month.index
                            view=move |month| {
                                let mon = month.clone();
                                view! {
                                    <div on:click=move |_e| select_month(mon.clone())
                                        class="month"
                                        class:is-staging=month.is_staging
                                        class:is-now=month.is_now
                                        class:disabled=month.disabled>
                                        {month.name.clone()}
                                    </div>
                                }
                            }
                        />
                    </div>
                </Show>

                <Show when=move || show.get() == Selection::Day fallback=|| ()>
                    <div class={"weekday-names"}>
                        // Not use For for this...?
                        <For
                            each=move || short_weekday_names.get()
                            key=|short_weekday_name| short_weekday_name.clone()
                            view=move |short_weekday_name| {
                                view! {
                                    <div class={"weekday-name"}>
                                        {short_weekday_name}
                                    </div>
                                }
                            }
                        />
                    </div>

                    <div class={"weeks"}>
                        <For
                            each=move || weeks.get()
                            key=|week| week.id
                            view=move |week| {
                                view! {
                                    <div class="week">
                                        <For
                                            each=move || week.days.clone()
                                            key=|day| day.id
                                            view=move |day| {
                                                let d = day.clone();
                                                view! {
                                                    <div
                                                        on:click=move |_e| select_day(d.clone())
                                                        class="day"
                                                        class:is-staging=day.is_staging
                                                        class:is-now=day.is_now
                                                        class:not-in-month=day.in_month != InMonth::Current
                                                        class:disabled=day.disabled
                                                    >
                                                        <span class="text">
                                                            {day.index}
                                                        </span>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
                                }
                            }
                        />
                    </div>
                </Show>

            </leptonic-calender-month>
        </leptonic-date-selector>
        </leptonic-datetime>
    }
}

pub fn create_week_day_names() -> Vec<String> {
    //let day_in_month = value.date().day(); // 1 based
    //let days_from_monday = value.date().weekday().num_days_from_monday(); // 0 based
    //let monday = value.date().with_day(day_in_month - days_from_monday).format("%a").to_string();

    vec![
        "Mon".to_owned(),
        "Tue".to_owned(),
        "Wed".to_owned(),
        "Thr".to_owned(),
        "Fri".to_owned(),
        "Sat".to_owned(),
        "Sun".to_owned(),
    ]
}

pub fn create_years(
    staging: time::OffsetDateTime,
    starting_year: Option<i32>,
    min: Option<&time::OffsetDateTime>,
    max: Option<&time::OffsetDateTime>,
) -> Vec<Year> {
    let amount = 3 * 5; // 5 rows of 3 year numbers each.
    let starting_year = starting_year.unwrap_or_else(|| staging.year() - 4);
    let mut years = Vec::<Year>::with_capacity(amount);
    let now = time::OffsetDateTime::now_utc();
    let this_year = now.year();
    let staging_year = staging.year();
    let min_year = min.map(|it| it.year()).unwrap_or(i32::MIN);
    let max_year = max.map(|it| it.year()).unwrap_or(i32::MAX);

    for i in 0..amount {
        let year_number = starting_year + i as i32;
        years.push(Year {
            number: year_number,
            is_staging: year_number == staging_year,
            is_now: year_number == this_year,
            disabled: year_number < min_year || year_number > max_year,
        });
    }
    years
}

pub fn create_months(
    staging: time::OffsetDateTime,
    min: Option<&time::OffsetDateTime>,
    max: Option<&time::OffsetDateTime>,
) -> Vec<Month> {
    let now = time::OffsetDateTime::now_utc();
    let this_year = now.year();
    let this_month = now.month();
    let staging_year = staging.year();
    let staging_month = staging.month();
    let mut months = Vec::<Month>::with_capacity(12);
    for i in 1..=12u8 {
        let month: time::OffsetDateTime = staging
            .save_replace_month(time::Month::try_from(i).unwrap())
            .unwrap();
        let month_year = month.year();
        let month_month = month.month();
        months.push(Month {
            index: i,
            name: month.format(format_description!("[month]")).unwrap(), // TODO: format_description does not work!
            is_staging: staging_year == month_year && staging_month == month_month,
            is_now: this_year == month_year && this_month == month_month,
            disabled: !is_in_range(&month, max, min),
        });
    }
    assert_eq!(months.len(), 12);
    months
}

pub fn create_weeks(
    staging: &time::OffsetDateTime,
    min: Option<&time::OffsetDateTime>,
    max: Option<&time::OffsetDateTime>,
) -> Vec<Week> {
    let now = time::OffsetDateTime::now_utc();
    // Calculate the index of the first day of the month (in current locale).

    let current_year = now.year();
    let current_month = now.month();
    let current_day = now.day();
    let staging_day = staging.day();

    let first_weekday_index = (*staging)
        .replace_day(1)
        .unwrap()
        .weekday()
        .number_days_from_monday(); // in range [0..6]
    let number_of_days_in_month = whole_days_in(staging.year(), staging.month());
    let index_of_last_day_in_month = first_weekday_index + number_of_days_in_month;

    let prev_month = start_of_previous_month(*staging);
    let this_month = staging;
    let next_month = start_of_next_month(*staging);

    let days_in_previous_month = whole_days_in(prev_month.year(), prev_month.month());

    const WEEKS_TO_DISPLAY: u8 = 6;
    let mut weeks = Vec::<Week>::with_capacity(WEEKS_TO_DISPLAY as usize);
    for w in 0..WEEKS_TO_DISPLAY {
        let mut week = Week {
            id: Uuid::new_v4(),
            days: Vec::with_capacity(7),
        };
        const DAYS_PER_WEEK: u8 = 7;
        for d in 0..DAYS_PER_WEEK {
            let i = d + w * DAYS_PER_WEEK;

            let in_month = if i < first_weekday_index {
                InMonth::Previous
            } else if i >= first_weekday_index && i < index_of_last_day_in_month {
                InMonth::Current
            } else {
                InMonth::Next
            };

            // base 1 (!)
            let day_in_month = match in_month {
                InMonth::Previous => days_in_previous_month - first_weekday_index + i + 1,
                InMonth::Current => i - first_weekday_index + 1,
                InMonth::Next => i - index_of_last_day_in_month + 1,
            };
            let relevant_month = match in_month {
                InMonth::Previous => &prev_month,
                InMonth::Current => this_month,
                InMonth::Next => &next_month,
            };
            let date_time: time::OffsetDateTime = relevant_month.replace_day(day_in_month).unwrap();
            let disabled = !is_in_range(&date_time, max, min); // TODO: inversion of min/max correct?
            let selected = in_month == InMonth::Current && day_in_month == staging_day; // TODO: Can a day form prev not be selected?

            week.days.push(Day {
                id: Uuid::new_v4(),
                index: day_in_month,
                in_month,
                date_time,
                disabled,
                highlighted: false,
                is_staging: selected,
                // TODO: is year check necessary?
                is_now: current_month == relevant_month.month()
                    && current_year == relevant_month.year()
                    && current_day == day_in_month,
            });
        }
        weeks.push(week);
    }
    weeks
}
