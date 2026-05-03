use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn LinkConceptDemo() -> impl IntoView {
    view! {
        <Link href="https://github.com/lpotthast/leptonic">"Leptonic on GitHub"</Link>
    }
}
