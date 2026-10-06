use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TypographyDemo() -> impl IntoView {
    view! {
        <h1>"Heading 1"</h1>
        <h2>"Heading 2"</h2>
        <h3>"Heading 3"</h3>
        <h4>"Heading 4"</h4>
        <h5>"Heading 5"</h5>
        <h6>"Heading 6"</h6>

        <p>"This is a paragraph."</p>

        <Code language=Language::Rust>"let answer = 42;"</Code>

        <p>
            "This is a paragraph containing an "
            <Code inline=true>"inlined"</Code>
            " piece of code."
        </p>
    }
}
