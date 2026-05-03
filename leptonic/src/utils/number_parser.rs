// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@internationalized/number/src/NumberParser.ts

use super::{
    i18n::Locale,
    number_formatter::{NumberFormatOptions, NumberFormatter, NumberStyle},
};

/// Locale-specific number symbols extracted by formatting probe values.
#[derive(Debug, Clone)]
struct NumberSymbols {
    /// The decimal separator (e.g., "." for en-US, "," for de-DE).
    decimal: String,
    /// The group (thousands) separator (e.g., "," for en-US, "." for de-DE).
    group: String,
    /// The minus sign character(s).
    minus: String,
    /// Mapping from locale numeral characters to ASCII digits (0-9).
    /// `None` for Latin numeral locales (ASCII digits are used directly).
    numeral_map: Option<Vec<(char, char)>>,
    /// The percent sign character(s).
    percent: String,
    /// Literal characters to strip (currency symbols, unit strings, etc.).
    literals: Vec<String>,
}

/// A locale-aware number parser backed by ICU4X.
///
/// The inverse of [`NumberFormatter`]. Discovers locale-specific symbols by formatting
/// probe values through `NumberFormatter`, then uses those symbols to parse
/// locale-formatted strings back to `f64`.
///
/// Based on react-aria's `NumberParser` from `@internationalized/number`.
#[derive(Debug, Clone)]
pub struct NumberParser {
    symbols: NumberSymbols,
    style: NumberStyle,
    max_fraction_digits: Option<u32>,
}

impl NumberParser {
    /// Creates a new number parser for the given locale and format options.
    #[must_use]
    pub fn new(locale: &Locale, options: &NumberFormatOptions) -> Self {
        let symbols = discover_symbols(locale, options);
        Self {
            symbols,
            style: options.style,
            max_fraction_digits: options.maximum_fraction_digits,
        }
    }

    /// Parse a locale-formatted string into `f64`.
    ///
    /// Returns `None` if the string is empty or cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let parser = NumberParser::new(&Locale::new("de-DE"), &NumberFormatOptions::default());
    /// assert_eq!(parser.parse("1.234,56"), Some(1234.56));
    /// ```
    #[must_use]
    pub fn parse(&self, value: &str) -> Option<f64> {
        let sanitized = self.sanitize(value);
        if sanitized.is_empty() {
            return None;
        }

        let mut s = sanitized;

        // Remove group separators.
        if !self.symbols.group.is_empty() {
            s = s.replace(&self.symbols.group, "");
        }

        // Replace locale decimal with ASCII '.'.
        if !self.symbols.decimal.is_empty() && self.symbols.decimal != "." {
            s = s.replace(&self.symbols.decimal, ".");
        }

        // Replace locale minus with ASCII '-'.
        if !self.symbols.minus.is_empty() && self.symbols.minus != "-" {
            s = s.replace(&self.symbols.minus, "-");
        }

        // Transliterate non-Latin numerals to ASCII 0-9.
        if let Some(ref numeral_map) = self.symbols.numeral_map {
            for &(locale_char, ascii_char) in numeral_map {
                s = s.replace(locale_char, &ascii_char.to_string());
            }
        }

        // Strip percent sign and handle percent-style division.
        let is_percent = self.style == NumberStyle::Percent || s.contains(&self.symbols.percent);
        s = s.replace(&self.symbols.percent, "");

        // Parse the ASCII number string.
        let mut result: f64 = s.trim().parse().ok()?;

        // For percent style: the value represents a percentage, divide by 100.
        if is_percent {
            // Use string manipulation to avoid floating-point precision loss,
            // following react-aria's approach.
            result /= 100.0;
        }

        Some(result)
    }

