// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/i18n/src/useNumberFormatter.ts
// and https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/i18n/src/useDateFormatter.ts

/// Number formatting options.
#[derive(Debug, Clone, Default)]
pub struct NumberFormatOptions {
    /// The formatting style. Default is "decimal".
    pub style: NumberStyle,

    /// The currency to use in currency formatting.
    pub currency: Option<String>,

    /// How to display the currency. Default is "symbol".
    pub currency_display: CurrencyDisplay,

    /// Whether to use grouping separators (e.g., thousands separators).
    pub use_grouping: bool,

    /// The minimum number of integer digits to use.
    pub minimum_integer_digits: Option<u32>,

    /// The minimum number of fraction digits to use.
    pub minimum_fraction_digits: Option<u32>,

    /// The maximum number of fraction digits to use.
    pub maximum_fraction_digits: Option<u32>,

    /// The minimum number of significant digits to use.
    pub minimum_significant_digits: Option<u32>,

    /// The maximum number of significant digits to use.
    pub maximum_significant_digits: Option<u32>,

    /// The unit to use in unit formatting.
    pub unit: Option<String>,

    /// How to display the unit. Default is "short".
    pub unit_display: UnitDisplay,

    /// How to display the sign. Default is "auto".
    pub sign_display: SignDisplay,

    /// The notation to use. Default is "standard".
    pub notation: Notation,

    /// How to display compact notation. Default is "short".
    pub compact_display: CompactDisplay,
}

/// Number formatting styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NumberStyle {
    /// Decimal number formatting.
    #[default]
    Decimal,
    /// Currency formatting.
    Currency,
    /// Percent formatting.
    Percent,
    /// Unit formatting.
    Unit,
}

/// Currency display options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CurrencyDisplay {
    /// Currency symbol (e.g., "$").
    #[default]
    Symbol,
    /// Narrow symbol (e.g., "$" instead of "US$").
    NarrowSymbol,
    /// Currency code (e.g., "USD").
    Code,
    /// Localized currency name (e.g., "US dollar").
    Name,
}

/// Unit display options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnitDisplay {
    /// Short unit (e.g., "16 l").
    #[default]
    Short,
    /// Narrow unit (e.g., "16l").
    Narrow,
    /// Long unit (e.g., "16 liters").
    Long,
}

/// Sign display options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SignDisplay {
    /// Sign displayed for negative numbers only.
    #[default]
    Auto,
    /// Sign always displayed.
    Always,
    /// Sign displayed only when value differs from zero.
    ExceptZero,
    /// Sign never displayed.
    Never,
}

/// Number notation options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Notation {
    /// Standard notation.
    #[default]
    Standard,
    /// Scientific notation (e.g., "1.23E4").
    Scientific,
    /// Engineering notation (e.g., "12.3E3").
    Engineering,
    /// Compact notation (e.g., "12K").
    Compact,
}

/// Compact display options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompactDisplay {
    /// Short compact display (e.g., "12K").
    #[default]
    Short,
    /// Long compact display (e.g., "12 thousand").
    Long,
}

/// A simple number formatter.
///
/// Note: This is a simplified implementation. For full locale-aware formatting,
/// consider using a dedicated i18n library like `icu4x` or `fluent`.
#[derive(Debug, Clone)]
pub struct NumberFormatter {
    _locale: String,
    options: NumberFormatOptions,
}

impl NumberFormatter {
    /// Creates a new number formatter with the given locale and options.
    #[must_use]
    pub fn new(locale: impl Into<String>, options: NumberFormatOptions) -> Self {
        Self {
            _locale: locale.into(),
            options,
        }
    }

    /// Formats a number according to the formatter's options.
    #[must_use]
    pub fn format(&self, value: f64) -> String {
        match self.options.style {
            NumberStyle::Percent => self.format_percent(value),
            NumberStyle::Currency => self.format_currency(value),
            NumberStyle::Unit => self.format_unit(value),
            NumberStyle::Decimal => self.format_decimal(value),
        }
    }

