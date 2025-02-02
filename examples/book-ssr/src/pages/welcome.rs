use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::routes;

#[component]
pub fn PageWelcome() -> impl IntoView {
    view! {
        <div id="welcome-page">
            <div id="intro">
                <h1 attr:id="slogan">
                    "LEPTONIC"
                </h1>

                <h2 attr:id="sub-slogan">
                    "Build blazingly fast, ergonomic sites with an outstanding developer experience."
                    //"Elevate Web Development with Unleashed Performance and Ergonomic Design using Leptos!"
                </h2>

                // TODO: Investigate: When using AppRoutes::Doc, browser navigation (back) does nothing...
                <LinkButton href=routes::doc::Overview.materialize() size=ButtonSize::Big attr:style="font-size: 1.5em; margin: 2em;">
                    "Read the docs"
                </LinkButton>
            </div>

            <div id="features"></div>
        </div>
    }
}
