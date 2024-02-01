use leptos::*;

use crate::{prelude::*, toggle::ToggleProps, toggle::ToggleSize};

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
    Default + PartialEq + Clone + Copy + serde::Serialize + serde::de::DeserializeOwned
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
    let (theme, set_theme) = theme.unwrap_or_else(|| create_signal(T::default()));

    provide_context(ThemeContext { theme, set_theme });

    view! {
        <leptonic-theme-provider
            data-theme=move || theme.get().name()
            style="height: 100%; width: auto; display: contents;"
        >
            { children() }
        </leptonic-theme-provider>
    }
}

#[component]
pub fn ThemeToggle<T>(
    off: T,
    on: T,
    #[prop(optional)] variant: ToggleVariant,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView
where
    T: Theme + 'static,
{
    let theme_context = use_context::<ThemeContext<T>>()
        .expect("<ThemeToggle/> component should be nested within a <ThemeProvider/>.");

    let toggle = Toggle(ToggleProps {
        state: MaybeSignal::derive(move || theme_context.theme.get() == on),
        set_state: Some(
            Callback::new(move |val: bool| {
                theme_context.set_theme.update(|current| match val {
                    true => *current = on,
                    false => *current = off,
                });
            })
            .into(),
        ),
        active: OptionalMaybeSignal(None),
        disabled: OptionalMaybeSignal(None),
        id: None,
        class: None,
        style: None,
        size: ToggleSize::default(),
        variant,
        icons: Some(ToggleIcons {
            on: on.icon(),
            off: off.icon(),
        }),
    });

    view! {
        <leptonic-theme-toggle class=class style=style>
            { toggle }
        </leptonic-theme-toggle>
    }
}
