use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::*;

use crate::pages::documentation::article::Article;

#[component]
pub fn PageSeparator() -> impl IntoView {
    view! {
        <Article>
            <H1>"Separators"</H1>

            <Code>
                {indoc!(r"
                    <Separator />
                ")}
            </Code>

            <Separator />
        </Article>
    }
}
