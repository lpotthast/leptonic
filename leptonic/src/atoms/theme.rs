// No upstream: application theme context and document styling.
//! Themes: a theme type, the provider applying it (`data-theme`) and access to it.
use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;
use leptos_use::use_document;

use crate::{Out, ValueBinding, utils::default_class::with_default_class};

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
/// On unmount, the outermost provider restores the previous document theme, or removes the
/// attribute if it was absent. A later value written by other code is left in place.
///
/// State props (C4): `theme` + `set_theme` (controlled, e.g. from `signal_ls`), or
/// `default_theme`; `on_theme_change` observes.
///
/// Renders a `<div style="display: contents">` carrying `data-theme`.
///
/// Default class: `leptonic-ThemeProvider`.
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
    #[prop(into, optional)] classes: Classes,
    children: Children,
) -> impl IntoView
where
    T: Theme + 'static,
{
    let classes = with_default_class("leptonic-ThemeProvider", classes);
    let (binding, on_theme_change) =
        ValueBinding::from_state_props(theme, set_theme, on_theme_change);
    let owned = RwSignal::new(default_theme.unwrap_or_default());
    let theme = binding.map_or_else(|| owned.into(), |binding| binding.value);
    let set_theme = Callback::new(move |new: T| {
        if new == theme.get_untracked() {
            return;
        }
        if let Some(binding) = binding {
            binding.set(new);
        } else {
            owned.set(new);
        }
        if let Some(on_theme_change) = on_theme_change {
            on_theme_change.run(new);
        }
    });

    // Provided around the children only (`<Provider>`): `provide_context` in the body would also
    // reach the provider's later siblings.
    let context = ThemeContext { theme, set_theme };

    // If no parent ThemeProvider has claimed the document element,
    // mirror data-theme onto <html> so Portal content inherits CSS variables.
    let is_root = use_context::<RootThemeApplied>().is_none();
    if is_root {
        // Store only strings: SSR owners may be disposed on a different thread.
        let applied = StoredValue::new(None::<(Option<String>, &'static str)>);
        Effect::new(move |_| {
            if let Some(doc) = use_document().as_ref()
                && let Some(el) = doc.document_element()
            {
                let name = theme.get().name();
                applied.update_value(|applied| match applied {
                    Some((_, last)) => *last = name,
                    None => *applied = Some((el.get_attribute("data-theme"), name)),
                });
                let _ = el.set_attribute("data-theme", name);
            }
        });
        on_cleanup(move || {
            if let Some((previous, last)) = applied.get_value()
                && let Some(doc) = use_document().as_ref()
                && let Some(el) = doc.document_element()
                && el.get_attribute("data-theme").as_deref() == Some(last)
            {
                if let Some(previous) = previous {
                    let _ = el.set_attribute("data-theme", &previous);
                } else {
                    let _ = el.remove_attribute("data-theme");
                }
            }
        });
    }

    view! {
        <Provider value=context>
            <Provider value=RootThemeApplied>
                <div
                    class=classes
                    data-theme=move || theme.get().name()
                    style="display: contents;"
                >
                    {children()}
                </div>
            </Provider>
        </Provider>
    }
}
