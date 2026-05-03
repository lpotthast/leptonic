use super::use_date_segment::DateSegmentType;
use crate::utils::time::whole_days_in;

// Page step constants matching react-aria behavior.
const PAGE_STEP_YEAR: i32 = 5;
const PAGE_STEP_MONTH: i32 = 2;
const PAGE_STEP_DAY: i32 = 7;
const PAGE_STEP_HOUR: i32 = 2;
const PAGE_STEP_MINUTE: i32 = 15;
const PAGE_STEP_SECOND: i32 = 15;

/// A partially-filled date/time value.
///
/// Each field is optional, enabling representation of partial edits (e.g., user
/// typed the month but not the day). This is the Rust equivalent of react-aria's
/// `DateFieldState` internal editing buffer.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncompleteDate {
    pub year: Option<i32>,
    /// 1-12
    pub month: Option<u8>,
    /// 1-31
    pub day: Option<u8>,
    /// In display hour cycle (0-23 for h24, 1-12 for h12)
    pub hour: Option<u8>,
    /// 0-59
    pub minute: Option<u8>,
    /// 0-59
    pub second: Option<u8>,
    /// `false` = AM, `true` = PM. Only meaningful for h12.
    pub day_period: Option<bool>,
    /// Whether we are in 24-hour mode.
    pub hour_cycle_24: bool,
}

impl IncompleteDate {
    /// Create an empty `IncompleteDate` with no fields set.
    pub fn empty(hour_cycle_24: bool) -> Self {
        Self {
            year: None,
            month: None,
            day: None,
            hour: None,
            minute: None,
            second: None,
            day_period: None,
            hour_cycle_24,
        }
    }

    /// Populate all fields from a complete date.
    pub fn from_date(date: &time::OffsetDateTime, hour_cycle_24: bool) -> Self {
        let (hour, day_period) = Self::from_24_hour(date.hour(), hour_cycle_24);
        Self {
            year: Some(date.year()),
            month: Some(date.month() as u8),
            day: Some(date.day()),
            hour: Some(hour),
            minute: Some(date.minute()),
            second: Some(date.second()),
            day_period,
            hour_cycle_24,
        }
    }

    /// Sync from an external complete date (when the controlled value changes).
    pub fn sync_from_date(&mut self, date: &time::OffsetDateTime) {
        let (hour, day_period) = Self::from_24_hour(date.hour(), self.hour_cycle_24);
        self.year = Some(date.year());
        self.month = Some(date.month() as u8);
        self.day = Some(date.day());
        self.hour = Some(hour);
        self.minute = Some(date.minute());
        self.second = Some(date.second());
        self.day_period = day_period;
    }

