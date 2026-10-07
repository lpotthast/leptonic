// Upstream: react-aria/src/i18n/useDateFormatter.ts @ 6f664fe911
// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/react-aria/src/i18n/useDateFormatter.ts

use icu_locale::Locale as IcuLocale;

use super::i18n::Locale;

/// Date/time formatting options.
#[derive(Debug, Clone, Default)]
pub struct DateTimeFormatOptions {
    /// How to format the weekday.
    pub weekday: Option<DateTimeFormat>,

    /// How to format the era.
    pub era: Option<DateTimeFormat>,

    /// How to format the year.
    pub year: Option<NumericFormat>,

    /// How to format the month.
    pub month: Option<MonthFormat>,

    /// How to format the day.
    pub day: Option<NumericFormat>,

    /// How to format the hour.
    pub hour: Option<NumericFormat>,

    /// How to format the minute.
    pub minute: Option<NumericFormat>,

    /// How to format the second.
    pub second: Option<NumericFormat>,

    /// How to format the time zone name.
    pub time_zone_name: Option<TimeZoneFormat>,

    /// Whether to use 12-hour time.
    pub hour12: Option<bool>,

    /// The time zone to use.
    pub time_zone: Option<String>,

    /// The date style (short, medium, long, full).
    pub date_style: Option<DateTimeStyle>,

    /// The time style (short, medium, long, full).
    pub time_style: Option<DateTimeStyle>,
}

/// Date/time format options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimeFormat {
    /// Long format (e.g., "Thursday").
    Long,
    /// Short format (e.g., "Thu").
    Short,
    /// Narrow format (e.g., "T").
    Narrow,
}

/// Numeric format options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericFormat {
    /// Numeric format (e.g., "2").
    Numeric,
    /// 2-digit format (e.g., "02").
    TwoDigit,
}

/// Month format options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonthFormat {
    /// Numeric format (e.g., "3").
    Numeric,
    /// 2-digit format (e.g., "03").
    TwoDigit,
    /// Long format (e.g., "March").
    Long,
    /// Short format (e.g., "Mar").
    Short,
    /// Narrow format (e.g., "M").
    Narrow,
}

/// Time zone format options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeZoneFormat {
    /// Long format (e.g., "Pacific Standard Time").
    Long,
    /// Short format (e.g., "PST").
    Short,
}

/// Date/time style options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimeStyle {
    /// Full style (e.g., "Thursday, March 14, 2024").
    Full,
    /// Long style (e.g., "March 14, 2024").
    Long,
    /// Medium style (e.g., "Mar 14, 2024").
    Medium,
    /// Short style (e.g., "3/14/24").
    Short,
}

/// The length of an ICU4X field set.
/// The CLDR pattern of a date with only one of weekday, year, month or day (standalone forms).
fn single_field_pattern(options: &DateTimeFormatOptions) -> Option<&'static str> {
    match (options.weekday, options.year, options.month, options.day) {
        (Some(weekday), None, None, None) => Some(match weekday {
            DateTimeFormat::Long => "cccc",
            DateTimeFormat::Short => "ccc",
            DateTimeFormat::Narrow => "ccccc",
        }),
        (None, None, Some(month), None) => Some(match month {
            MonthFormat::Numeric => "L",
            MonthFormat::TwoDigit => "LL",
            MonthFormat::Short => "LLL",
            MonthFormat::Long => "LLLL",
            MonthFormat::Narrow => "LLLLL",
        }),
        (None, Some(year), None, None) => Some(match year {
            NumericFormat::Numeric => "y",
            NumericFormat::TwoDigit => "yy",
        }),
        (None, None, None, Some(day)) => Some(match day {
            NumericFormat::Numeric => "d",
            NumericFormat::TwoDigit => "dd",
        }),
        _ => None,
    }
}

fn format_pattern(
    prefs: icu_datetime::DateTimeFormatterPreferences,
    pattern: &str,
    date: icu_calendar::Date<icu_calendar::Gregorian>,
) -> Option<String> {
    let pattern: icu_datetime::pattern::DateTimePattern = pattern.parse().ok()?;
    let mut names = icu_datetime::pattern::FixedCalendarDateTimeNames::<
        icu_calendar::Gregorian,
        icu_datetime::fieldsets::enums::DateFieldSet,
    >::try_new(prefs)
    .ok()?;
    let formatter = names.include_for_pattern(&pattern).ok()?;
    writeable::TryWriteable::try_write_to_string(&formatter.format(&date))
        .ok()
        .map(std::borrow::Cow::into_owned)
}

