use leptonic::components::{root::Root, theme::LeptonicTheme};
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{components::*, hooks::use_params_map, path};

use crate::pages::{FIXTURES, find_fixture};

pub const LEPTOS_OUTPUT_NAME: &str = env!("LEPTOS_OUTPUT_NAME");

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href=format!("/pkg/{LEPTOS_OUTPUT_NAME}.css") />
        <Title text="Leptonic Test App" />

        <Root default_theme=LeptonicTheme::default()>
            <Router>
                <Routes fallback=|| view! { <p>"Not Found"</p> }>
                    <Route path=path!("/") view=PageIndex />
                    // One route per group: a generic `/:group/:name` route would also match the
                    // static assets under `/pkg/...` and serve HTML in their place.
                    <Route path=path!("/atoms/:name") view=|| view! { <PageFixture group="atoms" /> } />
                    <Route path=path!("/hooks/:name") view=|| view! { <PageFixture group="hooks" /> } />
                    <Route
                        path=path!("/components/:name")
                        view=|| view! { <PageFixture group="components" /> }
                    />
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
                {FIXTURES
                    .iter()
                    .map(|fixture| {
                        view! {
                            <li>
                                <a href=fixture.path()>
                                    {format!("{}: {}", fixture.group, fixture.title)}
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}

#[component]
fn PageFixture(group: &'static str) -> impl IntoView {
    let params = use_params_map();
    move || {
        params.with(|params| {
            let name = params.get("name").unwrap_or_default();
            match find_fixture(group, &name) {
                Some(fixture) => (fixture.view)(),
                None => view! { <p>"Not Found"</p> }.into_any(),
            }
        })
    }
}
