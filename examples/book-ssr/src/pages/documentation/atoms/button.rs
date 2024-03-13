use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::atoms::prelude as atoms;
use leptonic::components::prelude::*;
use leptos::*;
use leptos_use::use_window;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageAtomButton() -> impl IntoView {
    view! {
        <Article>
            <H1 id="button" class="anchor">
                "Button"
                <AnchorLink href="#button" description="Direct link to article header"/>
            </H1>

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

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Button", link: "#button" },
            ]
        }/>
    }
}
