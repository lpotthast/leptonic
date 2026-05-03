// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/i18n/src/useFilter.ts
// and https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/i18n/src/useCollator.ts

use std::cmp::Ordering;

use icu_collator::{CollatorBorrowed, options::Strength};
use icu_normalizer::ComposingNormalizer;

use super::i18n::Locale;

/// Collation sensitivity options for string comparison.
///
/// Controls how differences in letters are handled during comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CollatorSensitivity {
    /// Only distinguish between base letters (a ≠ b, a = á, a = A).
    #[default]
    Base,
    /// Distinguish base letters and accents (a ≠ b, a ≠ á, a = A).
    Accent,
    /// Distinguish base letters and case (a ≠ b, a = á, a ≠ A).
    Case,
    /// Distinguish base letters, accents, and case (a ≠ b, a ≠ á, a ≠ A).
    Variant,
}

impl CollatorSensitivity {
    fn to_icu_strength(self) -> Strength {
        match self {
            Self::Base => Strength::Primary,
            Self::Accent => Strength::Secondary,
            Self::Case => Strength::Tertiary,
            Self::Variant => Strength::Quaternary,
        }
    }
}

/// Options for creating a [`Collator`].
#[derive(Debug, Clone, Default)]
pub struct CollatorOptions {
    /// The sensitivity level for comparisons. Defaults to `Base`.
    pub sensitivity: CollatorSensitivity,

    /// Whether punctuation should be ignored. Defaults to `false`.
    pub ignore_punctuation: bool,
}

/// Locale-aware string collator using ICU4X.
///
/// Provides locale-sensitive string comparison, useful for sorting
/// (tables, lists) and searching. This mirrors react-aria's `useCollator`.
///
/// # Example
///
/// ```ignore
/// let collator = Collator::new(&Locale::new("en-US"), &CollatorOptions::default());
/// let ordering = collator.compare("apple", "banana");
/// assert_eq!(ordering, Ordering::Less);
/// ```
pub struct Collator {
    inner: CollatorBorrowed<'static>,
}

impl Collator {
    /// Creates a new collator with the given locale and options.
    ///
    /// - `locale`: A `Locale` for locale-sensitive comparison.
    /// - `options`: Sensitivity and punctuation options.
    ///
    /// # Panics
    ///
    /// Panics if ICU4X cannot create a collator for the given locale or the default locale.
    #[must_use]
    pub fn new(locale: &Locale, options: &CollatorOptions) -> Self {
        let mut icu_options = icu_collator::options::CollatorOptions::default();
        icu_options.strength = Some(options.sensitivity.to_icu_strength());

        let prefs = icu_collator::CollatorPreferences::from(locale.icu_locale());
        let inner = CollatorBorrowed::try_new(prefs, icu_options).unwrap_or_else(|_| {
            CollatorBorrowed::try_new(icu_collator::CollatorPreferences::default(), icu_options)
                .unwrap()
        });

        Self { inner }
    }

    /// Compares two strings according to the collator's locale and options.
    ///
    /// Returns `Ordering::Less`, `Ordering::Equal`, or `Ordering::Greater`.
    #[must_use]
    pub fn compare(&self, a: &str, b: &str) -> Ordering {
        self.inner.compare(a, b)
    }
}

/// NFC-normalizes a string using ICU4X.
fn normalize_nfc(s: &str) -> String {
    let normalizer = ComposingNormalizer::new_nfc();
    normalizer.normalize(s).into_owned()
}

/// Locale-aware string filtering with `contains`, `starts_with`, and `ends_with`.
///
/// Uses ICU4X `Collator` with locale-sensitive matching,
/// and NFC-normalizes strings before comparison. This mirrors react-aria's `useFilter`.
///
/// SSR-safe: uses pure Rust ICU4X instead of browser `Intl` APIs.
///
/// # Example
///
/// ```ignore
/// let filter = Filter::new(&Locale::new("en-US"), &CollatorOptions::default());
///
/// assert!(filter.contains("café", "cafe"));   // base sensitivity: accent-insensitive
/// assert!(filter.starts_with("Hello", "hello")); // base sensitivity: case-insensitive
/// assert!(filter.ends_with("world!", "WORLD!")); // base sensitivity: case-insensitive
/// ```
pub struct Filter {
    collator: Collator,
}

impl Filter {
    /// Creates a new filter with the given locale and options.
    #[must_use]
    pub fn new(locale: &Locale, options: &CollatorOptions) -> Self {
        Self {
            collator: Collator::new(locale, options),
        }
    }

