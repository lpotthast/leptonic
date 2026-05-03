// Based on: https://github.com/adobe/react-spectrum/blob/main/packages/@internationalized/number/src/NumberFormatter.ts
// Provides plural category lookup for building locale-correct ARIA labels.

use icu_plurals::{PluralCategory, PluralOperands, PluralRules};

use super::i18n::Locale;

/// Returns the plural category for a given number in the specified locale.
///
/// This is useful for building locale-correct ARIA labels, e.g.,
/// "1 item" (One) vs "2 items" (Other) in English, but with proper
/// handling for languages with more complex plural rules (Arabic, Polish, etc.).
///
/// Returns `PluralCategory::Other` as fallback if the locale is not supported.
///
/// # Example
///
/// ```ignore
/// use icu_plurals::PluralCategory;
/// let locale = Locale::new("en-US");
/// assert_eq!(plural_category(&locale, 1), PluralCategory::One);
/// assert_eq!(plural_category(&locale, 2), PluralCategory::Other);
/// ```
#[must_use]
pub fn plural_category(locale: &Locale, number: u64) -> PluralCategory {
    let prefs = icu_plurals::PluralRulesPreferences::from(locale.icu_locale());
    let Ok(rules) = PluralRules::try_new_cardinal(prefs) else {
        return PluralCategory::Other;
    };
    rules.category_for(PluralOperands::from(number))
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use icu_plurals::PluralCategory;

    use super::*;

    #[test]
    fn test_english_plural_one() {
        let locale = Locale::new("en-US");
        assert_that(plural_category(&locale, 1)).is_equal_to(PluralCategory::One);
    }

    #[test]
    fn test_english_plural_other() {
        let locale = Locale::new("en-US");
        assert_that(plural_category(&locale, 0)).is_equal_to(PluralCategory::Other);
        assert_that(plural_category(&locale, 2)).is_equal_to(PluralCategory::Other);
        assert_that(plural_category(&locale, 5)).is_equal_to(PluralCategory::Other);
    }

    #[test]
    fn test_arabic_plural_categories() {
        let locale = Locale::new("ar");
        // Arabic has: zero, one, two, few, many, other
        assert_that(plural_category(&locale, 0)).is_equal_to(PluralCategory::Zero);
        assert_that(plural_category(&locale, 1)).is_equal_to(PluralCategory::One);
        assert_that(plural_category(&locale, 2)).is_equal_to(PluralCategory::Two);
    }

    #[test]
    fn test_japanese_always_other() {
        let locale = Locale::new("ja-JP");
        // Japanese has no grammatical plural distinctions
        assert_that(plural_category(&locale, 0)).is_equal_to(PluralCategory::Other);
        assert_that(plural_category(&locale, 1)).is_equal_to(PluralCategory::Other);
        assert_that(plural_category(&locale, 100)).is_equal_to(PluralCategory::Other);
    }
}
