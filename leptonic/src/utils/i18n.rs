// Upstream: react-aria/src/interactions/context.ts @ 6f664fe911
use leptos::prelude::*;

use super::locale::WritingDirection;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/react-aria/src/interactions/context.ts

/// Locale information for internationalization.
///
/// Wraps an `icu_locale::Locale`. Create one from a literal checked at compile time
/// (`Locale::from(locale!("de-DE"))`, with the re-exported [`locale!`] macro) or parse a runtime
/// string (`"de-DE".parse::<Locale>()?`).
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

/// ICU's `locale!` macro: a [`Locale`] literal checked at compile time.
pub use icu_locale::locale;

impl From<icu_locale::Locale> for Locale {
    fn from(inner: icu_locale::Locale) -> Self {
        let direction = Self::direction_for_icu_locale(&inner);
        Self { inner, direction }
    }
}

/// A string that isn't a valid BCP 47 locale.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidLocale(pub String);

impl std::fmt::Display for InvalidLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid locale: {:?}", self.0)
    }
}

impl std::error::Error for InvalidLocale {}

impl std::str::FromStr for Locale {
    type Err = InvalidLocale;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<icu_locale::Locale>()
            .map(Self::from)
            .map_err(|_| InvalidLocale(s.to_owned()))
    }
}

impl Locale {
    /// Returns the BCP 47 locale string (e.g., "en-US", "de-DE").
    #[must_use]
    pub fn locale_str(&self) -> String {
        self.inner.to_string()
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
///     <I18nProvider locale=Locale::from(locale!("de-DE"))>
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

/// The I18n context (to read or change the locale), if within an `I18nProvider`.
#[must_use]
pub fn use_i18n() -> Option<I18nContext> {
    use_context::<I18nContext>()
}

/// The current locale (reactive); the default locale outside of an `I18nProvider`.
#[must_use]
pub fn use_locale() -> Signal<Locale> {
    use_i18n().map_or_else(|| Signal::stored(Locale::default()), |ctx| ctx.locale)
}

/// The current writing direction as a signal (left-to-right outside of an `I18nProvider`).
#[must_use]
pub fn use_direction() -> Signal<WritingDirection> {
    let locale = use_locale();
    Signal::derive(move || locale.with(|l| l.direction))
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn test_locale_default() {
        let locale = Locale::default();
        assert_that!(locale.locale_str()).is_equal_to("en-US".to_string());
        assert_that!(locale.direction).is_equal_to(WritingDirection::Ltr);
    }

    #[test]
    fn test_locale_new() {
        let locale = Locale::from(locale!("de-DE"));
        assert_that!(locale.locale_str()).is_equal_to("de-DE".to_string());
        assert_that!(locale.direction).is_equal_to(WritingDirection::Ltr);
    }

    #[test]
    fn test_locale_rtl() {
        let locale = Locale::from(locale!("ar-SA"));
        assert_that!(locale.direction).is_equal_to(WritingDirection::Rtl);
        assert_that!(locale.is_rtl()).is_true();
    }

    #[test]
    fn test_locale_language() {
        let locale = Locale::from(locale!("en-US"));
        assert_that!(locale.language()).is_equal_to("en".to_string());
        assert_that!(locale.region())
            .get_some()
            .is_equal_to("US".to_string());
    }

    #[test]
    fn test_locale_rtl_via_script_detection() {
        // These should all be detected as RTL via script-based detection
        assert_that!(Locale::from(locale!("fa")).is_rtl()).is_true(); // Persian/Farsi (Arab script)
        assert_that!(Locale::from(locale!("ur")).is_rtl()).is_true(); // Urdu (Arab script)
        assert_that!(Locale::from(locale!("he")).is_rtl()).is_true(); // Hebrew (Hebr script)
        assert_that!(Locale::from(locale!("ps")).is_rtl()).is_true(); // Pashto (Arab script)
        assert_that!(Locale::from(locale!("yi")).is_rtl()).is_true(); // Yiddish (Hebr script)
    }

    #[test]
    fn test_locale_ltr_scripts() {
        assert_that!(Locale::from(locale!("zh-CN")).is_rtl()).is_false(); // Chinese
        assert_that!(Locale::from(locale!("ko-KR")).is_rtl()).is_false(); // Korean
        assert_that!(Locale::from(locale!("hi-IN")).is_rtl()).is_false(); // Hindi (Devanagari)
        assert_that!(Locale::from(locale!("th")).is_rtl()).is_false(); // Thai
    }

    #[test]
    fn test_invalid_locale() {
        assert_that!("".parse::<Locale>().is_err()).is_true();
        assert_that!("-".parse::<Locale>().is_err()).is_true();
        assert_that!("de-DE".parse::<Locale>().map(|l| l.locale_str()))
            .is_equal_to(Ok("de-DE".to_owned()));
    }

    #[test]
    fn test_icu_locale_accessor() {
        let locale = Locale::from(locale!("de-DE"));
        let icu = locale.icu_locale();
        assert_that!(icu.to_string()).is_equal_to("de-DE".to_string());
    }
}
