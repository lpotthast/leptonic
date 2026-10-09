// Upstream: react-aria/src/i18n/useDateFormatter.ts @ 99e6102368
// Upstream: @internationalized/date/src/DateFormatter.ts @ 99e6102368
// Upstream: @internationalized/date/tests/DateFormatter.test.js @ 99e6102368
//! Locale-aware formatting of dates and times with ICU4X (react-aria's `useDateFormatter`, a
//! cached `Intl.DateTimeFormat`).

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - A formatter is built from a `Locale` and `DateTimeFormatOptions` (react-aria: a hook memoizing
//   `Intl.DateTimeFormat` per locale and options); the options are enums, not strings.
// - Values are `jiff` types: `format_date` takes a `civil::Date`, `format` a `civil::DateTime`,
//   `format_zoned` a `Zoned` (react-aria: a JS `Date`, an instant). There is no `timeZone` option:
//   convert a `Zoned` with jiff (`Zoned::in_tz`) before formatting.
// - `hour_cycle` is 12 or 24 hours (`Intl`'s `hour12`/`hourCycle`).
//
// ## DIFFERENT BEHAVIOR
// - The options pick an ICU4X field set (`fieldsets::builder`), which is localized as a whole: the
//   date fields from the weekday, year, month and day options, the time precision from the
//   finest of hour, minute and second, the zone from `time_zone_name`. The length (long, medium,
//   short) follows the month's or the weekday's format; ICU4X has no per-field lengths, so a
//   narrow weekday or month in a combination is short, and a two-digit month, day or hour pads
//   all numeric fields (column alignment). A single date field is formatted standalone (narrow names, a day
//   without the locale's suffix), as `formatToParts` gives it.
// - Combinations CLDR has no skeleton for drop the weekday (e.g. a weekday with a month and no
//   day).
// - The era shows only when asked for (`era`; react-aria's calendar asks for it for dates before
//   Christ); the year is never shortened to two digits by the locale (`YearStyle::Full`).
// - `date_style`/`time_style` map to field sets as well: full is the long date with the weekday,
//   long and full times have seconds; ICU4X has no time zone in a time style (`format_zoned`
//   adds it with `time_zone_name`).
// - Only the Gregorian calendar (also for locales defaulting to another one).
//
// =============================================================================

use std::sync::{Arc, OnceLock};

use icu_datetime::{
    DateTimeFormatter as IcuFormatter, DateTimeFormatterPreferences,
    fieldsets::{
        builder::{DateFields, FieldSetBuilder, ZoneStyle},
        enums::CompositeFieldSet,
    },
    options::{Alignment, Length, TimePrecision, YearStyle},
    preferences::{CalendarAlgorithm, HourCycle as IcuHourCycle},
};
use icu_locale::Locale as IcuLocale;

use super::i18n::Locale;

/// Date/time formatting options.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DateTimeFormatOptions {
    /// How to format the weekday.
    pub weekday: Option<DateTimeFormat>,

    /// Shows the era ("AD", "BC"); its length follows the formatter's.
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

    /// How to format the time zone name (`format_zoned` only).
    pub time_zone_name: Option<TimeZoneFormat>,

    /// 12 or 24 hours. Default: the locale's.
    pub hour_cycle: Option<HourCycle>,

    /// The date style (short, medium, long, full), instead of the date options.
    pub date_style: Option<DateTimeStyle>,

    /// The time style (short, medium, long, full), instead of the time options.
    pub time_style: Option<DateTimeStyle>,
}

/// Whether times show 12 or 24 hours (react-aria's `hourCycle: 12 | 24`, `Intl`'s `hour12`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HourCycle {
    H12,
    H24,
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

/// The CLDR pattern of a date with only one of weekday, year, month or day (standalone forms).
fn single_field_pattern(options: &DateTimeFormatOptions) -> Option<&'static str> {
    if options.era.is_some() {
        return None;
    }
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
    prefs: DateTimeFormatterPreferences,
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