    /// Returns `true` if `string` contains `substring` according to locale-aware comparison.
    ///
    /// Uses a sliding window over the NFC-normalized `string`, comparing each slice
    /// of `substring.len()` characters via the collator. An empty `substring` always matches.
    #[must_use]
    pub fn contains(&self, string: &str, substring: &str) -> bool {
        if substring.is_empty() {
            return true;
        }

        let string = normalize_nfc(string);
        let substring = normalize_nfc(substring);

        let slice_len = substring.len();
        if slice_len > string.len() {
            return false;
        }

        // Slide a window of `slice_len` bytes over `string`.
        // Only compare at valid UTF-8 char boundaries.
        let mut scan = 0;
        while scan + slice_len <= string.len() {
            // Ensure we're at a char boundary.
            if !string.is_char_boundary(scan) {
                scan += 1;
                continue;
            }
            let end = scan + slice_len;
            if !string.is_char_boundary(end) {
                scan += 1;
                continue;
            }

            let slice = &string[scan..end];
            if self.collator.compare(slice, &substring) == Ordering::Equal {
                return true;
            }

            scan += 1;
        }

        false
    }

    /// Returns `true` if `string` starts with `substring` according to locale-aware comparison.
    ///
    /// Compares the first `substring.len()` characters of `string` via the collator.
    /// An empty `substring` always matches.
    #[must_use]
    pub fn starts_with(&self, string: &str, substring: &str) -> bool {
        if substring.is_empty() {
            return true;
        }

        let string = normalize_nfc(string);
        let substring = normalize_nfc(substring);

        let slice_len = substring.len();
        if slice_len > string.len() {
            return false;
        }

        // Ensure the slice end is at a char boundary.
        if !string.is_char_boundary(slice_len) {
            return false;
        }

        self.collator.compare(&string[..slice_len], &substring) == Ordering::Equal
    }

    /// Returns `true` if `string` ends with `substring` according to locale-aware comparison.
    ///
    /// Compares the last `substring.len()` characters of `string` via the collator.
    /// An empty `substring` always matches.
    #[must_use]
    pub fn ends_with(&self, string: &str, substring: &str) -> bool {
        if substring.is_empty() {
            return true;
        }

        let string = normalize_nfc(string);
        let substring = normalize_nfc(substring);

        let slice_len = substring.len();
        if slice_len > string.len() {
            return false;
        }

        let start = string.len() - slice_len;

        // Ensure the slice start is at a char boundary.
        if !string.is_char_boundary(start) {
            return false;
        }

        self.collator.compare(&string[start..], &substring) == Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn default_filter(locale_str: &str) -> Filter {
        Filter::new(&Locale::new(locale_str), &CollatorOptions::default())
    }

    #[test]
    fn test_collator_basic_ordering() {
        let locale = Locale::new("en-US");
        let collator = Collator::new(&locale, &CollatorOptions::default());
        assert_that(collator.compare("apple", "banana")).is_equal_to(Ordering::Less);
        assert_that(collator.compare("banana", "apple")).is_equal_to(Ordering::Greater);
        assert_that(collator.compare("apple", "apple")).is_equal_to(Ordering::Equal);
    }

    #[test]
    fn test_collator_case_insensitive_with_base_sensitivity() {
        let locale = Locale::new("en-US");
        let collator = Collator::new(
            &locale,
            &CollatorOptions {
                sensitivity: CollatorSensitivity::Base,
                ..CollatorOptions::default()
            },
        );
        assert_that(collator.compare("Apple", "apple")).is_equal_to(Ordering::Equal);
    }

    #[test]
    fn test_collator_accent_insensitive_with_base_sensitivity() {
        let locale = Locale::new("en-US");
        let collator = Collator::new(
            &locale,
            &CollatorOptions {
                sensitivity: CollatorSensitivity::Base,
                ..CollatorOptions::default()
            },
        );
        assert_that(collator.compare("café", "cafe")).is_equal_to(Ordering::Equal);
    }

    #[test]
    fn test_filter_contains_empty_substring() {
        let filter = default_filter("en-US");
        assert_that(filter.contains("hello", "")).is_true();
    }

    #[test]
    fn test_filter_contains_basic() {
        let filter = default_filter("en-US");
        assert_that(filter.contains("hello world", "world")).is_true();
        assert_that(filter.contains("hello world", "xyz")).is_false();
    }

    #[test]
    fn test_filter_starts_with_basic() {
        let filter = default_filter("en-US");
        assert_that(filter.starts_with("hello world", "hello")).is_true();
        assert_that(filter.starts_with("hello world", "world")).is_false();
    }

    #[test]
    fn test_filter_ends_with_basic() {
        let filter = default_filter("en-US");
        assert_that(filter.ends_with("hello world", "world")).is_true();
        assert_that(filter.ends_with("hello world", "hello")).is_false();
    }

    #[test]
    fn test_filter_case_insensitive() {
        let filter = default_filter("en-US");
        assert_that(filter.contains("Hello World", "hello")).is_true();
        assert_that(filter.starts_with("Hello World", "hello")).is_true();
    }

    #[test]
    fn test_filter_with_german_locale() {
        let filter = default_filter("de-DE");
        assert_that(filter.contains("Straße", "strass")).is_true();
    }

    #[test]
    fn test_filter_with_japanese_locale() {
        let filter = default_filter("ja-JP");
        assert_that(filter.contains("東京都", "東京")).is_true();
    }
}