/// The length of an ICU4X field set.
#[derive(Clone, Copy)]
enum Length {
    Long,
    Medium,
    Short,
}

/// A locale-aware date/time formatter backed by ICU4X.
///
/// Uses `icu_datetime` for locale-aware date and time formatting.
/// Accepts a `jiff::civil::DateTime` and converts internally to ICU4X types.
#[derive(Debug, Clone)]
pub struct DateTimeFormatter {
    locale: IcuLocale,
    options: DateTimeFormatOptions,
}

impl DateTimeFormatter {
    /// Creates a new date/time formatter with the given locale and options.
    #[must_use]
    pub fn new(locale: &Locale, options: DateTimeFormatOptions) -> Self {
        Self {
            locale: locale.icu_locale().clone(),
            options,
        }
    }

    /// Formats a date according to the date options (weekday, year, month, day), localized with
    /// the ICU4X field set that has them (react-aria: `Intl.DateTimeFormat` with these options),
    /// e.g. "March 2024" (year and long month) or "Thursday, March 14, 2024" (all four, long).
    /// The length follows the month's or the weekday's format; narrow weekdays and months are
    /// their first letter. Without any date option, the date's ISO form.
    #[must_use]
    pub fn format_date(&self, date: jiff::civil::Date) -> String {
        let (Ok(month), Ok(day)) = (u8::try_from(date.month()), u8::try_from(date.day())) else {
            return date.to_string();
        };
        let Ok(icu_date) =
            icu_calendar::Date::try_new_gregorian(i32::from(date.year()), month, day)
        else {
            return date.to_string();
        };
        let options = &self.options;
        let prefs = icu_datetime::DateTimeFormatterPreferences::from(&self.locale);
        // A single field: as a standalone name or number (narrow names, a day without the
        // locale's suffix like "日"), as `Intl.DateTimeFormat#formatToParts` gives it.
        if let Some(pattern) = single_field_pattern(options) {
            return format_pattern(prefs, pattern, icu_date).unwrap_or_else(|| date.to_string());
        }
        let length = match (options.month, options.weekday) {
            (Some(MonthFormat::Long), _) | (None, Some(DateTimeFormat::Long)) => Length::Long,
            (Some(MonthFormat::Short), _) | (None, Some(DateTimeFormat::Short)) => Length::Medium,
            _ => Length::Short,
        };
        macro_rules! format_with {
            ($fieldset:ident) => {{
                let fieldset = match length {
                    Length::Long => icu_datetime::fieldsets::$fieldset::long(),
                    Length::Medium => icu_datetime::fieldsets::$fieldset::medium(),
                    Length::Short => icu_datetime::fieldsets::$fieldset::short(),
                };
                icu_datetime::FixedCalendarDateTimeFormatter::<icu_calendar::Gregorian, _>::try_new(
                    prefs, fieldset,
                )
                .map(|formatter| formatter.format(&icu_date).to_string())
                .ok()
            }};
        }
        let (weekday, year, month, day) = (
            options.weekday.is_some(),
            options.year.is_some(),
            options.month.is_some(),
            options.day.is_some(),
        );
        let formatted = match (weekday, year, month, day) {
            (true, true, true, true) => format_with!(YMDE),
            (_, true, true, true) => format_with!(YMD),
            (true, _, true, true) => format_with!(MDE),
            (_, _, true, true) => format_with!(MD),
            (true, _, _, true) => format_with!(DE),
            (_, _, _, true) => format_with!(D),
            (_, true, true, false) => format_with!(YM),
            (true, false, false, false) => format_with!(E),
            (_, false, true, false) => format_with!(M),
            (_, true, false, false) => format_with!(Y),
            (false, false, false, false) => None,
        };
        formatted.unwrap_or_else(|| date.to_string())
    }

    /// Formats a date/time according to the formatter's options.
    #[must_use]
    pub fn format(&self, dt: &jiff::civil::DateTime) -> String {
        // Try ICU4X formatting for date_style/time_style
        if (self.options.date_style.is_some() || self.options.time_style.is_some())
            && let Some(result) = self.try_format_with_icu(dt)
        {
            return result;
        }

        // Fall back to component-based formatting
        self.format_components(dt)
    }

