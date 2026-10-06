use indoc::indoc;
use leptonic::{components::prelude::*, hooks::LinkTarget};
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageThemes() -> impl IntoView {
    view! {
        <DocPage title="Themes">
            <p>
                "Leptonic\u{2019}s components don\u{2019}t carry their styles. All styling comes from the "
                <LinkExt href="https://github.com/lpotthast/leptonic/tree/main/leptonic-theme" target=LinkTarget::_Blank>"leptonic-theme"</LinkExt>
                " crate, whose stylesheets leptonic\u{2019}s build script copies into your project (see "
                <Link href=routes::doc::Installation.materialize()>"Installation"</Link>"). They define two themes, "
                <Code inline=true>"light"</Code>" and "<Code inline=true>"dark"</Code>". Hooks and atoms bring no styles at all."
            </p>

            <Section title="Switching themes">
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

                <p>
                    <Code inline=true>"LeptonicTheme"</Code>" describes the two built-in themes. For themes of your own, define a type "
                    "implementing the "<Code inline=true>"Theme"</Code>" trait (a name, used as the "<Code inline=true>"data-theme"</Code>
                    " value, and an icon for the toggle) and use it with "<Code inline=true>"<Root>"</Code>" and "
                    <Code inline=true>"ThemeToggle"</Code>"."
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

                        [data-theme="light"] {
                            --brand-color: #e66956;
                            --drawer-background-color: none;
                            --drawer-box-shadow: none;
                        }

                        [data-theme="dark"] {
                            --brand-color: #e66956;
                            --drawer-background-color: none;
                            --drawer-box-shadow: none;
                        }
                    "#)}
                </Code>
            </Section>
        </DocPage>
    }
}
