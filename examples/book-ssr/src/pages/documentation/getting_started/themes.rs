use indoc::indoc;
use leptonic::hooks::link::LinkTarget;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageThemes() -> impl IntoView {
    view! {
        <DocPage title="Themes">
            <p>
                "Leptonic\u{2019}s hooks and atoms bring no styles. A theme is a value of the "<Code inline=true>"data-theme"</Code>
                " attribute: a "<Code inline=true>"ThemeProvider"</Code>" sets it, and your stylesheets select on it. This "
                "page shows how to provide and switch themes, and how to style the atoms: with your own CSS, or starting "
                "from leptonic\u{2019}s optional "<AnchorLink href="#the-atom-theme">"atom theme"</AnchorLink>"."
            </p>

            <Section title="ThemeProvider">
                <p>
                    "Render a "<Code inline=true>"ThemeProvider"</Code>" around your app. It provides the "
                    <Code inline=true>"ThemeContext"</Code>" that "<Code inline=true>"use_theme"</Code>" reads, and wraps its "
                    "children in a "<Code inline=true>"<div class=\"leptonic-ThemeProvider\" data-theme=\u{2026}>"</Code>" with "
                    <Code inline=true>"display: contents"</Code>". The outermost provider also sets "
                    <Code inline=true>"data-theme"</Code>" on "<Code inline=true>"<html>"</Code>", so that content rendered "
                    "into "<Code inline=true>"<body>"</Code>" (modals, popovers, toasts) is themed too. When that provider "
                    "unmounts, it restores the previous document theme, or removes the attribute if there was none. "
                    "It leaves later changes made by other code in place."
                </p>
                <p>
                    "Without "<Code inline=true>"theme"</Code>", the provider keeps the theme itself, starting with "
                    <Code inline=true>"default_theme"</Code>" (or the theme type\u{2019}s "<Code inline=true>"Default"</Code>
                    "); pass "<Code inline=true>"theme"</Code>" and "<Code inline=true>"set_theme"</Code>" to control it. To "
                    "remember the user\u{2019}s choice, control it with "<Code inline=true>"signal_ls"</Code>", which keeps a "
                    "value in local storage. It starts with the given default, as the server renders, and loads the stored "
                    "theme right after hydration, so a page in a server-rendered app shows the default theme for a moment ("
                    <AnchorLink href="#remembering-the-theme-on-the-server">"keep it in a cookie"</AnchorLink>" to avoid that):"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::theme::{LeptonicTheme, ThemeProvider},
                            signal_ls,
                        };

                        let (theme, set_theme) = signal_ls("theme", LeptonicTheme::default());

                        view! {
                            <ThemeProvider theme set_theme>
                                <App/>
                            </ThemeProvider>
                        }
                    "#)}
                </Code>
                <p>
                    "Render another provider inside to give a part of the page a different theme, e.g. a dark preview "
                    "inside a light page:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        view! {
                            <ThemeProvider default_theme=LeptonicTheme::Dark>
                                <section class="preview">"Always dark"</section>
                            </ThemeProvider>
                        }
                    "#)}
                </Code>
                <ApiTable kind=ApiKind::Props of="ThemeProvider">
                    <ApiRow name="theme" ty="Option<Signal<T>>" default="None">
                        "The theme (controlled): a value or any signal."
                    </ApiRow>
                    <ApiRow name="set_theme" ty="Option<Out<T>>" default="None">
                        "Receives a new theme, e.g. from a theme control calling "<Code inline=true>"set_theme"</Code>" of "
                        "the context: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                        ", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                    </ApiRow>
                    <ApiRow name="default_theme" ty="Option<T>" default="None">
                        "The theme to start with when "<Code inline=true>"theme"</Code>" isn\u{2019}t set. Default: "
                        <Code inline=true>"T::default()"</Code>"."
                    </ApiRow>
                    <ApiRow name="on_theme_change" ty="Option<Callback<T>>" default="None">"Called with every new theme."</ApiRow>
                    <ApiRow name="classes" ty="Classes" default="empty">
                        "Classes of the wrapping "<Code inline=true>"<div>"</Code>", after its default class "
                        <Code inline=true>"leptonic-ThemeProvider"</Code>"."
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"The themed content. Required."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Remembering the Theme on the Server">
                <p>
                    "The server can\u{2019}t read local storage: with "<Code inline=true>"signal_ls"</Code>", a server-rendered "
                    "page arrives in the default theme and switches when it hydrates, a visible flash for readers of the "
                    "other theme. Keep the theme in a cookie instead, which the browser sends with every request: "
                    "leptos-use\u{2019}s "<Code inline=true>"use_cookie_with_options"</Code>" (feature "
                    <Code inline=true>"use_cookie"</Code>", and "<Code inline=true>"axum"</Code>" or "
                    <Code inline=true>"actix"</Code>" in your "<Code inline=true>"ssr"</Code>" feature, so it reads the "
                    "request\u{2019}s cookies) reads it on both sides. Render the theme on "<Code inline=true>"<html>"</Code>
                    " with leptos_meta\u{2019}s "<Code inline=true>"Html"</Code>" too, so that styles on "
                    <Code inline=true>"<body>"</Code>" (outside the provider) have their theme colors from the start. This "
                    "book does it this way:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use codee::string::FromToStringCodec;
                        use leptonic::atoms::theme::{LeptonicTheme, Theme, ThemeProvider};
                        use leptos::prelude::*;
                        use leptos_meta::Html;
                        use leptos_use::{SameSite, UseCookieOptions, use_cookie_with_options};

                        // The theme's name ("light", "dark") in a cookie, for a year.
                        let (cookie, set_cookie) = use_cookie_with_options::<String, FromToStringCodec>(
                            "theme",
                            UseCookieOptions::default()
                                .path("/")
                                .same_site(SameSite::Lax)
                                .max_age(365 * 24 * 60 * 60 * 1000),
                        );
                        let theme = Signal::derive(move || {
                            cookie.with(|name| {
                                [LeptonicTheme::Light, LeptonicTheme::Dark]
                                    .into_iter()
                                    .find(|theme| name.as_deref() == Some(theme.name()))
                                    .unwrap_or_default()
                            })
                        });
                        let set_theme = move |theme: LeptonicTheme| set_cookie.set(Some(theme.name().to_owned()));

                        view! {
                            <Html {..} data-theme=move || theme.get().name()/>
                            <ThemeProvider theme set_theme>
                                <App/>
                            </ThemeProvider>
                        }
                    "#)}
                </Code>
                <p>
                    "The shell must render leptos_meta\u{2019}s "<Code inline=true>"<MetaTags/>"</Code>" for "
                    <Code inline=true>"Html"</Code>"\u{2019}s attributes to reach the server\u{2019}s "<Code inline=true>"<html>"</Code>"."
                </p>
            </Section>

            <Section title="Switching Themes">
                <p>
                    "Read and change the theme with "<Code inline=true>"use_theme"</Code>". It returns the "
                    <Code inline=true>"ThemeContext"</Code>" of the closest provider of that theme type, or "
                    <Code inline=true>"None"</Code>" outside of one. A theme toggle is a "
                    <Link href=routes::doc::Switch.materialize()>"switch"</Link>" on it:"
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::{
                                switch::{SwitchButton, SwitchField},
                                theme::{LeptonicTheme, use_theme},
                            },
                        };

                        let theme = use_theme::<LeptonicTheme>().expect("inside a ThemeProvider");
                        let is_dark = Signal::derive(move || theme.theme().get() == LeptonicTheme::Dark);

                        view! {
                            <SwitchField
                                is_selected=is_dark
                                set_selected=move |dark: bool| {
                                    theme.set_theme(if dark { LeptonicTheme::Dark } else { LeptonicTheme::Light });
                                }
                            >
                                <SwitchButton>
                                    "Dark theme"
                                </SwitchButton>
                            </SwitchField>
                        }
                    "#)}
                </Code>
                <p>
                    "Style the switch like any atom (see the "
                    <Link href=format!("{}#styling", routes::doc::switch::Atom.materialize())>"Switch Atoms"</Link>
                    "); an icon of the current theme inside it is decorative, as the label names the switch."
                </p>
            </Section>

            <Section title="Custom Themes">
                <p>
                    <Code inline=true>"LeptonicTheme"</Code>" names two themes, "<Code inline=true>"light"</Code>" and "
                    <Code inline=true>"dark"</Code>". For themes of your own, define a type implementing the "
                    <Code inline=true>"Theme"</Code>" trait: a name, used as the "<Code inline=true>"data-theme"</Code>
                    " value. The trait requires "<Code inline=true>"Default"</Code>" (the provider\u{2019}s initial theme), "
                    <Code inline=true>"Clone + Copy + PartialEq"</Code>", "<Code inline=true>"Send + Sync"</Code>", and "
                    "serde\u{2019}s "<Code inline=true>"Serialize"</Code>" and "<Code inline=true>"DeserializeOwned"</Code>
                    ", so that "<Code inline=true>"signal_ls"</Code>" can store it."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::theme::{Theme, ThemeProvider};
                        use serde::{Deserialize, Serialize};

                        #[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
                        pub enum AppTheme {
                            #[default]
                            Light,
                            Dark,
                            HighContrast,
                        }

                        impl Theme for AppTheme {
                            fn name(&self) -> &'static str {
                                match self {
                                    Self::Light => "light",
                                    Self::Dark => "dark",
                                    Self::HighContrast => "high-contrast",
                                }
                            }
                        }

                        view! {
                            <ThemeProvider default_theme=AppTheme::HighContrast>
                                <App/>
                            </ThemeProvider>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Styling with Your Own CSS">
                <p>
                    "Define your design tokens per theme, as CSS variables under a "<Code inline=true>"[data-theme=\"...\"]"</Code>
                    " selector, and style the atoms with them: through their default classes ("
                    <Code inline=true>"leptonic-<AtomName>"</Code>") or classes you pass, and the data attributes of their "
                    "state (see "<Link href=format!("{}#styling-atoms", routes::doc::Architecture.materialize())>"Styling Atoms"</Link>
                    "). The atoms then follow the theme without knowing about it:"
                </p>

                <Code language=Language::Css>
                    {indoc!(r#"
                        [data-theme="light"] { --surface: #ffffff; --text: #1d1d1d; --accent: #8856e6; --focus: #0066cc; }
                        [data-theme="dark"] { --surface: #1e1e1e; --text: #f0f0f0; --accent: #b18cff; --focus: #5aa2ff; }

                        .leptonic-Button { background: var(--surface); color: var(--text); }
                        .leptonic-Button[data-pressed] { background: var(--accent); }
                        .leptonic-Button[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 2px; }
                    "#)}
                </Code>

                <p>
                    "This book works that way: it defines its tokens for "<Code inline=true>"light"</Code>" and "
                    <Code inline=true>"dark"</Code>" and styles every atom it uses itself. The \u{201c}Styling\u{201d} "
                    "section of every atom page shows its rules."
                </p>
            </Section>

            <Section title="The Atom Theme">
                <p>
                    "For apps that don\u{2019}t want to style from scratch, leptonic ships an optional theme for its atoms, a "
                    "port of "
                    <Link href="https://react-spectrum.adobe.com/react-aria/" target=LinkTarget::Blank>"react-aria-components"</Link>
                    "\u{2019} starter styles. It styles the atoms through their default classes and data attributes. Include "
                    "it as described in "
                    <Link href=format!("{}#the-optional-atom-theme", routes::doc::Installation.materialize())>"Installation"</Link>":"
                </p>

                <Code language=Language::Css>
                    {indoc!(r#"
                        @use "./leptonic/leptonic-atoms";
                    "#)}
                </Code>

                <ul>
                    <li>
                        "Its light and dark mode follow the nearest "<Code inline=true>"data-theme"</Code>" of "
                        <Code inline=true>"light"</Code>" or "<Code inline=true>"dark"</Code>" (the names of "
                        <Code inline=true>"LeptonicTheme"</Code>"); without a "<Code inline=true>"ThemeProvider"</Code>
                        ", the system preference decides."
                    </li>
                    <li>
                        "Its tokens are CSS variables ("<Code inline=true>"--text-color"</Code>", "
                        <Code inline=true>"--focus-ring-color"</Code>", "<Code inline=true>"--highlight-background"</Code>
                        ", \u{2026}). "<Code inline=true>"--tint"</Code>" recolors every atom at once."
                    </li>
                    <li>
                        "Variants are data attributes you set, e.g. "<Code inline=true>"attr:data-variant=\"secondary\""</Code>
                        " on a "<Code inline=true>"Button"</Code>". Decorative parts the atoms don\u{2019}t render (a "
                        "checkbox\u{2019}s box, a select\u{2019}s chevron) are markup you add, with the classes the theme "
                        "expects; the header of each of its stylesheets lists them."
                    </li>
                    <li>"Rules of your own, on the default classes or on your classes, come after it and win."</li>
                </ul>

                <Code language=Language::Css>
                    {indoc!(r#"
                        @use "./leptonic/leptonic-atoms";

                        :root,
                        [data-theme] {
                            --tint: var(--green);
                        }
                    "#)}
                </Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Installation.materialize()>"Installation"</Link></li>
                <li><Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link></li>
                <li><Link href=routes::doc::ClassesAndStyles.materialize()>"Classes & Styles"</Link></li>
                <li><Link href=routes::doc::Accessibility.materialize()>"Accessibility"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
