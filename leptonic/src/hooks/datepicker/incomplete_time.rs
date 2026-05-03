use super::{
    incomplete_date::IncompleteDate, use_date_segment::DateSegmentType, use_time_field::TimeValue,
};

/// A partially-filled time value.
///
/// Time-only analog of [`IncompleteDate`], for use with `use_time_field`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncompleteTime {
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

impl IncompleteTime {
    /// Create an empty `IncompleteTime` with no fields set.
    pub fn empty(hour_cycle_24: bool) -> Self {
        Self {
            hour: None,
            minute: None,
            second: None,
            day_period: None,
            hour_cycle_24,
        }
    }

    /// Populate all fields from a complete `TimeValue`.
    pub fn from_time(time: &TimeValue, hour_cycle_24: bool) -> Self {
        let (hour, day_period) = IncompleteDate::from_24_hour(time.hour, hour_cycle_24);
        Self {
            hour: Some(hour),
            minute: Some(time.minute),
            second: Some(time.second),
            day_period,
            hour_cycle_24,
        }
    }

    /// Sync from an external complete time value.
    pub fn sync_from_time(&mut self, time: &TimeValue) {
        let (hour, day_period) = IncompleteDate::from_24_hour(time.hour, self.hour_cycle_24);
        self.hour = Some(hour);
        self.minute = Some(time.minute);
        self.second = Some(time.second);
        self.day_period = day_period;
    }

    /// Set a single field by segment type.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn set(&mut self, segment_type: DateSegmentType, value: i32) {
        match segment_type {
            DateSegmentType::Hour => {
                let (min, max) = self.hour_limits();
                self.hour = Some(value.clamp(min, max) as u8);
                if !self.hour_cycle_24 && self.day_period.is_none() {
                    self.day_period = Some(false);
                }
            }
            DateSegmentType::Minute => self.minute = Some(value.clamp(0, 59) as u8),
            DateSegmentType::Second => self.second = Some(value.clamp(0, 59) as u8),
            DateSegmentType::DayPeriod => self.day_period = Some(value != 0),
            _ => {}
        }
    }

    /// Clear a field.
    pub fn clear(&mut self, segment_type: DateSegmentType) {
        match segment_type {
            DateSegmentType::Hour => self.hour = None,
            DateSegmentType::Minute => self.minute = None,
            DateSegmentType::Second => self.second = None,
            DateSegmentType::DayPeriod => self.day_period = None,
            _ => {}
        }
    }

    /// Increment or decrement a segment with wrapping.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn cycle(&mut self, segment_type: DateSegmentType, amount: i32, placeholder: &TimeValue) {
        match segment_type {
            DateSegmentType::Hour => {
                let (min, max) = self.hour_limits();
                let current = self.hour.unwrap_or_else(|| {
                    let (h, dp) =
                        IncompleteDate::from_24_hour(placeholder.hour, self.hour_cycle_24);
                    if !self.hour_cycle_24 && self.day_period.is_none() {
                        self.day_period = dp;
                    }
                    h
                });
                let new_val = wrap(i32::from(current) + amount, min, max);
                self.hour = Some(new_val as u8);
            }
            DateSegmentType::Minute => {
                let current = self.minute.unwrap_or(placeholder.minute);
                let new_val = wrap(i32::from(current) + amount, 0, 59);
                self.minute = Some(new_val as u8);
            }
            DateSegmentType::Second => {
                let current = self.second.unwrap_or(placeholder.second);
                let new_val = wrap(i32::from(current) + amount, 0, 59);
                self.second = Some(new_val as u8);
            }
            DateSegmentType::DayPeriod => {
                let current = self.day_period.unwrap_or(placeholder.hour >= 12);
                self.day_period = Some(!current);
            }
            _ => {}
        }
    }

    /// Set a segment to its maximum value.
    pub fn set_to_max(&mut self, segment_type: DateSegmentType) {
        let (_, max) = self.get_segment_limits(segment_type);
        self.set(segment_type, max);
    }

    /// Set a segment to its minimum value.
    pub fn set_to_min(&mut self, segment_type: DateSegmentType) {
        let (min, _) = self.get_segment_limits(segment_type);
        self.set(segment_type, min);
    }

    /// Whether all required fields are filled.
    pub fn is_complete(&self, show_seconds: bool) -> bool {
        let base = self.hour.is_some() && self.minute.is_some();
        let seconds_ok = !show_seconds || self.second.is_some();
        let period_ok = self.hour_cycle_24 || self.day_period.is_some();
        base && seconds_ok && period_ok
    }

    /// Whether all fields are `None`.
    pub fn is_cleared(&self, show_seconds: bool) -> bool {
        let base = self.hour.is_none() && self.minute.is_none();
        let seconds_ok = !show_seconds || self.second.is_none();
        let period_ok = self.hour_cycle_24 || self.day_period.is_none();
        base && seconds_ok && period_ok
    }

    /// Convert to a complete `TimeValue` using `placeholder` for missing fields.
    pub fn to_time(&self, placeholder: &TimeValue) -> TimeValue {
        let hour_24 = self.to_24_hour(*placeholder);
        let minute = self.minute.unwrap_or(placeholder.minute);
        let second = self.second.unwrap_or(placeholder.second);
        TimeValue::new(hour_24, minute, second)
    }

    /// Get (min, max) for a segment type.
    pub fn get_segment_limits(&self, segment_type: DateSegmentType) -> (i32, i32) {
        match segment_type {
            DateSegmentType::Hour => self.hour_limits(),
            DateSegmentType::Minute | DateSegmentType::Second => (0, 59),
            DateSegmentType::DayPeriod => (0, 1),
            _ => (0, 0),
        }
    }

    /// Convert display hour + `day_period` to 24-hour value.
    fn to_24_hour(&self, placeholder: TimeValue) -> u8 {
        let display_hour = self.hour.unwrap_or_else(|| {
            let (h, _) = IncompleteDate::from_24_hour(placeholder.hour, self.hour_cycle_24);
            h
        });
        if self.hour_cycle_24 {
            return display_hour;
        }
        let is_pm = self.day_period.unwrap_or(placeholder.hour >= 12);
        match (display_hour, is_pm) {
            (12, false) => 0,
            (12, true) => 12,
            (h, false) => h,
            (h, true) => h + 12,
        }
    }

    fn hour_limits(&self) -> (i32, i32) {
        if self.hour_cycle_24 { (0, 23) } else { (1, 12) }
    }
}

/// Wrap `value` into `[min, max]` range with cycling.
fn wrap(value: i32, min: i32, max: i32) -> i32 {
    let range = max - min + 1;
    ((value - min).rem_euclid(range)) + min
}
