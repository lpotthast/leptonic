// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/i18n/src/useDateFormatter.ts

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

/// A locale-aware date/time formatter backed by ICU4X.
///
/// Uses `icu_datetime` for locale-aware date and time formatting.
/// Accepts `time::OffsetDateTime` and converts internally to ICU4X types.
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

    /// Formats a date/time according to the formatter's options.
    #[must_use]
    pub fn format(&self, dt: &time::OffsetDateTime) -> String {
        // Try ICU4X formatting for date_style/time_style
        if self.options.date_style.is_some() || self.options.time_style.is_some() {
            if let Some(result) = self.try_format_with_icu(dt) {
                return result;
            }
        }

        // Fall back to component-based formatting
        self.format_components(dt)
    }

    /// Attempts to format using ICU4X with length-based styles.
    fn try_format_with_icu(&self, dt: &time::OffsetDateTime) -> Option<String> {
        let icu_date =
            icu_calendar::Date::try_new_gregorian(dt.year(), dt.month() as u8, dt.day()).ok()?;
        let icu_time = icu_time::Time::try_new(dt.hour(), dt.minute(), dt.second(), 0).ok()?;
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
    fn format_components(&self, dt: &time::OffsetDateTime) -> String {
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
            format!("{:04}-{:02}-{:02}", dt.year(), dt.month() as u8, dt.day())
        } else {
            parts.join(" ")
        }
    }

    fn format_weekday(&self, dt: &time::OffsetDateTime, format: DateTimeFormat) -> String {
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
        dt: &time::OffsetDateTime,
        format: DateTimeFormat,
    ) -> Option<String> {
        let icu_date =
            icu_calendar::Date::try_new_gregorian(dt.year(), dt.month() as u8, dt.day()).ok()?;

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

    fn format_month(&self, dt: &time::OffsetDateTime, format: MonthFormat) -> String {
        match format {
            MonthFormat::Numeric => format!("{}", dt.month() as u8),
            MonthFormat::TwoDigit => format!("{:02}", dt.month() as u8),
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

    fn try_icu_month_name(&self, dt: &time::OffsetDateTime, format: MonthFormat) -> Option<String> {
        let icu_date =
            icu_calendar::Date::try_new_gregorian(dt.year(), dt.month() as u8, dt.day()).ok()?;

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

    fn format_hour(&self, dt: &time::OffsetDateTime, format: NumericFormat) -> String {
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

fn format_day(dt: &time::OffsetDateTime, format: NumericFormat) -> String {
    match format {
        NumericFormat::Numeric => format!("{}", dt.day()),
        NumericFormat::TwoDigit => format!("{:02}", dt.day()),
    }
}

fn format_year(dt: &time::OffsetDateTime, format: NumericFormat) -> String {
    match format {
        NumericFormat::Numeric => format!("{}", dt.year()),
        NumericFormat::TwoDigit => format!("{:02}", dt.year() % 100),
    }
}

fn format_minute(dt: &time::OffsetDateTime, format: NumericFormat) -> String {
    match format {
        NumericFormat::Numeric => format!("{}", dt.minute()),
        NumericFormat::TwoDigit => format!("{:02}", dt.minute()),
    }
}

fn format_second(dt: &time::OffsetDateTime, format: NumericFormat) -> String {
    match format {
        NumericFormat::Numeric => format!("{}", dt.second()),
        NumericFormat::TwoDigit => format!("{:02}", dt.second()),
    }
}

fn weekday_name_en(weekday: time::Weekday, short: bool) -> String {
    let name = match weekday {
        time::Weekday::Monday => ("Monday", "Mon"),
        time::Weekday::Tuesday => ("Tuesday", "Tue"),
        time::Weekday::Wednesday => ("Wednesday", "Wed"),
        time::Weekday::Thursday => ("Thursday", "Thu"),
        time::Weekday::Friday => ("Friday", "Fri"),
        time::Weekday::Saturday => ("Saturday", "Sat"),
        time::Weekday::Sunday => ("Sunday", "Sun"),
    };

    if short {
        name.1.to_string()
    } else {
        name.0.to_string()
    }
}

fn month_name_en(month: time::Month, short: bool) -> String {
    let name = match month {
        time::Month::January => ("January", "Jan"),
        time::Month::February => ("February", "Feb"),
        time::Month::March => ("March", "Mar"),
        time::Month::April => ("April", "Apr"),
        time::Month::May => ("May", "May"),
        time::Month::June => ("June", "Jun"),
        time::Month::July => ("July", "Jul"),
        time::Month::August => ("August", "Aug"),
        time::Month::September => ("September", "Sep"),
        time::Month::October => ("October", "Oct"),
        time::Month::November => ("November", "Nov"),
        time::Month::December => ("December", "Dec"),
    };

    if short {
        name.1.to_string()
    } else {
        name.0.to_string()
    }
}
