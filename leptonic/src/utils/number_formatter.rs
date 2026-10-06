// Upstream: react-aria/src/i18n/useNumberFormatter.ts @ 6f664fe911
// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/react-aria/src/i18n/useNumberFormatter.ts

use fixed_decimal::{Decimal, Sign, SignedRoundingMode, UnsignedRoundingMode};
use icu_decimal::{DecimalFormatter, options::DecimalFormatterOptions};
use icu_locale::Locale as IcuLocale;

use super::{i18n::Locale, number_value::NumberValue};

/// Number formatting options (as `Intl.NumberFormatOptions`).
#[derive(Debug, Clone, PartialEq)]
pub struct NumberFormatOptions {
    /// The formatting style. Default is "decimal".
    pub style: NumberStyle,

    /// The currency to use in currency formatting.
    pub currency: Option<String>,

    /// How to display the currency. Default is "symbol".
    pub currency_display: CurrencyDisplay,

    /// Whether to use grouping separators (e.g., thousands separators). Default: `true`.
    pub use_grouping: bool,

    /// The minimum number of integer digits to use (padded with zeros).
    pub minimum_integer_digits: Option<u32>,

    /// The minimum number of fraction digits to use.
    pub minimum_fraction_digits: Option<u32>,

    /// The maximum number of fraction digits to use.
    pub maximum_fraction_digits: Option<u32>,

    /// The minimum number of significant digits to use. With either significant digit option,
    /// the fraction digit options are ignored (as `Intl.NumberFormat`).
    pub minimum_significant_digits: Option<u32>,

    /// The maximum number of significant digits to use.
    pub maximum_significant_digits: Option<u32>,

    /// The unit to use in unit formatting.
    pub unit: Option<String>,

    /// How to display the unit. Default is "short".
    pub unit_display: UnitDisplay,

    /// How to display the sign. Default is "auto".
    pub sign_display: SignDisplay,
}

impl Default for NumberFormatOptions {
    fn default() -> Self {
        Self {
            style: NumberStyle::default(),
            currency: None,
            currency_display: CurrencyDisplay::default(),
            use_grouping: true,
            minimum_integer_digits: None,
            minimum_fraction_digits: None,
            maximum_fraction_digits: None,
            minimum_significant_digits: None,
            maximum_significant_digits: None,
            unit: None,
            unit_display: UnitDisplay::default(),
            sign_display: SignDisplay::default(),
        }
    }
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

    /// Formats a number according to the formatter's options. Empty for infinite and NaN floats.
    #[must_use]
    pub fn format<T: NumberValue>(&self, value: T) -> String {
        let Some(decimal) = value.to_decimal() else {
            return String::new();
        };
        match self.options.style {
            NumberStyle::Percent => self.format_percent(decimal),
            NumberStyle::Currency => self.format_currency(decimal),
            NumberStyle::Unit => self.format_unit(decimal),
            NumberStyle::Decimal => self.format_decimal(decimal),
        }
    }

    /// Applies the digit options: significant digits if set, else fraction digits (`min` and
    /// `max` are the style's defaults), rounding half away from zero (as `Intl.NumberFormat`);
    /// then the minimum integer digits.
    fn with_digits(&self, mut decimal: Decimal, min: u32, max: u32) -> Decimal {
        let to_i16 = |digits: u32| i16::try_from(digits).unwrap_or(i16::MAX);
        let half_expand = SignedRoundingMode::Unsigned(UnsignedRoundingMode::HalfExpand);
        let options = &self.options;
        if options.minimum_significant_digits.is_some()
            || options.maximum_significant_digits.is_some()
        {
            let min = options.minimum_significant_digits.unwrap_or(1).max(1);
            let max = options.maximum_significant_digits.unwrap_or(21).max(min);
            let first = |d: &Decimal| d.absolute.nonzero_magnitude_start();
            decimal.round_with_mode(first(&decimal) - to_i16(max) + 1, half_expand);
            decimal.absolute.trim_end();
            let position = (first(&decimal) - to_i16(min) + 1).min(0);
            decimal.absolute.pad_end(position);
        } else {
            let min = options.minimum_fraction_digits.unwrap_or(min);
            let max = options.maximum_fraction_digits.unwrap_or(max).max(min);
            decimal.round_with_mode(-to_i16(max), half_expand);
            decimal.absolute.trim_end();
            decimal.absolute.pad_end(-to_i16(min));
        }
        if let Some(digits) = options.minimum_integer_digits {
            decimal.absolute.pad_start(to_i16(digits));
        }
        decimal
    }

    /// Sets the sign of `decimal` as the sign display shows it.
    fn apply_sign_display(&self, decimal: &mut Decimal) {
        decimal.apply_sign_display(match self.options.sign_display {
            SignDisplay::Auto => fixed_decimal::SignDisplay::Auto,
            SignDisplay::Always => fixed_decimal::SignDisplay::Always,
            SignDisplay::ExceptZero => fixed_decimal::SignDisplay::ExceptZero,
            SignDisplay::Never => fixed_decimal::SignDisplay::Never,
        });
    }

    fn format_decimal(&self, decimal: Decimal) -> String {
        let mut decimal = self.with_digits(decimal, 0, 3);
        self.apply_sign_display(&mut decimal);
        self.format_with_icu(&decimal)
    }

    fn format_percent(&self, mut decimal: Decimal) -> String {
        decimal.absolute.multiply_pow10(2);
        decimal.absolute.trim_start();
        let mut decimal = self.with_digits(decimal, 0, 0);
        self.apply_sign_display(&mut decimal);
        let formatted = self.format_with_icu(&decimal);
        format!("{formatted}%")
    }

