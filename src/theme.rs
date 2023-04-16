use leptos::*;

use crate::{prelude::*, toggle::ToggleSize};

pub trait Theme:
    Default + PartialEq + Clone + Copy + serde::Serialize + serde::de::DeserializeOwned
{
    fn name(&self) -> &'static str;
    fn icon(&self) -> leptos_icons::Icon;
}

#[derive(Debug, Clone, Copy)]
pub struct ThemeContext<T: Theme + 'static> {
    theme: ReadSignal<T>,
    set_theme: WriteSignal<T>,
}

#[component]
pub fn ThemeProvider<T>(
    cx: Scope,
    #[prop(into, optional)] theme: Option<(ReadSignal<T>, WriteSignal<T>)>,
    children: Children,
) -> impl IntoView
where
    T: Theme + 'static,
{
    let (theme, set_theme) = theme.unwrap_or_else(|| create_signal(cx, T::default()));

    provide_context(cx, ThemeContext { theme, set_theme });

    view! {cx,
        <leptonic-theme-provider
            data-theme=move || theme.get().name()
            style="height: 100%; width: auto; display: contents;"
        >
            { children(cx) }
        </leptonic-theme-provider>
    }
}

#[component]
pub fn ThemeToggle<T>(
    cx: Scope,
    off: T,
    on: T,
    /// Sets the `class`, making it easier to style.
    #[prop(into, optional)]
    class: Option<AttributeValue>,
) -> impl IntoView
where
    T: Theme + 'static,
{
    let theme_context = use_context::<ThemeContext<T>>(cx)
        .expect("<ThemeToggle/> component should be nested within a <ThemeProvider/>.");

    let toggle = Toggle(
        cx,
        ToggleProps {
            on: move || theme_context.theme.get() == on,
            set_on: move |val| {
                theme_context.set_theme.update(|current| match val {
                    true => *current = on,
                    false => *current = off,
                })
            },
            active: None,
            disabled: None,
            id: None,
            class: None,
            size: ToggleSize::default(),
            icons: Some(ToggleIcons {
                on: on.icon(),
                off: off.icon(),
            }),
        },
    );

    view! {cx,
        <leptonic-theme-toggle class=class>
            { toggle }
        </leptonic-theme-toggle>
    }
}
