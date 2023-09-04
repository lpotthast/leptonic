use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

use crate::pages::documentation::doc_root::DocRoutes;

#[component]
pub fn PageThemes() -> impl IntoView {
    view! {
        <H1>"Theming"</H1>

        <P>"From the first day on, Leptonic was built with theming in mind."</P>

        <P>
            "Styles are not attached to components directly, are not injected in any way upon rendering, "
            "which means that all components could be describes as \"headless\" in that regard."
        </P>

        <P>
            "All styling is provided through the "<LinkExt href="https://github.com/lpotthast/leptonic/tree/main/leptonic-theme" target=LinkExtTarget::Blank>"leptonic-theme"</LinkExt>" crate. "
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
            "We are using the "<Code inline=true>"<LeptonicTheme>"</Code>" enum here, which describes the two out-of-the-box themes (light and dark). "
            "This is not mandatory though as you could create your own theme-defining type and your own theme toggle components."
        </P>

        <H2>"Customization"</H2>

        <P>
            "Having a theme, even better a theme provided by default is, is great, but only if that them can be customized in a meaningful way."
        </P>

        <P>
            "All components styles therefore make broad use of CSS variables, with which many aspects of the two default themes can be changed. "
            "In every page of this book explaining a component, we hint you to the styles you might want to override. A theme generator may come in the future..."
        </P>

        <P>
            "Have a look at this excerpt from this book's main "<Code inline=true>"style.scss"</Code>" file, "
            "showing you how we include the leptonic standard themes previously written by our build script "
            "and overwrite a few variables to meet our design needs."
        </P>

        <Code>
            {indoc!(r#"
                @import "../generated/leptonic/leptonic-themes";

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
    }
}
