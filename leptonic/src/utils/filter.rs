// Upstream: react-aria/src/i18n/useFilter.ts @ 6f664fe911
// Upstream: react-aria/src/i18n/useCollator.ts @ 6f664fe911
// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/react-aria/src/i18n/useFilter.ts
// and https://github.com/adobe/react-spectrum/blob/main/packages/react-aria/src/i18n/useCollator.ts

use std::{cmp::Ordering, sync::Arc};

use icu_collator::{
    CollatorBorrowed,
    options::{CaseLevel, Strength},
};
use icu_normalizer::ComposingNormalizer;
use leptos::prelude::*;

use super::i18n::{Locale, use_locale};

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
    /// ICU's strength and case level for the sensitivity (ECMA-402's mapping: "case" is primary
    /// strength with the case level, "variant" tertiary strength).
    fn to_icu(self) -> (Strength, CaseLevel) {
        match self {
            Self::Base => (Strength::Primary, CaseLevel::Off),
            Self::Accent => (Strength::Secondary, CaseLevel::Off),
            Self::Case => (Strength::Primary, CaseLevel::On),
            Self::Variant => (Strength::Tertiary, CaseLevel::Off),
        }
    }
}

/// Options for creating a [`Collator`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
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
/// let collator = Collator::new(&Locale::from(locale!("en-US")), &CollatorOptions::default());
/// let ordering = collator.compare("apple", "banana");
/// assert_eq!(ordering, Ordering::Less);
/// ```
#[derive(Debug)]
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
        let (strength, case_level) = options.sensitivity.to_icu();
        icu_options.strength = Some(strength);
        icu_options.case_level = Some(case_level);
        if options.ignore_punctuation {
            // `Intl.Collator`'s `ignorePunctuation`: punctuation and whitespace are ignored on
            // every level.
            icu_options.alternate_handling =
                Some(icu_collator::options::AlternateHandling::Shifted);
        }

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

/// A [`Collator`] for the current locale (react-aria: `useCollator`), created once per locale
/// and shared by every read (creating one per comparison or per keyboard delegate read is
/// costly).
pub fn use_collator(options: CollatorOptions) -> Signal<Arc<Collator>> {
    let locale = use_locale();
    // Recomputed only when the locale changes, and then always a change.
    Memo::new_with_compare(
        move |_| Arc::new(locale.with(|locale| Collator::new(locale, &options))),
        |_, _| true,
    )
    .into()
}