    /// Attempts to format using ICU4X with length-based styles.
    fn try_format_with_icu(&self, dt: &jiff::civil::DateTime) -> Option<String> {
        let icu_date = icu_calendar::Date::try_new_gregorian(
            i32::from(dt.year()),
            dt.month().unsigned_abs(),
            dt.day().unsigned_abs(),
        )
        .ok()?;
        let icu_time = icu_time::Time::try_new(
            dt.hour().unsigned_abs(),
            dt.minute().unsigned_abs(),
            dt.second().unsigned_abs(),
            0,
        )
        .ok()?;
        let icu_dt = icu_time::DateTime {
            date: icu_date,
            time: icu_time,
        };

        let prefs = icu_datetime::DateTimeFormatterPreferences::from(&self.locale);

        match (self.options.date_style, self.options.time_style) {
            (Some(date_style), Some(_time_style)) => {
                // Use YMDE (year, month, day, weekday) with time for full style,
                // YMD with time for other styles.
                let result = if date_style == DateTimeStyle::Full {
                    let fieldset = icu_datetime::fieldsets::YMDE::long().with_time_hms();
                    let formatter =
                        icu_datetime::FixedCalendarDateTimeFormatter::try_new(prefs, fieldset)
                            .ok()?;
                    formatter.format(&icu_dt).to_string()
                } else {
                    let fieldset = match date_style {
                        DateTimeStyle::Long => icu_datetime::fieldsets::YMD::long().with_time_hms(),
                        DateTimeStyle::Medium => {
                            icu_datetime::fieldsets::YMD::medium().with_time_hm()
                        }
                        DateTimeStyle::Short => {
                            icu_datetime::fieldsets::YMD::short().with_time_hm()
                        }
                        DateTimeStyle::Full => unreachable!(),
                    };
                    let formatter =
                        icu_datetime::FixedCalendarDateTimeFormatter::try_new(prefs, fieldset)
                            .ok()?;
                    formatter.format(&icu_dt).to_string()
                };
                Some(result)
            }
            (Some(date_style), None) => {
                let fieldset = match date_style {
                    DateTimeStyle::Full | DateTimeStyle::Long => {
                        icu_datetime::fieldsets::YMD::long()
                    }
                    DateTimeStyle::Medium => icu_datetime::fieldsets::YMD::medium(),
                    DateTimeStyle::Short => icu_datetime::fieldsets::YMD::short(),
                };
                let formatter =
                    icu_datetime::FixedCalendarDateTimeFormatter::try_new(prefs, fieldset).ok()?;
                Some(formatter.format(&icu_dt).to_string())
            }
            (None, Some(time_style)) => {
                let result = match time_style {
                    DateTimeStyle::Full | DateTimeStyle::Long | DateTimeStyle::Medium => {
                        let formatter = icu_datetime::NoCalendarFormatter::try_new(
                            prefs,
                            icu_datetime::fieldsets::T::hms(),
                        )
                        .ok()?;
                        formatter.format(&icu_time).to_string()
                    }
                    DateTimeStyle::Short => {
                        let formatter = icu_datetime::NoCalendarFormatter::try_new(
                            prefs,
                            icu_datetime::fieldsets::T::hm(),
                        )
                        .ok()?;
                        formatter.format(&icu_time).to_string()
                    }
                };
                Some(result)
            }
            (None, None) => None,
        }
    }

    /// Formats using individual component options (weekday, month, day, year, etc.).
    fn format_components(&self, dt: &jiff::civil::DateTime) -> String {
        let mut parts = Vec::new();

        if let Some(weekday) = self.options.weekday {
            parts.push(self.format_weekday(dt, weekday));
        }

        if let Some(month) = self.options.month {
            parts.push(self.format_month(dt, month));
        }

        if let Some(day) = self.options.day {
            parts.push(format_day(dt, day));
        }

        if let Some(year) = self.options.year {
            parts.push(format_year(dt, year));
        }

        if let Some(hour) = self.options.hour {
            let minute_part = self
                .options
                .minute
                .map(|m| format!(":{}", format_minute(dt, m)))
                .unwrap_or_default();
            let second_part = self
                .options
                .second
                .map(|s| format!(":{}", format_second(dt, s)))
                .unwrap_or_default();
            let hour_str = self.format_hour(dt, hour);
            parts.push(format!("{hour_str}{minute_part}{second_part}"));
        }

        if parts.is_empty() {
            // Default to ISO format
            format!("{:04}-{:02}-{:02}", dt.year(), dt.month(), dt.day())
        } else {
            parts.join(" ")
        }
    }

    fn format_weekday(&self, dt: &jiff::civil::DateTime, format: DateTimeFormat) -> String {
        if let Some(name) = self.try_icu_weekday_name(dt, format) {
            return name;
        }
        // Fallback to English
        match format {
            DateTimeFormat::Long => weekday_name_en(dt.weekday(), false),
            DateTimeFormat::Short => weekday_name_en(dt.weekday(), true),
            DateTimeFormat::Narrow => weekday_name_en(dt.weekday(), true)[..1].to_string(),
        }
    }

