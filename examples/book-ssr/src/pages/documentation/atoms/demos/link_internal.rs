use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::routes;

#[component]
pub fn LinkInternalDemo() -> impl IntoView {
    view! {
        <Link href=routes::doc::link::LinkAtom.materialize()>"This is a link to the current route."</Link>
    }
}
