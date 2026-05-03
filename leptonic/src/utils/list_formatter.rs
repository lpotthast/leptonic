// Based on: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/i18n/src/useListFormatter.tsx

use icu_list::{ListFormatter as IcuListFormatter, options::ListLength};

use super::i18n::Locale;

/// The type of list formatting.
///
/// Controls whether the list uses "and", "or", or neutral joining.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListFormatType {
    /// "A, B, and C" — conjunction list.
    #[default]
    Conjunction,
    /// "A, B, or C" — disjunction list.
    Disjunction,
    /// "A, B, C" — unit list (no conjunction word).
    Unit,
}

/// The style of list formatting.
///
/// Controls the verbosity of the formatted output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListFormatStyle {
    /// "A, B, and C" — full formatting.
    #[default]
    Long,
    /// "A, B, & C" — shorter formatting.
    Short,
    /// "A, B, C" — narrowest formatting.
    Narrow,
}

impl ListFormatStyle {
    fn to_icu_length(self) -> ListLength {
        match self {
            Self::Long => ListLength::Wide,
            Self::Short => ListLength::Short,
            Self::Narrow => ListLength::Narrow,
        }
    }
}

/// Options for creating a [`ListFormatter`].
#[derive(Debug, Clone, Default)]
pub struct ListFormatOptions {
    /// The type of list formatting. Defaults to `Conjunction`.
    pub r#type: ListFormatType,

    /// The style of list formatting. Defaults to `Long`.
    pub style: ListFormatStyle,
}

/// Locale-aware list formatter using ICU4X.
///
/// Provides locale-sensitive list formatting, producing natural-language
/// list strings like "A, B, and C" (English) or "A, B et C" (French).
/// This mirrors react-aria's `useListFormatter`.
///
/// SSR-safe: uses pure Rust ICU4X instead of browser `Intl` APIs.
///
/// # Example
///
/// ```ignore
/// let formatter = ListFormatter::new(&Locale::new("en-US"), &ListFormatOptions::default());
/// let result = formatter.format(&["Alice", "Bob", "Charlie"]);
/// assert_eq!(result, "Alice, Bob, and Charlie");
/// ```
pub struct ListFormatter {
    inner: IcuListFormatter,
}

impl ListFormatter {
    /// Creates a new list formatter with the given locale and options.
    ///
    /// - `locale`: A `Locale` for locale-sensitive formatting.
    /// - `options`: Type and style options.
    ///
    /// # Panics
    ///
    /// Panics if ICU4X cannot create a list formatter for the given locale or the default locale.
    #[must_use]
    pub fn new(locale: &Locale, options: &ListFormatOptions) -> Self {
        let prefs = icu_list::ListFormatterPreferences::from(locale.icu_locale());
        let icu_options = icu_list::options::ListFormatterOptions::default()
            .with_length(options.style.to_icu_length());

        let inner = match options.r#type {
            ListFormatType::Conjunction => IcuListFormatter::try_new_and(prefs, icu_options),
            ListFormatType::Disjunction => IcuListFormatter::try_new_or(prefs, icu_options),
            ListFormatType::Unit => IcuListFormatter::try_new_unit(prefs, icu_options),
        }
        .unwrap_or_else(|_| {
            // Fallback to default locale
            IcuListFormatter::try_new_and(
                icu_list::ListFormatterPreferences::default(),
                icu_list::options::ListFormatterOptions::default(),
            )
            .expect("ICU4X default list formatter should always be available")
        });

        Self { inner }
    }

    /// Formats a list of strings according to the formatter's locale and options.
    ///
    /// Returns a locale-appropriate formatted string (e.g., "A, B, and C").
    #[must_use]
    pub fn format(&self, list: &[&str]) -> String {
        self.inner.format(list.iter().copied()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn test_conjunction_long_en() {
        let formatter = ListFormatter::new(
            &Locale::new("en-US"),
            &ListFormatOptions {
                r#type: ListFormatType::Conjunction,
                style: ListFormatStyle::Long,
            },
        );
        assert_that(formatter.format(&["Alice", "Bob", "Charlie"]))
            .is_equal_to("Alice, Bob, and Charlie".to_string());
    }

    #[test]
    fn test_disjunction_long_en() {
        let formatter = ListFormatter::new(
            &Locale::new("en-US"),
            &ListFormatOptions {
                r#type: ListFormatType::Disjunction,
                style: ListFormatStyle::Long,
            },
        );
        assert_that(formatter.format(&["Alice", "Bob", "Charlie"]))
            .is_equal_to("Alice, Bob, or Charlie".to_string());
    }

    #[test]
    fn test_conjunction_long_de() {
        let formatter = ListFormatter::new(
            &Locale::new("de-DE"),
            &ListFormatOptions {
                r#type: ListFormatType::Conjunction,
                style: ListFormatStyle::Long,
            },
        );
        let result = formatter.format(&["Alice", "Bob", "Charlie"]);
        // German uses "und" instead of "and"
        assert_that(result.contains("und")).is_true();
    }

    #[test]
    fn test_conjunction_long_fr() {
        let formatter = ListFormatter::new(
            &Locale::new("fr-FR"),
            &ListFormatOptions {
                r#type: ListFormatType::Conjunction,
                style: ListFormatStyle::Long,
            },
        );
        let result = formatter.format(&["Alice", "Bob", "Charlie"]);
        // French uses "et" instead of "and"
        assert_that(result.contains("et")).is_true();
    }

    #[test]
    fn test_single_item() {
        let formatter = ListFormatter::new(&Locale::new("en-US"), &ListFormatOptions::default());
        assert_that(formatter.format(&["Alice"])).is_equal_to("Alice".to_string());
    }

    #[test]
    fn test_two_items() {
        let formatter = ListFormatter::new(&Locale::new("en-US"), &ListFormatOptions::default());
        assert_that(formatter.format(&["Alice", "Bob"])).is_equal_to("Alice and Bob".to_string());
    }
}
