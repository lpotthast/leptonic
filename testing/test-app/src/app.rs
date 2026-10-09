use leptonic::atoms::theme::{LeptonicTheme, ThemeProvider};
use leptos::prelude::*;
use leptos_meta::{HashedStylesheet, MetaTags, Title, provide_meta_context};
use leptos_router::{components::*, hooks::use_params_map, path};

use crate::pages::{FIXTURES, find_fixture};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                // What the page reports, for `PageActions::diagnostics` (each list separately):
                // Rust panics (the panic hook logs them with `console.error` before the wasm
                // traps), uncaught errors and unhandled rejections, every other `console.error`,
                // and `console.warn`. `PageActions::expect_no_page_errors` fails a test on any of
                // them (tests expecting a warning clear them).
                <script>
                    "window.__panics = [];
                    window.__uncaughtErrors = [];
                    window.__consoleErrors = [];
                    window.__consoleWarnings = [];
                    window.addEventListener('error', e => window.__uncaughtErrors.push(String(e.message)));
                    window.addEventListener('unhandledrejection', e => window.__uncaughtErrors.push(String(e.reason)));
                    const consoleError = console.error.bind(console);
                    console.error = (...args) => {
                        const message = args.map(String).join(' ');
                        if (message.includes('panicked at')) {
                            window.__panics.push(message);
                        } else {
                            window.__consoleErrors.push(message);
                        }
                        consoleError(...args);
                    };
                    const consoleWarn = console.warn.bind(console);
                    console.warn = (...args) => {
                        window.__consoleWarnings.push(args.map(String).join(' '));
                        consoleWarn(...args);
                    };"
                </script>
                <AutoReload options=options.clone() />
                // The content-hashed stylesheet (the server reads the hashes).
                <HashedStylesheet options=options.clone() id="leptos" />
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
        <Title text="Leptonic Test App" />

        <ThemeProvider default_theme=LeptonicTheme::default()>
            <Router>
                <Routes fallback=|| view! { <p>"Not Found"</p> }>
                    <Route path=path!("/") view=PageIndex />
                    // One route per group: a generic `/:group/:name` route would also match the
                    // static assets under `/pkg/...` and serve HTML in their place.
                    <Route
                        path=path!("/atoms/:name")
                        view=|| view! { <PageFixture group="atoms" /> }
                    />
                    <Route
                        path=path!("/hooks/:name")
                        view=|| view! { <PageFixture group="hooks" /> }
                    />
                </Routes>
            </Router>
        </ThemeProvider>
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
                                <a href=fixture
                                    .path()>{format!("{}: {}", fixture.group, fixture.title)}</a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
            <p>"Fixtures with query parameters:"</p>
            <ul id="test-index-variants">
                {QUERY_VARIANTS
                    .iter()
                    .map(|path| {
                        view! {
                            <li>
                                <a href=*path>{*path}</a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}

/// The fixtures' variants the browser tests load with query parameters (the fixtures read them
/// with `use_query_map`). Linked from the index, so that the hydration test
/// (`leptonic/tests/ui_tests/test_hydration_ids.rs`) checks them like every fixture: add a variant
/// here when a test loads a new one.
const QUERY_VARIANTS: &[&str] = &[
    "/hooks/dnd-targets?allowed=link&op=copy",
    "/hooks/dnd-targets?ancestor",
    "/hooks/dnd-targets?cancel-2",
    "/hooks/dnd-targets?disabled-drag",
    "/hooks/dnd-targets?disabled-drop",
    "/hooks/dnd-targets?hidden-tree",
    "/hooks/dnd-targets?op=copy",
    "/hooks/dnd-collection?items=6",
    "/hooks/dnd-collection?items=6&only-on&cancel=1,4",
    "/hooks/dnd-collection?only-on&cancel=",
    "/hooks/clipboard?cut",
    "/hooks/clipboard?cut&items=none",
    "/hooks/clipboard?items=action",
    "/hooks/clipboard?items=action&cut",
    "/hooks/clipboard?items=custom&paste",
    "/hooks/clipboard?items=multiple&paste",
    "/hooks/clipboard?items=multiple-types&paste",
    "/hooks/clipboard?items=none",
    "/hooks/clipboard?items=types&paste",
    "/hooks/clipboard?paste",
];

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
