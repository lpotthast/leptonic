use leptonic::prelude::*;
use leptos::*;

use crate::pages::documentation::doc_root::DocRoutes;

#[component]
pub fn PageWelcome() -> impl IntoView {
    view! {
        <Box id="welcome-page">
            <img src="/res/leptonic.svg" id="big-logo" alt="Big Leptonic logo"/>

            <H1 id="slogan">
                "LEPTONIC"
            </H1>

            <H2 id="sub-slogan">
                "Build blazingly fast, ergonomic sites with an outstanding developer experience."
                //"Elevate Web Development with Unleashed Performance and Ergonomic Design using Leptos!"
            </H2>

            // TODO: Investigae: When using AppRoutes::Doc, browser navigation (back) does nothing...
            <LinkButton href=DocRoutes::Overview size=ButtonSize::Big style="font-size: 1.5em; margin: 2em;">
                "Read the docs"
            </LinkButton>
        </Box>
    }
}
