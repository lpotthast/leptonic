use leptonic::atoms::prelude::Link;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::{
    app::{MainLandmark, SITE_DESCRIPTION},
    routes,
};

#[component]
pub(crate) fn PageErr404() -> impl IntoView {
    view! {
        <Title text="Page not found \u{2013} Leptonic"/>
        <Meta name="description" content=SITE_DESCRIPTION/>

        <MainLandmark class="book-404">
            <div class="book-404-text">
                <p class="book-404-code" aria-hidden="true">"404"</p>
                <h1 class="book-404-title">"This page doesn\u{2019}t exist"</h1>
                <p>"The address may be mistyped, or the page has moved."</p>
                <div class="book-404-actions">
                    <Link href=routes::Root.materialize() classes="book-button">"Go to the start page"</Link>
                    <Link href=routes::Doc.materialize() classes="book-button" attr:data-variant="secondary">
                        "Browse the docs"
                    </Link>
                </div>
            </div>
            <img class="book-404-ferris" src="/res/icon/ferris-panic_transparent.svg" alt="Ferris, the Rust crab, panicking"/>
        </MainLandmark>
    }
}
