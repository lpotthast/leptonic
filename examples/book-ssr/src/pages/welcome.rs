use leptonic::components::prelude::*;
use leptos::*;

use crate::pages::documentation::doc_root::DocRoutes;

#[component]
pub fn PageWelcome() -> impl IntoView {
    view! {
        <Box id="welcome-page">
            <div id="intro">
                <H1 id="slogan">
                    "LEPTONIC"
                </H1>

                <H2 id="sub-slogan">
                    "Build blazingly fast, ergonomic sites with an outstanding developer experience."
                    //"Elevate Web Development with Unleashed Performance and Ergonomic Design using Leptos!"
                </H2>

                // TODO: Investigate: When using AppRoutes::Doc, browser navigation (back) does nothing...
                <LinkButton href=DocRoutes::Overview size=ButtonSize::Big style="font-size: 1.5em; margin: 2em;">
                    "Read the docs"
                </LinkButton>
            </div>

            <div id="features"></div>
        </Box>
    }
}
