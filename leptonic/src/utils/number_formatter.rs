// Upstream: react-aria/src/i18n/useNumberFormatter.ts @ 99e6102368
// Upstream: @internationalized/number/src/NumberFormatter.ts @ 99e6102368

use std::{fmt, sync::Arc};

use fixed_decimal::{Decimal, Sign, SignedRoundingMode, UnsignedRoundingMode};
use icu_decimal::{DecimalFormatter, options::DecimalFormatterOptions};
use icu_locale::{
    Locale as IcuLocale,
    extensions::unicode::{Value, key},
};
use leptos::prelude::*;
use writeable::{Part, PartsWrite, Writeable};

use super::{i18n::Locale, number_value::NumberValue};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `Intl.NumberFormat` is replaced by an ICU4X-based formatter: typed options (enums instead of
//   strings), exact formatting of any `NumberValue`, and `format_to_parts` with typed parts.
//
// ## OMITTED FEATURES
// - CLDR currency, unit and percent patterns: currency symbols come from a small built-in table
//   and always precede the number, units are written as given, the percent sign always follows
//   the number without a space, and the accounting format wraps negative amounts in parentheses
//   in every locale. Reason: ICU4X's currency, unit and percent formatters are experimental
//   (`icu_experimental`).
//
// =============================================================================

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

    /// How negative currency amounts are shown. Default: with a minus sign.
    pub currency_sign: CurrencySign,

    /// The digits to format with. Default (`None`): the locale's (a `-u-nu-` keyword in the
    /// locale, else its default numbering system).
    pub numbering_system: Option<NumberingSystem>,
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
            currency_sign: CurrencySign::default(),
            numbering_system: None,
        }
    }
}

impl NumberFormatOptions {
    /// The minimum and maximum fraction digits, resolved as `Intl.NumberFormat` does: the style's
    /// defaults (percent 0-0, currency 2-2, else 0-3), where a given maximum below the default
    /// minimum lowers the minimum and a given minimum above the default maximum raises the
    /// maximum.
    pub(crate) fn fraction_digits(&self) -> (u32, u32) {
        let (default_min, default_max) = match self.style {
            NumberStyle::Percent => (0, 0),
            NumberStyle::Currency => (2, 2),
            NumberStyle::Decimal | NumberStyle::Unit => (0, 3),
        };
        match (self.minimum_fraction_digits, self.maximum_fraction_digits) {
            (None, None) => (default_min, default_max),
            (Some(min), None) => (min, default_max.max(min)),
            (None, Some(max)) => (default_min.min(max), max),
            // `Intl.NumberFormat` throws a `RangeError` for a minimum above the maximum.
            (Some(min), Some(max)) => (min, max.max(min)),
        }
    }

    /// `decimal` with the digit options applied: significant digits if set, else fraction digits, rounding half
    /// away from zero (as `Intl.NumberFormat`); then the minimum integer digits.
    #[must_use]
    pub(crate) fn round(&self, mut decimal: Decimal) -> Decimal {
        let to_i16 = |digits: u32| i16::try_from(digits).unwrap_or(i16::MAX);
        let half_expand = SignedRoundingMode::Unsigned(UnsignedRoundingMode::HalfExpand);
        let options = self;
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
            let (min, max) = self.fraction_digits();
            decimal.round_with_mode(-to_i16(max), half_expand);
            decimal.absolute.trim_end();
            decimal.absolute.pad_end(-to_i16(min));
        }
        if let Some(digits) = options.minimum_integer_digits {
            decimal.absolute.pad_start(to_i16(digits));
        }
        decimal
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

/// How negative currency amounts are shown (`Intl.NumberFormat`'s `currencySign`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CurrencySign {
    /// With a minus sign: "-$1.50".
    #[default]
    Standard,
    /// In parentheses, as in accounting: "($1.50)".
    Accounting,
}

/// A numbering system: the digits numbers are written with (a BCP 47 `nu` value).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumberingSystem {
    /// Latin digits: 0123456789.
    Latn,
    /// Arabic-Indic digits: ٠١٢٣٤٥٦٧٨٩.
    Arab,
    /// Extended Arabic-Indic digits (Persian, Urdu): ۰۱۲۳۴۵۶۷۸۹.
    ArabExt,
    /// Bengali digits: ০১২৩৪৫৬৭৮৯.
    Beng,
    /// Devanagari digits: ०१२३४५६७८९.
    Deva,
    /// Full-width digits: ０１２３４５６７８９.
    FullWide,
    /// Han decimal digits: 〇一二三四五六七八九.
    HaniDec,
    /// Myanmar digits: ၀၁၂၃၄၅၆၇၈၉.
    Mymr,
    /// Tamil digits: ௦௧௨௩௪௫௬௭௮௯.
    TamlDec,
    /// Thai digits: ๐๑๒๓๔๕๖๗๘๙.
    Thai,
}

