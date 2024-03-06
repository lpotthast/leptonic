use indoc::indoc;
use leptonic::atoms::prelude as atoms;
use leptonic::components::prelude::*;
use leptos::*;
use leptos_use::use_window;

use crate::pages::documentation::article::Article;

#[component]
pub fn PageAtomButton() -> impl IntoView {
    view! {
        <Article>
            <H1>"Button"</H1>

            <P>"..."</P>

            <Code>
                {indoc!(r#"
                    use leptonic::atoms::prelude as atoms;
                    use leptos_use::use_window;

                    view! {
                        <atoms::Button on_press=move |_| {
                            if let Some(window) = use_window().as_ref() {
                                let _ = window.alert_with_message("Pressed!");
                            }
                        }>
                            "Press me"
                        </atoms::Button>
                    }
                "#)}
            </Code>

            <atoms::Button on_press=move |_| {
                if let Some(window) = use_window().as_ref() {
                    let _ = window.alert_with_message("Pressed!");
                }
            }>
                "Press me"
            </atoms::Button>
        </Article>
    }
}
