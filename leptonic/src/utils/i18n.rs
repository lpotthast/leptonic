use leptos::prelude::*;

use super::locale::WritingDirection;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/i18n/src/context.tsx

/// Locale information for internationalization.
///
/// Wraps an `icu_locale::Locale` internally for proper locale-aware operations
/// while maintaining a simple string-based public API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locale {
    /// The parsed ICU locale.
    inner: icu_locale::Locale,

    /// The writing direction for the locale.
    pub direction: WritingDirection,
}

impl Default for Locale {
    fn default() -> Self {
        Self {
            inner: icu_locale::locale!("en-US"),
            direction: WritingDirection::Ltr,
        }
    }
}

impl Locale {
    /// Creates a new Locale with the given locale string.
    /// Automatically determines the writing direction based on the locale.
    /// Falls back to en-US if the locale string cannot be parsed.
    #[must_use]
    pub fn new(locale: impl AsRef<str>) -> Self {
        let locale_str = locale.as_ref();
        let inner = locale_str
            .parse::<icu_locale::Locale>()
            .unwrap_or(icu_locale::locale!("en-US"));
        let direction = Self::direction_for_icu_locale(&inner);
        Self { inner, direction }
    }

    /// Returns the BCP 47 locale string (e.g., "en-US", "de-DE").
    #[must_use]
    pub fn locale_str(&self) -> String {
        self.inner.to_string()
    }

    /// Determines the writing direction for a locale string.
    #[must_use]
    pub fn direction_for_locale(locale: &str) -> WritingDirection {
        match locale.parse::<icu_locale::Locale>() {
            Ok(parsed) => Self::direction_for_icu_locale(&parsed),
            Err(_) => WritingDirection::Ltr,
        }
    }

    /// Determines the writing direction from a parsed ICU locale.
    ///
    /// Uses the likely-subtags algorithm to determine the script, then
    /// checks if the script is inherently RTL.
    fn direction_for_icu_locale(locale: &icu_locale::Locale) -> WritingDirection {
        // Use likely subtags to maximize the language identifier — this fills in the script subtag.
        let mut langid = locale.id.clone();
        let expander = icu_locale::LocaleExpander::new_extended();
        expander.maximize(&mut langid);

        // Check the script for RTL direction.
        if let Some(script) = langid.script {
            let script_str = script.as_str();
            // Scripts that are written right-to-left.
            if matches!(
                script_str,
                "Arab" | "Hebr" | "Thaa" | "Syrc" | "Mand" | "Nkoo" | "Adlm" | "Samr"
            ) {
                return WritingDirection::Rtl;
            }
        }

        WritingDirection::Ltr
    }

    /// Returns true if the locale is right-to-left.
    #[must_use]
    pub fn is_rtl(&self) -> bool {
        self.direction == WritingDirection::Rtl
    }

    /// Returns the language code from the locale (e.g., "en" from "en-US").
    #[must_use]
    pub fn language(&self) -> String {
        self.inner.id.language.to_string()
    }

    /// Returns the region code from the locale if present (e.g., "US" from "en-US").
    #[must_use]
    pub fn region(&self) -> Option<String> {
        self.inner.id.region.map(|r| r.to_string())
    }

    /// Returns a reference to the inner `icu_locale::Locale`.
    #[must_use]
    pub fn icu_locale(&self) -> &icu_locale::Locale {
        &self.inner
    }
}

/// Context type for providing locale information throughout the application.
#[derive(Clone)]
pub struct I18nContext {
    /// The current locale.
    pub locale: Signal<Locale>,

    /// Set the locale.
    pub set_locale: Callback<Locale>,
}

impl I18nContext {
    /// Returns the current locale value.
    #[must_use]
    pub fn get_locale(&self) -> Locale {
        self.locale.get()
    }

    /// Returns the current writing direction.
    #[must_use]
    pub fn direction(&self) -> WritingDirection {
        self.locale.get().direction
    }

    /// Returns true if the current locale is RTL.
    #[must_use]
    pub fn is_rtl(&self) -> bool {
        self.locale.get().is_rtl()
    }
}

