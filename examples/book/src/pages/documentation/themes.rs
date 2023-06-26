use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

use crate::pages::documentation::doc_root::DocRoutes;

#[component]
pub fn PageThemes(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"Theming"</H1>

        <P>"From the first day on, Leptonic was built with theming in mind."</P>

        <P>
            "Styles are not attached to components directly, are not injected in any way upon rendering, "
            "which means that all components could be describes as \"headless\" in that regard."
        </P>

        <P>
            "All styling is provided though the "<LinkExt href="https://github.com/lpotthast/leptonic/tree/main/leptonic-theme" target=LinkExtTarget::Blank>"leptonic-theme"</LinkExt>" crate. "
            "When used as a build.rs dependency, this crate can write out SCSS code, styling the Leptonic components in two themes: "<Code inline=true>"light"</Code>" and "<Code inline=true>"dark"</Code>"."
        </P>

        <P>
            "The "<Code inline=true>"<Root>"</Code>" component already discussed in the "<Link href=DocRoutes::Usage>"Usage"</Link>" section provides everything required (namely rendering the "<Code inline=true>"<ThemeProvider>"</Code>" component) to active a theme."
        </P>

        <P>
            "A simple theme-switching component, realized using a "<Link href=DocRoutes::Usage>"Toggle"</Link>", is provided, which can be used simply like this anywhere you want:"
        </P>

        <Code>
            {indoc!(r#"
                <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark/>
            "#)}
        </Code>

        <P>
            "All components make broad use of CSS variables, with which many aspects of the two default themes can be changed."
        </P>

        <P>
            "For example, this excerpt from the main "<Code inline=true>"style.scss"</Code>" of this book shows you how we include the leptonic theme and overwrite a few variables."
        </P>

        <Code>
            {indoc!(r#"
                @import "../generated/leptonic/leptonic-themes";

                [theme="light"] {
                    --brand-color: #e66956;

                    --drawer-background-color: none;
                    --drawer-box-shadow: none;
                }

                [theme="dark"] {
                    --brand-color: #e66956;

                    --drawer-background-color: none;
                    --drawer-box-shadow: none;
                }
            "#)}
        </Code>
    }
}
