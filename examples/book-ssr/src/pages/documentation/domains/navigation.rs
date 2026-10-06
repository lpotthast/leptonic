use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageNavigation() -> impl IntoView {
    view! {
        <DocPage title="Navigation">
            <p>
                "Components for moving between views and locations \u{2014} links, menus, and breadcrumb trails."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Navigation.materialize()/>
            </Section>

            <Section title="Relationships">
                <p>"How navigation components compose together:"</p>

                <ul>
                    <li><strong>"Breadcrumbs"</strong>" show the user\u{2019}s position within a hierarchy, with each level being a Link."</li>
                    <li><strong>"Links"</strong>" handle direct page-to-page navigation. They can appear standalone, inside menus, or as breadcrumb segments."</li>
                    <li><strong>"Menus"</strong>" group multiple actions (or navigation links) behind a single trigger, keeping the interface uncluttered."</li>
                </ul>
            </Section>
        </DocPage>
    }
}