    /// Set a single field by segment type.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn set(&mut self, segment_type: DateSegmentType, value: i32) {
        match segment_type {
            DateSegmentType::Year => self.year = Some(value),
            DateSegmentType::Month => {
                self.month = Some(value.clamp(1, 12) as u8);
                self.clamp_day_to_month();
            }
            DateSegmentType::Day => self.day = Some(value.clamp(1, 31) as u8),
            DateSegmentType::Hour => {
                let (min, max) = self.hour_limits();
                self.hour = Some(value.clamp(min, max) as u8);
                // Auto-populate day_period from default if not set.
                if !self.hour_cycle_24 && self.day_period.is_none() {
                    self.day_period = Some(false); // default AM
                }
            }
            DateSegmentType::Minute => self.minute = Some(value.clamp(0, 59) as u8),
            DateSegmentType::Second => self.second = Some(value.clamp(0, 59) as u8),
            DateSegmentType::DayPeriod => self.day_period = Some(value != 0),
            DateSegmentType::Literal => {}
        }
    }

    /// Clear a field (e.g., on Backspace).
    pub fn clear(&mut self, segment_type: DateSegmentType) {
        match segment_type {
            DateSegmentType::Year => self.year = None,
            DateSegmentType::Month => self.month = None,
            DateSegmentType::Day => self.day = None,
            DateSegmentType::Hour => self.hour = None,
            DateSegmentType::Minute => self.minute = None,
            DateSegmentType::Second => self.second = None,
            DateSegmentType::DayPeriod => self.day_period = None,
            DateSegmentType::Literal => {}
        }
    }

    /// Increment or decrement a segment with wrapping.
    /// Initializes from `placeholder` if the field is currently `None`.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn cycle(
        &mut self,
        segment_type: DateSegmentType,
        amount: i32,
        placeholder: &time::OffsetDateTime,
    ) {
        match segment_type {
            DateSegmentType::Year => {
                let current = self.year.unwrap_or_else(|| placeholder.year());
                self.year = Some(current + amount);
            }
            DateSegmentType::Month => {
                let current = self.month.unwrap_or_else(|| placeholder.month() as u8);
                let new_val = wrap(i32::from(current) + amount, 1, 12);
                self.month = Some(new_val as u8);
                // Adjust day if it exceeds new month's max.
                self.clamp_day_to_month();
            }
            DateSegmentType::Day => {
                let max = self.day_max(placeholder);
                let current = self.day.unwrap_or_else(|| placeholder.day());
                let new_val = wrap(i32::from(current) + amount, 1, i32::from(max));
                self.day = Some(new_val as u8);
            }
            DateSegmentType::Hour => {
                let (min, max) = self.hour_limits();
                let current = self.hour.unwrap_or_else(|| {
                    let (h, dp) = Self::from_24_hour(placeholder.hour(), self.hour_cycle_24);
                    if !self.hour_cycle_24 && self.day_period.is_none() {
                        self.day_period = dp;
                    }
                    h
                });
                let new_val = wrap(i32::from(current) + amount, min, max);
                self.hour = Some(new_val as u8);
            }
            DateSegmentType::Minute => {
                let current = self.minute.unwrap_or_else(|| placeholder.minute());
                let new_val = wrap(i32::from(current) + amount, 0, 59);
                self.minute = Some(new_val as u8);
            }
            DateSegmentType::Second => {
                let current = self.second.unwrap_or_else(|| placeholder.second());
                let new_val = wrap(i32::from(current) + amount, 0, 59);
                self.second = Some(new_val as u8);
            }
            DateSegmentType::DayPeriod => {
                // Toggle AM/PM.
                let current = self.day_period.unwrap_or_else(|| placeholder.hour() >= 12);
                self.day_period = Some(!current);
            }
            DateSegmentType::Literal => {}
        }
    }

    /// Get the page step for a segment type.
    pub fn page_step(segment_type: DateSegmentType) -> i32 {
        match segment_type {
            DateSegmentType::Year => PAGE_STEP_YEAR,
            DateSegmentType::Month => PAGE_STEP_MONTH,
            DateSegmentType::Day => PAGE_STEP_DAY,
            DateSegmentType::Hour => PAGE_STEP_HOUR,
            DateSegmentType::Minute => PAGE_STEP_MINUTE,
            DateSegmentType::Second => PAGE_STEP_SECOND,
            DateSegmentType::DayPeriod | DateSegmentType::Literal => 1,
        }
    }

    /// Set a segment to its maximum value.
    pub fn set_to_max(
        &mut self,
        segment_type: DateSegmentType,
        placeholder: &time::OffsetDateTime,
    ) {
        let (_, max) = self.get_segment_limits(segment_type, placeholder);
        self.set(segment_type, max);
    }

    /// Set a segment to its minimum value.
    pub fn set_to_min(
        &mut self,
        segment_type: DateSegmentType,
        placeholder: &time::OffsetDateTime,
    ) {
        let (min, _) = self.get_segment_limits(segment_type, placeholder);
        self.set(segment_type, min);
    }

    /// Whether all date fields (and time fields, if `show_time`) are filled.
    pub fn is_complete(&self, show_time: bool) -> bool {
        let date_complete = self.year.is_some() && self.month.is_some() && self.day.is_some();
        if !show_time {
            return date_complete;
        }
        let time_complete = self.hour.is_some() && self.minute.is_some();
        let period_complete = self.hour_cycle_24 || self.day_period.is_some();
        date_complete && time_complete && period_complete
    }

    /// Whether all fields are `None`.
    pub fn is_cleared(&self, show_time: bool) -> bool {
        let date_cleared = self.year.is_none() && self.month.is_none() && self.day.is_none();
        if !show_time {
            return date_cleared;
        }
        let time_cleared = self.hour.is_none() && self.minute.is_none() && self.second.is_none();
        let period_cleared = self.hour_cycle_24 || self.day_period.is_none();
        date_cleared && time_cleared && period_cleared
    }

    /// Convert to a complete `OffsetDateTime`, using `placeholder` for any missing fields.
    /// Returns `None` if the resulting date is invalid (e.g., Feb 30).
    pub fn to_date(&self, placeholder: &time::OffsetDateTime) -> Option<time::OffsetDateTime> {
        let year = self.year.unwrap_or_else(|| placeholder.year());
        let month_num = self.month.unwrap_or_else(|| placeholder.month() as u8);
        let month = month_from_u8(month_num)?;
        let day = self.day.unwrap_or_else(|| placeholder.day());
        let hour_24 = self.to_24_hour(placeholder);
        let minute = self.minute.unwrap_or_else(|| placeholder.minute());
        let second = self.second.unwrap_or_else(|| placeholder.second());

        // Clamp day to the valid range for the given year/month.
        let max_day = whole_days_in(year, month);
        let day = day.min(max_day);

        let date = time::Date::from_calendar_date(year, month, day).ok()?;
        let time = time::Time::from_hms(hour_24, minute, second).ok()?;
        Some(date.with_time(time).assume_utc())
    }

    /// Get the value of a specific segment field.
    pub fn get_field(&self, segment_type: DateSegmentType) -> Option<i32> {
        match segment_type {
            DateSegmentType::Year => self.year,
            DateSegmentType::Month => self.month.map(i32::from),
            DateSegmentType::Day => self.day.map(i32::from),
            DateSegmentType::Hour => self.hour.map(i32::from),
            DateSegmentType::Minute => self.minute.map(i32::from),
            DateSegmentType::Second => self.second.map(i32::from),
            DateSegmentType::DayPeriod => self.day_period.map(i32::from),
            DateSegmentType::Literal => None,
        }
    }

    /// Get (min, max) for a segment type.
    pub fn get_segment_limits(
        &self,
        segment_type: DateSegmentType,
        placeholder: &time::OffsetDateTime,
    ) -> (i32, i32) {
        match segment_type {
            DateSegmentType::Year => (1, 9999),
            DateSegmentType::Month => (1, 12),
            DateSegmentType::Day => (1, i32::from(self.day_max(placeholder))),
            DateSegmentType::Hour => self.hour_limits(),
            DateSegmentType::Minute | DateSegmentType::Second => (0, 59),
            DateSegmentType::DayPeriod => (0, 1),
            DateSegmentType::Literal => (0, 0),
        }
    }

    /// Convert display hour + `day_period` to 24-hour value.
    fn to_24_hour(&self, placeholder: &time::OffsetDateTime) -> u8 {
        let display_hour = self.hour.unwrap_or_else(|| {
            let (h, _) = Self::from_24_hour(placeholder.hour(), self.hour_cycle_24);
            h
        });
        if self.hour_cycle_24 {
            return display_hour;
        }
        let is_pm = self.day_period.unwrap_or_else(|| placeholder.hour() >= 12);
        match (display_hour, is_pm) {
            (12, false) => 0,    // 12 AM = 0
            (12, true) => 12,    // 12 PM = 12
            (h, false) => h,     // 1-11 AM
            (h, true) => h + 12, // 1-11 PM
        }
    }

    /// Convert 24-hour value to (`display_hour`, `day_period`).
    pub fn from_24_hour(hour: u8, hour_cycle_24: bool) -> (u8, Option<bool>) {
        if hour_cycle_24 {
            (hour, None)
        } else {
            let is_pm = hour >= 12;
            let display = match hour {
                0 => 12,
                1..=12 => hour,
                _ => hour - 12,
            };
            (display, Some(is_pm))
        }
    }

    /// Max day for the currently selected year/month (or placeholder).
    fn day_max(&self, placeholder: &time::OffsetDateTime) -> u8 {
        let year = self.year.unwrap_or_else(|| placeholder.year());
        let month_num = self.month.unwrap_or_else(|| placeholder.month() as u8);
        month_from_u8(month_num).map_or(31, |m| whole_days_in(year, m))
    }

    /// Hour min/max depending on hour cycle.
    fn hour_limits(&self) -> (i32, i32) {
        if self.hour_cycle_24 { (0, 23) } else { (1, 12) }
    }

    /// Clamp day if it exceeds the max for current month.
    fn clamp_day_to_month(&mut self) {
        if let (Some(year), Some(month_num), Some(day)) = (self.year, self.month, self.day) {
            if let Some(month) = month_from_u8(month_num) {
                let max = whole_days_in(year, month);
                if day > max {
                    self.day = Some(max);
                }
            }
        }
    }

    /// The maximum number of digits for a segment type.
    pub fn max_digits(segment_type: DateSegmentType) -> usize {
        match segment_type {
            DateSegmentType::Year => 4,
            DateSegmentType::Month
            | DateSegmentType::Day
            | DateSegmentType::Hour
            | DateSegmentType::Minute
            | DateSegmentType::Second => 2,
            DateSegmentType::DayPeriod | DateSegmentType::Literal => 0,
        }
    }
}

