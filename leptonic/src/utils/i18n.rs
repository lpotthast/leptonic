use leptos::prelude::*;

use super::locale::WritingDirection;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/i18n/src/context.tsx

/// Locale information for internationalization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locale {
    /// The BCP 47 language tag (e.g., "en-US", "de-DE", "ja-JP").
    pub locale: String,

    /// The writing direction for the locale.
    pub direction: WritingDirection,
}

impl Default for Locale {
    fn default() -> Self {
        Self {
            locale: "en-US".to_string(),
            direction: WritingDirection::Ltr,
        }
    }
}

impl Locale {
    /// Creates a new Locale with the given locale string.
    /// Automatically determines the writing direction based on the locale.
    #[must_use]
    pub fn new(locale: impl Into<String>) -> Self {
        let locale = locale.into();
        let direction = Self::direction_for_locale(&locale);
        Self { locale, direction }
    }

    /// Determines the writing direction for a locale.
    #[must_use]
    pub fn direction_for_locale(locale: &str) -> WritingDirection {
        // RTL languages based on their ISO 639-1 codes
        let rtl_languages = [
            "ar", // Arabic
            "he", // Hebrew
            "fa", // Persian/Farsi
            "ur", // Urdu
            "yi", // Yiddish
            "ps", // Pashto
            "sd", // Sindhi
            "ug", // Uyghur
            "ku", // Kurdish (some variants)
            "dv", // Divehi
        ];

        // Extract the language code from the locale (e.g., "ar-SA" -> "ar")
        let language = locale.split('-').next().unwrap_or(locale);
        let language = language.split('_').next().unwrap_or(language);

        if rtl_languages.contains(&language.to_lowercase().as_str()) {
            WritingDirection::Rtl
        } else {
            WritingDirection::Ltr
        }
    }

    /// Returns true if the locale is right-to-left.
    #[must_use]
    pub fn is_rtl(&self) -> bool {
        self.direction == WritingDirection::Rtl
    }

    /// Returns the language code from the locale (e.g., "en" from "en-US").
    #[must_use]
    pub fn language(&self) -> &str {
        self.locale.split('-').next().unwrap_or(&self.locale)
    }

    /// Returns the region code from the locale if present (e.g., "US" from "en-US").
    #[must_use]
    pub fn region(&self) -> Option<&str> {
        self.locale.split('-').nth(1)
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
    use super::*;

    #[test]
    fn test_locale_default() {
        let locale = Locale::default();
        assert_eq!(locale.locale, "en-US");
        assert_eq!(locale.direction, WritingDirection::Ltr);
    }

    #[test]
    fn test_locale_new() {
        let locale = Locale::new("de-DE");
        assert_eq!(locale.locale, "de-DE");
        assert_eq!(locale.direction, WritingDirection::Ltr);
    }

    #[test]
    fn test_locale_rtl() {
        let locale = Locale::new("ar-SA");
        assert_eq!(locale.direction, WritingDirection::Rtl);
        assert!(locale.is_rtl());
    }

    #[test]
    fn test_locale_language() {
        let locale = Locale::new("en-US");
        assert_eq!(locale.language(), "en");
        assert_eq!(locale.region(), Some("US"));
    }

    #[test]
    fn test_direction_for_locale() {
        assert_eq!(Locale::direction_for_locale("ar"), WritingDirection::Rtl);
        assert_eq!(Locale::direction_for_locale("he-IL"), WritingDirection::Rtl);
        assert_eq!(Locale::direction_for_locale("en"), WritingDirection::Ltr);
        assert_eq!(Locale::direction_for_locale("ja-JP"), WritingDirection::Ltr);
    }
}
