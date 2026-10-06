use leptos::prelude::*;
use leptos_use::use_document;

use crate::{
    components::switch::{Switch, SwitchIcons, SwitchVariant},
    hooks::ToggleState,
    utils::{classes::Classes, styles::Styles},
};

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

    fn icon(&self) -> icondata::Icon {
        match self {
            Self::Light => icondata::BsSun,
            Self::Dark => icondata::BsMoon,
        }
    }
}

pub trait Theme:
    Default + PartialEq + Clone + Copy + Send + Sync + serde::Serialize + serde::de::DeserializeOwned
{
    fn name(&self) -> &'static str;
    fn icon(&self) -> icondata::Icon;
}

/// The current theme of a [`ThemeProvider`] (see [`use_theme`]).
#[derive(Debug, Clone, Copy)]
pub struct ThemeContext<T: Theme + 'static> {
    theme: ReadSignal<T>,
    set_theme: WriteSignal<T>,
}

impl<T: Theme + 'static> ThemeContext<T> {
    /// The current theme.
    pub fn theme(&self) -> Signal<T> {
        self.theme.into()
    }

    /// Switch to `theme`.
    pub fn set_theme(&self, theme: T) {
        self.set_theme.set(theme);
    }
}

/// The theme of the closest [`ThemeProvider`] with theme type `T`, to build theme controls.
pub fn use_theme<T: Theme + 'static>() -> Option<ThemeContext<T>> {
    use_context::<ThemeContext<T>>()
}

#[component]
pub fn ThemeProvider<T>(
    #[prop(into, optional)] theme: Option<(ReadSignal<T>, WriteSignal<T>)>,
    children: Children,
) -> impl IntoView
where
    T: Theme + 'static,
{
    let (theme, set_theme) = theme.unwrap_or_else(|| signal(T::default()));

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

/// A switch between two themes: on selects `on`, off selects `off`. Shows their icons.
#[component]
pub fn ThemeToggle<T>(
    off: T,
    on: T,
    #[prop(optional)] variant: SwitchVariant,
    /// The switch's accessible name. Defaults to "`<on theme name>` theme".
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView
where
    T: Theme + 'static,
{
    let theme = use_theme::<T>()
        .expect("<ThemeToggle/> component should be nested within a <ThemeProvider/>.");
    let state = ToggleState::new(
        Signal::derive(move || theme.theme().get() == on),
        theme.theme().get_untracked() == on,
        Callback::new(move |selected: bool| theme.set_theme(if selected { on } else { off })),
    );
    let aria_label = MaybeProp::derive(move || {
        Some(
            aria_label
                .get()
                .unwrap_or_else(|| format!("{} theme", on.name())),
        )
    });

    view! {
        <Switch
            state
            aria_label
            variant
            icons=SwitchIcons { on: on.icon(), off: off.icon() }
            classes=classes.add("leptonic-theme-toggle")
            styles
        />
    }
}
