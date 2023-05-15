use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageTypography(cx: Scope) -> impl IntoView {
    view! { cx,
        <H2>"Typography"</H2>

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
    }
}
