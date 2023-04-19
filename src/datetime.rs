use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Year {
    pub number: i32,
    pub is_now: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Month {
    /// base 1
    pub index: u8,
    pub name: String,
    pub is_now: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Week {
    pub id: Uuid,
    pub days: Vec<Day>, // Not always full?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InMonth {
    Previous,
    Current,
    Next,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Day {
    pub id: Uuid,
    pub index: u8,
    pub display_name: String,
    pub in_month: InMonth,
    pub date_time: time::OffsetDateTime,
    pub disabled: bool,
    pub highlighted: bool,
    pub selected: bool,
    pub is_now: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuideMode {
    CalendarFirst,
    YearFirst,
}

impl Default for GuideMode {
    fn default() -> Self {
        Self::CalendarFirst
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Date,
    Time,
    DateTime,
}

impl Default for Type {
    fn default() -> Self {
        Self::DateTime
    }
}

pub trait SaveReplaceYear
where
    Self: Sized,
{
    type Error;

    /// Replacing the year might fail if this date represents Feb 29 and the new year is normal!
    /// In that case, we fall back to the 28th of February.
    fn save_replace_year(self, year: i32) -> Result<Self, Self::Error>;

    /// Replacing the month might fail if this date represents March 31 and the new month is April, which only has 30 days!
    /// In that case, we fall back to the last day of the requested month.
    fn save_replace_month(self, month: time::Month) -> Result<Self, Self::Error>;
}

impl SaveReplaceYear for time::OffsetDateTime {
    type Error = time::error::ComponentRange;

    fn save_replace_year(self, year: i32) -> Result<Self, Self::Error> {
        self.replace_year(year).or_else(|_| {
            self.replace_day(self.day() - 1)
                .and_then(|it| it.replace_year(year))
        })
    }

    fn save_replace_month(self, month: time::Month) -> Result<Self, Self::Error> {
        self.replace_month(month).or_else(|_| {
            let max_available_days = whole_days_in(self.year(), month);
            self.replace_day(max_available_days)
                .and_then(|it| it.replace_month(month))
        })
    }
}

pub fn whole_days_in(year: i32, month: time::Month) -> u8 {
    let duration = time::Date::from_calendar_date(
        match month {
            time::Month::December => year + 1,
            _ => year,
        },
        month.next(),
        1,
    )
    .unwrap()
        - time::Date::from_calendar_date(year, month, 1).unwrap();
    duration.whole_days() as u8
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::SaveReplaceYear;

    #[test]
    fn save_replace_year_replaces_when_coming_from_feb_29() {
        let dt = datetime!(2000-02-29 0:00 UTC);
        let result = dt.save_replace_year(1999).unwrap();
        assert_eq!(result, datetime!(1999-02-28 0:00 UTC))
    }

    #[test]
    fn save_replace_month_replaces_when_coming_from_day_out_of_targeted_months_range() {
        let dt = datetime!(2023-03-31 0:00 UTC);
        let result = dt.save_replace_month(time::Month::February).unwrap();
        assert_eq!(result, datetime!(2023-02-28 0:00 UTC))
    }
}
