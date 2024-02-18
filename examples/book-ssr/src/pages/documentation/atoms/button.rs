use indoc::indoc;
use leptonic::atoms::prelude as la;
use leptonic::components::prelude::*;
use leptos::*;

#[component]
pub fn PageAtomButton() -> impl IntoView {
    view! {
        <H1>"Button"</H1>

        <P>"..."</P>

        <Code>
            {indoc!(r#"
                use leptonic::atoms::prelude as la;

                view! {
                    <la::Button on_press=move |_| {}>
                        "Press me"
                    </la::Button>
                }
            "#)}
        </Code>

        <la::Button on_press=move |_| {}>
            "Press me"
        </la::Button>
    }
}
