use leptonic::atoms::theme::{LeptonicTheme, ThemeProvider, use_theme};
use leptos::prelude::*;

/// The theme of the closest `ThemeProvider`, as text.
#[component]
fn ThemeName(id: &'static str) -> impl IntoView {
    let theme = use_theme::<LeptonicTheme>();
    let name = move || match theme {
        Some(ctx) => format!("{:?}", ctx.theme().get()),
        None => "none".to_owned(),
    };
    view! { <span id=id>{name}</span> }
}

/// Theme providers, inside the app's (light) one:
/// - `#test-theme-dark`: a dark provider; `#test-theme-inside` shows the theme inside it,
///   `#test-theme-after` the theme of its next sibling (the app's: the provider's context must not
///   leak to its siblings).
/// - `#test-theme-toggle` switches the dark provider to light and back (`use_theme`).
#[component]
pub fn PageAtomTheme() -> impl IntoView {
    view! {
        <div id="test-page-atom-theme">
            <ThemeProvider default_theme=LeptonicTheme::Dark classes="test-theme-dark">
                <ThemeName id="test-theme-inside" />
                <ThemeToggle />
            </ThemeProvider>
            <ThemeName id="test-theme-after" />
        </div>
    }
}

#[component]
fn ThemeToggle() -> impl IntoView {
    let theme = use_theme::<LeptonicTheme>().expect("inside a ThemeProvider");
    view! {
        <button
            id="test-theme-toggle"
            on:click=move |_| {
                theme
                    .set_theme(
                        match theme.theme().get_untracked() {
                            LeptonicTheme::Light => LeptonicTheme::Dark,
                            LeptonicTheme::Dark => LeptonicTheme::Light,
                        },
                    );
            }
        >
            "Toggle"
        </button>
    }
}
