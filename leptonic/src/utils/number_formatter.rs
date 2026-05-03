// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/i18n/src/useNumberFormatter.ts

use icu_decimal::{DecimalFormatter, options::DecimalFormatterOptions};
use icu_locale::Locale as IcuLocale;

use super::i18n::Locale;

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

/// A locale-aware number formatter backed by ICU4X.
///
/// Uses `icu_decimal::DecimalFormatter` for locale-aware grouping
/// and decimal separators. Currency symbol lookup uses a minimal built-in table
/// (to be replaced by ICU4X currency support when available).
#[derive(Debug, Clone)]
pub struct NumberFormatter {
    locale: IcuLocale,
    options: NumberFormatOptions,
}

impl NumberFormatter {
    /// Creates a new number formatter with the given locale and options.
    #[must_use]
    pub fn new(locale: &Locale, options: NumberFormatOptions) -> Self {
        Self {
            locale: locale.icu_locale().clone(),
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

    /// Formats a number as a decimal with locale-aware separators.
    #[must_use]
    pub fn format_decimal(&self, value: f64) -> String {
        let min_frac = self.options.minimum_fraction_digits.unwrap_or(0);
        let max_frac = self.options.maximum_fraction_digits.unwrap_or(3);

        let raw = if max_frac == 0 {
            format!("{value:.0}")
        } else {
            let prec = max_frac as usize;
            let full = format!("{value:.prec$}");
            trim_fraction_digits(&full, min_frac as usize)
        };

        if self.options.use_grouping {
            self.format_with_icu_grouping(&raw)
        } else {
            self.format_with_icu_no_grouping(&raw)
        }
    }

    /// Formats a number as a percentage.
    #[must_use]
    pub fn format_percent(&self, value: f64) -> String {
        let percent_value = value * 100.0;
        let min_frac = self.options.minimum_fraction_digits.unwrap_or(0);
        let max_frac = self.options.maximum_fraction_digits.unwrap_or(0);

        let raw = if max_frac == 0 {
            format!("{percent_value:.0}")
        } else {
            let prec = max_frac as usize;
            let full = format!("{percent_value:.prec$}");
            trim_fraction_digits(&full, min_frac as usize)
        };

        let formatted = if self.options.use_grouping {
            self.format_with_icu_grouping(&raw)
        } else {
            self.format_with_icu_no_grouping(&raw)
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
        let raw = format!("{abs_value:.prec$}");
        let raw = trim_fraction_digits(&raw, min_frac as usize);

        let formatted = if self.options.use_grouping {
            self.format_with_icu_grouping(&raw)
        } else {
            self.format_with_icu_no_grouping(&raw)
        };

        let sign = if value < 0.0 { "-" } else { "" };
        let symbol = get_currency_symbol(currency);

        match self.options.currency_display {
            CurrencyDisplay::Code => format!("{sign}{currency}\u{a0}{formatted}"),
            CurrencyDisplay::Name => {
                let name = get_currency_name(currency);
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

    /// Uses ICU4X `DecimalFormatter` to apply locale-aware grouping and decimal separators.
    fn format_with_icu_grouping(&self, raw_number: &str) -> String {
        let mut options = DecimalFormatterOptions::default();
        options.grouping_strategy = Some(icu_decimal::options::GroupingStrategy::Auto);
        self.format_with_icu(raw_number, options)
    }

    /// Uses ICU4X `DecimalFormatter` to apply locale-aware decimal separator without grouping.
    fn format_with_icu_no_grouping(&self, raw_number: &str) -> String {
        let mut options = DecimalFormatterOptions::default();
        options.grouping_strategy = Some(icu_decimal::options::GroupingStrategy::Never);
        self.format_with_icu(raw_number, options)
    }

    fn format_with_icu(&self, raw_number: &str, options: DecimalFormatterOptions) -> String {
        let prefs = icu_decimal::DecimalFormatterPreferences::from(&self.locale);

        let Ok(formatter) = DecimalFormatter::try_new(prefs, options) else {
            return raw_number.to_string();
        };

        // Parse the raw number string into a Decimal.
        // raw_number is something like "1234.56" or "-1234" (always with '.' as decimal separator).
        let Ok(decimal) = raw_number.parse::<icu_decimal::input::Decimal>() else {
            return raw_number.to_string();
        };

        formatter.format(&decimal).to_string()
    }
}

fn trim_fraction_digits(s: &str, min_digits: usize) -> String {
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

fn get_currency_symbol(currency: &str) -> &'static str {
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

fn get_currency_name(currency: &str) -> &'static str {
    match currency {
        "USD" => "US dollars",
        "EUR" => "euros",
        "GBP" => "British pounds",
        "JPY" => "Japanese yen",
        "CNY" => "Chinese yuan",
        _ => "dollars",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_formatter_decimal() {
        let locale = Locale::new("en-US");
        let formatter = NumberFormatter::new(&locale, NumberFormatOptions::default());
        assert_eq!(formatter.format(1234.567), "1234.567");
    }

    #[test]
    fn test_number_formatter_percent() {
        let locale = Locale::new("en-US");
        let formatter = NumberFormatter::new(
            &locale,
            NumberFormatOptions {
                style: NumberStyle::Percent,
                ..Default::default()
            },
        );
        assert_eq!(formatter.format(0.75), "75%");
    }

    #[test]
    fn test_number_formatter_currency() {
        let locale = Locale::new("en-US");
        let formatter = NumberFormatter::new(
            &locale,
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
        let locale = Locale::new("en-US");
        let formatter = NumberFormatter::new(
            &locale,
            NumberFormatOptions {
                use_grouping: true,
                maximum_fraction_digits: Some(0),
                ..Default::default()
            },
        );
        assert_eq!(formatter.format(1_234_567.0), "1,234,567");
    }

    #[test]
    fn test_number_formatter_german_locale() {
        let locale = Locale::new("de-DE");
        let formatter = NumberFormatter::new(
            &locale,
            NumberFormatOptions {
                use_grouping: true,
                maximum_fraction_digits: Some(2),
                minimum_fraction_digits: Some(2),
                ..Default::default()
            },
        );
        // German uses '.' for grouping and ',' for decimal
        assert_eq!(formatter.format(1234.56), "1.234,56");
    }

    #[test]
    fn test_number_formatter_no_grouping_german() {
        let locale = Locale::new("de-DE");
        let formatter = NumberFormatter::new(
            &locale,
            NumberFormatOptions {
                use_grouping: false,
                maximum_fraction_digits: Some(2),
                minimum_fraction_digits: Some(2),
                ..Default::default()
            },
        );
        // German uses ',' for decimal, no grouping
        assert_eq!(formatter.format(1234.56), "1234,56");
    }
}
