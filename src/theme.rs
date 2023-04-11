use leptos::*;
use leptos_icons::BsIcon;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ThemeContext {
    theme: ReadSignal<Theme>,
    set_theme: WriteSignal<Theme>,
}

#[component]
pub fn ThemeProvider(
    cx: Scope,
    #[prop(into, optional)] theme: Option<(ReadSignal<Theme>, WriteSignal<Theme>)>,
    children: Children,
) -> impl IntoView {
    let (theme, set_theme) = theme.unwrap_or_else(|| create_signal(cx, Theme::default()));

    provide_context(cx, ThemeContext { theme, set_theme });

    view! {cx,
        <div
            class="leptonic-theme-provider"
            data-theme=move || match theme.get() {
                Theme::Light => "light",
                Theme::Dark => "dark",
            }
            style="height: 100%; width: auto;"
        >
            { children(cx) }
        </div>
    }
}

#[component]
pub fn DarkThemeToggle(cx: Scope) -> impl IntoView {
    let theme_context = use_context::<ThemeContext>(cx).expect("to be present");

    let (toggle, set_toggle) = create_signal(cx, theme_context.theme.get() == Theme::Dark);

    // Our "Theme" must always match / should always be derived from our toggle state.
    create_effect(cx, move |_old| {
        theme_context
            .set_theme
            .update(|current| match toggle.get() {
                true => *current = Theme::Dark,
                false => *current = Theme::Light,
            });
    });

    view! {cx,
        <Toggle on=toggle set_on=set_toggle icons=ToggleIcons { on: BsIcon::BsMoon.into(), off: BsIcon::BsSun.into() } />
    }
}
