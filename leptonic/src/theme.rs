use leptos::*;

use crate::{prelude::*, toggle::ToggleProps, toggle::ToggleSize};

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
            theme=move || theme.get().name()
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
    #[prop(optional)] variant: ToggleVariant,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView
where
    T: Theme + 'static,
{
    let theme_context = use_context::<ThemeContext<T>>(cx)
        .expect("<ThemeToggle/> component should be nested within a <ThemeProvider/>.");

    let toggle = Toggle(
        cx,
        ToggleProps {
            state: MaybeSignal::derive(cx, move || theme_context.theme.get() == on),
            on_toggle: move |val| {
                theme_context.set_theme.update(|current| match val {
                    true => *current = on,
                    false => *current = off,
                })
            },
            active: OptionalMaybeSignal(None),
            disabled: OptionalMaybeSignal(None),
            id: None,
            class: None,
            size: ToggleSize::default(),
            variant,
            icons: Some(ToggleIcons {
                on: on.icon(),
                off: off.icon(),
            }),
        },
    );

    view! {cx,
        <leptonic-theme-toggle class=class style=style>
            { toggle }
        </leptonic-theme-toggle>
    }
}
