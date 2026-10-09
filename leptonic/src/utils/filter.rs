// Upstream: react-aria/src/i18n/useFilter.ts @ 99e6102368
// Upstream: react-aria/src/i18n/useCollator.ts @ 99e6102368

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - ICU4X collation instead of `Intl.Collator` (SSR-safe).
// - The search text is a `FilterQuery`, normalized once for any number of strings (react-aria
//   normalizes it on every call).
// - Windows are counted in characters (react-aria: UTF-16 code units).
//
// =============================================================================

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

/// NFC-normalizes a string using ICU4X (borrowed when it already is).
fn normalize_nfc(s: &str) -> std::borrow::Cow<'_, str> {
    ComposingNormalizer::new_nfc().normalize(s)
}

/// A search text prepared for [`Filter`]'s matching: normalized once, to be matched against many
/// strings (e.g. every option of a combo box, per keystroke).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterQuery {
    normalized: String,
    /// The number of characters of `normalized`: the length of the windows compared.
    chars: usize,
}

impl FilterQuery {
    /// Prepares `query` (NFC-normalized, so that composed and decomposed characters match).
    #[must_use]
    pub fn new(query: &str) -> Self {
        let normalized = normalize_nfc(query).into_owned();
        Self {
            chars: normalized.chars().count(),
            normalized,
        }
    }

    /// Whether the query is empty (it matches every string).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.normalized.is_empty()
    }
}

