use leptos::*;
use leptos_icons::BsIcon;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
pub fn ThemeProvider(cx: Scope, children: Children) -> impl IntoView {
    let (theme, set_theme) = create_signal(cx, Theme::default());

    provide_context(cx, ThemeContext { theme, set_theme });

    view! {cx,
        <div class=move || match theme.get() {
            Theme::Light => "leptonic-theme-light",
            Theme::Dark => "leptonic-theme-dark",
        }>
            { children(cx) }
        </div>
    }
}

#[component]
pub fn DarkThemeToggle(cx: Scope) -> impl IntoView {
    let theme_context = use_context::<ThemeContext>(cx).expect("to be present");

    let (toggle, set_toggle) =
        create_signal_ls(cx, "prefer_dark", theme_context.theme.get() == Theme::Dark);

    // Out "Theme" must always match / should always be derived from our toggle state.
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