    /// Formats a number as a decimal.
    #[must_use]
    pub fn format_decimal(&self, value: f64) -> String {
        let min_frac = self.options.minimum_fraction_digits.unwrap_or(0);
        let max_frac = self.options.maximum_fraction_digits.unwrap_or(3);

        let formatted = if max_frac == 0 {
            format!("{value:.0}")
        } else {
            // Format with max digits, then trim trailing zeros but keep min
            let prec = max_frac as usize;
            let full = format!("{value:.prec$}");
            self.trim_fraction_digits(&full, min_frac as usize)
        };

        if self.options.use_grouping {
            self.add_grouping_separators(&formatted)
        } else {
            formatted
        }
    }

    /// Formats a number as a percentage.
    #[must_use]
    pub fn format_percent(&self, value: f64) -> String {
        let percent_value = value * 100.0;
        let min_frac = self.options.minimum_fraction_digits.unwrap_or(0);
        let max_frac = self.options.maximum_fraction_digits.unwrap_or(0);

        let formatted = if max_frac == 0 {
            format!("{percent_value:.0}")
        } else {
            let prec = max_frac as usize;
            let full = format!("{percent_value:.prec$}");
            self.trim_fraction_digits(&full, min_frac as usize)
        };

        format!("{formatted}%")
    }

    /// Formats a number as currency.
    #[must_use]
    pub fn format_currency(&self, value: f64) -> String {
        let currency = self.options.currency.as_deref().unwrap_or("USD");
        let min_frac = self.options.minimum_fraction_digits.unwrap_or(2);
        let max_frac = self.options.maximum_fraction_digits.unwrap_or(2);

        let abs_value = value.abs();
        let prec = max_frac as usize;
        let formatted = format!("{abs_value:.prec$}");
        let formatted = self.trim_fraction_digits(&formatted, min_frac as usize);
        let formatted = if self.options.use_grouping {
            self.add_grouping_separators(&formatted)
        } else {
            formatted
        };

        let sign = if value < 0.0 { "-" } else { "" };
        let symbol = self.get_currency_symbol(currency);

        match self.options.currency_display {
            CurrencyDisplay::Code => format!("{sign}{currency} {formatted}"),
            CurrencyDisplay::Name => {
                let name = self.get_currency_name(currency);
                format!("{sign}{formatted} {name}")
            }
            CurrencyDisplay::Symbol | CurrencyDisplay::NarrowSymbol => {
                format!("{sign}{symbol}{formatted}")
            }
        }
    }

    /// Formats a number with a unit.
    #[must_use]
    pub fn format_unit(&self, value: f64) -> String {
        let unit = self.options.unit.as_deref().unwrap_or("unit");
        let formatted = self.format_decimal(value);

        match self.options.unit_display {
            UnitDisplay::Narrow => format!("{formatted}{unit}"),
            UnitDisplay::Short => format!("{formatted} {unit}"),
            UnitDisplay::Long => format!("{formatted} {unit}s"),
        }
    }

    #[allow(clippy::unused_self)]
    fn trim_fraction_digits(&self, s: &str, min_digits: usize) -> String {
        if let Some(dot_pos) = s.find('.') {
            let (integer, fraction) = s.split_at(dot_pos);
            let fraction = &fraction[1..]; // Remove the dot

            if min_digits == 0 && fraction.chars().all(|c| c == '0') {
                return integer.to_string();
            }

            let trimmed = fraction.trim_end_matches('0');
            let final_frac = if trimmed.len() < min_digits {
                format!("{trimmed:0<min_digits$}")
            } else {
                trimmed.to_string()
            };

            if final_frac.is_empty() {
                integer.to_string()
            } else {
                format!("{integer}.{final_frac}")
            }
        } else {
            s.to_string()
        }
    }