    fn try_icu_weekday_name(
        &self,
        dt: &jiff::civil::DateTime,
        format: DateTimeFormat,
    ) -> Option<String> {
        let icu_date = icu_calendar::Date::try_new_gregorian(
            i32::from(dt.year()),
            dt.month().unsigned_abs(),
            dt.day().unsigned_abs(),
        )
        .ok()?;

        let fieldset = match format {
            DateTimeFormat::Long => icu_datetime::fieldsets::E::long(),
            DateTimeFormat::Short | DateTimeFormat::Narrow => icu_datetime::fieldsets::E::short(),
        };

        let prefs = icu_datetime::DateTimeFormatterPreferences::from(&self.locale);
        let formatter = icu_datetime::DateTimeFormatter::try_new(prefs, fieldset).ok()?;
        let result = formatter.format(&icu_date).to_string();

        // For narrow, just take the first character
        if format == DateTimeFormat::Narrow {
            Some(result.chars().next()?.to_string())
        } else {
            Some(result)
        }
    }

    fn format_month(&self, dt: &jiff::civil::DateTime, format: MonthFormat) -> String {
        match format {
            MonthFormat::Numeric => format!("{}", dt.month()),
            MonthFormat::TwoDigit => format!("{:02}", dt.month()),
            MonthFormat::Long | MonthFormat::Short | MonthFormat::Narrow => {
                if let Some(name) = self.try_icu_month_name(dt, format) {
                    return name;
                }
                // Fallback to English
                match format {
                    MonthFormat::Long => month_name_en(dt.month(), false),
                    MonthFormat::Short => month_name_en(dt.month(), true),
                    MonthFormat::Narrow => month_name_en(dt.month(), true)[..1].to_string(),
                    _ => unreachable!(),
                }
            }
        }
    }

    fn try_icu_month_name(
        &self,
        dt: &jiff::civil::DateTime,
        format: MonthFormat,
    ) -> Option<String> {
        let icu_date = icu_calendar::Date::try_new_gregorian(
            i32::from(dt.year()),
            dt.month().unsigned_abs(),
            dt.day().unsigned_abs(),
        )
        .ok()?;

        let fieldset = match format {
            MonthFormat::Long => icu_datetime::fieldsets::M::long(),
            MonthFormat::Short | MonthFormat::Narrow => icu_datetime::fieldsets::M::short(),
            _ => return None,
        };

        let prefs = icu_datetime::DateTimeFormatterPreferences::from(&self.locale);
        let formatter = icu_datetime::DateTimeFormatter::try_new(prefs, fieldset).ok()?;
        let result = formatter.format(&icu_date).to_string();

        if format == MonthFormat::Narrow {
            Some(result.chars().next()?.to_string())
        } else {
            Some(result)
        }
    }

    fn format_hour(&self, dt: &jiff::civil::DateTime, format: NumericFormat) -> String {
        let hour12 = self.options.hour12.unwrap_or(false);
        let hour = if hour12 {
            let h = dt.hour();
            if h == 0 {
                12
            } else if h > 12 {
                h - 12
            } else {
                h
            }
        } else {
            dt.hour()
        };

        match format {
            NumericFormat::Numeric => format!("{hour}"),
            NumericFormat::TwoDigit => format!("{hour:02}"),
        }
    }
}

fn format_day(dt: &jiff::civil::DateTime, format: NumericFormat) -> String {
    match format {
        NumericFormat::Numeric => format!("{}", dt.day()),
        NumericFormat::TwoDigit => format!("{:02}", dt.day()),
    }
}

fn format_year(dt: &jiff::civil::DateTime, format: NumericFormat) -> String {
    match format {
        NumericFormat::Numeric => format!("{}", dt.year()),
        NumericFormat::TwoDigit => format!("{:02}", dt.year() % 100),
    }
}

fn format_minute(dt: &jiff::civil::DateTime, format: NumericFormat) -> String {
    match format {
        NumericFormat::Numeric => format!("{}", dt.minute()),
        NumericFormat::TwoDigit => format!("{:02}", dt.minute()),
    }
}

fn format_second(dt: &jiff::civil::DateTime, format: NumericFormat) -> String {
    match format {
        NumericFormat::Numeric => format!("{}", dt.second()),
        NumericFormat::TwoDigit => format!("{:02}", dt.second()),
    }
}

