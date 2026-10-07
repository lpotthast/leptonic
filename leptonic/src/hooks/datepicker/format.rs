// Upstream: react-stately/src/datepicker/utils.ts @ 99e6102368 (`getFormatOptions`)
// Upstream: react-stately/src/datepicker/useDateFieldState.ts @ 99e6102368 (`processSegments`)
//! Formatting date field values with ICU4X: the whole value, and its parts in the locale's
//! order (`Intl.DateTimeFormat#formatToParts`), from which the segments are made.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
// - The options pick an ICU4X field set (react-aria: `Intl.DateTimeFormat` options from
//   `getFormatOptions`): year, month and day (or month and day, or the day) up to the
//   granularity's time precision, short (descriptions: long), the time zone's short specific name
//   unless hidden. ICU4X has no per-field options: leading zeros come from column alignment.
// - Era stripping: ICU4X adds an era to years before 1000; `Intl`'s numeric years have none
//   unless asked for, so it is removed (with its separating literal) unless the era shows
//   (`show_era`: a value before Christ).
// - `resolve_hour_cycle` finds the locale's hour cycle (h11, h12, h23, h24) by formatting 0:00
//   and 13:00 (react-aria: `Intl.DateTimeFormat#resolvedOptions().hourCycle`).
// - The parts are ICU4X's datetime parts mapped to segment types (react-stately's
//   `TYPE_MAPPING`); numbers are formatted again from the shown value (it may be an invalid date
//   such as February 30).
//
// =============================================================================

use std::fmt;

use icu_datetime::{
    DateTimeFormatter, DateTimeFormatterPreferences,
    fieldsets::{
        builder::{DateFields, FieldSetBuilder, ZoneStyle},
        enums::CompositeFieldSet,
    },
    options::{Alignment, Length, TimePrecision, YearStyle},
    preferences::{HourCycle as IcuHourCycle, NumberingSystem},
};
use jiff::tz::TimeZone;
use writeable::{Part, PartsWrite, Writeable};

use super::{
    incomplete_date::IncompleteDate,
    placeholders::placeholder,
    types::{
        DateSegment, DateSegmentType, DateValue, Granularity, HourCycle, MaxGranularity,
        ResolvedHourCycle,
    },
};
use crate::utils::{
    i18n::Locale,
    number_formatter::{NumberFormatOptions, NumberFormatter},
};

/// What a date field formats (react-stately's `FormatterOptions`).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FormatOptions {
    pub granularity: Granularity,
    pub max_granularity: MaxGranularity,
    /// The time zone of zoned values (shown unless `hide_time_zone`).
    pub time_zone: Option<TimeZone>,
    pub hide_time_zone: bool,
    pub hour_cycle: Option<HourCycle>,
    pub show_era: bool,
    pub should_force_leading_zeros: bool,
}

/// Formats date field values (react-aria's `DateFormatter` with the field's options).
pub(crate) struct DateFormatter {
    formatter: Option<DateTimeFormatter<CompositeFieldSet>>,
    /// Whether an era part shows: ICU4X adds one to years before 1000, `Intl`'s numeric years
    /// (react-aria) have none unless asked for.
    show_era: bool,
}

fn preferences(
    locale: &Locale,
    hour_cycle: Option<HourCycle>,
    latin_digits: bool,
) -> DateTimeFormatterPreferences {
    let mut prefs = DateTimeFormatterPreferences::from(locale.icu_locale());
    prefs.hour_cycle = hour_cycle.map(|hour_cycle| match hour_cycle {
        HourCycle::H12 => IcuHourCycle::H12,
        HourCycle::H24 => IcuHourCycle::H23,
    });
    if latin_digits {
        prefs.numbering_system =
            NumberingSystem::try_from(&icu_locale::extensions::unicode::value!("latn")).ok();
    }
    prefs
}

impl DateFormatter {
    pub fn new(locale: &Locale, options: &FormatOptions) -> Self {
        Self::with_length(locale, options, Length::Short)
    }

    /// For descriptions: months by name ("June 15, 2024").
    pub fn long(locale: &Locale, options: &FormatOptions) -> Self {
        Self::with_length(locale, options, Length::Long)
    }