/// A [`Filter`] for the current locale (react-aria: `useFilter`), created once per locale.
pub fn use_filter(options: CollatorOptions) -> Signal<Arc<Filter>> {
    let locale = use_locale();
    Memo::new_with_compare(
        move |_| Arc::new(locale.with(|locale| Filter::new(locale, &options))),
        |_, _| true,
    )
    .into()
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
/// let filter = Filter::new(&Locale::from(locale!("en-US")), &CollatorOptions::default());
///
/// assert!(filter.contains("café", "cafe"));   // base sensitivity: accent-insensitive
/// assert!(filter.starts_with("Hello", "hello")); // base sensitivity: case-insensitive
/// assert!(filter.ends_with("world!", "WORLD!")); // base sensitivity: case-insensitive
/// ```
#[derive(Debug)]
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
    /// Slides a window of as many characters as `substring` has over the NFC-normalized
    /// `string`, comparing each window via the collator. An empty `substring` always matches.
    #[must_use]
    pub fn contains(&self, string: &str, substring: &str) -> bool {
        if substring.is_empty() {
            return true;
        }

        let string = normalize_nfc(string);
        let substring = normalize_nfc(substring);
        let window = substring.chars().count();

        // Byte offsets of every character start, plus the end of the string, so that the
        // windows can be sliced by character count (upstream slices by JS string length).
        let boundaries: Vec<usize> = string
            .char_indices()
            .map(|(index, _)| index)
            .chain(std::iter::once(string.len()))
            .collect();
        boundaries.windows(window + 1).any(|bounds| {
            self.collator
                .compare(&string[bounds[0]..bounds[window]], &substring)
                == Ordering::Equal
        })
    }

    /// Returns `true` if `string` starts with `substring` according to locale-aware comparison.
    ///
    /// Compares as many leading characters of `string` as `substring` has via the collator.
    /// An empty `substring` always matches.
    #[must_use]
    pub fn starts_with(&self, string: &str, substring: &str) -> bool {
        if substring.is_empty() {
            return true;
        }

        let string = normalize_nfc(string);
        let substring = normalize_nfc(substring);
        let window = substring.chars().count();

        // JS `slice` clamps: a shorter `string` is compared as a whole.
        let end = string
            .char_indices()
            .nth(window)
            .map_or(string.len(), |(index, _)| index);
        self.collator.compare(&string[..end], &substring) == Ordering::Equal
    }

    /// Returns `true` if `string` ends with `substring` according to locale-aware comparison.
    ///
    /// Compares as many trailing characters of `string` as `substring` has via the collator.
    /// An empty `substring` always matches.
    #[must_use]
    pub fn ends_with(&self, string: &str, substring: &str) -> bool {
        if substring.is_empty() {
            return true;
        }

        let string = normalize_nfc(string);
        let substring = normalize_nfc(substring);
        let window = substring.chars().count();

        // JS `slice` clamps: a shorter `string` is compared as a whole.
        let start = string
            .char_indices()
            .rev()
            .nth(window - 1)
            .map_or(0, |(index, _)| index);
        self.collator.compare(&string[start..], &substring) == Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::utils::i18n::locale;

    /// Each sensitivity tells apart what its documentation says (ECMA-402's `sensitivity`).
    #[test]
    fn sensitivities_distinguish_what_intl_does() {
        let en: Locale = "en-US".parse().expect("a locale");
        let equal = |sensitivity, a: &str, b: &str| {
            Collator::new(
                &en,
                &CollatorOptions {
                    sensitivity,
                    ..CollatorOptions::default()
                },
            )
            .compare(a, b)
                == Ordering::Equal
        };
        for (sensitivity, accent_equal, case_equal) in [
            (CollatorSensitivity::Base, true, true),
            (CollatorSensitivity::Accent, false, true),
            (CollatorSensitivity::Case, true, false),
            (CollatorSensitivity::Variant, false, false),
        ] {
            assert_that!(equal(sensitivity, "a", "á"))
                .with_detail_message(format!("{sensitivity:?}: a = á"))
                .is_equal_to(accent_equal);
            assert_that!(equal(sensitivity, "a", "A"))
                .with_detail_message(format!("{sensitivity:?}: a = A"))
                .is_equal_to(case_equal);
            assert_that!(equal(sensitivity, "a", "b")).is_false();
        }
    }

    fn default_filter(locale_str: &str) -> Filter {
        let locale: Locale = locale_str.parse().expect("test locales are valid");
        Filter::new(&locale, &CollatorOptions::default())
    }

    #[test]
    fn test_collator_basic_ordering() {
        let locale = Locale::from(locale!("en-US"));
        let collator = Collator::new(&locale, &CollatorOptions::default());
        assert_that!(collator.compare("apple", "banana")).is_equal_to(Ordering::Less);
        assert_that!(collator.compare("banana", "apple")).is_equal_to(Ordering::Greater);
        assert_that!(collator.compare("apple", "apple")).is_equal_to(Ordering::Equal);
    }

    #[test]
    fn test_collator_case_insensitive_with_base_sensitivity() {
        let locale = Locale::from(locale!("en-US"));
        let collator = Collator::new(
            &locale,
            &CollatorOptions {
                sensitivity: CollatorSensitivity::Base,
                ..CollatorOptions::default()
            },
        );
        assert_that!(collator.compare("Apple", "apple")).is_equal_to(Ordering::Equal);
    }

    #[test]
    fn test_collator_accent_insensitive_with_base_sensitivity() {
        let locale = Locale::from(locale!("en-US"));
        let collator = Collator::new(
            &locale,
            &CollatorOptions {
                sensitivity: CollatorSensitivity::Base,
                ..CollatorOptions::default()
            },
        );
        assert_that!(collator.compare("café", "cafe")).is_equal_to(Ordering::Equal);
    }

    #[test]
    fn test_filter_contains_empty_substring() {
        let filter = default_filter("en-US");
        assert_that!(filter.contains("hello", "")).is_true();
    }

    #[test]
    fn test_filter_contains_basic() {
        let filter = default_filter("en-US");
        assert_that!(filter.contains("hello world", "world")).is_true();
        assert_that!(filter.contains("hello world", "xyz")).is_false();
    }

    #[test]
    fn test_filter_starts_with_basic() {
        let filter = default_filter("en-US");
        assert_that!(filter.starts_with("hello world", "hello")).is_true();
        assert_that!(filter.starts_with("hello world", "world")).is_false();
    }

    #[test]
    fn test_filter_ends_with_basic() {
        let filter = default_filter("en-US");
        assert_that!(filter.ends_with("hello world", "world")).is_true();
        assert_that!(filter.ends_with("hello world", "hello")).is_false();
    }

    #[test]
    fn test_filter_case_insensitive() {
        let filter = default_filter("en-US");
        assert_that!(filter.contains("Hello World", "hello")).is_true();
        assert_that!(filter.starts_with("Hello World", "hello")).is_true();
    }

    #[test]
    fn test_filter_with_german_locale() {
        let filter = default_filter("de-DE");
        assert_that!(filter.contains("Straße", "STRAß")).is_true();
        // Windows are as long as the substring in characters (upstream: JS string length), so
        // "ß" doesn't match two characters.
        assert_that!(filter.contains("Straße", "strass")).is_false();
    }

    #[test]
    fn test_filter_matches_umlauts_and_accents() {
        let filter = default_filter("de-DE");
        assert_that!(filter.contains("Müller", "mul")).is_true();
        assert_that!(filter.contains("Herr Müller", "muller")).is_true();
        assert_that!(filter.contains("café", "cafe")).is_true();
        assert_that!(filter.contains("Crème brûlée", "brulee")).is_true();
        assert_that!(filter.contains("Müller", "mux")).is_false();
        assert_that!(filter.starts_with("Ärger", "arg")).is_true();
        assert_that!(filter.starts_with("Österreich", "OST")).is_true();
        assert_that!(filter.ends_with("Gemüse", "muse")).is_true();
        assert_that!(filter.ends_with("café", "fe")).is_true();
        assert_that!(filter.ends_with("café", "ca")).is_false();
    }

    #[test]
    fn test_filter_substring_longer_than_string() {
        let filter = default_filter("en-US");
        assert_that!(filter.contains("ab", "abc")).is_false();
        assert_that!(filter.starts_with("ab", "abc")).is_false();
        assert_that!(filter.ends_with("ab", "abc")).is_false();
        assert_that!(filter.contains("é", "é")).is_true();
    }

    #[test]
    fn test_filter_normalizes_decomposed_input() {
        let filter = default_filter("en-US");
        // "e" + COMBINING ACUTE ACCENT is NFC-normalized to "é" (one character).
        assert_that!(filter.contains("cafe\u{301} au lait", "fé a")).is_true();
        assert_that!(filter.starts_with("e\u{301}clair", "ec")).is_true();
    }

    #[test]
    fn test_collator_ignore_punctuation() {
        let locale = Locale::from(locale!("en-US"));
        let collator = Collator::new(
            &locale,
            &CollatorOptions {
                sensitivity: CollatorSensitivity::Base,
                ignore_punctuation: true,
            },
        );
        assert_that!(collator.compare("e-mail", "email")).is_equal_to(Ordering::Equal);
    }

    #[test]
    fn test_filter_with_japanese_locale() {
        let filter = default_filter("ja-JP");
        assert_that!(filter.contains("東京都", "東京")).is_true();
    }
}
