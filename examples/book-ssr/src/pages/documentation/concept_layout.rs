use leptonic::components::prelude::*;
use leptos::prelude::*;
use leptos_router::{components::Outlet, hooks::use_location};

use crate::nav::nav;

/// Layout of a concept (Button, Slider, ...): the concept name and tabs for its overview and layer pages, above the
/// routed page. The concept is looked up in the [navigation](crate::nav) by the current path.
#[component]
pub fn ConceptLayout() -> impl IntoView {
    let location = use_location();
    let concept = Memo::new(move |_| {
        let concept = nav().concept_at(&location.pathname.get());
        concept.map(|concept| concept.href.as_str())
    });

    let header = move || {
        let concept = concept.get().and_then(|href| nav().concept_at(href))?;
        let tabs = std::iter::once(("Overview", concept.href.clone()))
            .chain(concept.tabs.iter().map(|tab| (tab.label, tab.href.clone())))
            .map(|(label, href)| view! { <Link href exact=true classes="concept-tab">{label}</Link> })
            .collect_view();
        Some(view! {
            <div class="concept-name">{concept.title}</div>
            <nav class="concept-tabs" aria-label=format!("{} pages", concept.title)>{tabs}</nav>
        })
    };

    view! {
        <div class="concept-layout">
            {header}
            <Outlet/>
        </div>
    }
}