impl NumberingSystem {
    const ALL: [Self; 10] = [
        Self::Latn,
        Self::Arab,
        Self::ArabExt,
        Self::Beng,
        Self::Deva,
        Self::FullWide,
        Self::HaniDec,
        Self::Mymr,
        Self::TamlDec,
        Self::Thai,
    ];

    /// The BCP 47 `nu` value, e.g. `"arab"`.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Latn => "latn",
            Self::Arab => "arab",
            Self::ArabExt => "arabext",
            Self::Beng => "beng",
            Self::Deva => "deva",
            Self::FullWide => "fullwide",
            Self::HaniDec => "hanidec",
            Self::Mymr => "mymr",
            Self::TamlDec => "tamldec",
            Self::Thai => "thai",
        }
    }

    /// The digit zero of this system.
    fn zero(self) -> char {
        match self {
            Self::Latn => '0',
            Self::Arab => '\u{660}',
            Self::ArabExt => '\u{6f0}',
            Self::Beng => '\u{9e6}',
            Self::Deva => '\u{966}',
            Self::FullWide => '\u{ff10}',
            Self::HaniDec => '\u{3007}',
            Self::Mymr => '\u{1040}',
            Self::TamlDec => '\u{be6}',
            Self::Thai => '\u{e50}',
        }
    }

    /// The system whose zero is `zero`.
    pub(crate) fn of_zero(zero: char) -> Option<Self> {
        Self::ALL.into_iter().find(|system| system.zero() == zero)
    }

    /// `locale` with this numbering system (its `-u-nu-` keyword).
    pub(crate) fn apply_to(self, locale: &Locale) -> Locale {
        let mut locale = locale.icu_locale().clone();
        if let Ok(value) = Value::try_from_str(self.as_str()) {
            locale.extensions.unicode.keywords.set(key!("nu"), value);
        }
        Locale::from(locale)
    }
}

/// What a part of a formatted number is (`Intl.NumberFormat.formatToParts`' part types).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberPartKind {
    MinusSign,
    PlusSign,
    /// Integer digits (between group separators).
    Integer,
    /// A group (thousands) separator.
    Group,
    /// The decimal separator.
    Decimal,
    /// Fraction digits.
    Fraction,
    PercentSign,
    /// A currency symbol, code or name.
    Currency,
    Unit,
    /// Anything else: spaces, parentheses, direction marks.
    Literal,
}

/// A part of a formatted number, see [`NumberFormatter::format_to_parts`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberPart {
    pub kind: NumberPartKind,
    pub value: String,
}

impl NumberPart {
    fn new(kind: NumberPartKind, value: impl Into<String>) -> Self {
        Self {
            kind,
            value: value.into(),
        }
    }
}

/// Collects the parts ICU4X's decimal formatter writes, merging adjacent text of one kind.
struct PartsCollector(Vec<NumberPart>, Vec<NumberPartKind>);

impl fmt::Write for PartsCollector {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.is_empty() {
            return Ok(());
        }
        let kind = self.1.last().copied().unwrap_or(NumberPartKind::Literal);
        match self.0.last_mut() {
            Some(last) if last.kind == kind => last.value.push_str(s),
            _ => self.0.push(NumberPart::new(kind, s)),
        }
        Ok(())
    }
}

impl PartsWrite for PartsCollector {
    type SubPartsWrite = Self;

    fn with_part(
        &mut self,
        part: Part,
        mut f: impl FnMut(&mut Self::SubPartsWrite) -> fmt::Result,
    ) -> fmt::Result {
        use icu_decimal::parts;
        let kind = match part {
            parts::MINUS_SIGN => NumberPartKind::MinusSign,
            parts::PLUS_SIGN => NumberPartKind::PlusSign,
            parts::INTEGER => NumberPartKind::Integer,
            parts::GROUP => NumberPartKind::Group,
            parts::DECIMAL => NumberPartKind::Decimal,
            parts::FRACTION => NumberPartKind::Fraction,
            _ => NumberPartKind::Literal,
        };
        self.1.push(kind);
        let result = f(self);
        self.1.pop();
        result
    }
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
    /// The locale's decimal formatter for the grouping option, built once. `None` if the locale
    /// has no data (digits are then formatted plainly).
    decimal: Option<Arc<DecimalFormatter>>,
}

impl PartialEq for NumberFormatter {
    fn eq(&self, other: &Self) -> bool {
        // The decimal formatter follows from these.
        self.locale == other.locale && self.options == other.options
    }
}

