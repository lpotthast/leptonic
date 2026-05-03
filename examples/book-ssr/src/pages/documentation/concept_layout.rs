use leptos::prelude::*;
use leptos_router::{components::Outlet, hooks::use_location};

#[component]
pub fn ConceptLayout(
    name: &'static str,
    #[prop(into)] tabs: Vec<(&'static str, String)>,
) -> impl IntoView {
    let location = use_location();

    view! {
        <div class="concept-layout">
            <h1 class="concept-name">{name}</h1>
            <nav class="concept-tabs">
                {tabs.into_iter().map(|(label, href)| {
                    let href_clone = href.clone();
                    let is_active = Signal::derive(move || {
                        location.pathname.get() == href_clone
                    });
                    view! {
                        <a
                            href=href
                            class=move || if is_active.get() {
                                "concept-tab active"
                            } else {
                                "concept-tab"
                            }
                        >
                            {label}
                        </a>
                    }
                }).collect_view()}
            </nav>
            <Outlet/>
        </div>
    }
}