    fn format_currency(&self, decimal: Decimal) -> String {
        let currency = self.options.currency.as_deref().unwrap_or("USD");
        let mut decimal = self.with_digits(decimal, 2, 2);
        self.apply_sign_display(&mut decimal);
        let sign = match decimal.sign {
            Sign::Negative => "-",
            Sign::Positive => "+",
            Sign::None => "",
        };
        decimal.sign = Sign::None;
        let formatted = self.format_with_icu(&decimal);
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

    fn format_unit(&self, decimal: Decimal) -> String {
        let unit = self.options.unit.as_deref().unwrap_or("unit");
        let formatted = self.format_decimal(decimal);

        match self.options.unit_display {
            UnitDisplay::Narrow => format!("{formatted}{unit}"),
            UnitDisplay::Short => format!("{formatted} {unit}"),
            UnitDisplay::Long => format!("{formatted} {unit}s"),
        }
    }

    /// Applies the locale's separators (and grouping, if enabled) with ICU4X.
    fn format_with_icu(&self, decimal: &Decimal) -> String {
        let mut options = DecimalFormatterOptions::default();
        options.grouping_strategy = Some(if self.options.use_grouping {
            icu_decimal::options::GroupingStrategy::Auto
        } else {
            icu_decimal::options::GroupingStrategy::Never
        });
        let prefs = icu_decimal::DecimalFormatterPreferences::from(&self.locale);
        match DecimalFormatter::try_new(prefs, options) {
            Ok(formatter) => formatter.format(decimal).to_string(),
            Err(_) => decimal.to_string(),
        }
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
    use assertr::prelude::*;

    use super::*;
    use crate::utils::i18n::locale;

    #[test]
    fn test_number_formatter_decimal() {
        let locale = Locale::from(locale!("en-US"));
        let formatter = NumberFormatter::new(&locale, NumberFormatOptions::default());
        // Grouped by default, as `Intl.NumberFormat`.
        assert_that!(formatter.format(1234.567)).is_equal_to("1,234.567".to_owned());
    }

    #[test]
    fn test_number_formatter_percent() {
        let locale = Locale::from(locale!("en-US"));
        let formatter = NumberFormatter::new(
            &locale,
            NumberFormatOptions {
                style: NumberStyle::Percent,
                ..Default::default()
            },
        );
        assert_that!(formatter.format(0.75)).is_equal_to("75%".to_owned());
    }

    #[test]
    fn test_number_formatter_currency() {
        let locale = Locale::from(locale!("en-US"));
        let formatter = NumberFormatter::new(
            &locale,
            NumberFormatOptions {
                style: NumberStyle::Currency,
                currency: Some("USD".to_string()),
                use_grouping: true,
                ..Default::default()
            },
        );
        assert_that!(formatter.format(1234.56)).is_equal_to("$1,234.56".to_owned());
    }

    #[test]
    fn test_number_formatter_grouping() {
        let locale = Locale::from(locale!("en-US"));
        let formatter = NumberFormatter::new(
            &locale,
            NumberFormatOptions {
                use_grouping: true,
                maximum_fraction_digits: Some(0),
                ..Default::default()
            },
        );
        assert_that!(formatter.format(1_234_567.0)).is_equal_to("1,234,567".to_owned());
    }

    #[test]
    fn test_number_formatter_german_locale() {
        let locale = Locale::from(locale!("de-DE"));
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
        assert_that!(formatter.format(1234.56)).is_equal_to("1.234,56".to_owned());
    }

    #[test]
    fn test_number_formatter_no_grouping_german() {
        let locale = Locale::from(locale!("de-DE"));
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
        assert_that!(formatter.format(1234.56)).is_equal_to("1234,56".to_owned());
    }

    #[test]
    fn digit_and_sign_options() {
        let format = |options: NumberFormatOptions, value: f64| {
            NumberFormatter::new(&Locale::from(locale!("en-US")), options).format(value)
        };
        let significant = NumberFormatOptions {
            maximum_significant_digits: Some(3),
            ..NumberFormatOptions::default()
        };
        assert_that!(format(significant.clone(), 1234.5)).is_equal_to("1,230".to_owned());
        assert_that!(format(significant, 0.012_345)).is_equal_to("0.0123".to_owned());
        let padded = NumberFormatOptions {
            minimum_significant_digits: Some(3),
            ..NumberFormatOptions::default()
        };
        assert_that!(format(padded, 1.0)).is_equal_to("1.00".to_owned());
        let integer_digits = NumberFormatOptions {
            minimum_integer_digits: Some(3),
            ..NumberFormatOptions::default()
        };
        assert_that!(format(integer_digits, 7.5)).is_equal_to("007.5".to_owned());
        let always = NumberFormatOptions {
            sign_display: SignDisplay::Always,
            ..NumberFormatOptions::default()
        };
        assert_that!(format(always, 5.0)).is_equal_to("+5".to_owned());
        let never = NumberFormatOptions {
            sign_display: SignDisplay::Never,
            ..NumberFormatOptions::default()
        };
        assert_that!(format(never, -5.0)).is_equal_to("5".to_owned());
        // Half away from zero.
        let rounded = NumberFormatOptions {
            maximum_fraction_digits: Some(0),
            ..NumberFormatOptions::default()
        };
        assert_that!(format(rounded.clone(), 2.5)).is_equal_to("3".to_owned());
        assert_that!(format(rounded, -2.5)).is_equal_to("-3".to_owned());
    }

    #[test]
    fn integers_format_exactly() {
        let formatter = NumberFormatter::new(
            &Locale::from(locale!("en-US")),
            NumberFormatOptions::default(),
        );
        assert_that!(formatter.format(u128::MAX))
            .is_equal_to("340,282,366,920,938,463,463,374,607,431,768,211,455".to_owned());
    }
}
