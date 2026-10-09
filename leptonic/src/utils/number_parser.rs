// Upstream: @internationalized/number/src/NumberParser.ts @ 99e6102368
// Upstream: @internationalized/number/test/NumberParser.test.js @ 99e6102368
//! Locale-aware parsing of formatted numbers and validation of partial input.

use std::sync::{Arc, OnceLock};

use fixed_decimal::{Decimal, Sign};
use icu_locale::extensions::unicode::key;
use icu_properties::{CodePointMapData, props::GeneralCategory};

use super::{
    i18n::Locale,
    number_formatter::{
        CurrencySign, NumberFormatOptions, NumberFormatter, NumberPart, NumberPartKind,
        NumberStyle, NumberingSystem, SignDisplay,
    },
    number_value::NumberValue,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the value type: `parse::<T>` returns `Option<T>` (`None` for NaN), exactly, from
//   ICU4X decimals (react-aria: JS numbers). Integers saturate at their type's bounds (see
//   `NumberValue::from_decimal`); integer types reject a decimal separator while typing.
// - `is_valid_partial_number` takes optional typed bounds; without them, the type's bounds apply.
// - `numbering_system` returns a typed `NumberingSystem` (react-aria: `getNumberingSystem`, a
//   string).
// - Parsers for other numbering systems are cached per parser (react-aria: a global cache).
//
// ## DIFFERENT BEHAVIOR
// - The symbols come from leptonic's `NumberFormatter`, which lacks CLDR currency, unit and
//   percent patterns (see its deviations): only the literals it writes are stripped (e.g. no
//   localized unit names or plural forms, no "ر.س.‏" for SAR).
// - Numbers are read with `Decimal`'s grammar after the locale's symbols are replaced (react-aria:
//   JS `+value`, which also accepts exponents like "1e5" and "Infinity").
// - Full-width digits aren't detected: ICU4X has no data for the `fullwide` numbering system.
// - ICU4X writes Arabic digits with Latin separators in some locales (en-u-nu-arab: "٠.٥"; CLDR
//   and browsers: "٠٫٥"). There, the Arabic separators are read as the Latin ones; the keyboard
//   replacements (`,` and `،` for the Arabic decimal separator) apply only with Arabic ones.
//
// ## OMITTED FEATURES
// - fr-FR: replacing spaces by the group separator. Spaces are literals, stripped before, so
//   upstream's replacement never applies.
// - `roundingIncrement`: not a leptonic format option.
//
// =============================================================================

/// The numbering systems tried when the text isn't valid in the locale's own (react-aria's list).
const NUMBERING_SYSTEMS: [NumberingSystem; 6] = [
    NumberingSystem::Latn,
    NumberingSystem::Arab,
    NumberingSystem::HaniDec,
    NumberingSystem::Deva,
    NumberingSystem::Beng,
    NumberingSystem::FullWide,
];

/// The Arabic decimal and group separators.
const ARABIC_DECIMAL: &str = "\u{66b}";
const ARABIC_GROUP: &str = "\u{66c}";

/// Every number whose plural form differs in some locale (react-aria's list, from CLDR's plural
/// rules), formatted to find all forms of a unit or currency name.
const PLURAL_NUMBERS: [&str; 12] = [
    "0", "4", "2", "1", "11", "20", "3", "7", "100", "21", "0.1", "1.1",
];

/// A locale-aware number parser: the inverse of [`NumberFormatter`] with the same options.
///
/// It detects the numbering system of the text (e.g. Arabic-Indic digits typed in an en-US
/// field), ignores literals of the format (currency symbols, codes and names, units, percent
/// signs, spaces, direction marks), and validates partial input while typing.
///
/// Based on react-aria's `NumberParser` from `@internationalized/number`.
#[derive(Debug, Clone)]
pub struct NumberParser {
    /// The parser of the locale's numbering system.
    default: Arc<ParserImpl>,
    /// The parsers of the other numbering systems (`NUMBERING_SYSTEMS`), created when needed.
    others: Arc<[OnceLock<Arc<ParserImpl>>; NUMBERING_SYSTEMS.len()]>,
    locale: Locale,
    options: NumberFormatOptions,
    /// Whether the locale fixes the numbering system (a `-u-nu-` keyword).
    fixed_numbering_system: bool,
}

impl NumberParser {
    /// Creates a new number parser for the given locale and format options.
    #[must_use]
    pub fn new(locale: &Locale, options: &NumberFormatOptions) -> Self {
        let fixed_numbering_system = locale
            .icu_locale()
            .extensions
            .unicode
            .keywords
            .get(&key!("nu"))
            .is_some();
        Self {
            default: Arc::new(ParserImpl::new(locale, options)),
            others: Arc::new(std::array::from_fn(|_| OnceLock::new())),
            locale: locale.clone(),
            options: options.clone(),
            fixed_numbering_system,
        }
    }

    /// The parser for `value`: the locale's numbering system, else the first other one in which
    /// `value` is valid.
    fn parser_for(&self, value: &str) -> &ParserImpl {
        if self.fixed_numbering_system || self.default.is_valid_partial(value, true, true) {
            return &self.default;
        }
        for (index, system) in NUMBERING_SYSTEMS.into_iter().enumerate() {
            if system == self.default.numbering_system {
                continue;
            }
            let parser = self.others[index].get_or_init(|| {
                Arc::new(ParserImpl::new(
                    &system.apply_to(&self.locale),
                    &self.options,
                ))
            });
            // ICU4X may lack data for a system and fall back to another.
            if parser.numbering_system == system && parser.is_valid_partial(value, true, true) {
                return parser;
            }
        }
        &self.default
    }

    /// Parses a locale-formatted string into a number.
    ///
    /// Returns `None` if the string is empty or can't be parsed, and for fraction digits in
    /// integer types. Integers beyond `T`'s range saturate at its bounds.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let parser = NumberParser::new(&Locale::from(locale!("de-DE")), &NumberFormatOptions::default());
    /// assert_eq!(parser.parse::<f64>("1.234,56"), Some(1234.56));
    /// ```
    #[must_use]
    pub fn parse<T: NumberValue>(&self, value: &str) -> Option<T> {
        T::from_decimal(&self.parser_for(value).parse(value)?)
    }

    /// Whether `value` could become a valid number while the user types (e.g. `"-"`, `"1."`,
    /// `"1,2"` in German). With bounds (else `T`'s), a minus sign is only valid if negative
    /// numbers are possible, a plus sign only if positive ones are.
    #[must_use]
    pub fn is_valid_partial_number<T: NumberValue>(
        &self,
        value: &str,
        min_value: Option<T>,
        max_value: Option<T>,
    ) -> bool {
        let allows_negative = T::lower_bound(min_value).is_none_or(|min| min < T::ZERO);
        let allows_positive = T::upper_bound(max_value).is_none_or(|max| max > T::ZERO);
        let parser = self.parser_for(value);
        if T::IS_INTEGER
            && parser
                .symbols
                .decimal
                .as_deref()
                .is_some_and(|decimal| parser.sanitize(value).contains(decimal))
        {
            return false;
        }
        parser.is_valid_partial(value, allows_negative, allows_positive)
    }

    /// The numbering system `value` is written in: the locale's, or another one in which it is
    /// valid (react-aria's `getNumberingSystem`).
    #[must_use]
    pub fn numbering_system(&self, value: &str) -> NumberingSystem {
        self.parser_for(value).numbering_system
    }
}

/// The parser of one locale and numbering system.
#[derive(Debug)]
struct ParserImpl {
    options: NumberFormatOptions,
    formatter: NumberFormatter,
    numbering_system: NumberingSystem,
    symbols: Symbols,
}

/// The symbols of a format, found by formatting numbers to parts.
#[derive(Debug)]
struct Symbols {
    minus_sign: String,
    plus_sign: Option<String>,
    decimal: Option<String>,
    group: Option<String>,
    /// Text that doesn't contribute to the value, longest first.
    literals: Vec<String>,
    /// The digits zero to nine.
    numerals: Vec<char>,
    /// Formatted plural numbers without digits (e.g. Arabic "يومان", two days), with their value.
    no_numeral_units: Vec<(String, &'static str)>,
}

impl ParserImpl {
    fn new(locale: &Locale, options: &NumberFormatOptions) -> Self {
        let formatter = NumberFormatter::new(locale, options.clone());
        let symbols = symbols(locale, options);
        Self {
            options: options.clone(),
            numbering_system: formatter.numbering_system(),
            formatter,
            symbols,
        }
    }

    fn parse(&self, value: &str) -> Option<Decimal> {
        let symbols = &self.symbols;
        let mut sanitized = self.sanitize(value);
        if let Some(group) = &symbols.group {
            // Group separators are invalid without grouping.
            if !self.options.use_grouping && sanitized.contains(group.as_str()) {
                return None;
            }
            sanitized = sanitized.replace(group.as_str(), "");
        }
        if let Some(decimal) = &symbols.decimal {
            sanitized = sanitized.replacen(decimal.as_str(), ".", 1);
        }
        sanitized = sanitized.replacen(symbols.minus_sign.as_str(), "-", 1);
        let sanitized: String = sanitized
            .chars()
            .map(|c| {
                symbols
                    .numerals
                    .iter()
                    .position(|numeral| *numeral == c)
                    .and_then(|digit| char::from_digit(u32::try_from(digit).ok()?, 10))
                    .unwrap_or(c)
            })
            .collect();
        let mut decimal = parse_ascii_number(&sanitized)?;

        if self.options.style == NumberStyle::Percent {
            decimal.absolute.multiply_pow10(-2);
            decimal.absolute.trim_start();
            // Rounded as the formatter shows it: two more fraction digits than the percent.
            let rounding = NumberFormatOptions {
                style: NumberStyle::Decimal,
                minimum_fraction_digits: Some(
                    (self.formatter.minimum_fraction_digits() + 2).min(20),
                ),
                maximum_fraction_digits: Some(
                    (self.formatter.maximum_fraction_digits().unwrap_or(0) + 2).min(20),
                ),
                ..self.options.clone()
            };
            return Some(rounding.round(decimal));
        }

        // Accounting strips the parentheses around negative amounts: negate them again.
        if self.options.currency_sign == CurrencySign::Accounting
            && value
                .find('(')
                .is_some_and(|open| value[open..].contains(')'))
        {
            decimal.sign = match decimal.sign {
                Sign::Negative => Sign::None,
                Sign::None | Sign::Positive => Sign::Negative,
            };
        }
        Some(decimal)
    }

    /// `value` without the format's literals, with the separators users type replaced by the
    /// locale's.
    fn sanitize(&self, value: &str) -> String {
        let symbols = &self.symbols;
        let grouping = self.options.use_grouping;
        // A number written without digits (e.g. Arabic "two days").
        if let Some((_, number)) = symbols
            .no_numeral_units
            .iter()
            .find(|(unit, _)| unit == value)
        {
            return (*number).to_owned();
        }

        let mut value = strip_literals(value, &symbols.literals);

        // An ASCII minus sign counts as the locale's (keyboards may lack it).
        value = value.replacen('-', &symbols.minus_sign, 1);

        if self.numbering_system == NumberingSystem::Arab {
            if symbols.decimal.as_deref() == Some(ARABIC_DECIMAL) {
                // Keyboards for Arabic digits type `,` or `،` rather than the Arabic decimal
                // separator (and `.` rather than its group separator).
                value = value.replace([',', '\u{60c}'], ARABIC_DECIMAL);
                if let Some(group) = symbols.group.as_ref().filter(|_| grouping) {
                    value = value.replace('.', group);
                }
            } else {
                // Arabic digits with Latin separators (ICU4X: en-u-nu-arab): the Arabic
                // separators are typed too (as CLDR's separators of the `arab` system).
                if let Some(decimal) = &symbols.decimal {
                    value = value.replace(ARABIC_DECIMAL, decimal);
                }
                if let Some(group) = symbols.group.as_ref().filter(|_| grouping) {
                    value = value.replace(ARABIC_GROUP, group);
                }
            }
        }

        // Swiss group separators: the typographic and the ASCII apostrophe are interchangeable.
        if grouping {
            match symbols.group.as_deref() {
                Some("\u{2019}") => value = value.replace('\'', "\u{2019}"),
                Some("'") => value = value.replace('\u{2019}', "'"),
                _ => {}
            }
        }
        value
    }

    fn is_valid_partial(&self, value: &str, allows_negative: bool, allows_positive: bool) -> bool {
        let symbols = &self.symbols;
        let mut value = self.sanitize(value);

        // A sign, at the start.
        if allows_negative && value.starts_with(symbols.minus_sign.as_str()) {
            value.drain(..symbols.minus_sign.len());
        } else if let Some(plus) = symbols.plus_sign.as_deref()
            && allows_positive
            && value.starts_with(plus)
        {
            value.drain(..plus.len());
        }

        // Numbers without fraction digits reject a decimal separator.
        if let Some(decimal) = &symbols.decimal
            && value.contains(decimal.as_str())
            && self.formatter.maximum_fraction_digits() == Some(0)
        {
            return false;
        }

        if let Some(group) = symbols.group.as_ref().filter(|_| self.options.use_grouping) {
            value = value.replace(group.as_str(), "");
        }
        value.retain(|c| !symbols.numerals.contains(&c));
        if let Some(decimal) = &symbols.decimal {
            value = value.replacen(decimal.as_str(), "", 1);
        }
        value.is_empty()
    }
}

/// The symbols of `options` in `locale`, from formatting numbers to parts.
fn symbols(locale: &Locale, options: &NumberFormatOptions) -> Symbols {
    // Significant digits, so that every symbol (and plural form) appears.
    let symbol_formatter = NumberFormatter::new(
        locale,
        NumberFormatOptions {
            minimum_significant_digits: Some(1),
            maximum_significant_digits: Some(21),
            use_grouping: true,
            ..options.clone()
        },
    );
    let format = |number: &str| {
        Decimal::try_from_str(number)
            .map(|decimal| symbol_formatter.format_decimal_to_parts(decimal))
            .unwrap_or_default()
    };
    // Some locales group only from ten thousand on.
    let all_parts = format("-10000.111");
    let positive_parts = format("10000.111");
    let plural_parts: Vec<Vec<NumberPart>> = PLURAL_NUMBERS.iter().map(|n| format(n)).collect();

    // Without direction marks (e.g. Arabic's minus sign "\u{61c}-"): the text is read without
    // them.
    let find = |parts: &[NumberPart], kind: NumberPartKind| {
        parts
            .iter()
            .find(|part| part.kind == kind)
            .map(|part| {
                part.value
                    .chars()
                    .filter(|c| !is_ignored_char(*c))
                    .collect()
            })
            .filter(|symbol: &String| !symbol.is_empty())
    };

    // Units that include the number (no digits): their text means the value.
    let no_numeral_units = plural_parts
        .iter()
        .zip(PLURAL_NUMBERS)
        .filter_map(|(parts, number)| {
            let unit = find(parts, NumberPartKind::Unit)?;
            let has_digits = parts
                .iter()
                .any(|p| matches!(p.kind, NumberPartKind::Integer | NumberPartKind::Fraction));
            (!has_digits).then_some((unit, number))
        })
        .collect();

    let minus_sign = find(&all_parts, NumberPartKind::MinusSign).unwrap_or_else(|| "-".to_owned());
    let plus_sign = find(&positive_parts, NumberPartKind::PlusSign).or_else(|| {
        matches!(
            options.sign_display,
            SignDisplay::Always | SignDisplay::ExceptZero
        )
        .then(|| "+".to_owned())
    });

    // Fraction digits, so that the decimal separator appears (also for percents).
    let decimal = NumberFormatter::new(
        locale,
        NumberFormatOptions {
            minimum_fraction_digits: Some(2),
            maximum_fraction_digits: Some(2),
            minimum_significant_digits: None,
            maximum_significant_digits: None,
            ..options.clone()
        },
    )
    .format_to_parts(0.001);
    let decimal = find(&decimal, NumberPartKind::Decimal);
    // A whitespace group separator (fr-FR) is ignored as such.
    let group = find(&all_parts, NumberPartKind::Group);

    let is_literal = |part: &&NumberPart| {
        !matches!(
            part.kind,
            NumberPartKind::Decimal
                | NumberPartKind::Fraction
                | NumberPartKind::Integer
                | NumberPartKind::MinusSign
                | NumberPartKind::PlusSign
                | NumberPartKind::Group
        )
    };
    let mut literals: Vec<String> = all_parts
        .iter()
        .chain(plural_parts.iter().flatten())
        .filter(is_literal)
        .map(|part| part.value.clone())
        .filter(|literal| !literal.is_empty())
        .collect();
    literals.sort_by(|a, b| b.chars().count().cmp(&a.chars().count()).then(a.cmp(b)));
    literals.dedup();

    // The digits, to read other numbering systems.
    let digits = NumberFormatter::new(
        locale,
        NumberFormatOptions {
            use_grouping: false,
            ..NumberFormatOptions::default()
        },
    )
    .format(9_876_543_210_u64);
    let mut numerals: Vec<char> = digits.chars().filter(|c| !is_ignored_char(*c)).collect();
    numerals.reverse();

    Symbols {
        minus_sign,
        plus_sign,
        decimal,
        group,
        literals,
        numerals,
        no_numeral_units,
    }
}

/// Whitespace and format characters (direction marks, ...): never part of a number.
fn is_ignored_char(c: char) -> bool {
    c.is_whitespace()
        || CodePointMapData::<GeneralCategory>::new().get(c) == GeneralCategory::Format
}

/// `value` without `literals` (longest first) and ignored characters (react-aria's literal
/// regular expression).
fn strip_literals(value: &str, literals: &[String]) -> String {
    let mut result = String::with_capacity(value.len());
    let mut rest = value;
    while let Some(c) = rest.chars().next() {
        if let Some(literal) = literals
            .iter()
            .find(|literal| rest.starts_with(literal.as_str()))
        {
            rest = &rest[literal.len()..];
            continue;
        }
        if !is_ignored_char(c) {
            result.push(c);
        }
        rest = &rest[c.len_utf8()..];
    }
    result
}

/// Reads a number of ASCII digits, an optional sign and an optional `.` (also `".5"` and `"5."`,
/// as JavaScript's `Number`).
fn parse_ascii_number(value: &str) -> Option<Decimal> {
    let (negative, unsigned) = match value.as_bytes().first() {
        Some(b'-') => (true, &value[1..]),
        Some(b'+') => (false, &value[1..]),
        _ => (false, value),
    };
    // At most one decimal point, which may end the number ("5.", as `Number`): "1.2." is NaN.
    if unsigned.bytes().filter(|b| *b == b'.').count() > 1 {
        return None;
    }
    let unsigned = unsigned.strip_suffix('.').unwrap_or(unsigned);
    if unsigned.is_empty() || !unsigned.bytes().all(|b| b.is_ascii_digit() || b == b'.') {
        return None;
    }
    let unsigned = if unsigned.starts_with('.') {
        format!("0{unsigned}")
    } else {
        unsigned.to_owned()
    };
    let mut decimal = Decimal::try_from_str(&unsigned).ok()?;
    if negative {
        decimal.sign = Sign::Negative;
    }
    Some(decimal)
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::utils::number_formatter::{CurrencyDisplay, UnitDisplay};

    fn number_parser(locale: &str, options: NumberFormatOptions) -> NumberParser {
        NumberParser::new(&locale.parse::<Locale>().expect("a locale"), &options)
    }

    fn decimal(locale: &str) -> NumberParser {
        number_parser(locale, NumberFormatOptions::default())
    }

    fn currency(display: CurrencyDisplay, sign: CurrencySign) -> NumberFormatOptions {
        NumberFormatOptions {
            style: NumberStyle::Currency,
            currency: Some("USD".to_owned()),
            currency_display: display,
            currency_sign: sign,
            ..NumberFormatOptions::default()
        }
    }

    fn usd() -> NumberParser {
        number_parser(
            "en-US",
            currency(CurrencyDisplay::Symbol, CurrencySign::Standard),
        )
    }

    fn accounting(display: CurrencyDisplay) -> NumberParser {
        number_parser("en-US", currency(display, CurrencySign::Accounting))
    }

    fn percent(options: NumberFormatOptions) -> NumberFormatOptions {
        NumberFormatOptions {
            style: NumberStyle::Percent,
            ..options
        }
    }

    fn signs(sign_display: SignDisplay) -> NumberFormatOptions {
        NumberFormatOptions {
            sign_display,
            ..NumberFormatOptions::default()
        }
    }

    fn no_grouping() -> NumberFormatOptions {
        NumberFormatOptions {
            use_grouping: false,
            ..NumberFormatOptions::default()
        }
    }

    fn valid(parser: &NumberParser, value: &str) -> bool {
        parser.is_valid_partial_number::<f64>(value, None, None)
    }

    // ---- parse ----

    #[test]
    fn parses_basic_numbers() {
        assert_that!(decimal("en-US").parse::<f64>("10")).is_equal_to(Some(10.0));
        assert_that!(decimal("en-US").parse::<f64>("-10")).is_equal_to(Some(-10.0));
    }

    #[test]
    fn parses_decimals() {
        let parser = decimal("en-US");
        assert_that!(parser.parse::<f64>("10.5")).is_equal_to(Some(10.5));
        assert_that!(parser.parse::<f64>("-10.5")).is_equal_to(Some(-10.5));
        assert_that!(parser.parse::<f64>(".5")).is_equal_to(Some(0.5));
        assert_that!(parser.parse::<f64>("-.5")).is_equal_to(Some(-0.5));
    }

    #[test]
    fn parses_group_characters() {
        let parser = decimal("en-US");
        assert_that!(parser.parse::<f64>("1,000")).is_equal_to(Some(1000.0));
        assert_that!(parser.parse::<f64>("-1,000")).is_equal_to(Some(-1000.0));
        assert_that!(parser.parse::<f64>("1,000,000")).is_equal_to(Some(1_000_000.0));
        assert_that!(parser.parse::<f64>("-1,000,000")).is_equal_to(Some(-1_000_000.0));
    }

    #[test]
    fn parses_sign_display() {
        assert_that!(decimal("en-US").parse::<f64>("+10")).is_equal_to(Some(10.0));
        assert_that!(number_parser("en-US", signs(SignDisplay::Always)).parse::<f64>("+10"))
            .is_equal_to(Some(10.0));
    }

    #[test]
    fn parses_negative_numbers_with_different_minus_signs() {
        assert_that!(decimal("en-US").parse::<f64>("-10")).is_equal_to(Some(-10.0));
        assert_that!(decimal("en-US").parse::<f64>("\u{2212}10")).is_none();
        assert_that!(decimal("fi-FI").parse::<f64>("-10")).is_equal_to(Some(-10.0));
        assert_that!(decimal("fi-FI").parse::<f64>("\u{2212}10")).is_equal_to(Some(-10.0));
    }

    #[test]
    fn returns_none_for_random_characters() {
        assert_that!(decimal("en-US").parse::<f64>("g")).is_none();
        assert_that!(decimal("en-US").parse::<f64>("1abc")).is_none();
        assert_that!(decimal("en-US").parse::<f64>("")).is_none();
        assert_that!(decimal("en-US").parse::<f64>("12.34.56")).is_none();
    }

    // JavaScript's `Number`: one trailing decimal point is allowed, a second one makes it NaN.
    #[test]
    fn returns_none_for_a_trailing_point_after_a_fraction() {
        let parser = decimal("en-US");
        assert_that!(parser.parse::<f64>("1.")).is_equal_to(Some(1.0));
        assert_that!(parser.parse::<f64>("1.2.")).is_none();
        assert_that!(parser.parse::<f64>("1..")).is_none();
    }

    #[test]
    fn returns_none_for_invalid_grouping() {
        assert_that!(number_parser("en-US", no_grouping()).parse::<f64>("1234,7")).is_none();
        assert_that!(number_parser("de-DE", no_grouping()).parse::<f64>("1234.7")).is_none();
    }

    #[test]
    fn parses_currency_without_the_symbol() {
        assert_that!(usd().parse::<f64>("10.50")).is_equal_to(Some(10.5));
    }

    #[test]
    fn ignores_currency_symbols_codes_and_names() {
        assert_that!(usd().parse::<f64>("$10.50")).is_equal_to(Some(10.5));
        let code = number_parser(
            "en-US",
            currency(CurrencyDisplay::Code, CurrencySign::Standard),
        );
        assert_that!(code.parse::<f64>("USD 10.50")).is_equal_to(Some(10.5));
        let name = number_parser(
            "en-US",
            currency(CurrencyDisplay::Name, CurrencySign::Standard),
        );
        assert_that!(name.parse::<f64>("10.50 US dollars")).is_equal_to(Some(10.5));
    }

    #[test]
    fn parses_the_accounting_format() {
        let symbol = accounting(CurrencyDisplay::Symbol);
        assert_that!(symbol.parse::<f64>("(1.50)")).is_equal_to(Some(-1.5));
        assert_that!(symbol.parse::<f64>("($1.50)")).is_equal_to(Some(-1.5));
        let code = accounting(CurrencyDisplay::Code);
        assert_that!(code.parse::<f64>("(USD 1.50)")).is_equal_to(Some(-1.5));
    }

    #[test]
    fn parses_normal_negative_numbers_in_the_accounting_format() {
        let symbol = accounting(CurrencyDisplay::Symbol);
        assert_that!(symbol.parse::<f64>("-1.5")).is_equal_to(Some(-1.5));
        assert_that!(symbol.parse::<f64>("-$1.50")).is_equal_to(Some(-1.5));
        let code = accounting(CurrencyDisplay::Code);
        assert_that!(code.parse::<f64>("USD -1.50")).is_equal_to(Some(-1.5));
    }

    #[test]
    fn returns_none_for_unknown_or_partial_currencies() {
        assert_that!(usd().parse::<f64>("€10.50")).is_none();
        let code = number_parser(
            "en-US",
            currency(CurrencyDisplay::Code, CurrencySign::Standard),
        );
        assert_that!(code.parse::<f64>("EUR 10.50")).is_none();
        assert_that!(code.parse::<f64>("EU 10.50")).is_none();
        let name = number_parser(
            "en-US",
            currency(CurrencyDisplay::Name, CurrencySign::Standard),
        );
        assert_that!(name.parse::<f64>("10.50 euros")).is_none();
        assert_that!(name.parse::<f64>("10.50 eur")).is_none();
    }

    #[test]
    fn parses_units_as_formatted() {
        for (display, text) in [
            (UnitDisplay::Short, "23.5 inch"),
            (UnitDisplay::Narrow, "23.5inch"),
            (UnitDisplay::Long, "23.5 inchs"),
        ] {
            let units = number_parser(
                "en-US",
                NumberFormatOptions {
                    style: NumberStyle::Unit,
                    unit: Some("inch".to_owned()),
                    unit_display: display,
                    ..NumberFormatOptions::default()
                },
            );
            assert_that!(units.parse::<f64>(text)).is_equal_to(Some(23.5));
            assert_that!(units.parse::<f64>("23.5 ft")).is_none();
            assert_that!(units.parse::<f64>("23.5 i")).is_none();
        }
    }

    #[test]
    fn parses_percents() {
        let parser = number_parser("en-US", percent(NumberFormatOptions::default()));
        assert_that!(parser.parse::<f64>("10%")).is_equal_to(Some(0.1));
        // Rounded as displayed: no fraction digits.
        assert_that!(parser.parse::<f64>("10.5%")).is_equal_to(Some(0.11));
        let two_digits = number_parser(
            "en-US",
            percent(NumberFormatOptions {
                minimum_fraction_digits: Some(2),
                ..NumberFormatOptions::default()
            }),
        );
        assert_that!(two_digits.parse::<f64>("10.5%")).is_equal_to(Some(0.105));
    }

    #[test]
    fn parses_percents_with_signs() {
        let always = number_parser("en-GB", percent(signs(SignDisplay::Always)));
        assert_that!(always.parse::<f64>("+10%")).is_equal_to(Some(0.1));
        assert_that!(always.parse::<f64>("+0%")).is_equal_to(Some(0.0));
        assert_that!(always.parse::<f64>("-10%")).is_equal_to(Some(-0.1));
        assert_that!(always.parse::<f64>("-0%")).is_equal_to(Some(-0.0));
        let except_zero = number_parser(
            "en-US",
            percent(NumberFormatOptions {
                sign_display: SignDisplay::ExceptZero,
                minimum_fraction_digits: Some(2),
                ..NumberFormatOptions::default()
            }),
        );
        assert_that!(except_zero.parse::<f64>("+0.50%")).is_equal_to(Some(0.005));
    }

    #[test]
    fn parses_percents_with_decimals_and_except_zero() {
        let parser = number_parser("en-GB", percent(signs(SignDisplay::ExceptZero)));
        assert_that!(parser.parse::<f64>("+0.532%")).is_equal_to(Some(0.01));
        assert_that!(parser.parse::<f64>("+0%")).is_equal_to(Some(0.0));
        assert_that!(parser.parse::<f64>("0.532%")).is_equal_to(Some(0.01));
        assert_that!(parser.parse::<f64>("-0.532%")).is_equal_to(Some(-0.01));
    }

    #[test]
    fn percent_signs_are_invalid_in_decimal_style() {
        assert_that!(decimal("en-US").parse::<f64>("10%")).is_none();
        assert_that!(valid(&decimal("en-US"), "10%")).is_false();
    }

    #[test]
    fn parses_swiss_group_separators() {
        let chf = number_parser(
            "de-CH",
            NumberFormatOptions {
                style: NumberStyle::Currency,
                currency: Some("CHF".to_owned()),
                ..NumberFormatOptions::default()
            },
        );
        assert_that!(chf.parse::<f64>("CHF 1\u{2019}000.00")).is_equal_to(Some(1000.0));
        assert_that!(chf.parse::<f64>("CHF 1'000.00")).is_equal_to(Some(1000.0));
    }

    #[test]
    fn parses_french_group_separators() {
        let parser = decimal("fr-FR");
        for text in ["1\u{202f}000,5", "1\u{a0}000,5", "1 000,5"] {
            assert_that!(parser.parse::<f64>(text)).is_equal_to(Some(1000.5));
        }
    }

    #[test]
    fn parses_locales_with_non_latin_default_digits() {
        // Arabic-Indic digits, separators and minus sign (ar-EG), Bengali (bn), Devanagari (mr).
        let arabic = decimal("ar-EG");
        assert_that!(arabic.parse::<f64>("١٢")).is_equal_to(Some(12.0));
        assert_that!(arabic.parse::<f64>("١٬٢٣٤٫٥")).is_equal_to(Some(1234.5));
        assert_that!(arabic.parse::<f64>("\u{61c}-١٢")).is_equal_to(Some(-12.0));
        assert_that!(arabic.parse::<f64>("-١٢")).is_equal_to(Some(-12.0));
        assert_that!(arabic.parse::<f64>("12.5")).is_equal_to(Some(12.5));
        assert_that!(decimal("bn").parse::<f64>("১,২৩৪.৫")).is_equal_to(Some(1234.5));
        assert_that!(decimal("mr").parse::<f64>("१२३")).is_equal_to(Some(123.0));
        assert_that!(decimal("fa").parse::<f64>("۱۲٫۵")).is_equal_to(Some(12.5));
    }

    #[test]
    fn accepts_comma_and_arabic_comma_as_the_arabic_decimal_separator() {
        let arabic = decimal("ar-EG");
        assert_that!(arabic.parse::<f64>("١٢,٥")).is_equal_to(Some(12.5));
        assert_that!(arabic.parse::<f64>("١٢\u{60c}٥")).is_equal_to(Some(12.5));
        assert_that!(arabic.parse::<f64>("١.٢٣٤,٥")).is_equal_to(Some(1234.5));
    }

    #[test]
    fn parses_other_numbering_systems() {
        let parser = decimal("en-US");
        assert_that!(parser.parse::<f64>("١٢")).is_equal_to(Some(12.0));
        assert_that!(parser.parse::<f64>("١٫٢")).is_equal_to(Some(1.2));
        assert_that!(parser.parse::<f64>("一二.五")).is_equal_to(Some(12.5));
        assert_that!(parser.parse::<f64>("१२४,२")).is_equal_to(Some(1242.0));
        assert_that!(parser.parse::<f64>("১.২৫৩")).is_equal_to(Some(1.253));
    }

    #[test]
    fn integers_parse_exactly_and_saturate() {
        let parser = decimal("en-US");
        assert_that!(parser.parse::<u64>("18,446,744,073,709,551,615")).is_equal_to(Some(u64::MAX));
        assert_that!(parser.parse::<u8>("256")).is_equal_to(Some(255));
        assert_that!(parser.parse::<i8>("-1,000")).is_equal_to(Some(-128));
        assert_that!(parser.parse::<i32>("1.5")).is_none();
        assert_that!(parser.parse::<i32>("-7")).is_equal_to(Some(-7));
    }

    #[test]
    fn parses_german() {
        let parser = decimal("de-DE");
        assert_that!(parser.parse::<f64>("1.234,56")).is_equal_to(Some(1234.56));
        assert_that!(parser.parse::<f64>("42,5")).is_equal_to(Some(42.5));
        assert_that!(parser.parse::<f64>("-1.234,56")).is_equal_to(Some(-1234.56));
    }

    /// The formatter's output parses back to the same text, in many locales, numbering systems,
    /// styles and options (a deterministic version of upstream's property test).
    #[test]
    fn round_trips() {
        let locales = [
            "en-US", "de-DE", "de-CH", "fr-FR", "fi-FI", "ar-EG", "ar-AE", "fa", "bn", "mr",
            "ja-JP", "pl-PL", "ru-RU", "he-IL", "hi-IN",
        ];
        let systems = [
            None,
            Some(NumberingSystem::Latn),
            Some(NumberingSystem::Arab),
            Some(NumberingSystem::HaniDec),
            Some(NumberingSystem::Deva),
            Some(NumberingSystem::Beng),
        ];
        let styles = [
            NumberFormatOptions::default(),
            no_grouping(),
            percent(NumberFormatOptions::default()),
            percent(NumberFormatOptions {
                minimum_fraction_digits: Some(2),
                ..NumberFormatOptions::default()
            }),
            currency(CurrencyDisplay::Symbol, CurrencySign::Standard),
            currency(CurrencyDisplay::Code, CurrencySign::Accounting),
            currency(CurrencyDisplay::Name, CurrencySign::Standard),
            NumberFormatOptions {
                style: NumberStyle::Unit,
                unit: Some("liter".to_owned()),
                unit_display: UnitDisplay::Long,
                ..NumberFormatOptions::default()
            },
            NumberFormatOptions {
                minimum_integer_digits: Some(4),
                maximum_significant_digits: Some(1),
                ..NumberFormatOptions::default()
            },
            NumberFormatOptions {
                minimum_significant_digits: Some(4),
                maximum_significant_digits: Some(6),
                ..NumberFormatOptions::default()
            },
            signs(SignDisplay::Always),
        ];
        let values = [
            0.0,
            1.0,
            -1.0,
            0.5,
            -12.25,
            1234.5678,
            -98_765.432_1,
            60_048.95,
            2.220_446_049_250_313e-16,
            1e15,
        ];
        for locale in locales {
            for system in systems {
                let locale_with_system = match system {
                    Some(system) => system.apply_to(&locale.parse().expect("a locale")),
                    None => locale.parse().expect("a locale"),
                };
                for options in &styles {
                    let formatter = NumberFormatter::new(&locale_with_system, options.clone());
                    let parser = NumberParser::new(&locale.parse().expect("a locale"), options);
                    for value in values {
                        let formatted = formatter.format(value);
                        let parsed = parser.parse::<f64>(&formatted);
                        assert_that!(parsed.map(|parsed| formatter.format(parsed)))
                            .with_detail_message(format!(
                                "{locale} {system:?} {options:?}: {value} as {formatted:?}"
                            ))
                            .is_equal_to(Some(formatted.clone()));
                    }
                }
            }
        }
    }

    // "should handle percent with minimum integer digits", "should handle non-grouping in
    // russian locale".
    #[test]
    fn round_trips_upstream_counterexamples() {
        let cases = [
            (
                "ar-AE-u-nu-latn",
                percent(NumberFormatOptions {
                    minimum_integer_digits: Some(4),
                    minimum_fraction_digits: Some(9),
                    maximum_significant_digits: Some(1),
                    ..NumberFormatOptions::default()
                }),
                0.0095,
            ),
            (
                "ru-RU",
                percent(NumberFormatOptions {
                    use_grouping: false,
                    ..NumberFormatOptions::default()
                }),
                2.220_446_049_250_313e-16,
            ),
        ];
        for (locale, options, value) in cases {
            let locale: Locale = locale.parse().expect("a locale");
            let formatter = NumberFormatter::new(&locale, options.clone());
            let parser = NumberParser::new(&locale, &options);
            let formatted = formatter.format(value);
            let parsed = parser.parse::<f64>(&formatted);
            assert_that!(parsed.map(|parsed| formatter.format(parsed)))
                .with_detail_message(format!("{locale:?}: {formatted:?}"))
                .is_equal_to(Some(formatted.clone()));
        }
    }

    // ---- is_valid_partial_number ----

    #[test]
    fn partial_basic_numbers() {
        let parser = decimal("en-US");
        assert_that!(valid(&parser, "")).is_true();
        assert_that!(valid(&parser, "10")).is_true();
        assert_that!(valid(&parser, "-10")).is_true();
        assert_that!(valid(&parser, "-")).is_true();
    }

    #[test]
    fn partial_decimals() {
        let parser = decimal("en-US");
        for text in ["10.5", "-10.5", ".", ".5", "1.", "-1."] {
            assert_that!(valid(&parser, text))
                .with_detail_message(text)
                .is_true();
        }
        assert_that!(valid(&parser, "1.2.3")).is_false();
        // An Arabic decimal separator in an Arabic locale (ar-AE: Latin digits by default).
        let arabic = decimal("ar-AE");
        for text in ["٫", "٫٥", ".", ".5"] {
            assert_that!(valid(&arabic, text))
                .with_detail_message(text)
                .is_true();
        }
    }

    #[test]
    fn partial_group_characters() {
        let parser = decimal("en-US");
        for text in [
            ",",
            ",000",
            "000,000",
            "1,000",
            "-1,000",
            "1,000,000",
            "-1,000,000",
        ] {
            assert_that!(valid(&parser, text))
                .with_detail_message(text)
                .is_true();
        }
    }

    #[test]
    fn partial_invalid_grouping() {
        assert_that!(valid(&number_parser("en-US", no_grouping()), "1234,7")).is_false();
        assert_that!(valid(&number_parser("de-DE", no_grouping()), "1234.7")).is_false();
    }

    #[test]
    fn partial_rejects_random_characters() {
        assert_that!(valid(&decimal("en-US"), "g")).is_false();
        assert_that!(valid(&decimal("en-US"), "1abc")).is_false();
        assert_that!(valid(&decimal("en-US"), "1a")).is_false();
    }

    #[test]
    fn partial_sign_display() {
        let parser = decimal("en-US");
        assert_that!(valid(&parser, "+")).is_false();
        assert_that!(valid(&parser, "+10")).is_false();
        let always = number_parser("en-US", signs(SignDisplay::Always));
        assert_that!(valid(&always, "+")).is_true();
        assert_that!(valid(&always, "+10")).is_true();
    }

    #[test]
    fn partial_minus_signs() {
        let english = decimal("en-US");
        assert_that!(valid(&english, "-")).is_true();
        assert_that!(valid(&english, "-10")).is_true();
        assert_that!(valid(&english, "\u{2212}")).is_false();
        assert_that!(valid(&english, "\u{2212}10")).is_false();
        let finnish = decimal("fi-FI");
        for text in ["-", "-10", "\u{2212}", "\u{2212}10"] {
            assert_that!(valid(&finnish, text))
                .with_detail_message(text)
                .is_true();
        }
    }

    #[test]
    fn partial_negative_numbers_need_a_negative_minimum() {
        let parser = decimal("en-US");
        assert_that!(parser.is_valid_partial_number("-", Some(0.0), None)).is_false();
        assert_that!(parser.is_valid_partial_number("-", Some(10.0), None)).is_false();
        assert_that!(parser.is_valid_partial_number("-10", Some(0.0), None)).is_false();
        assert_that!(usd().is_valid_partial_number("-$", Some(0.0), None)).is_false();
        assert_that!(usd().is_valid_partial_number("-$1", Some(0.0), None)).is_false();
        assert_that!(parser.is_valid_partial_number("1", Some(0.0), None)).is_true();
        assert_that!(parser.is_valid_partial_number::<u8>("-", None, None)).is_false();
        assert_that!(parser.is_valid_partial_number::<i8>("-", None, None)).is_true();
    }

    #[test]
    fn partial_positive_numbers_need_a_positive_maximum() {
        let always = number_parser("en-US", signs(SignDisplay::Always));
        assert_that!(always.is_valid_partial_number("+", Some(-10.0), Some(-5.0))).is_false();
        assert_that!(always.is_valid_partial_number("+", Some(-10.0), Some(0.0))).is_false();
        assert_that!(always.is_valid_partial_number("+10", Some(-10.0), Some(-5.0))).is_false();
        let usd_always = number_parser(
            "en-US",
            NumberFormatOptions {
                sign_display: SignDisplay::Always,
                ..currency(CurrencyDisplay::Symbol, CurrencySign::Standard)
            },
        );
        assert_that!(usd_always.is_valid_partial_number("+$", Some(-10.0), Some(-5.0))).is_false();
        assert_that!(usd_always.is_valid_partial_number("+$1", Some(-10.0), Some(-5.0))).is_false();
    }

    #[test]
    fn partial_currency() {
        let symbol = usd();
        for text in ["10", "10.5", "$10", "$10.5"] {
            assert_that!(valid(&symbol, text))
                .with_detail_message(text)
                .is_true();
        }
        let code = number_parser(
            "en-US",
            currency(CurrencyDisplay::Code, CurrencySign::Standard),
        );
        assert_that!(valid(&code, "USD 10")).is_true();
        assert_that!(valid(&code, "US 10")).is_false();
        let name = number_parser(
            "en-US",
            currency(CurrencyDisplay::Name, CurrencySign::Standard),
        );
        assert_that!(valid(&name, "10 US dollars")).is_true();
        assert_that!(valid(&name, "10 US d")).is_false();
        assert_that!(valid(&symbol, "(")).is_false();
        let accounting = accounting(CurrencyDisplay::Symbol);
        for text in ["(", "($10)", "-", "-10", "-$10"] {
            assert_that!(valid(&accounting, text))
                .with_detail_message(text)
                .is_true();
        }
        // Latin characters and an Arabic letter mark in an Arabic locale.
        let arabic = number_parser(
            "ar-AE",
            currency(CurrencyDisplay::Symbol, CurrencySign::Accounting),
        );
        for text in ["(\u{61c}", "(\u{61c}10)", "-", "-10"] {
            assert_that!(valid(&arabic, text))
                .with_detail_message(text)
                .is_true();
        }
    }

    #[test]
    fn partial_units() {
        let units = number_parser(
            "en-US",
            NumberFormatOptions {
                style: NumberStyle::Unit,
                unit: Some("inch".to_owned()),
                ..NumberFormatOptions::default()
            },
        );
        for (text, expected) in [
            ("10", true),
            ("10.5", true),
            ("10 inch", true),
            ("10.5 inch", true),
            ("10 i", false),
            ("10.5 i", false),
        ] {
            assert_that!(valid(&units, text))
                .with_detail_message(text)
                .is_equal_to(expected);
        }
    }

    #[test]
    fn partial_percents() {
        let plain = number_parser("en-US", percent(NumberFormatOptions::default()));
        let min_two = number_parser(
            "en-US",
            percent(NumberFormatOptions {
                minimum_fraction_digits: Some(2),
                ..NumberFormatOptions::default()
            }),
        );
        let max_two = number_parser(
            "en-US",
            percent(NumberFormatOptions {
                maximum_fraction_digits: Some(2),
                ..NumberFormatOptions::default()
            }),
        );
        for (parser, text, expected) in [
            (&plain, "10", true),
            (&plain, "10.5", false),
            (&min_two, "10.5", true),
            (&max_two, "10.5", true),
            (&plain, "10%", true),
            (&plain, "10.5%", false),
            (&min_two, "10.5%", true),
            (&max_two, "10.5%", true),
            (&plain, "%", true),
            (&plain, "10 %", true),
        ] {
            assert_that!(valid(parser, text))
                .with_detail_message(text)
                .is_equal_to(expected);
        }
    }

    #[test]
    fn partial_integers_and_no_fraction_digits() {
        let parser = decimal("en-US");
        assert_that!(parser.is_valid_partial_number::<i32>("1.", None, None)).is_false();
        assert_that!(parser.is_valid_partial_number::<f32>("1.", None, None)).is_true();
        let no_decimals = number_parser(
            "en-US",
            NumberFormatOptions {
                maximum_fraction_digits: Some(0),
                ..NumberFormatOptions::default()
            },
        );
        assert_that!(valid(&no_decimals, "1.")).is_false();
        assert_that!(valid(&no_decimals, "123")).is_true();
    }

    // A maximum of 0 fraction digits lowers the currency's default minimum of 2: no decimal point.
    #[test]
    fn partial_currency_without_fraction_digits() {
        let no_cents = number_parser(
            "en-US",
            NumberFormatOptions {
                maximum_fraction_digits: Some(0),
                ..currency(CurrencyDisplay::Symbol, CurrencySign::Standard)
            },
        );
        assert_that!(valid(&no_cents, "$5.")).is_false();
        assert_that!(valid(&no_cents, "$5")).is_true();
    }

    #[test]
    fn partial_non_latin_default_digits() {
        let arabic = decimal("ar-EG");
        for text in ["١٢", "١٢٫", "-١٢", "١٬٢٣٤", "12"] {
            assert_that!(valid(&arabic, text))
                .with_detail_message(text)
                .is_true();
        }
        assert_that!(valid(&arabic, "١a")).is_false();
    }

    // ---- numbering_system ----

    #[test]
    fn numbering_system_defaults_to_the_locale() {
        let english = decimal("en-US");
        for text in [" ", "12", ".", "12.5"] {
            assert_that!(english.numbering_system(text)).is_equal_to(NumberingSystem::Latn);
        }
        let arabic = decimal("ar-EG");
        for text in ["١٢", "٫", "١٫٢"] {
            assert_that!(arabic.numbering_system(text)).is_equal_to(NumberingSystem::Arab);
        }
    }

    #[test]
    fn numbering_system_detects_other_systems() {
        let english = decimal("en-US");
        for (text, system) in [
            ("١٢", NumberingSystem::Arab),
            ("٫", NumberingSystem::Arab),
            ("١٫٢", NumberingSystem::Arab),
            ("一二", NumberingSystem::HaniDec),
            ("一二.五", NumberingSystem::HaniDec),
            ("१२३४", NumberingSystem::Deva),
            ("१२४,२", NumberingSystem::Deva),
            ("२.३५१", NumberingSystem::Deva),
            ("১২৩", NumberingSystem::Beng),
            ("১.২৫৩", NumberingSystem::Beng),
            ("১২৮,৪", NumberingSystem::Beng),
        ] {
            assert_that!(english.numbering_system(text))
                .with_detail_message(text)
                .is_equal_to(system);
        }
        // ar-AE: Latin digits by default, Arabic ones detected.
        let arabic = decimal("ar-AE");
        for text in ["12", ".", "12.5"] {
            assert_that!(arabic.numbering_system(text)).is_equal_to(NumberingSystem::Latn);
        }
        for text in ["١٢", "٫", "١٫٢"] {
            assert_that!(arabic.numbering_system(text)).is_equal_to(NumberingSystem::Arab);
        }
        // A locale with a numbering system keeps it.
        let fixed = decimal("en-US-u-nu-latn");
        assert_that!(fixed.numbering_system("١٢")).is_equal_to(NumberingSystem::Latn);
    }
}