    /// Returns `true` if the string is a valid partial number input for this locale.
    ///
    /// This is used by the `beforeinput` handler to filter keystrokes in real-time.
    /// Allows intermediate states like `"-"`, `"1."`, `"1,2"` (German) while typing.
    #[must_use]
    pub fn is_valid_partial_number(
        &self,
        value: &str,
        min_value: Option<f64>,
        max_value: Option<f64>,
    ) -> bool {
        if value.is_empty() {
            return true;
        }

        let sanitized = self.sanitize(value);
        if sanitized.is_empty() {
            // All characters were literals (e.g., just a currency symbol) — allow it.
            return true;
        }

        let mut s = sanitized.as_str();

        // Allow a leading minus sign if negative values are possible.
        let allows_negative = min_value.is_none_or(|min| min < 0.0);
        if allows_negative && s.starts_with(self.symbols.minus.as_str()) {
            s = &s[self.symbols.minus.len()..];
        } else if allows_negative && s.starts_with('-') {
            s = &s[1..];
        }

        // Allow a leading plus sign if positive values are possible.
        let allows_positive = max_value.is_none_or(|max| max > 0.0);
        if allows_positive && s.starts_with('+') {
            s = &s[1..];
        }

        if s.is_empty() {
            // Just a sign character — valid partial input.
            return true;
        }

        // Reject if starts with group separator.
        if !self.symbols.group.is_empty() && s.starts_with(self.symbols.group.as_str()) {
            return false;
        }

        // Reject if decimal separator is present but max fraction digits is 0.
        if self.max_fraction_digits == Some(0) && s.contains(self.symbols.decimal.as_str()) {
            return false;
        }

        // Remove group separators for validation.
        let s = if self.symbols.group.is_empty() {
            s.to_string()
        } else {
            s.replace(&self.symbols.group, "")
        };

        // Allow at most one decimal separator.
        let decimal_count = s.matches(self.symbols.decimal.as_str()).count();
        if decimal_count > 1 {
            return false;
        }

        // Remove the decimal separator.
        let s = s.replace(&self.symbols.decimal, "");

        // Remove percent sign.
        let s = s.replace(&self.symbols.percent, "");

        // All remaining characters must be digits (ASCII or locale-specific).
        s.chars().all(|c| self.is_numeral(c))
    }

    /// Strip literal characters (currency symbols, unit strings, whitespace).
    fn sanitize(&self, value: &str) -> String {
        let mut s = value.to_string();
        for literal in &self.symbols.literals {
            s = s.replace(literal.as_str(), "");
        }
        // Remove various whitespace characters (regular space, non-breaking space, narrow no-break space).
        s = s.replace('\u{00a0}', ""); // NBSP
        s = s.replace('\u{202f}', ""); // Narrow NBSP
        s.trim().to_string()
    }

    /// Returns whether a character is a valid numeral (ASCII or locale-specific).
    fn is_numeral(&self, c: char) -> bool {
        if c.is_ascii_digit() {
            return true;
        }
        if let Some(ref numeral_map) = self.symbols.numeral_map {
            return numeral_map.iter().any(|&(locale_char, _)| locale_char == c);
        }
        false
    }
}

/// Discover locale-specific number symbols by formatting probe values.
fn discover_symbols(locale: &Locale, options: &NumberFormatOptions) -> NumberSymbols {
    // Create a formatter with grouping enabled and known fraction digits
    // to produce predictable output for symbol extraction.
    let probe_options = NumberFormatOptions {
        style: NumberStyle::Decimal,
        use_grouping: true,
        minimum_fraction_digits: Some(3),
        maximum_fraction_digits: Some(3),
        ..Default::default()
    };
    let formatter = NumberFormatter::new(locale, probe_options);

    // Format probe values to extract symbols.
    // Format a large number with decimals to find group and decimal separators.
    let positive_probe = formatter.format(12345.111);
    let negative_probe = formatter.format(-1.0);

    // Extract decimal separator: the character between "345" and "111" in the formatted output.
    // We look for the separator between the integer and fraction parts.
    let decimal = extract_decimal_separator(&positive_probe);

    // Extract group separator: the character between "12" and "345" in the formatted output.
    let group = extract_group_separator(&positive_probe, &decimal);

    // Extract minus sign from the negative probe.
    let minus = extract_minus_sign(&negative_probe);

    // Extract percent sign.
    let percent = if options.style == NumberStyle::Percent {
        let percent_formatter = NumberFormatter::new(
            locale,
            NumberFormatOptions {
                style: NumberStyle::Percent,
                ..Default::default()
            },
        );
        let percent_probe = percent_formatter.format(0.01);
        extract_percent_sign(&percent_probe)
    } else {
        "%".to_string()
    };

    // Discover numeral mapping for non-Latin locales.
    let numeral_map = discover_numeral_map(locale);

    // Discover literal characters to strip.
    let literals = discover_literals(locale, options);

    NumberSymbols {
        decimal,
        group,
        minus,
        numeral_map,
        percent,
        literals,
    }
}

