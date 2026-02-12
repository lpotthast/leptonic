use leptonic::{
    atoms::button::Button,
    components::{root::Root, theme::LeptonicTheme},
};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{components::*, path};

pub const LEPTOS_OUTPUT_NAME: &str = env!("LEPTOS_OUTPUT_NAME");

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href=format!("/pkg/{LEPTOS_OUTPUT_NAME}.css")/>
        <Title text="Leptonic Test App"/>

        <Root default_theme=LeptonicTheme::default()>
            <Router>
                <Routes fallback=|| view! { <p>"Not Found"</p> }>
                    <Route path=path!("/") view=PageIndex/>
                    <Route path=path!("/atoms/button") view=PageAtomButton/>
                </Routes>
            </Router>
        </Root>
    }
}

#[component]
fn PageIndex() -> impl IntoView {
    view! {
        <div id="test-index">
            <h1>"Leptonic Test App"</h1>
            <p>"Available test pages:"</p>
            <ul>
                <li><a href="/atoms/button">"Atoms: Button"</a></li>
            </ul>
        </div>
    }
}

#[component]
fn PageAtomButton() -> impl IntoView {
    let (basic_count, set_basic_count) = signal(0u32);
    let (disabled_count, set_disabled_count) = signal(0u32);

    view! {
        <div id="test-page-atom-button">
            <h1>"Button Atom Test Page"</h1>

            <section>
                <h2>"Basic Button"</h2>
                <Button
                    on_press=move |_| set_basic_count.update(|c| *c += 1)
                    attr:id="test-button-basic"
                >
                    "Press me"
                </Button>
                <span id="test-button-basic-count">{basic_count}</span>
            </section>

            <section>
                <h2>"Disabled Button"</h2>
                <Button
                    on_press=move |_| set_disabled_count.update(|c| *c += 1)
                    disabled=Signal::from(true)
                    attr:id="test-button-disabled"
                >
                    "Disabled"
                </Button>
                <span id="test-button-disabled-count">{disabled_count}</span>
            </section>
        </div>
    }
}