/// The ICU4X date fields of the date options (`None`: no date).
fn date_fields(options: &DateTimeFormatOptions) -> Option<DateFields> {
    if let Some(style) = options.date_style {
        return Some(if style == DateTimeStyle::Full {
            DateFields::YMDE
        } else {
            DateFields::YMD
        });
    }
    let (weekday, year, month, day) = (
        options.weekday.is_some(),
        options.year.is_some() || options.era.is_some(),
        options.month.is_some(),
        options.day.is_some(),
    );
    Some(match (weekday, year, month, day) {
        (true, true, true, true) => DateFields::YMDE,
        (_, true, true, true) => DateFields::YMD,
        (true, _, true, true) => DateFields::MDE,
        (_, _, true, true) => DateFields::MD,
        (true, _, _, true) => DateFields::DE,
        (_, _, _, true) => DateFields::D,
        (_, true, true, false) => DateFields::YM,
        (true, false, false, false) => DateFields::E,
        (_, false, true, false) => DateFields::M,
        (_, true, false, false) => DateFields::Y,
        (false, false, false, false) => return None,
    })
}

/// The time precision of the time options (`None`: no time).
fn time_precision(options: &DateTimeFormatOptions) -> Option<TimePrecision> {
    if let Some(style) = options.time_style {
        return Some(match style {
            DateTimeStyle::Full | DateTimeStyle::Long | DateTimeStyle::Medium => {
                TimePrecision::Second
            }
            DateTimeStyle::Short => TimePrecision::Minute,
        });
    }
    if options.second.is_some() {
        Some(TimePrecision::Second)
    } else if options.minute.is_some() {
        Some(TimePrecision::Minute)
    } else if options.hour.is_some() {
        Some(TimePrecision::Hour)
    } else {
        None
    }
}

/// The length of the field set: of the date style, else of the month's or the weekday's
/// format, else of the time style.
fn length(options: &DateTimeFormatOptions) -> Length {
    if let Some(style) = options.date_style.or(options.time_style) {
        return match style {
            DateTimeStyle::Full | DateTimeStyle::Long => Length::Long,
            DateTimeStyle::Medium => Length::Medium,
            DateTimeStyle::Short => Length::Short,
        };
    }
    match (options.month, options.weekday) {
        (Some(MonthFormat::Long), _) | (None, Some(DateTimeFormat::Long)) => Length::Long,
        (Some(MonthFormat::Short), _) | (None, Some(DateTimeFormat::Short)) => Length::Medium,
        _ => Length::Short,
    }
}

/// Whether a month, day or hour asks for two digits (minutes and seconds always have them).
fn pads(options: &DateTimeFormatOptions) -> bool {
    let two_digit = |format: Option<NumericFormat>| format == Some(NumericFormat::TwoDigit);
    options.month == Some(MonthFormat::TwoDigit)
        || two_digit(options.day)
        || two_digit(options.hour)
}

/// A locale-aware date/time formatter backed by ICU4X.
///
/// Formats `jiff` values: `format_date` a `civil::Date`, `format` a `civil::DateTime`,
/// `format_zoned` a `Zoned` (with its time zone's name, if asked for). The ICU4X formatters are
/// built on first use and kept (also by clones), so keep a formatter (e.g. in a `Memo`) rather
/// than creating one per value. Formatters are equal when their locale and options are.
#[derive(Debug, Clone)]
pub struct DateTimeFormatter {
    locale: IcuLocale,
    options: DateTimeFormatOptions,
    /// The ICU4X formatters: for dates, dates and times, zoned dates and times.
    formatters: Arc<[OnceLock<Option<IcuFormatter<CompositeFieldSet>>>; 3]>,
}

impl PartialEq for DateTimeFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.locale == other.locale && self.options == other.options
    }
}

impl DateTimeFormatter {
    /// Creates a new date/time formatter with the given locale and options.
    #[must_use]
    pub fn new(locale: &Locale, options: DateTimeFormatOptions) -> Self {
        Self {
            locale: locale.icu_locale().clone(),
            options,
            formatters: Arc::default(),
        }
    }

    /// The formatter's options.
    #[must_use]
    pub fn options(&self) -> &DateTimeFormatOptions {
        &self.options
    }

