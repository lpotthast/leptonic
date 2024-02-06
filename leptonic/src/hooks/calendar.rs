use leptos::*;
use time::macros::format_description;
use uuid::Uuid;

use crate::utils::time::{
    is_in_range, start_of_next_month, start_of_previous_month, whole_days_in, Day, InMonth, Month,
    SaveReplaceYear, Week, Year,
};

#[derive(Debug, Clone, Copy)]
pub struct UseCalendarReturn {
    pub staging: ReadSignal<time::OffsetDateTime>,
    pub staging_year: Memo<i32>,
    pub staging_month_name: Memo<String>,
    set_staging: WriteSignal<time::OffsetDateTime>,

    pub selected: Memo<time::OffsetDateTime>,

    years_start: RwSignal<i32>,
    pub years_range: Signal<String>, // This could be an i32 tuple!
    pub years: Signal<Vec<Year>>,
    pub months: Signal<Vec<Month>>,
    pub weeks: Signal<Vec<Week>>,
}

impl UseCalendarReturn {
    pub fn select_previous_month(&self) {
        self.set_staging
            .update(move |staging| *staging = start_of_previous_month(*staging))
    }

    pub fn select_next_month(&self) {
        self.set_staging
            .update(move |staging| *staging = start_of_next_month(*staging));
    }

    pub fn select_previous_year(&self) {
        let current = self.staging.get();
        let new_year = current.year() - 1;
        self.set_staging
            .set(current.save_replace_year(new_year).expect("always safe"));
        self.years_start.set(new_year - 5);
    }

    pub fn select_next_year(&self) {
        let current = self.staging.get();
        let new_year = current.year() + 1;
        self.set_staging
            .set(current.save_replace_year(new_year).expect("always safe"));
        self.years_start.set(new_year - 5);
    }

    pub fn select_previous_years(&self) {
        self.years.with(|years| {
            self.years_start.update(|starting| {
                *starting = match years.len() {
                    0 => self.staging.get_untracked().year() - 5,
                    _ => years[0].number - 12,
                }
            });
        });
    }

    pub fn select_next_years(&self) {
        self.years.with(|years| {
            self.years_start.update(|starting| {
                *starting = match years.len() {
                    0 => self.staging.get_untracked().year() + 1,
                    _ => years[years.len() - 1].number + 1,
                }
            });
        });
    }

    pub fn select_year(&self, year: Year) {
        self.set_staging
            .update(|staging| *staging = staging.save_replace_year(year.number).unwrap());
    }

    pub fn select_month(&self, month: Month) {
        self.set_staging.update(|staging| {
            *staging = staging
                .save_replace_month(time::Month::try_from(month.index).unwrap())
                .unwrap();
        });
    }

    pub fn select_day(&self, day: Day) {
        self.set_staging.update(|staging| *staging = day.date_time);
    }
}

pub fn use_calendar(
    initial_value: time::OffsetDateTime,
    min: Option<time::OffsetDateTime>,
    max: Option<time::OffsetDateTime>,
) -> UseCalendarReturn {
    let (staging, set_staging) = create_signal(initial_value);

    let staging_year = create_memo(move |_| staging.get().year());
    let staging_month_name = create_memo(move |_| staging.get().month().to_string());

    let selected: Memo<time::OffsetDateTime> = create_memo(move |_| staging.get());

    let years_start = create_rw_signal(staging.get_untracked().year() - 4);
    let years = Signal::derive(move || {
        staging
            .with(|staging| create_years(*staging, years_start.get(), min.as_ref(), max.as_ref()))
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
    let weeks = Signal::derive(move || create_weeks(&staging.get(), min.as_ref(), max.as_ref()));

    UseCalendarReturn {
        staging,
        staging_year,
        staging_month_name,
        set_staging,
        selected,
        years_start,
        years_range,
        years,
        months,
        weeks,
    }
}

pub fn create_years(
    staging: time::OffsetDateTime,
    starting_year: i32,
    min: Option<&time::OffsetDateTime>,
    max: Option<&time::OffsetDateTime>,
) -> Vec<Year> {
    let amount = 3 * 4; // 4 rows of 3 year numbers each.
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
    const WEEKS_TO_DISPLAY: u8 = 6;
    const DAYS_PER_WEEK: u8 = 7;

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

    let mut weeks = Vec::<Week>::with_capacity(WEEKS_TO_DISPLAY as usize);
    for w in 0..WEEKS_TO_DISPLAY {
        let mut week = Week {
            id: Uuid::new_v4(),
            days: Vec::with_capacity(DAYS_PER_WEEK as usize),
        };
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
