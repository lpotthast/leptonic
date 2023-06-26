use leptonic::prelude::*;
use leptos::*;

use crate::pages::documentation::doc_root::DocRoutes;

#[component]
pub fn PageOverview(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"Overview"</H1>

        <P>"Leptonic is a component library for Leptos. This site was built using Leptonic."</P>
        <P>"Explore the available components and other features using the side menu."</P>

        <P>
            "If you want to dive right in, follow our " <Link href=DocRoutes::Installation>"installation"</Link> " instructions."
        </P>
    }
}