    #[allow(clippy::unused_self)]
    fn add_grouping_separators(&self, s: &str) -> String {
        let (integer, fraction) = if let Some(dot_pos) = s.find('.') {
            let (int, frac) = s.split_at(dot_pos);
            (int, Some(frac))
        } else {
            (s, None)
        };

        let (sign, digits) = if let Some(stripped) = integer.strip_prefix('-') {
            ("-", stripped)
        } else {
            ("", integer)
        };

        // Build grouped string by iterating in reverse and inserting commas
        let mut result = String::new();
        for (i, c) in digits.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                result.push(',');
            }
            result.push(c);
        }
        let grouped: String = result.chars().rev().collect();

        match fraction {
            Some(frac) => format!("{sign}{grouped}{frac}"),
            None => format!("{sign}{grouped}"),
        }
    }

    #[allow(clippy::unused_self)]
    fn get_currency_symbol(&self, currency: &str) -> &'static str {
        match currency {
            "EUR" => "€",
            "GBP" => "£",
            "JPY" | "CNY" => "¥",
            "KRW" => "₩",
            "INR" => "₹",
            "RUB" => "₽",
            "BRL" => "R$",
            "CHF" => "CHF",
            "CAD" => "CA$",
            "AUD" => "A$",
            _ => "$",
        }
    }

    #[allow(clippy::unused_self)]
    fn get_currency_name(&self, currency: &str) -> &'static str {
        match currency {
            "USD" => "US dollars",
            "EUR" => "euros",
            "GBP" => "British pounds",
            "JPY" => "Japanese yen",
            "CNY" => "Chinese yuan",
            _ => "dollars",
        }
    }
}

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

/// A simple date/time formatter.
///
/// Note: This is a simplified implementation. For full locale-aware formatting,
/// consider using a dedicated i18n library like `icu4x` or `chrono`.
#[derive(Debug, Clone)]
pub struct DateTimeFormatter {
    _locale: String,
    options: DateTimeFormatOptions,
}

impl DateTimeFormatter {
    /// Creates a new date/time formatter with the given locale and options.
    #[must_use]
    pub fn new(locale: impl Into<String>, options: DateTimeFormatOptions) -> Self {
        Self {
            _locale: locale.into(),
            options,
        }
    }