/// Extract the decimal separator from a formatted number like "12,345.111" or "12.345,111".
fn extract_decimal_separator(formatted: &str) -> String {
    // The formatted string is something like "12,345.111" (en-US) or "12.345,111" (de-DE).
    // The last 3 characters are always "111" (our probe fraction).
    // The character just before "111" is the decimal separator.
    let chars: Vec<char> = formatted.chars().collect();
    if chars.len() >= 4 {
        // Find the position of "111" from the end.
        if let Some(pos) = formatted.rfind("111") {
            if pos > 0 {
                let sep_end = pos;
                // Walk backward to find start of separator (could be multi-char).
                let sep_start = sep_end - 1;
                return formatted[sep_start..sep_end].to_string();
            }
        }
    }
    ".".to_string()
}

/// Extract the group separator from a formatted number like "12,345.111".
fn extract_group_separator(formatted: &str, decimal_sep: &str) -> String {
    // The formatted probe is 12345.111 -> something like "12,345.111".
    // Split at the decimal separator, take the integer part.
    let integer_part = formatted.split(decimal_sep).next().unwrap_or(formatted);

    // The group separator is between "12" and "345".
    // Find any non-digit character in the integer part.
    let mut group = String::new();
    for c in integer_part.chars() {
        if !c.is_ascii_digit() {
            group.push(c);
            break;
        }
    }
    group
}

/// Extract the minus sign from a formatted negative number like "-1" or "−1".
fn extract_minus_sign(formatted: &str) -> String {
    // Remove digits and common separators, what remains includes the minus sign.
    let cleaned: String = formatted
        .chars()
        .filter(|c| !c.is_ascii_digit() && *c != '.' && *c != ',')
        .collect();
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        "-".to_string()
    } else {
        trimmed.to_string()
    }
}

/// Extract the percent sign from a formatted percentage like "1%" or "1 %".
fn extract_percent_sign(formatted: &str) -> String {
    let cleaned: String = formatted
        .chars()
        .filter(|c| !c.is_ascii_digit() && !c.is_whitespace() && *c != '.' && *c != ',')
        .collect();
    if cleaned.is_empty() {
        "%".to_string()
    } else {
        cleaned
    }
}

/// Discover numeral mapping for non-Latin locales.
/// Returns `None` for Latin-numeral locales (most Western locales).
#[allow(clippy::similar_names)]
fn discover_numeral_map(locale: &Locale) -> Option<Vec<(char, char)>> {
    let formatter = NumberFormatter::new(
        locale,
        NumberFormatOptions {
            style: NumberStyle::Decimal,
            maximum_fraction_digits: Some(0),
            ..Default::default()
        },
    );

    let mut map = Vec::new();
    let mut all_ascii = true;

    for digit in 0..=9 {
        #[allow(clippy::cast_precision_loss)]
        let formatted = formatter.format(f64::from(digit));
        let formatted = formatted.trim();
        if let Some(c) = formatted.chars().next() {
            if !c.is_ascii_digit() {
                all_ascii = false;
            }
            let ascii_digit = char::from_digit(digit, 10).unwrap();
            map.push((c, ascii_digit));
        }
    }

    if all_ascii { None } else { Some(map) }
}