    fn with_length(locale: &Locale, options: &FormatOptions, length: Length) -> Self {
        let time_precision = match options.granularity {
            Granularity::Day => None,
            Granularity::Hour => Some(TimePrecision::Hour),
            Granularity::Minute => Some(TimePrecision::Minute),
            Granularity::Second => Some(TimePrecision::Second),
        };
        // The date's fields from the coarsest to the day (a field always ends with the day or
        // has no date).
        let date_fields = match options.max_granularity {
            MaxGranularity::Year => Some(DateFields::YMD),
            MaxGranularity::Month => Some(DateFields::MD),
            MaxGranularity::Day => Some(DateFields::D),
            MaxGranularity::Hour | MaxGranularity::Minute | MaxGranularity::Second => None,
        };
        let mut builder = FieldSetBuilder::new();
        builder.length = Some(length);
        builder.date_fields = date_fields;
        builder.time_precision = time_precision;
        builder.year_style = date_fields
            .filter(|fields| *fields == DateFields::YMD)
            .map(|_| {
                if options.show_era {
                    YearStyle::WithEra
                } else {
                    YearStyle::Full
                }
            });
        builder.alignment = (options.should_force_leading_zeros && length == Length::Short)
            .then_some(Alignment::Column);
        builder.zone_style =
            (time_precision.is_some() && options.time_zone.is_some() && !options.hide_time_zone)
                .then_some(ZoneStyle::SpecificShort);
        let formatter = builder.build_composite().ok().and_then(|field_set| {
            DateTimeFormatter::try_new(preferences(locale, options.hour_cycle, false), field_set)
                .ok()
        });
        Self {
            formatter,
            show_era: options.show_era,
        }
    }

    /// The value formatted.
    pub fn format<V: DateValue>(&self, value: &V) -> String {
        self.format_to_parts(value)
            .into_iter()
            .map(|(_, text)| text)
            .collect()
    }

    /// The value's parts: a segment type (`None`: literal) and its text.
    pub fn format_to_parts<V: DateValue>(
        &self,
        value: &V,
    ) -> Vec<(Option<DateSegmentType>, String)> {
        let Some(formatter) = &self.formatter else {
            return vec![(None, value.date_time().to_string())];
        };
        let Some(input) = icu_input(value) else {
            return vec![(None, value.date_time().to_string())];
        };
        let mut collector = PartsCollector::default();
        let formatted = formatter.format(&input);
        if formatted.write_to_parts(&mut collector).is_err() {
            return vec![(None, value.date_time().to_string())];
        }
        let mut parts = collector.into_parts();
        if !self.show_era
            && let Some(index) = parts
                .iter()
                .position(|(kind, _)| *kind == Some(DateSegmentType::Era))
        {
            parts.remove(index);
            // With the literal separating it from the year.
            if index > 0 && parts.get(index - 1).is_some_and(|(kind, _)| kind.is_none()) {
                parts.remove(index - 1);
            } else if parts.get(index).is_some_and(|(kind, _)| kind.is_none()) {
                parts.remove(index);
            }
        }
        parts
    }
}

/// The ICU4X input of a value: its date, time and time zone (UTC without one).
fn icu_input<V: DateValue>(
    value: &V,
) -> Option<
    icu_time::ZonedDateTime<
        icu_calendar::Iso,
        icu_time::TimeZoneInfo<icu_time::zone::models::AtTime>,
    >,
> {
    let date_time = value.date_time();
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
    let iso = icu_time::DateTime { date, time };
    let (id, offset) = match (value.time_zone(), value.offset()) {
        (Some(time_zone), Some(offset)) => (
            time_zone.iana_name().map_or(
                icu_time::TimeZone::UNKNOWN,
                icu_time::TimeZone::from_iana_id,
            ),
            offset.seconds(),
        ),
        _ => (icu_time::TimeZone::from_iana_id("Etc/UTC"), 0),
    };
    let zone = id
        .with_offset(icu_time::zone::UtcOffset::try_from_seconds(offset).ok())
        .at_date_time(iso);
    Some(icu_time::ZonedDateTime { date, time, zone })
}

/// Collects the datetime parts of formatted text.
#[derive(Default)]
struct PartsCollector {
    text: String,
    /// Byte ranges of the outermost datetime parts.
    parts: Vec<(usize, usize, Part)>,
}

impl fmt::Write for PartsCollector {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.text.push_str(s);
        Ok(())
    }
}

impl PartsWrite for PartsCollector {
    type SubPartsWrite = Self;

    fn with_part(
        &mut self,
        part: Part,
        mut f: impl FnMut(&mut Self) -> fmt::Result,
    ) -> fmt::Result {
        let start = self.text.len();
        f(self)?;
        // Datetime parts only (numbers inside them have parts of their own).
        if part.category == icu_datetime::parts::YEAR.category {
            self.parts.push((start, self.text.len(), part));
        }
        Ok(())
    }
}

impl PartsCollector {
    fn into_parts(mut self) -> Vec<(Option<DateSegmentType>, String)> {
        self.parts.sort_by_key(|(start, ..)| *start);
        let mut result = Vec::new();
        let mut position = 0;
        for (start, end, part) in self.parts {
            if start < position {
                continue;
            }
            if start > position {
                result.push((None, self.text[position..start].to_owned()));
            }
            result.push((segment_type(part), self.text[start..end].to_owned()));
            position = end;
        }
        if position < self.text.len() {
            result.push((None, self.text[position..].to_owned()));
        }
        result
    }
}

