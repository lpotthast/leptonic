use indoc::indoc;
use leptonic::hooks::LinkTarget;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageThemes() -> impl IntoView {
    view! {
        <DocPage title="Themes">
            <p>
                "Leptonic\u{2019}s components don\u{2019}t carry their styles. All styling comes from the "
                <Link href="https://github.com/lpotthast/leptonic/tree/main/leptonic-theme" target=LinkTarget::Blank>"leptonic-theme"</Link>
                " crate, whose stylesheets leptonic\u{2019}s build script copies into your project (see "
                <Link href=routes::doc::Installation.materialize()>"Installation"</Link>"). They define two themes, "
                <Code inline=true>"light"</Code>" and "<Code inline=true>"dark"</Code>". Hooks and atoms bring no styles at all."
            </p>

            <Section title="Switching Themes">
                <p>
                    <Code inline=true>"<Root>"</Code>" provides the active theme. It starts with its "<Code inline=true>"default_theme"</Code>
                    ", remembers the user\u{2019}s choice in local storage (key "<Code inline=true>"theme"</Code>") and sets the "
                    <Code inline=true>"data-theme"</Code>" attribute on the document element, which the theme stylesheets select on."
                </p>

                <p>"Let users switch between two themes with a "<Code inline=true>"ThemeToggle"</Code>":"</p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark/>
                    ")}
                </Code>

                <p>
                    "The toggle is a "<Link href=routes::doc::Switch.materialize()>"switch"</Link>" showing the icons of both "
                    "themes; it is on while the "<Code inline=true>"on"</Code>" theme is active. Screen readers name it after that theme "
                    "(\u{201c}dark theme\u{201d}); pass "<Code inline=true>"aria_label"</Code>" for a different name."
                </p>

                <p>
                    "To build your own theme control, read and change the theme with "<Code inline=true>"use_theme"</Code>
                    ". It returns the "<Code inline=true>"ThemeContext"</Code>" of the closest provider of that theme type, or "
                    <Code inline=true>"None"</Code>" outside of one:"
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        let theme = use_theme::<LeptonicTheme>().expect("inside <Root>");
                        view! {
                            <Button on_press=move |_| theme.set_theme(LeptonicTheme::Dark)>"Dark"</Button>
                            <p>"Current theme: " {move || theme.theme().get().name()}</p>
                        }
                    "#)}
                </Code>

            </Section>

            <Section title="ThemeProvider">
                <p>
                    <Code inline=true>"<Root>"</Code>" renders a "<Code inline=true>"ThemeProvider"</Code>" for you. It "
                    "provides the "<Code inline=true>"ThemeContext"</Code>" that "<Code inline=true>"use_theme"</Code>" and "
                    <Code inline=true>"ThemeToggle"</Code>" read, and wraps its children in a "
                    <Code inline=true>"<div data-theme=\u{2026}>"</Code>" with "<Code inline=true>"display: contents"</Code>
                    ". The outermost provider also sets "<Code inline=true>"data-theme"</Code>" on "
                    <Code inline=true>"<html>"</Code>"."
                </p>
                <p>
                    "Render one of your own to give a part of the page a different theme, e.g. a dark preview inside a "
                    "light page. Without "<Code inline=true>"theme"</Code>", it keeps the theme itself, starting with "
                    <Code inline=true>"default_theme"</Code>" (or the theme type\u{2019}s "<Code inline=true>"Default"</Code>
                    "); pass "<Code inline=true>"theme"</Code>" and "<Code inline=true>"set_theme"</Code>" to control it. "
                    <Code inline=true>"<Root>"</Code>" controls its provider with "<Code inline=true>"signal_ls"</Code>
                    ", which starts with the default, as the server renders, and loads the stored theme right after "
                    "hydration:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        view! {
                            <ThemeProvider default_theme=LeptonicTheme::Dark>
                                <Card>"Always dark"</Card>
                            </ThemeProvider>
                        }
                    "#)}
                </Code>
                <ApiTable kind=ApiKind::Props of="ThemeProvider">
                    <ApiRow name="theme" ty="Option<Signal<T>>" default="None">
                        "The theme (controlled): a value or any signal."
                    </ApiRow>
                    <ApiRow name="set_theme" ty="Option<Out<T>>" default="None">
                        "Receives a new theme, e.g. from a "<Code inline=true>"ThemeToggle"</Code>": an "
                        <Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "
                        <Code inline=true>"Callback"</Code>", \u{2026}"
                    </ApiRow>
                    <ApiRow name="default_theme" ty="Option<T>" default="None">
                        "The theme to start with when "<Code inline=true>"theme"</Code>" isn\u{2019}t set. Default: "
                        <Code inline=true>"T::default()"</Code>"."
                    </ApiRow>
                    <ApiRow name="on_theme_change" ty="Option<Callback<T>>" default="None">"Called with every new theme."</ApiRow>
                    <ApiRow name="children" ty="Children">"The themed content. Required."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Custom Themes">
                <p>
                    <Code inline=true>"LeptonicTheme"</Code>" describes the two built-in themes. For themes of your own, define "
                    "a type implementing the "<Code inline=true>"Theme"</Code>" trait: a name, used as the "
                    <Code inline=true>"data-theme"</Code>" value. For "<Code inline=true>"ThemeToggle"</Code>", also implement "
                    <Code inline=true>"ThemeIcon"</Code>" (the components\u{2019} trait for its icons). "
                    "The trait requires "<Code inline=true>"Default"</Code>" (the provider\u{2019}s initial theme), "
                    <Code inline=true>"Clone + Copy + PartialEq"</Code>", "<Code inline=true>"Send + Sync"</Code>", and "
                    "serde\u{2019}s "<Code inline=true>"Serialize"</Code>" and "<Code inline=true>"DeserializeOwned"</Code>
                    ", as "<Code inline=true>"<Root>"</Code>" stores the theme in local storage."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{components::prelude::*, prelude::*};
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

                        impl ThemeIcon for AppTheme {
                            fn icon(&self) -> icondata::Icon {
                                match self {
                                    Self::Light => icondata::BsSun,
                                    Self::Dark => icondata::BsMoon,
                                    Self::HighContrast => icondata::BsCircleHalf,
                                }
                            }
                        }

                        view! {
                            <Root default_theme=AppTheme::default()>
                                <ThemeToggle off=AppTheme::Light on=AppTheme::HighContrast/>
                            </Root>
                        }
                    "#)}
                </Code>
                <p>
                    "The theme stylesheets define the variables of "<Code inline=true>"light"</Code>" and "
                    <Code inline=true>"dark"</Code>" only. A theme with another name needs all of them under its own "
                    "selector ("<Code inline=true>"[data-theme=\"high-contrast\"]"</Code>"): start from a copy of "
                    <Code inline=true>"leptonic/themes/light.scss"</Code>" in your style directory."
                </p>
            </Section>

            <Section title="Customization">
                <p>
                    "The theme stylesheets are built on CSS variables, so you adapt a theme by overriding variables for its "
                    <Code inline=true>"data-theme"</Code>". Each component page lists the variables of its component under "
                    "\u{201c}Styling\u{201d}."
                </p>

                <p>"This book, for example, includes the themes and changes a few variables:"</p>

                <Code language=Language::Css>
                    {indoc!(r#"
                        @use "./leptonic/leptonic-themes";

                        [data-theme] {
                            --font-family: 'Roboto', sans-serif;
                            --typography-code-font-family: 'JetBrains Mono', monospace;
                            --link-color: var(--book-brand-text-color);
                        }

                        [data-theme="light"] {
                            --book-brand-text-color: #a8352a;
                        }

                        [data-theme="dark"] {
                            --app-bar-background-color: #141414;
                            --book-brand-text-color: #f08a7a;
                        }
                    "#)}
                </Code>
            </Section>
        </DocPage>
    }
}