/// The number formatter of the current locale (see [`use_locale`](super::i18n::use_locale)) and
/// `options`, rebuilt when either changes (react-aria's `useNumberFormatter`).
pub fn use_number_formatter(options: Signal<NumberFormatOptions>) -> Memo<NumberFormatter> {
    let locale = super::i18n::use_locale();
    Memo::new(move |_| NumberFormatter::new(&locale.get(), options.get()))
}

impl NumberFormatter {
    /// Creates a new number formatter with the given locale and options.
    #[must_use]
    pub fn new(locale: &Locale, options: NumberFormatOptions) -> Self {
        let locale = match options.numbering_system {
            Some(system) => system.apply_to(locale).icu_locale().clone(),
            None => locale.icu_locale().clone(),
        };
        let mut decimal_options = DecimalFormatterOptions::default();
        decimal_options.grouping_strategy = Some(if options.use_grouping {
            icu_decimal::options::GroupingStrategy::Auto
        } else {
            icu_decimal::options::GroupingStrategy::Never
        });
        let prefs = icu_decimal::DecimalFormatterPreferences::from(&locale);
        let decimal = DecimalFormatter::try_new(prefs, decimal_options)
            .ok()
            .map(Arc::new);
        Self {
            locale,
            options,
            decimal,
        }
    }

    /// The options this formatter formats with.
    #[must_use]
    pub fn options(&self) -> &NumberFormatOptions {
        &self.options
    }

    /// Formats a number according to the formatter's options. Empty for infinite and NaN floats.
    #[must_use]
    pub fn format<T: NumberValue>(&self, value: T) -> String {
        self.format_to_parts(value)
            .into_iter()
            .map(|part| part.value)
            .collect()
    }

    /// Formats a number into its parts (`Intl.NumberFormat.formatToParts`). Empty for infinite
    /// and NaN floats.
    #[must_use]
    pub fn format_to_parts<T: NumberValue>(&self, value: T) -> Vec<NumberPart> {
        value
            .to_decimal()
            .map(|decimal| self.format_decimal_to_parts(decimal))
            .unwrap_or_default()
    }

    /// Formats a decimal into its parts.
    pub(crate) fn format_decimal_to_parts(&self, decimal: Decimal) -> Vec<NumberPart> {
        match self.options.style {
            NumberStyle::Percent => self.format_percent(decimal),
            NumberStyle::Currency => self.format_currency(decimal),
            NumberStyle::Unit => self.format_unit(decimal),
            NumberStyle::Decimal => self.format_decimal(decimal),
        }
    }

    /// `decimal` rounded as this formatter shows it.
    pub(crate) fn round(&self, decimal: Decimal) -> Decimal {
        self.options.round(decimal)
    }

    /// The maximum fraction digits this formatter shows, `None` when significant digits decide
    /// (`Intl.NumberFormat`'s resolved `maximumFractionDigits`).
    pub(crate) fn maximum_fraction_digits(&self) -> Option<u32> {
        let options = &self.options;
        if options.minimum_significant_digits.is_some()
            || options.maximum_significant_digits.is_some()
        {
            return None;
        }
        Some(self.options.fraction_digits().1)
    }

    /// The minimum fraction digits this formatter shows.
    pub(crate) fn minimum_fraction_digits(&self) -> u32 {
        self.options.fraction_digits().0
    }

