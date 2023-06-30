use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageTypography(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"Typography"</H1>

        <Separator />

        <H1>"Typography - H1"</H1>
        <H2>"Typography - H2"</H2>
        <H3>"Typography - H3"</H3>
        <H4>"Typography - H4"</H4>
        <H5>"Typography - H5"</H5>
        <H6>"Typography - H6"</H6>

        <P>"This is a paragraph"</P>

        <Code inline=false>"Typography - Code"</Code>

        <P>
            "This is a paragraph containing an "
            <Code inline=true>"inlined"</Code>
            " piece of code."
        </P>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --typography-font-family
                --typography-h1-margin
                --typography-h1-font-size
                --typography-h1-font-weight
                --typography-h2-margin
                --typography-h2-font-size
                --typography-h2-font-weight
                --typography-h3-margin
                --typography-h3-font-size
                --typography-h3-font-weight
                --typography-h4-margin
                --typography-h4-font-size
                --typography-h4-font-weight
                --typography-h5-margin
                --typography-h5-font-size
                --typography-h5-font-weight
                --typography-h6-margin
                --typography-h6-font-size
                --typography-h6-font-weight
                --typography-p-margin
                --typography-p-font-size
                --typography-p-font-weight
                --typography-p-line-height
                --typography-code-margin
                --typography-code-padding
                --typography-code-font-size
                --typography-code-font-weight
                --typography-code-line-height
                --typography-code-border-radius
                --typography-code-background-color
                --typography-code-color
                --typography-inline-code-margin
                --typography-inline-code-padding
                --typography-inline-code-line-height
            "#)}
        </Code>
    }
}