/// The segment type of a part (react-stately's `TYPE_MAPPING`; `None`: a literal).
fn segment_type(part: Part) -> Option<DateSegmentType> {
    use icu_datetime::parts;
    Some(match part {
        part if part == parts::ERA => DateSegmentType::Era,
        part if part == parts::YEAR
            || part == parts::RELATED_YEAR
            || part == parts::EXTENDED_YEAR =>
        {
            DateSegmentType::Year
        }
        part if part == parts::MONTH => DateSegmentType::Month,
        part if part == parts::DAY => DateSegmentType::Day,
        part if part == parts::DAY_PERIOD => DateSegmentType::DayPeriod,
        part if part == parts::HOUR => DateSegmentType::Hour,
        part if part == parts::MINUTE => DateSegmentType::Minute,
        part if part == parts::SECOND => DateSegmentType::Second,
        part if part == parts::TIME_ZONE_NAME => DateSegmentType::TimeZoneName,
        // Year names, weekdays, ...: not editable.
        _ => return None,
    })
}

/// The locale's hour cycle, or the preferred one (`Intl`'s resolved `hourCycle`).
pub(crate) fn resolve_hour_cycle(
    locale: &Locale,
    preference: Option<HourCycle>,
) -> ResolvedHourCycle {
    let mut builder = FieldSetBuilder::new();
    builder.length = Some(Length::Short);
    builder.time_precision = Some(TimePrecision::Hour);
    let Some(formatter) = builder.build_composite().ok().and_then(|field_set| {
        DateTimeFormatter::try_new(preferences(locale, preference, true), field_set).ok()
    }) else {
        return ResolvedHourCycle::H23;
    };
    let formatter = DateFormatter {
        formatter: Some(formatter),
        show_era: false,
    };
    let parts =
        |hour: i8| formatter.format_to_parts(&jiff::civil::date(2001, 1, 1).at(hour, 0, 0, 0));
    let hour_text = |parts: &[(Option<DateSegmentType>, String)]| {
        parts
            .iter()
            .find(|(kind, _)| *kind == Some(DateSegmentType::Hour))
            .and_then(|(_, text)| text.trim().parse::<i32>().ok())
    };
    let afternoon = parts(13);
    let midnight = hour_text(&parts(0));
    let twelve_hour = afternoon
        .iter()
        .any(|(kind, _)| *kind == Some(DateSegmentType::DayPeriod))
        || hour_text(&afternoon) == Some(1);
    match (twelve_hour, midnight) {
        (true, Some(0)) => ResolvedHourCycle::H11,
        (true, _) => ResolvedHourCycle::H12,
        (false, Some(24)) => ResolvedHourCycle::H24,
        (false, _) => ResolvedHourCycle::H23,
    }
}

