use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::doc_root::{doc_routes, SegmentRenderer};

#[component]
pub fn PageWelcome() -> impl IntoView {
    view! {
        <Box attr:id="welcome-page">
            <div id="intro">
                <h1 attr:id="slogan">
                    "LEPTONIC"
                </h1>

                <h2 attr:id="sub-slogan">
                    "Build blazingly fast, ergonomic sites with an outstanding developer experience."
                    //"Elevate Web Development with Unleashed Performance and Ergonomic Design using Leptos!"
                </h2>

                // TODO: Investigate: When using AppRoutes::Doc, browser navigation (back) does nothing...
                <LinkButton href=doc_routes::OVERVIEW.to_href() size=ButtonSize::Big attr:style="font-size: 1.5em; margin: 2em;">
                    "Read the docs"
                </LinkButton>
            </div>

            <div id="features"></div>
        </Box>
    }
}
