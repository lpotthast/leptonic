use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::routes;

#[component]
pub fn LinkButtonDemo() -> impl IntoView {
    view! {
        <LinkButton href=routes::doc::Overview.materialize()>
            "Read the docs"
        </LinkButton>
    }
}