    /// Formats a date/time according to the formatter's options.
    #[must_use]
    pub fn format(&self, dt: &time::OffsetDateTime) -> String {
        // Use date_style/time_style if provided
        if let Some(date_style) = self.options.date_style {
            let time_part = self
                .options
                .time_style
                .map(|ts| format!(" {}", self.format_time_style(dt, ts)))
                .unwrap_or_default();
            return format!("{}{time_part}", self.format_date_style(dt, date_style));
        }

        if let Some(time_style) = self.options.time_style {
            return self.format_time_style(dt, time_style);
        }

        // Build custom format from individual options
        let mut parts = Vec::new();

        if let Some(weekday) = self.options.weekday {
            parts.push(self.format_weekday(dt, weekday));
        }

        if let Some(month) = self.options.month {
            parts.push(self.format_month(dt, month));
        }

        if let Some(day) = self.options.day {
            parts.push(self.format_day(dt, day));
        }

        if let Some(year) = self.options.year {
            parts.push(self.format_year(dt, year));
        }

        if let Some(hour) = self.options.hour {
            let minute_part = self
                .options
                .minute
                .map(|m| format!(":{}", self.format_minute(dt, m)))
                .unwrap_or_default();
            let second_part = self
                .options
                .second
                .map(|s| format!(":{}", self.format_second(dt, s)))
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

    fn format_date_style(&self, dt: &time::OffsetDateTime, style: DateTimeStyle) -> String {
        match style {
            DateTimeStyle::Full => {
                let weekday = self.weekday_name(dt.weekday(), false);
                let month = self.month_name(dt.month(), false);
                format!("{weekday}, {month} {}, {}", dt.day(), dt.year())
            }
            DateTimeStyle::Long => {
                let month = self.month_name(dt.month(), false);
                format!("{month} {}, {}", dt.day(), dt.year())
            }
            DateTimeStyle::Medium => {
                let month = self.month_name(dt.month(), true);
                format!("{month} {}, {}", dt.day(), dt.year())
            }
            DateTimeStyle::Short => {
                format!("{}/{}/{}", dt.month() as u8, dt.day(), dt.year() % 100)
            }
        }
    }

    fn format_time_style(&self, dt: &time::OffsetDateTime, style: DateTimeStyle) -> String {
        let hour12 = self.options.hour12.unwrap_or(true);
        let (hour, am_pm) = if hour12 {
            let h = dt.hour();
            let am_pm = if h < 12 { "AM" } else { "PM" };
            let h = if h == 0 {
                12
            } else if h > 12 {
                h - 12
            } else {
                h
            };
            (h, Some(am_pm))
        } else {
            (dt.hour(), None)
        };

        let am_pm_str = am_pm.map(|s| format!(" {s}")).unwrap_or_default();

        match style {
            DateTimeStyle::Full | DateTimeStyle::Long => {
                let tz = self.options.time_zone.as_deref().unwrap_or("UTC");
                format!(
                    "{hour}:{:02}:{:02}{am_pm_str} {tz}",
                    dt.minute(),
                    dt.second()
                )
            }
            DateTimeStyle::Medium => {
                format!("{hour}:{:02}:{:02}{am_pm_str}", dt.minute(), dt.second())
            }
            DateTimeStyle::Short => {
                format!("{hour}:{:02}{am_pm_str}", dt.minute())
            }
        }
    }

    fn format_weekday(&self, dt: &time::OffsetDateTime, format: DateTimeFormat) -> String {
        match format {
            DateTimeFormat::Long => self.weekday_name(dt.weekday(), false),
            DateTimeFormat::Short => self.weekday_name(dt.weekday(), true),
            DateTimeFormat::Narrow => self.weekday_name(dt.weekday(), true)[..1].to_string(),
        }
    }

    fn format_month(&self, dt: &time::OffsetDateTime, format: MonthFormat) -> String {
        match format {
            MonthFormat::Numeric => format!("{}", dt.month() as u8),
            MonthFormat::TwoDigit => format!("{:02}", dt.month() as u8),
            MonthFormat::Long => self.month_name(dt.month(), false),
            MonthFormat::Short => self.month_name(dt.month(), true),
            MonthFormat::Narrow => self.month_name(dt.month(), true)[..1].to_string(),
        }
    }

    #[allow(clippy::unused_self)]
    fn format_day(&self, dt: &time::OffsetDateTime, format: NumericFormat) -> String {
        match format {
            NumericFormat::Numeric => format!("{}", dt.day()),
            NumericFormat::TwoDigit => format!("{:02}", dt.day()),
        }
    }

    #[allow(clippy::unused_self)]
    fn format_year(&self, dt: &time::OffsetDateTime, format: NumericFormat) -> String {
        match format {
            NumericFormat::Numeric => format!("{}", dt.year()),
            NumericFormat::TwoDigit => format!("{:02}", dt.year() % 100),
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

    #[allow(clippy::unused_self)]
    fn format_minute(&self, dt: &time::OffsetDateTime, format: NumericFormat) -> String {
        match format {
            NumericFormat::Numeric => format!("{}", dt.minute()),
            NumericFormat::TwoDigit => format!("{:02}", dt.minute()),
        }
    }

    #[allow(clippy::unused_self)]
    fn format_second(&self, dt: &time::OffsetDateTime, format: NumericFormat) -> String {
        match format {
            NumericFormat::Numeric => format!("{}", dt.second()),
            NumericFormat::TwoDigit => format!("{:02}", dt.second()),
        }
    }

    #[allow(clippy::unused_self)]
    fn weekday_name(&self, weekday: time::Weekday, short: bool) -> String {
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

    #[allow(clippy::unused_self)]
    fn month_name(&self, month: time::Month, short: bool) -> String {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_formatter_decimal() {
        let formatter = NumberFormatter::new("en-US", NumberFormatOptions::default());
        assert_eq!(formatter.format(1234.567), "1234.567");
    }

    #[test]
    fn test_number_formatter_percent() {
        let formatter = NumberFormatter::new(
            "en-US",
            NumberFormatOptions {
                style: NumberStyle::Percent,
                ..Default::default()
            },
        );
        assert_eq!(formatter.format(0.75), "75%");
    }

    #[test]
    fn test_number_formatter_currency() {
        let formatter = NumberFormatter::new(
            "en-US",
            NumberFormatOptions {
                style: NumberStyle::Currency,
                currency: Some("USD".to_string()),
                use_grouping: true,
                ..Default::default()
            },
        );
        assert_eq!(formatter.format(1234.56), "$1,234.56");
    }

    #[test]
    fn test_number_formatter_grouping() {
        let formatter = NumberFormatter::new(
            "en-US",
            NumberFormatOptions {
                use_grouping: true,
                maximum_fraction_digits: Some(0),
                ..Default::default()
            },
        );
        assert_eq!(formatter.format(1234567.0), "1,234,567");
    }
}
