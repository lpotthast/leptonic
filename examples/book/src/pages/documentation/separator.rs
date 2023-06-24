use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageSeparator(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"Separators"</H1>

        <Code>
            {indoc!(r#"
                <Separator />
            "#)}
        </Code>

        <Separator />
    }
}
