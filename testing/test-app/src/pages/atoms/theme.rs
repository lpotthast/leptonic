use leptonic::atoms::theme::{LeptonicTheme, ThemeProvider, use_theme};
use leptos::{prelude::*, tachys::reactive_graph::OwnedView};

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
/// - `.test-theme-setter`: a provider with `set_theme` but no `theme` (starts light); its buttons
///   `#test-theme-setter-light`/`-dark` switch it, `#test-theme-setter-written` shows what
///   `set_theme` received, `#test-theme-setter-changes` counts `on_theme_change` calls.
/// - `.test-theme-controlled`: a provider with a fixed `theme` (dark) and no setter; its buttons
///   only request changes, listed in `#test-theme-controlled-requests`.
#[component]
pub fn PageAtomTheme() -> impl IntoView {
    let written = RwSignal::new(None::<LeptonicTheme>);
    let changes = RwSignal::new(0u32);
    let requests = RwSignal::new(String::new());
    view! {
        <div id="test-page-atom-theme">
            <ThemeProvider default_theme=LeptonicTheme::Dark classes="test-theme-dark">
                <ThemeName id="test-theme-inside" />
                <ThemeToggle />
            </ThemeProvider>
            <ThemeName id="test-theme-after" />

            <ThemeProvider<LeptonicTheme>
                set_theme=move |theme: LeptonicTheme| written.set(Some(theme))
                on_theme_change=move |_: LeptonicTheme| changes.update(|c| *c += 1)
                classes="test-theme-setter"
            >
                <ThemeName id="test-theme-setter-inside" />
                <ThemeButtons prefix="setter" />
            </ThemeProvider<LeptonicTheme>>
            <div>
                "Written: "
                <span id="test-theme-setter-written">
                    {move || written.get().map(|theme| format!("{theme:?}")).unwrap_or_default()}
                </span>
                " Changes: "
                <span id="test-theme-setter-changes">{changes}</span>
            </div>

            <ThemeProvider<LeptonicTheme>
                theme=LeptonicTheme::Dark
                on_theme_change=move |theme: LeptonicTheme| requests.update(|r| r.push_str(&format!("{theme:?};")))
                classes="test-theme-controlled"
            >
                <ThemeName id="test-theme-controlled-inside" />
                <ThemeButtons prefix="controlled" />
            </ThemeProvider<LeptonicTheme>>
            <div>"Requests: " <span id="test-theme-controlled-requests">{requests}</span></div>
            <RootThemeLifecycle />
        </div>
    }
}

#[component]
fn RootThemeLifecycle() -> impl IntoView {
    let parent = Owner::current().expect("the fixture has an owner");
    // This root must not inherit the app's ThemeProvider. Keep the shared hydration context,
    // restore the current owner after creating it, and let Show dispose providers normally.
    let isolated = parent.with(|| Owner::new_root(parent.shared_context()));
    let view = isolated.with(|| {
        let mounted = RwSignal::new(false);
        let nested_mounted = RwSignal::new(true);
        view! {
            <section aria-label="Root theme lifecycle">
                <button disabled=move || mounted.get() on:click=move |_| mounted.set(true)>
                    "Mount root provider"
                </button>
                <button disabled=move || !mounted.get() on:click=move |_| mounted.set(false)>
                    "Unmount root provider"
                </button>
                <Show when=move || mounted.get()>
                    <ThemeProvider default_theme=LeptonicTheme::Dark classes="test-theme-root">
                        <ThemeName id="test-theme-root-inside" />
                        <ThemeButtons prefix="root" />
                        <button
                            disabled=move || !nested_mounted.get()
                            on:click=move |_| nested_mounted.set(false)
                        >
                            "Unmount nested provider"
                        </button>
                        <Show when=move || nested_mounted.get()>
                            <ThemeProvider default_theme=LeptonicTheme::Light classes="test-theme-nested">
                                <ThemeName id="test-theme-nested-inside" />
                            </ThemeProvider>
                        </Show>
                    </ThemeProvider>
                </Show>
            </section>
        }
    });
    OwnedView::new_with_owner(view, isolated)
}

/// Buttons `#test-theme-{prefix}-light` and `-dark` switching the closest provider (`use_theme`).
#[component]
fn ThemeButtons(prefix: &'static str) -> impl IntoView {
    let theme = use_theme::<LeptonicTheme>().expect("inside a ThemeProvider");
    view! {
        <button id=format!("test-theme-{prefix}-light") on:click=move |_| theme.set_theme(LeptonicTheme::Light)>
            "Light"
        </button>
        <button id=format!("test-theme-{prefix}-dark") on:click=move |_| theme.set_theme(LeptonicTheme::Dark)>
            "Dark"
        </button>
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