/// The segments of a date field (react-stately's `processSegments`): the parts of `date_value`
/// (the display value completed by a placeholder), the numbers taken from the display value
/// itself (they may form an invalid date, e.g. February 30), placeholders for missing values,
/// and the time isolated left-to-right (in right-to-left locales, it'd read minute:hour).
pub(crate) fn segments<V: DateValue>(
    date_value: &V,
    display_value: &IncompleteDate,
    formatter: &DateFormatter,
    locale: &Locale,
    granularity: Granularity,
) -> Vec<DateSegment> {
    let parts = formatter.format_to_parts(date_value);
    let number = NumberFormatter::new(
        locale,
        NumberFormatOptions {
            use_grouping: false,
            ..NumberFormatOptions::default()
        },
    );
    let two_digits = NumberFormatter::new(
        locale,
        NumberFormatOptions {
            use_grouping: false,
            minimum_integer_digits: Some(2),
            ..NumberFormatOptions::default()
        },
    );
    let mut segments = Vec::new();
    for (kind, text) in parts {
        let Some(kind) = kind else {
            segments.push(DateSegment::literal(text));
            continue;
        };
        let text = match kind {
            DateSegmentType::Year
            | DateSegmentType::Month
            | DateSegmentType::Day
            | DateSegmentType::Hour => {
                let value = display_value.get(kind).unwrap_or(0);
                // As the locale pads it (e.g. "05.06.2024").
                let padded = text.chars().count() >= 2 && kind != DateSegmentType::Year;
                if padded {
                    two_digits.format(value)
                } else {
                    number.format(value)
                }
            }
            _ => text,
        };
        let is_editable = kind.is_editable();
        let is_placeholder = is_editable && display_value.get(kind).is_none();
        let placeholder = if is_editable {
            placeholder(kind, &text, locale)
        } else {
            String::new()
        };
        let limits = display_value.segment_limits(kind);
        let segment = DateSegment {
            kind,
            text: if is_placeholder {
                placeholder.clone()
            } else {
                text
            },
            value: limits.and_then(|limits| limits.value),
            min_value: limits.map(|limits| limits.min_value),
            max_value: limits.map(|limits| limits.max_value),
            is_placeholder,
            placeholder,
            is_editable,
        };
        let is_time = matches!(
            kind,
            DateSegmentType::Hour | DateSegmentType::Minute | DateSegmentType::Second
        );
        if kind == DateSegmentType::Hour {
            segments.push(DateSegment::literal("\u{2066}"));
            segments.push(segment);
            if granularity == Granularity::Hour {
                segments.push(DateSegment::literal("\u{2069}"));
            }
        } else if is_time && kind == granularity.segment() {
            segments.push(segment);
            segments.push(DateSegment::literal("\u{2069}"));
        } else {
            segments.push(segment);
        }
    }
    segments
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use jiff::civil::date;

    use super::*;

    fn locale(id: &str) -> Locale {
        id.parse().expect("a locale")
    }

    fn options(granularity: Granularity) -> FormatOptions {
        FormatOptions {
            granularity,
            max_granularity: MaxGranularity::Year,
            time_zone: None,
            hide_time_zone: false,
            hour_cycle: None,
            show_era: false,
            should_force_leading_zeros: false,
        }
    }

    fn kinds(parts: &[(Option<DateSegmentType>, String)]) -> Vec<Option<DateSegmentType>> {
        parts.iter().map(|(kind, _)| *kind).collect()
    }

    #[test]
    fn formats_parts_in_the_locales_order() {
        let value = date(2024, 6, 5);
        let us = DateFormatter::new(&locale("en-US"), &options(Granularity::Day));
        assert_that!(us.format(&value)).is_equal_to("6/5/2024".to_owned());
        assert_that!(kinds(&us.format_to_parts(&value))).is_equal_to(vec![
            Some(DateSegmentType::Month),
            None,
            Some(DateSegmentType::Day),
            None,
            Some(DateSegmentType::Year),
        ]);
        let de = DateFormatter::new(&locale("de-DE"), &options(Granularity::Day));
        assert_that!(de.format(&value)).is_equal_to("05.06.2024".to_owned());
        let time = DateFormatter::new(&locale("en-US"), &options(Granularity::Minute));
        assert_that!(time.format(&value.at(13, 5, 0, 0)))
            .is_equal_to("6/5/2024, 1:05\u{202f}PM".to_owned());
    }

    #[test]
    fn forces_leading_zeros() {
        let mut options = options(Granularity::Minute);
        options.should_force_leading_zeros = true;
        let formatter = DateFormatter::new(&locale("en-US"), &options);
        assert_that!(formatter.format(&date(2024, 6, 5).at(9, 5, 0, 0)))
            .is_equal_to("06/05/2024, 09:05\u{202f}AM".to_owned());
    }

    #[test]
    fn shows_eras_only_when_asked_for() {
        let formatter = DateFormatter::new(&locale("en-US"), &options(Granularity::Day));
        assert_that!(formatter.format(&date(202, 6, 5))).is_equal_to("6/5/202".to_owned());
        let mut with_era = options(Granularity::Day);
        with_era.show_era = true;
        let formatter = DateFormatter::new(&locale("en-US"), &with_era);
        assert_that!(formatter.format(&date(0, 6, 5)).as_str()).contains("BC");
    }

    #[test]
    fn resolves_hour_cycles() {
        assert_that!(resolve_hour_cycle(&locale("en-US"), None))
            .is_equal_to(ResolvedHourCycle::H12);
        assert_that!(resolve_hour_cycle(&locale("de-DE"), None))
            .is_equal_to(ResolvedHourCycle::H23);
        assert_that!(resolve_hour_cycle(&locale("en-US"), Some(HourCycle::H24)))
            .is_equal_to(ResolvedHourCycle::H23);
        assert_that!(resolve_hour_cycle(&locale("de-DE"), Some(HourCycle::H12)))
            .is_equal_to(ResolvedHourCycle::H12);
    }

    #[test]
    fn makes_segments_with_placeholders() {
        let value = date(2024, 6, 5);
        let en = locale("en-US");
        let formatter = DateFormatter::new(&en, &options(Granularity::Day));
        let display =
            IncompleteDate::new(ResolvedHourCycle::H12, Some(&value)).clear(DateSegmentType::Day);
        let segments = segments(&value, &display, &formatter, &en, Granularity::Day);
        let texts: Vec<&str> = segments
            .iter()
            .map(|segment| segment.text.as_str())
            .collect();
        assert_that!(texts).is_equal_to(vec!["6", "/", "dd", "/", "2024"]);
        assert_that!(segments[2].is_placeholder).is_true();
        assert_that!(segments[0].max_value).is_equal_to(Some(12));
    }
}
