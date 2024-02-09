use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::*;

#[component]
pub fn PageSeparator() -> impl IntoView {
    view! {
        <H1>"Separators"</H1>

        <Code>
            {indoc!(r"
                <Separator />
            ")}
        </Code>

        <Separator />
    }
}
