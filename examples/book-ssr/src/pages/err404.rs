use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::routes;

#[component]
pub(crate) fn PageErr404() -> impl IntoView {
    view! {
        <div class="err-404">
            <div class="info">
                <h1 id="error">"404"</h1>
                <h2 id="whoops">"Whoops, this page doesn\u{2019}t exist :-("</h2>
                <LinkButton href=routes::Root.materialize() size=ButtonSize::Big>"Go to the start page"</LinkButton>
            </div>
            <img id="ferris" src="/res/icon/ferris-panic_transparent.svg" alt="Ferris (the Rust mascot, a crab) panicking"/>
        </div>
    }
}
