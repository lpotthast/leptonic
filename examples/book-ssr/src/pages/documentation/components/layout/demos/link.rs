use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn LinkComponentDemo() -> impl IntoView {
    view! {
        <p>
            "Start with the "<Link href="/doc/installation">"installation guide"</Link>", or read "
            <Link href="https://github.com/lpotthast/leptonic" target=LinkTarget::Blank>"the source on GitHub"</Link>"."
        </p>
    }
}
