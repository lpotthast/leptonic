use leptonic::components::prelude::*;
use leptos::prelude::*;
use leptos_router::{components::Outlet, hooks::use_location};

use crate::{
    kit::DocPageHeader,
    nav::{NavEntry, nav},
};

/// A concept of the navigation, compared by identity.
#[derive(Clone, Copy)]
struct Concept(&'static NavEntry);

impl PartialEq for Concept {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

/// Layout of a concept (Button, Slider, ...): the concept name and tabs for its overview and layer pages, shown by the
/// routed [`DocPage`](crate::kit::DocPage) above its article (inside its `<main>`). The concept is looked up in the
/// [navigation](crate::nav) by the current path.
#[component]
pub fn ConceptLayout() -> impl IntoView {
    let location = use_location();
    // Switching between the tabs of one concept keeps the header; only the current tab changes.
    let concept = Memo::new(move |_| {
        location
            .pathname
            .with(|path| nav().concept_at(path))
            .map(Concept)
    });

    let header = move || {
        let Concept(concept) = concept.get()?;
        let tabs = std::iter::once(("Overview", concept.href.clone()))
            .chain(concept.tabs.iter().map(|tab| (tab.label(), tab.href.clone())))
            .map(|(label, href)| view! { <Link href current_match=CurrentMatch::Exact classes="doc-concept-tab">{label}</Link> })
            .collect_view();
        Some(view! {
            <div class="doc-concept-name">{concept.title}</div>
            <nav class="doc-concept-tabs" aria-label=format!("{} pages", concept.title)>{tabs}</nav>
        })
    };
    provide_context(DocPageHeader(ViewFn::from(header)));

    view! { <Outlet/> }
}