    /// The numbering system this formatter writes digits with.
    #[must_use]
    pub fn numbering_system(&self) -> NumberingSystem {
        let digits = self.format_with_icu(&Decimal::from(0));
        digits
            .chars()
            .find_map(NumberingSystem::of_zero)
            .unwrap_or(NumberingSystem::Latn)
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

    fn format_decimal(&self, decimal: Decimal) -> Vec<NumberPart> {
        let mut decimal = self.round(decimal);
        self.apply_sign_display(&mut decimal);
        self.parts_with_icu(&decimal)
    }

    fn format_percent(&self, mut decimal: Decimal) -> Vec<NumberPart> {
        decimal.absolute.multiply_pow10(2);
        decimal.absolute.trim_start();
        let mut decimal = self.round(decimal);
        self.apply_sign_display(&mut decimal);
        let mut parts = self.parts_with_icu(&decimal);
        parts.push(NumberPart::new(NumberPartKind::PercentSign, "%"));
        parts
    }

    fn format_currency(&self, decimal: Decimal) -> Vec<NumberPart> {
        let currency = self.options.currency.as_deref().unwrap_or("USD");
        let mut decimal = self.round(decimal);
        self.apply_sign_display(&mut decimal);
        let accounting = self.options.currency_sign == CurrencySign::Accounting
            && decimal.sign == Sign::Negative;
        if accounting {
            decimal.sign = Sign::None;
        }
        let mut parts = self.parts_with_icu(&decimal);
        // The sign comes first.
        let sign_end = parts
            .iter()
            .position(|part| {
                !matches!(
                    part.kind,
                    NumberPartKind::MinusSign | NumberPartKind::PlusSign
                )
            })
            .unwrap_or(parts.len());
        match self.options.currency_display {
            CurrencyDisplay::Code => {
                parts.splice(
                    sign_end..sign_end,
                    [
                        NumberPart::new(NumberPartKind::Currency, currency),
                        NumberPart::new(NumberPartKind::Literal, "\u{a0}"),
                    ],
                );
            }
            CurrencyDisplay::Name => {
                parts.push(NumberPart::new(NumberPartKind::Literal, " "));
                parts.push(NumberPart::new(
                    NumberPartKind::Currency,
                    get_currency_name(currency),
                ));
            }
            CurrencyDisplay::Symbol | CurrencyDisplay::NarrowSymbol => {
                parts.insert(
                    sign_end,
                    NumberPart::new(NumberPartKind::Currency, get_currency_symbol(currency)),
                );
            }
        }
        if accounting {
            parts.insert(0, NumberPart::new(NumberPartKind::Literal, "("));
            parts.push(NumberPart::new(NumberPartKind::Literal, ")"));
        }
        parts
    }

    fn format_unit(&self, decimal: Decimal) -> Vec<NumberPart> {
        let unit = self.options.unit.as_deref().unwrap_or("unit");
        let mut parts = self.format_decimal(decimal);
        match self.options.unit_display {
            UnitDisplay::Narrow => parts.push(NumberPart::new(NumberPartKind::Unit, unit)),
            UnitDisplay::Short => {
                parts.push(NumberPart::new(NumberPartKind::Literal, " "));
                parts.push(NumberPart::new(NumberPartKind::Unit, unit));
            }
            UnitDisplay::Long => {
                parts.push(NumberPart::new(NumberPartKind::Literal, " "));
                parts.push(NumberPart::new(NumberPartKind::Unit, format!("{unit}s")));
            }
        }
        parts
    }

    /// The parts of `decimal` with the locale's digits, separators and signs (and grouping, if
    /// enabled), from ICU4X.
    fn parts_with_icu(&self, decimal: &Decimal) -> Vec<NumberPart> {
        let mut collector = PartsCollector(Vec::new(), Vec::new());
        match &self.decimal {
            Some(formatter) => {
                let _ = formatter.format(decimal).write_to_parts(&mut collector);
            }
            None => {
                let _ = fmt::Write::write_str(&mut collector, &decimal.to_string());
            }
        }
        collector.0
    }

    /// `decimal` with the locale's digits and separators.
    fn format_with_icu(&self, decimal: &Decimal) -> String {
        self.parts_with_icu(decimal)
            .into_iter()
            .map(|part| part.value)
            .collect()
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

    // `Intl.NumberFormat`: a maximum below the style's default minimum lowers the minimum.
    #[test]
    fn a_maximum_below_the_default_minimum_lowers_the_minimum() {
        let formatter = NumberFormatter::new(
            &Locale::from(locale!("en-US")),
            NumberFormatOptions {
                style: NumberStyle::Currency,
                currency: Some("USD".to_owned()),
                maximum_fraction_digits: Some(0),
                ..NumberFormatOptions::default()
            },
        );
        assert_that!(formatter.format(5.0)).is_equal_to("$5".to_owned());
        assert_that!(formatter.format(5.5)).is_equal_to("$6".to_owned());
        assert_that!(formatter.minimum_fraction_digits()).is_equal_to(0);
        assert_that!(formatter.maximum_fraction_digits()).is_equal_to(Some(0));
        let one = NumberFormatter::new(
            &Locale::from(locale!("en-US")),
            NumberFormatOptions {
                style: NumberStyle::Currency,
                currency: Some("USD".to_owned()),
                maximum_fraction_digits: Some(1),
                ..NumberFormatOptions::default()
            },
        );
        assert_that!(one.format(5.0)).is_equal_to("$5.0".to_owned());
        // A minimum above the default maximum raises the maximum.
        let three = NumberFormatter::new(
            &Locale::from(locale!("en-US")),
            NumberFormatOptions {
                style: NumberStyle::Percent,
                minimum_fraction_digits: Some(1),
                ..NumberFormatOptions::default()
            },
        );
        assert_that!(three.format(0.5)).is_equal_to("50.0%".to_owned());
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