/// Discover literal characters that should be stripped during parsing.
fn discover_literals(locale: &Locale, options: &NumberFormatOptions) -> Vec<String> {
    let mut literals = Vec::new();

    match options.style {
        NumberStyle::Currency => {
            if let Some(ref currency) = options.currency {
                // Add common currency symbols.
                let symbol_formatter = NumberFormatter::new(
                    locale,
                    NumberFormatOptions {
                        style: NumberStyle::Currency,
                        currency: Some(currency.clone()),
                        currency_display: options.currency_display,
                        maximum_fraction_digits: Some(0),
                        ..Default::default()
                    },
                );
                let probe = symbol_formatter.format(0.0);
                // Extract non-digit, non-separator characters as literals.
                let literal: String = probe
                    .chars()
                    .filter(|c| !c.is_ascii_digit() && *c != '.' && *c != ',' && *c != '-')
                    .collect();
                let literal = literal.trim().to_string();
                if !literal.is_empty() {
                    literals.push(literal);
                }
            }
        }
        NumberStyle::Unit => {
            if let Some(ref unit) = options.unit {
                literals.push(unit.clone());
                // Also add common unit suffixes with spaces.
                literals.push(format!(" {unit}"));
                literals.push(format!(" {unit}s"));
            }
        }
        NumberStyle::Percent | NumberStyle::Decimal => {}
    }

    literals
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::utils::number_formatter::NumberFormatOptions;

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_parse_en_us_simple() {
        let locale = Locale::new("en-US");
        let parser = NumberParser::new(&locale, &NumberFormatOptions::default());

        assert_that(parser.parse("123")).is_equal_to(Some(123.0));
        assert_that(parser.parse("0")).is_equal_to(Some(0.0));
        assert_that(parser.parse("-42")).is_equal_to(Some(-42.0));
        assert_that(parser.parse("3.14")).is_equal_to(Some(3.14));
    }

    #[test]
    fn test_parse_en_us_with_grouping() {
        let locale = Locale::new("en-US");
        let options = NumberFormatOptions {
            use_grouping: true,
            ..Default::default()
        };
        let parser = NumberParser::new(&locale, &options);

        assert_that(parser.parse("1,234")).is_equal_to(Some(1234.0));
        assert_that(parser.parse("1,234.56")).is_equal_to(Some(1234.56));
        assert_that(parser.parse("1,234,567")).is_equal_to(Some(1_234_567.0));
    }

    #[test]
    fn test_parse_de_de() {
        let locale = Locale::new("de-DE");
        let options = NumberFormatOptions {
            use_grouping: true,
            minimum_fraction_digits: Some(2),
            maximum_fraction_digits: Some(2),
            ..Default::default()
        };
        let parser = NumberParser::new(&locale, &options);

        // German uses '.' for grouping and ',' for decimal.
        assert_that(parser.parse("1.234,56")).is_equal_to(Some(1234.56));
        assert_that(parser.parse("42,5")).is_equal_to(Some(42.5));
        assert_that(parser.parse("-1.234,56")).is_equal_to(Some(-1234.56));
    }

    #[test]
    fn test_parse_empty_and_invalid() {
        let locale = Locale::new("en-US");
        let parser = NumberParser::new(&locale, &NumberFormatOptions::default());

        assert_that(parser.parse("")).is_equal_to(None);
        assert_that(parser.parse("abc")).is_equal_to(None);
        assert_that(parser.parse("12.34.56")).is_equal_to(None);
    }

    #[test]
    fn test_parse_percent() {
        let locale = Locale::new("en-US");
        let options = NumberFormatOptions {
            style: NumberStyle::Percent,
            ..Default::default()
        };
        let parser = NumberParser::new(&locale, &options);

        assert_that(parser.parse("75%")).is_equal_to(Some(0.75));
        assert_that(parser.parse("100%")).is_equal_to(Some(1.0));
    }

    #[test]
    fn test_is_valid_partial_en_us() {
        let locale = Locale::new("en-US");
        let parser = NumberParser::new(&locale, &NumberFormatOptions::default());

        // Valid partial inputs.
        assert_that(parser.is_valid_partial_number("", None, None)).is_true();
        assert_that(parser.is_valid_partial_number("1", None, None)).is_true();
        assert_that(parser.is_valid_partial_number("1.", None, None)).is_true();
        assert_that(parser.is_valid_partial_number("1.2", None, None)).is_true();
        assert_that(parser.is_valid_partial_number("-", None, None)).is_true();
        assert_that(parser.is_valid_partial_number("-1", None, None)).is_true();
        assert_that(parser.is_valid_partial_number("-1.", None, None)).is_true();

        // Invalid partial inputs.
        assert_that(parser.is_valid_partial_number("abc", None, None)).is_false();
        assert_that(parser.is_valid_partial_number("1.2.3", None, None)).is_false();
        assert_that(parser.is_valid_partial_number("1a", None, None)).is_false();
    }

    #[test]
    fn test_is_valid_partial_no_negative() {
        let locale = Locale::new("en-US");
        let parser = NumberParser::new(&locale, &NumberFormatOptions::default());

        // When min_value >= 0, minus is not allowed.
        assert_that(parser.is_valid_partial_number("-", Some(0.0), None)).is_false();
        assert_that(parser.is_valid_partial_number("-1", Some(0.0), None)).is_false();
        assert_that(parser.is_valid_partial_number("1", Some(0.0), None)).is_true();
    }

    #[test]
    fn test_is_valid_partial_no_decimals() {
        let locale = Locale::new("en-US");
        let options = NumberFormatOptions {
            maximum_fraction_digits: Some(0),
            ..Default::default()
        };
        let parser = NumberParser::new(&locale, &options);

        assert_that(parser.is_valid_partial_number("1.", None, None)).is_false();
        assert_that(parser.is_valid_partial_number("1.5", None, None)).is_false();
        assert_that(parser.is_valid_partial_number("123", None, None)).is_true();
    }

    #[test]
    fn test_is_valid_partial_rejects_leading_group_separator() {
        let locale = Locale::new("en-US");
        let options = NumberFormatOptions {
            use_grouping: true,
            ..Default::default()
        };
        let parser = NumberParser::new(&locale, &options);

        assert_that(parser.is_valid_partial_number(",123", None, None)).is_false();
        assert_that(parser.is_valid_partial_number("1,234", None, None)).is_true();
    }

    #[test]
    fn test_roundtrip_en_us() {
        let locale = Locale::new("en-US");
        let options = NumberFormatOptions {
            use_grouping: true,
            minimum_fraction_digits: Some(2),
            maximum_fraction_digits: Some(2),
            ..Default::default()
        };
        let formatter = NumberFormatter::new(&locale, options.clone());
        let parser = NumberParser::new(&locale, &options);

        let original = 1234.56;
        let formatted = formatter.format(original);
        let parsed = parser.parse(&formatted);
        assert_that(parsed).is_equal_to(Some(original));
    }

    #[test]
    fn test_roundtrip_de_de() {
        let locale = Locale::new("de-DE");
        let options = NumberFormatOptions {
            use_grouping: true,
            minimum_fraction_digits: Some(2),
            maximum_fraction_digits: Some(2),
            ..Default::default()
        };
        let formatter = NumberFormatter::new(&locale, options.clone());
        let parser = NumberParser::new(&locale, &options);

        let original = 1234.56;
        let formatted = formatter.format(original);
        let parsed = parser.parse(&formatted);
        assert_that(parsed).is_equal_to(Some(original));
    }
}