/// Wrap `value` into `[min, max]` range with cycling.
fn wrap(value: i32, min: i32, max: i32) -> i32 {
    let range = max - min + 1;
    ((value - min).rem_euclid(range)) + min
}

/// Convert a 1-12 number to `time::Month`.
fn month_from_u8(m: u8) -> Option<time::Month> {
    match m {
        1 => Some(time::Month::January),
        2 => Some(time::Month::February),
        3 => Some(time::Month::March),
        4 => Some(time::Month::April),
        5 => Some(time::Month::May),
        6 => Some(time::Month::June),
        7 => Some(time::Month::July),
        8 => Some(time::Month::August),
        9 => Some(time::Month::September),
        10 => Some(time::Month::October),
        11 => Some(time::Month::November),
        12 => Some(time::Month::December),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn wrap_cycles_forward() {
        assert_that(wrap(13, 1, 12)).is_equal_to(1);
        assert_that(wrap(32, 1, 31)).is_equal_to(1);
        assert_that(wrap(60, 0, 59)).is_equal_to(0);
    }

    #[test]
    fn wrap_cycles_backward() {
        assert_that(wrap(0, 1, 12)).is_equal_to(12);
        assert_that(wrap(0, 1, 31)).is_equal_to(31);
        assert_that(wrap(-1, 0, 59)).is_equal_to(59);
    }

    #[test]
    fn from_24_hour_h24() {
        assert_that(IncompleteDate::from_24_hour(0, true)).is_equal_to((0, None));
        assert_that(IncompleteDate::from_24_hour(13, true)).is_equal_to((13, None));
        assert_that(IncompleteDate::from_24_hour(23, true)).is_equal_to((23, None));
    }

    #[test]
    fn from_24_hour_h12() {
        assert_that(IncompleteDate::from_24_hour(0, false)).is_equal_to((12, Some(false)));
        assert_that(IncompleteDate::from_24_hour(1, false)).is_equal_to((1, Some(false)));
        assert_that(IncompleteDate::from_24_hour(12, false)).is_equal_to((12, Some(true)));
        assert_that(IncompleteDate::from_24_hour(13, false)).is_equal_to((1, Some(true)));
        assert_that(IncompleteDate::from_24_hour(23, false)).is_equal_to((11, Some(true)));
    }

    #[test]
    fn cycle_month_wraps() {
        let placeholder = time::macros::datetime!(2024-06-15 12:00 UTC);
        let mut date = IncompleteDate::from_date(&placeholder, true);
        date.cycle(DateSegmentType::Month, 1, &placeholder);
        assert_that(date.month).is_equal_to(Some(7));

        date.month = Some(12);
        date.cycle(DateSegmentType::Month, 1, &placeholder);
        assert_that(date.month).is_equal_to(Some(1));

        date.month = Some(1);
        date.cycle(DateSegmentType::Month, -1, &placeholder);
        assert_that(date.month).is_equal_to(Some(12));
    }

    #[test]
    fn cycle_day_wraps() {
        let placeholder = time::macros::datetime!(2024-02-15 12:00 UTC);
        let mut date = IncompleteDate::from_date(&placeholder, true);
        date.day = Some(29); // leap year
        date.cycle(DateSegmentType::Day, 1, &placeholder);
        assert_that(date.day).is_equal_to(Some(1));

        date.day = Some(1);
        date.cycle(DateSegmentType::Day, -1, &placeholder);
        assert_that(date.day).is_equal_to(Some(29));
    }

    #[test]
    fn is_complete_date_only() {
        let mut date = IncompleteDate::empty(true);
        assert_that(date.is_complete(false)).is_false();
        date.year = Some(2024);
        date.month = Some(6);
        assert_that(date.is_complete(false)).is_false();
        date.day = Some(15);
        assert_that(date.is_complete(false)).is_true();
    }

    #[test]
    fn is_complete_with_time_h24() {
        let mut date = IncompleteDate::empty(true);
        date.year = Some(2024);
        date.month = Some(6);
        date.day = Some(15);
        assert_that(date.is_complete(true)).is_false();
        date.hour = Some(12);
        date.minute = Some(30);
        assert_that(date.is_complete(true)).is_true();
    }

    #[test]
    fn is_complete_with_time_h12() {
        let mut date = IncompleteDate::empty(false);
        date.year = Some(2024);
        date.month = Some(6);
        date.day = Some(15);
        date.hour = Some(12);
        date.minute = Some(30);
        assert_that(date.is_complete(true)).is_false();
        date.day_period = Some(false);
        assert_that(date.is_complete(true)).is_true();
    }

    #[test]
    fn to_date_basic() {
        let placeholder = time::macros::datetime!(2024-01-01 0:00 UTC);
        let mut date = IncompleteDate::empty(true);
        date.year = Some(2024);
        date.month = Some(6);
        date.day = Some(15);
        date.hour = Some(14);
        date.minute = Some(30);
        date.second = Some(0);
        let result = date.to_date(&placeholder);
        assert_that(result).is_some();
        let dt = result.unwrap();
        assert_that(dt.year()).is_equal_to(2024);
        assert_that(dt.month() as u8).is_equal_to(6);
        assert_that(dt.day()).is_equal_to(15);
        assert_that(dt.hour()).is_equal_to(14);
        assert_that(dt.minute()).is_equal_to(30);
    }

    #[test]
    fn to_date_h12_pm() {
        let placeholder = time::macros::datetime!(2024-01-01 0:00 UTC);
        let mut date = IncompleteDate::empty(false);
        date.year = Some(2024);
        date.month = Some(6);
        date.day = Some(15);
        date.hour = Some(3);
        date.minute = Some(30);
        date.second = Some(0);
        date.day_period = Some(true); // PM
        let dt = date.to_date(&placeholder).unwrap();
        assert_that(dt.hour()).is_equal_to(15);
    }

    #[test]
    fn month_change_clamps_day() {
        let placeholder = time::macros::datetime!(2024-01-31 0:00 UTC);
        let mut date = IncompleteDate::from_date(&placeholder, true);
        assert_that(date.day).is_equal_to(Some(31));
        // Change to February (max 29 in 2024 leap year)
        date.set(DateSegmentType::Month, 2);
        assert_that(date.day).is_equal_to(Some(29));
    }
}
