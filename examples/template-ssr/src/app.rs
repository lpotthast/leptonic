use crate::error_template::{AppError, ErrorTemplate};

use std::fmt::Display;

use leptos::*;
use leptos_meta::{provide_meta_context, Link as MetaLink, Meta, Script, Stylesheet, Title};
use leptos_router::*;

use leptonic::prelude::*;

use crate::pages::welcome::Welcome;

#[derive(Debug, Copy, Clone)]
pub enum AppRoutes {
    Welcome,
}

impl AppRoutes {
    pub const fn route(self) -> &'static str {
        match self {
            Self::Welcome => "",
        }
    }
}

/// Required so that `Routes` variants can be used in `<Route path=Routes::Foo ...>` definitions.
impl Display for AppRoutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.route())
    }
}

/// Required so that `Routes` variants can be used in `<Link href=Routes::Foo ...>` definitions.
impl ToHref for AppRoutes {
    fn to_href(&self) -> Box<dyn Fn() -> String + '_> {
        Box::new(move || format!("/{}", self.route()))
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Meta name="description" content="Leptonic"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Stylesheet id="leptos" href="/pkg/leptonic-template-ssr.css"/>
        <Stylesheet href="https://fonts.googleapis.com/css?family=Roboto&display=swap"/>

        <MetaLink rel="icon" href="/res/icon/leptonic_x64.png"/>
        <MetaLink rel="apple-touch-icon" href="/res/icon/maskable_icon_x192.png"/>

        <Script type_="module" src="/js/tiptap-bundle.min.js"/>
        <Script type_="module" src="/js/tiptap.js"/>

        <Title text="Leptonic"/>

        <Root default_theme=LeptonicTheme::default()>
            <Router fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! {
                    <ErrorTemplate outside_errors/>
                }
                .into_view()
            }>
                <Routes>
                    <Route path=AppRoutes::Welcome view=|| view! { <Welcome/> }/>
                </Routes>
            </Router>
        </Root>
    }
}
