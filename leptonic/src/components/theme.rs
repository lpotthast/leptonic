use leptos::prelude::*;
use leptos_use::use_document;

use crate::{
    components::{
        prelude::{Toggle, ToggleIcons},
        toggle::{ToggleProps, ToggleSize, ToggleVariant},
    },
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

#[derive(Debug, Clone, Copy)]
pub struct ThemeContext<T: Theme + 'static> {
    theme: ReadSignal<T>,
    set_theme: WriteSignal<T>,
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
            if let Some(doc) = use_document().as_ref() {
                if let Some(el) = doc.document_element() {
                    let _ = el.set_attribute("data-theme", theme.get().name());
                }
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

#[component]
pub fn ThemeToggle<T>(
    off: T,
    on: T,
    #[prop(optional)] variant: ToggleVariant,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView
where
    T: Theme + 'static,
{
    let theme_context = use_context::<ThemeContext<T>>()
        .expect("<ThemeToggle/> component should be nested within a <ThemeProvider/>.");

    let toggle = Toggle(ToggleProps {
        state: Signal::derive(move || theme_context.theme.get() == on),
        set_state: Some(Into::into(move |val: bool| {
            theme_context.set_theme.update(|current| {
                if val {
                    *current = on;
                } else {
                    *current = off;
                }
            });
        })),
        active: None,
        disabled: None,
        size: ToggleSize::default(),
        variant,
        icons: Some(ToggleIcons {
            on: on.icon(),
            off: off.icon(),
        }),
        classes: Classes::default(),
        styles: Styles::default(),
    });

    view! { <div class=classes.add("leptonic-theme-toggle") style=styles>{toggle}</div> }
}