/// Locale-aware string filtering with `contains`, `starts_with`, and `ends_with`.
///
/// Uses ICU4X `Collator` with locale-sensitive matching,
/// and NFC-normalizes strings before comparison. This mirrors react-aria's `useFilter`. The
/// query is a [`FilterQuery`], prepared once for any number of strings.
///
/// SSR-safe: uses pure Rust ICU4X instead of browser `Intl` APIs.
///
/// # Example
///
/// ```ignore
/// let filter = Filter::new(&Locale::from(locale!("en-US")), &CollatorOptions::default());
///
/// let cafe = FilterQuery::new("cafe");
/// assert!(filter.contains("café", &cafe));   // base sensitivity: accent-insensitive
/// assert!(filter.starts_with("Hello", &FilterQuery::new("hello"))); // case-insensitive
/// assert!(filter.ends_with("world!", &FilterQuery::new("WORLD!")));
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

    fn equals(&self, part: &str, query: &FilterQuery) -> bool {
        self.collator.compare(part, &query.normalized) == Ordering::Equal
    }

    /// Returns `true` if `string` contains `query` according to locale-aware comparison.
    ///
    /// Slides a window of as many characters as `query` has over the NFC-normalized `string`,
    /// comparing each window via the collator. An empty `query` always matches.
    #[must_use]
    pub fn contains(&self, string: &str, query: &FilterQuery) -> bool {
        if query.is_empty() {
            return true;
        }
        let string = normalize_nfc(string);
        // Windows by character count (upstream slices by JS string length): the byte offsets of
        // every character start, plus the end of the string, `chars` apart.
        let boundaries = || {
            string
                .char_indices()
                .map(|(index, _)| index)
                .chain(std::iter::once(string.len()))
        };
        boundaries()
            .zip(boundaries().skip(query.chars))
            .any(|(start, end)| self.equals(&string[start..end], query))
    }

    /// Returns `true` if `string` starts with `query` according to locale-aware comparison.
    ///
    /// Compares as many leading characters of `string` as `query` has via the collator. An
    /// empty `query` always matches.
    #[must_use]
    pub fn starts_with(&self, string: &str, query: &FilterQuery) -> bool {
        if query.is_empty() {
            return true;
        }
        let string = normalize_nfc(string);
        // JS `slice` clamps: a shorter `string` is compared as a whole.
        let end = string
            .char_indices()
            .nth(query.chars)
            .map_or(string.len(), |(index, _)| index);
        self.equals(&string[..end], query)
    }

    /// Returns `true` if `string` ends with `query` according to locale-aware comparison.
    ///
    /// Compares as many trailing characters of `string` as `query` has via the collator. An
    /// empty `query` always matches.
    #[must_use]
    pub fn ends_with(&self, string: &str, query: &FilterQuery) -> bool {
        if query.is_empty() {
            return true;
        }
        let string = normalize_nfc(string);
        // JS `slice` clamps: a shorter `string` is compared as a whole.
        let start = string
            .char_indices()
            .rev()
            .nth(query.chars - 1)
            .map_or(0, |(index, _)| index);
        self.equals(&string[start..], query)
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
        assert_that!(filter.contains("hello", &FilterQuery::new(""))).is_true();
    }

    #[test]
    fn test_filter_contains_basic() {
        let filter = default_filter("en-US");
        assert_that!(filter.contains("hello world", &FilterQuery::new("world"))).is_true();
        assert_that!(filter.contains("hello world", &FilterQuery::new("xyz"))).is_false();
    }

    #[test]
    fn test_filter_starts_with_basic() {
        let filter = default_filter("en-US");
        assert_that!(filter.starts_with("hello world", &FilterQuery::new("hello"))).is_true();
        assert_that!(filter.starts_with("hello world", &FilterQuery::new("world"))).is_false();
    }

    #[test]
    fn test_filter_ends_with_basic() {
        let filter = default_filter("en-US");
        assert_that!(filter.ends_with("hello world", &FilterQuery::new("world"))).is_true();
        assert_that!(filter.ends_with("hello world", &FilterQuery::new("hello"))).is_false();
    }

    #[test]
    fn test_filter_case_insensitive() {
        let filter = default_filter("en-US");
        assert_that!(filter.contains("Hello World", &FilterQuery::new("hello"))).is_true();
        assert_that!(filter.starts_with("Hello World", &FilterQuery::new("hello"))).is_true();
    }

    #[test]
    fn test_filter_with_german_locale() {
        let filter = default_filter("de-DE");
        assert_that!(filter.contains("Straße", &FilterQuery::new("STRAß"))).is_true();
        // Windows are as long as the substring in characters (upstream: JS string length), so
        // "ß" doesn't match two characters.
        assert_that!(filter.contains("Straße", &FilterQuery::new("strass"))).is_false();
    }

    #[test]
    fn test_filter_matches_umlauts_and_accents() {
        let filter = default_filter("de-DE");
        assert_that!(filter.contains("Müller", &FilterQuery::new("mul"))).is_true();
        assert_that!(filter.contains("Herr Müller", &FilterQuery::new("muller"))).is_true();
        assert_that!(filter.contains("café", &FilterQuery::new("cafe"))).is_true();
        assert_that!(filter.contains("Crème brûlée", &FilterQuery::new("brulee"))).is_true();
        assert_that!(filter.contains("Müller", &FilterQuery::new("mux"))).is_false();
        assert_that!(filter.starts_with("Ärger", &FilterQuery::new("arg"))).is_true();
        assert_that!(filter.starts_with("Österreich", &FilterQuery::new("OST"))).is_true();
        assert_that!(filter.ends_with("Gemüse", &FilterQuery::new("muse"))).is_true();
        assert_that!(filter.ends_with("café", &FilterQuery::new("fe"))).is_true();
        assert_that!(filter.ends_with("café", &FilterQuery::new("ca"))).is_false();
    }

    #[test]
    fn test_filter_substring_longer_than_string() {
        let filter = default_filter("en-US");
        assert_that!(filter.contains("ab", &FilterQuery::new("abc"))).is_false();
        assert_that!(filter.starts_with("ab", &FilterQuery::new("abc"))).is_false();
        assert_that!(filter.ends_with("ab", &FilterQuery::new("abc"))).is_false();
        assert_that!(filter.contains("é", &FilterQuery::new("é"))).is_true();
    }

    #[test]
    fn test_filter_normalizes_decomposed_input() {
        let filter = default_filter("en-US");
        // "e" + COMBINING ACUTE ACCENT is NFC-normalized to "é" (one character).
        assert_that!(filter.contains("cafe\u{301} au lait", &FilterQuery::new("fé a"))).is_true();
        assert_that!(filter.starts_with("e\u{301}clair", &FilterQuery::new("ec"))).is_true();
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
        assert_that!(filter.contains("東京都", &FilterQuery::new("東京"))).is_true();
    }
}
