use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TypographyDemo() -> impl IntoView {
    view! {
        <Separator />

        <h1>"Typography - H1"</h1>
        <h2>"Typography - H2"</h2>
        <h3>"Typography - H3"</h3>
        <h4>"Typography - H4"</h4>
        <h5>"Typography - H5"</h5>
        <h6>"Typography - H6"</h6>

        <p>"This is a paragraph"</p>

        <Code inline=false>"Typography - Code"</Code>

        <p>
            "This is a paragraph containing an "
            <Code inline=true>"inlined"</Code>
            " piece of code."
        </p>
    }
}
