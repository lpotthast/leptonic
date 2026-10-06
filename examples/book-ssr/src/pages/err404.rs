use leptonic::components::prelude::*;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::{
    app::{MAIN_ID, SITE_DESCRIPTION},
    routes,
};

#[component]
pub(crate) fn PageErr404() -> impl IntoView {
    view! {
        <Title text="Page not found \u{2013} Leptonic"/>
        <Meta name="description" content=SITE_DESCRIPTION/>

        <main id=MAIN_ID class="book-404" tabindex="-1">
            <div class="book-404-text">
                <p class="book-404-code" aria-hidden="true">"404"</p>
                <h1 class="book-404-title">"This page doesn\u{2019}t exist"</h1>
                <p>"The address may be mistyped, or the page has moved."</p>
                <div class="book-404-actions">
                    <LinkButton href=routes::Root.materialize() size=ButtonSize::Big>"Go to the start page"</LinkButton>
                    <LinkButton
                        href=routes::Doc.materialize()
                        size=ButtonSize::Big
                        variant=ButtonVariant::Outlined
                        color=ButtonColor::Secondary
                    >
                        "Browse the docs"
                    </LinkButton>
                </div>
            </div>
            <img class="book-404-ferris" src="/res/icon/ferris-panic_transparent.svg" alt="Ferris, the Rust crab, panicking"/>
        </main>
    }
}