    fn preferences(&self) -> DateTimeFormatterPreferences {
        let mut prefs = DateTimeFormatterPreferences::from(&self.locale);
        prefs.calendar_algorithm = Some(CalendarAlgorithm::Gregory);
        if let Some(hour_cycle) = self.options.hour_cycle {
            // The locale's own 12- or 24-hour clock (`Intl`'s `hour12`): a 12-hour clock is h11
            // in Japan, h12 elsewhere.
            prefs.hour_cycle = Some(match hour_cycle {
                HourCycle::H12 => IcuHourCycle::Clock12,
                HourCycle::H24 => IcuHourCycle::Clock24,
            });
        }
        prefs
    }

    /// The formatter of the options' field set (`with_time`: the time options apply; `zone`:
    /// with the time zone name).
    fn formatter(&self, with_time: bool, zone: bool) -> Option<IcuFormatter<CompositeFieldSet>> {
        let options = &self.options;
        let mut date = date_fields(options);
        let time = if with_time {
            time_precision(options)
        } else {
            None
        };
        let zone_style = options
            .time_zone_name
            .filter(|_| zone && time.is_some())
            .map(|format| match format {
                TimeZoneFormat::Long => ZoneStyle::SpecificLong,
                TimeZoneFormat::Short => ZoneStyle::SpecificShort,
            });
        // Date fields don't combine with a time without a day.
        if time.is_some()
            && matches!(
                date,
                Some(DateFields::E | DateFields::M | DateFields::YM | DateFields::Y)
            )
        {
            date = None;
        }
        let mut builder = FieldSetBuilder::new();
        builder.length = Some(length(options));
        builder.date_fields = date;
        builder.time_precision = time;
        builder.zone_style = zone_style;
        builder.year_style = date
            .filter(|fields| {
                matches!(
                    fields,
                    DateFields::YMD | DateFields::YMDE | DateFields::YM | DateFields::Y
                )
            })
            .map(|_| {
                if options.era.is_some() {
                    YearStyle::WithEra
                } else {
                    YearStyle::Full
                }
            });
        builder.alignment = pads(options).then_some(Alignment::Column);
        let field_set = builder.build_composite().ok()?;
        IcuFormatter::try_new(self.preferences(), field_set).ok()
    }

    /// Formats a date according to the date options (weekday, era, year, month, day), localized
    /// with the ICU4X field set that has them (react-aria: `Intl.DateTimeFormat` with these
    /// options), e.g. "March 2024" (year and long month) or "Thursday, March 14, 2024" (all four,
    /// long). Without any date option, the date's ISO form.
    #[must_use]
    pub fn format_date(&self, date: jiff::civil::Date) -> String {
        // A single field: as a standalone name or number (narrow names, a day without the
        // locale's suffix like "日"), as `Intl.DateTimeFormat#formatToParts` gives it.
        if let Some(pattern) = single_field_pattern(&self.options)
            && let (Ok(month), Ok(day)) = (u8::try_from(date.month()), u8::try_from(date.day()))
            && let Ok(gregorian) =
                icu_calendar::Date::try_new_gregorian(i32::from(date.year()), month, day)
            && let Some(formatted) = format_pattern(self.preferences(), pattern, gregorian)
        {
            return formatted;
        }
        self.format_with(
            &date.to_datetime(jiff::civil::Time::midnight()),
            None,
            false,
        )
        .unwrap_or_else(|| date.to_string())
    }

    /// Formats a date and time according to the options (date and time options, or styles),
    /// e.g. "Mar 14, 2026, 3:09:26 PM" (medium date and time styles). Without a time option, the
    /// date only.
    #[must_use]
    pub fn format(&self, date_time: &jiff::civil::DateTime) -> String {
        self.format_with(date_time, None, true)
            .unwrap_or_else(|| date_time.to_string())
    }

    /// Formats a zoned date and time like [`Self::format`], with its time zone's name if
    /// `time_zone_name` asks for it, e.g. "3:09 PM EDT".
    #[must_use]
    pub fn format_zoned(&self, zoned: &jiff::Zoned) -> String {
        self.format_with(&zoned.datetime(), Some(zoned), true)
            .unwrap_or_else(|| zoned.to_string())
    }