/// Provides locale context to descendant components.
///
/// # Example
///
/// ```ignore
/// view! {
///     <I18nProvider locale=Locale::new("de-DE")>
///         // Components can use use_locale() to access locale info
///     </I18nProvider>
/// }
/// ```
#[component]
pub fn I18nProvider(
    /// The initial locale. Defaults to "en-US".
    #[prop(optional, into)]
    locale: Option<Locale>,
    /// Children to render.
    children: Children,
) -> impl IntoView {
    let initial_locale = locale.unwrap_or_default();
    let (locale_signal, set_locale_signal) = signal(initial_locale);

    let context = I18nContext {
        locale: locale_signal.into(),
        set_locale: Callback::new(move |new_locale: Locale| {
            set_locale_signal.set(new_locale);
        }),
    };

    provide_context(context);

    children()
}

/// Returns the current I18n context.
///
/// # Panics
///
/// Panics if called outside of an `I18nProvider`.
#[must_use]
pub fn use_locale() -> I18nContext {
    use_context::<I18nContext>().expect("use_locale must be used within an I18nProvider")
}

/// Returns the current I18n context, or `None` if not within a provider.
#[must_use]
pub fn try_use_locale() -> Option<I18nContext> {
    use_context::<I18nContext>()
}

/// Returns the current locale, with a fallback to default if not in a provider.
#[must_use]
pub fn use_locale_or_default() -> Locale {
    try_use_locale()
        .map(|ctx| ctx.get_locale())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn test_locale_default() {
        let locale = Locale::default();
        assert_that(locale.locale_str()).is_equal_to("en-US".to_string());
        assert_that(locale.direction).is_equal_to(WritingDirection::Ltr);
    }

    #[test]
    fn test_locale_new() {
        let locale = Locale::new("de-DE");
        assert_that(locale.locale_str()).is_equal_to("de-DE".to_string());
        assert_that(locale.direction).is_equal_to(WritingDirection::Ltr);
    }

    #[test]
    fn test_locale_rtl() {
        let locale = Locale::new("ar-SA");
        assert_that(locale.direction).is_equal_to(WritingDirection::Rtl);
        assert_that(locale.is_rtl()).is_true();
    }

    #[test]
    fn test_locale_language() {
        let locale = Locale::new("en-US");
        assert_that(locale.language()).is_equal_to("en".to_string());
        assert_that(locale.region())
            .is_some()
            .is_equal_to("US".to_string());
    }

    #[test]
    fn test_direction_for_locale() {
        assert_that(Locale::direction_for_locale("ar")).is_equal_to(WritingDirection::Rtl);
        assert_that(Locale::direction_for_locale("he-IL")).is_equal_to(WritingDirection::Rtl);
        assert_that(Locale::direction_for_locale("en")).is_equal_to(WritingDirection::Ltr);
        assert_that(Locale::direction_for_locale("ja-JP")).is_equal_to(WritingDirection::Ltr);
    }

    #[test]
    fn test_locale_rtl_via_script_detection() {
        // These should all be detected as RTL via script-based detection
        assert_that(Locale::new("fa").is_rtl()).is_true(); // Persian/Farsi (Arab script)
        assert_that(Locale::new("ur").is_rtl()).is_true(); // Urdu (Arab script)
        assert_that(Locale::new("he").is_rtl()).is_true(); // Hebrew (Hebr script)
        assert_that(Locale::new("ps").is_rtl()).is_true(); // Pashto (Arab script)
        assert_that(Locale::new("yi").is_rtl()).is_true(); // Yiddish (Hebr script)
    }

    #[test]
    fn test_locale_ltr_scripts() {
        assert_that(Locale::new("zh-CN").is_rtl()).is_false(); // Chinese
        assert_that(Locale::new("ko-KR").is_rtl()).is_false(); // Korean
        assert_that(Locale::new("hi-IN").is_rtl()).is_false(); // Hindi (Devanagari)
        assert_that(Locale::new("th").is_rtl()).is_false(); // Thai
    }

    #[test]
    fn test_invalid_locale_falls_back() {
        let locale = Locale::new("not-a-real-locale");
        // Should fall back to en-US
        assert_that(locale.direction).is_equal_to(WritingDirection::Ltr);
    }

    #[test]
    fn test_icu_locale_accessor() {
        let locale = Locale::new("de-DE");
        let icu = locale.icu_locale();
        assert_that(icu.to_string()).is_equal_to("de-DE".to_string());
    }
}