fn weekday_name_en(weekday: jiff::civil::Weekday, short: bool) -> String {
    let name = match weekday {
        jiff::civil::Weekday::Monday => ("Monday", "Mon"),
        jiff::civil::Weekday::Tuesday => ("Tuesday", "Tue"),
        jiff::civil::Weekday::Wednesday => ("Wednesday", "Wed"),
        jiff::civil::Weekday::Thursday => ("Thursday", "Thu"),
        jiff::civil::Weekday::Friday => ("Friday", "Fri"),
        jiff::civil::Weekday::Saturday => ("Saturday", "Sat"),
        jiff::civil::Weekday::Sunday => ("Sunday", "Sun"),
    };

    if short {
        name.1.to_string()
    } else {
        name.0.to_string()
    }
}

fn month_name_en(month: i8, short: bool) -> String {
    const NAMES: [(&str, &str); 12] = [
        ("January", "Jan"),
        ("February", "Feb"),
        ("March", "Mar"),
        ("April", "Apr"),
        ("May", "May"),
        ("June", "Jun"),
        ("July", "Jul"),
        ("August", "Aug"),
        ("September", "Sep"),
        ("October", "Oct"),
        ("November", "Nov"),
        ("December", "Dec"),
    ];
    let Some(name) = usize::from(month.unsigned_abs())
        .checked_sub(1)
        .and_then(|index| NAMES.get(index))
    else {
        return month.to_string();
    };
    if short {
        name.1.to_string()
    } else {
        name.0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use jiff::civil::date;

    use super::*;

    fn format(locale: &str, options: DateTimeFormatOptions) -> String {
        let locale = locale.parse::<Locale>().expect("a locale");
        DateTimeFormatter::new(&locale, options).format_date(date(2024, 3, 14))
    }

    #[test]
    fn formats_dates_with_the_matching_field_set() {
        let month_year = DateTimeFormatOptions {
            month: Some(MonthFormat::Long),
            year: Some(NumericFormat::Numeric),
            ..DateTimeFormatOptions::default()
        };
        assert_that!(format("en-US", month_year.clone())).is_equal_to("March 2024".to_owned());
        assert_that!(format("de-DE", month_year)).is_equal_to("März 2024".to_owned());
        let full = DateTimeFormatOptions {
            weekday: Some(DateTimeFormat::Long),
            month: Some(MonthFormat::Long),
            day: Some(NumericFormat::Numeric),
            year: Some(NumericFormat::Numeric),
            ..DateTimeFormatOptions::default()
        };
        assert_that!(format("en-US", full.clone()))
            .is_equal_to("Thursday, March 14, 2024".to_owned());
        assert_that!(format("de-DE", full)).is_equal_to("Donnerstag, 14. März 2024".to_owned());
        let day = DateTimeFormatOptions {
            day: Some(NumericFormat::Numeric),
            ..DateTimeFormatOptions::default()
        };
        assert_that!(format("en-US", day)).is_equal_to("14".to_owned());
        let weekday = |format| DateTimeFormatOptions {
            weekday: Some(format),
            ..DateTimeFormatOptions::default()
        };
        assert_that!(format("en-US", weekday(DateTimeFormat::Long)))
            .is_equal_to("Thursday".to_owned());
        assert_that!(format("en-US", weekday(DateTimeFormat::Narrow))).is_equal_to("T".to_owned());
    }

    /// A single field is a standalone name or number: CLDR's narrow names (distinct in zh, where
    /// cutting "周四" would leave "周" for every day), a day without the locale's suffix.
    #[test]
    fn formats_a_single_field_standalone() {
        let weekday = DateTimeFormatOptions {
            weekday: Some(DateTimeFormat::Narrow),
            ..DateTimeFormatOptions::default()
        };
        assert_that!(format("zh-CN", weekday)).is_equal_to("四".to_owned());
        let day = DateTimeFormatOptions {
            day: Some(NumericFormat::Numeric),
            ..DateTimeFormatOptions::default()
        };
        assert_that!(format("ja-JP", day.clone())).is_equal_to("14".to_owned());
        assert_that!(format("ko-KR", day)).is_equal_to("14".to_owned());
        let month = DateTimeFormatOptions {
            month: Some(MonthFormat::Narrow),
            ..DateTimeFormatOptions::default()
        };
        assert_that!(format("en-US", month)).is_equal_to("M".to_owned());
        let year = DateTimeFormatOptions {
            year: Some(NumericFormat::Numeric),
            ..DateTimeFormatOptions::default()
        };
        assert_that!(format("en-US", year)).is_equal_to("2024".to_owned());
    }
}
