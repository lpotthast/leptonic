//! Themes: a theme type, the provider applying it (`data-theme`) and access to it.
use leptos::prelude::*;
use leptos_use::use_document;

use crate::Out;

/// Marker indicating that a `ThemeProvider` has already claimed the
/// document-element `data-theme` attribute. Nested providers skip it.
#[derive(Clone, Copy)]
struct RootThemeApplied;

/// Leptonic's default themes. You may want to create your own theme-defining-type if you have additional or differently named themes.
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum LeptonicTheme {
    #[default]
    Light,
    Dark,
}

impl Theme for LeptonicTheme {
    fn name(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

pub trait Theme:
    Default + PartialEq + Clone + Copy + Send + Sync + serde::Serialize + serde::de::DeserializeOwned
{
    /// The value of the `data-theme` attribute, which the theme's styles select.
    fn name(&self) -> &'static str;
}

/// The current theme of a [`ThemeProvider`] (see [`use_theme`]).
#[derive(Debug, Clone, Copy)]
pub struct ThemeContext<T: Theme + 'static> {
    theme: Signal<T>,
    set_theme: Callback<T>,
}

impl<T: Theme + 'static> ThemeContext<T> {
    /// The current theme.
    pub fn theme(&self) -> Signal<T> {
        self.theme
    }

    /// Switch to `theme`.
    pub fn set_theme(&self, theme: T) {
        self.set_theme.run(theme);
    }
}

/// The theme of the closest [`ThemeProvider`] with theme type `T`, to build theme controls.
pub fn use_theme<T: Theme + 'static>() -> Option<ThemeContext<T>> {
    use_context::<ThemeContext<T>>()
}

/// Applies a theme (`data-theme`) to its children and, for the outermost provider, to the
/// document element (portaled content inherits it); [`use_theme`] reads and switches it.
///
/// State props (C4): `theme` + `set_theme` (controlled, e.g. from `signal_ls`), or
/// `default_theme`; `on_theme_change` observes.
#[component]
pub fn ThemeProvider<T>(
    /// The theme (controlled): a value or any signal.
    #[prop(into, optional)]
    theme: Option<Signal<T>>,
    /// Receives the new theme: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_theme: Option<Out<T>>,
    /// The theme to start with. Default: `T::default()`.
    #[prop(optional)]
    default_theme: Option<T>,
    #[prop(into, optional)] on_theme_change: Option<Callback<T>>,
    children: Children,
) -> impl IntoView
where
    T: Theme + 'static,
{
    let owned = RwSignal::new(default_theme.unwrap_or_default());
    let theme = theme.unwrap_or_else(|| owned.into());
    let set_theme = Callback::new(move |new: T| {
        match set_theme {
            Some(set_theme) => set_theme.set(new),
            None => owned.set(new),
        }
        if let Some(on_theme_change) = on_theme_change {
            on_theme_change.run(new);
        }
    });

    provide_context(ThemeContext { theme, set_theme });

    // If no parent ThemeProvider has claimed the document element,
    // mirror data-theme onto <html> so Portal content inherits CSS variables.
    let is_root = use_context::<RootThemeApplied>().is_none();
    if is_root {
        provide_context(RootThemeApplied);

        Effect::new(move |_| {
            if let Some(doc) = use_document().as_ref()
                && let Some(el) = doc.document_element()
            {
                let _ = el.set_attribute("data-theme", theme.get().name());
            }
        });
    }

    view! {
        <div
            class="leptonic-theme-provider"
            data-theme=move || theme.get().name()
            style="display: contents;"
        >
            {children()}
        </div>
    }
}