    fn format_with(
        &self,
        date_time: &jiff::civil::DateTime,
        zoned: Option<&jiff::Zoned>,
        with_time: bool,
    ) -> Option<String> {
        let index = match (with_time, zoned.is_some()) {
            (false, _) => 0,
            (true, false) => 1,
            (true, true) => 2,
        };
        let formatter = self.formatters[index]
            .get_or_init(|| self.formatter(with_time, zoned.is_some()))
            .as_ref()?;
        let input = icu_input(date_time, zoned)?;
        Some(formatter.format(&input).to_string())
    }
}

/// The ICU4X input of a date and time: in its time zone, else UTC (no zone is shown then).
fn icu_input(
    date_time: &jiff::civil::DateTime,
    zoned: Option<&jiff::Zoned>,
) -> Option<
    icu_time::ZonedDateTime<
        icu_calendar::Iso,
        icu_time::TimeZoneInfo<icu_time::zone::models::AtTime>,
    >,
> {
    let date = icu_calendar::Date::try_new_iso(
        i32::from(date_time.year()),
        u8::try_from(date_time.month()).ok()?,
        u8::try_from(date_time.day()).ok()?,
    )
    .ok()?;
    let time = icu_time::Time::try_new(
        u8::try_from(date_time.hour()).ok()?,
        u8::try_from(date_time.minute()).ok()?,
        u8::try_from(date_time.second()).ok()?,
        u32::try_from(date_time.subsec_nanosecond()).ok()?,
    )
    .ok()?;
    let (id, offset) = match zoned {
        Some(zoned) => (
            zoned.time_zone().iana_name().map_or(
                icu_time::TimeZone::UNKNOWN,
                icu_time::TimeZone::from_iana_id,
            ),
            zoned.offset().seconds(),
        ),
        None => (icu_time::TimeZone::from_iana_id("Etc/UTC"), 0),
    };
    let zone = id
        .with_offset(icu_time::zone::UtcOffset::try_from_seconds(offset).ok())
        .at_date_time(icu_time::DateTime { date, time });
    Some(icu_time::ZonedDateTime { date, time, zone })
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use jiff::civil::date;

    use super::*;

    fn formatter(locale: &str, options: DateTimeFormatOptions) -> DateTimeFormatter {
        let locale = locale.parse::<Locale>().expect("a locale");
        DateTimeFormatter::new(&locale, options)
    }

    fn format(locale: &str, options: DateTimeFormatOptions) -> String {
        formatter(locale, options).format_date(date(2024, 3, 14))
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
        // The year in full (ICU4X's automatic style would shorten it in short formats).
        let numeric = DateTimeFormatOptions {
            month: Some(MonthFormat::Numeric),
            day: Some(NumericFormat::Numeric),
            year: Some(NumericFormat::Numeric),
            ..DateTimeFormatOptions::default()
        };
        assert_that!(format("en-US", numeric)).is_equal_to("3/14/2024".to_owned());
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

    /// The era shows when asked for (react-aria's calendar does for dates before Christ), not
    /// otherwise.
    #[test]
    fn shows_the_era_when_asked_for() {
        let full = DateTimeFormatOptions {
            month: Some(MonthFormat::Long),
            day: Some(NumericFormat::Numeric),
            year: Some(NumericFormat::Numeric),
            ..DateTimeFormatOptions::default()
        };
        // The proleptic year 0 is 1 BC.
        let bc = date(0, 3, 5);
        let with_era = DateTimeFormatOptions {
            era: Some(DateTimeFormat::Short),
            ..full.clone()
        };
        assert_that!(formatter("en-US", with_era).format_date(bc))
            .is_equal_to("March 5, 1 BC".to_owned());
        assert_that!(formatter("en-US", full).format_date(date(2024, 3, 14)))
            .is_equal_to("March 14, 2024".to_owned());
        let year = DateTimeFormatOptions {
            year: Some(NumericFormat::Numeric),
            era: Some(DateTimeFormat::Short),
            ..DateTimeFormatOptions::default()
        };
        assert_that!(formatter("en-US", year).format_date(bc)).is_equal_to("1 BC".to_owned());
    }

    /// Dates and times are localized as a whole: the locale's order and separators, its names
    /// (not cut), the hour cycle asked for.
    #[test]
    fn formats_dates_and_times() {
        let moment = date(2026, 3, 14).at(15, 9, 26, 0);
        let styles = |date_style, time_style| DateTimeFormatOptions {
            date_style,
            time_style,
            ..DateTimeFormatOptions::default()
        };
        assert_that!(
            formatter(
                "en-US",
                styles(Some(DateTimeStyle::Medium), Some(DateTimeStyle::Medium))
            )
            .format(&moment)
        )
        .is_equal_to("Mar 14, 2026, 3:09:26\u{202f}PM".to_owned());
        assert_that!(formatter("en-US", styles(Some(DateTimeStyle::Long), None)).format(&moment))
            .is_equal_to("March 14, 2026".to_owned());
        assert_that!(formatter("de-DE", styles(None, Some(DateTimeStyle::Short))).format(&moment))
            .is_equal_to("15:09".to_owned());

        let weekday_time = DateTimeFormatOptions {
            weekday: Some(DateTimeFormat::Short),
            hour: Some(NumericFormat::Numeric),
            minute: Some(NumericFormat::TwoDigit),
            ..DateTimeFormatOptions::default()
        };
        // A weekday without a day doesn't combine with a time: the time only.
        assert_that!(formatter("en-US", weekday_time).format(&moment))
            .is_equal_to("3:09\u{202f}PM".to_owned());
        let short_names = DateTimeFormatOptions {
            weekday: Some(DateTimeFormat::Short),
            month: Some(MonthFormat::Short),
            day: Some(NumericFormat::Numeric),
            ..DateTimeFormatOptions::default()
        };
        // Not cut to the first characters: "周六" stays.
        assert_that!(formatter("zh-CN", short_names).format(&moment).as_str()).contains("周六");
        let time = |hour_cycle| DateTimeFormatOptions {
            hour: Some(NumericFormat::Numeric),
            minute: Some(NumericFormat::Numeric),
            hour_cycle,
            ..DateTimeFormatOptions::default()
        };
        assert_that!(formatter("en-US", time(Some(HourCycle::H24))).format(&moment))
            .is_equal_to("15:09".to_owned());
        assert_that!(
            formatter("de-DE", time(Some(HourCycle::H12)))
                .format(&moment)
                .as_str()
        )
        .starts_with("3:09");
    }

    /// A 12-hour clock is the locale's (`Intl`'s `hour12: true`): Japan counts 0 to 11
    /// ("午前0:30"), not 12 to 11.
    #[test]
    fn formats_the_locales_own_twelve_hour_clock() {
        let options = DateTimeFormatOptions {
            hour: Some(NumericFormat::Numeric),
            minute: Some(NumericFormat::TwoDigit),
            hour_cycle: Some(HourCycle::H12),
            ..DateTimeFormatOptions::default()
        };
        assert_that!(formatter("ja-JP", options.clone()).format(&date(2001, 1, 1).at(0, 30, 0, 0)))
            .is_equal_to("午前0:30".to_owned());
        assert_that!(formatter("en-US", options).format(&date(2001, 1, 1).at(0, 30, 0, 0)))
            .is_equal_to("12:30\u{202f}AM".to_owned());
    }

    #[test]
    fn formats_zoned_values_with_their_time_zone() {
        let zoned = date(2024, 6, 5)
            .at(9, 30, 0, 0)
            .in_tz("America/New_York")
            .expect("a zoned value");
        let options = DateTimeFormatOptions {
            hour: Some(NumericFormat::Numeric),
            minute: Some(NumericFormat::Numeric),
            time_zone_name: Some(TimeZoneFormat::Short),
            ..DateTimeFormatOptions::default()
        };
        assert_that!(formatter("en-US", options).format_zoned(&zoned))
            .is_equal_to("9:30\u{202f}AM EDT".to_owned());
    }
}
