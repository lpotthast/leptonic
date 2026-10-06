use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageLayoutCategory() -> impl IntoView {
    view! {
        <DocPage title="Layout">
            <p>
                "Components for structuring and organizing page content \u{2014} collapsible sections, dividers, tabbed panels, and more."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::LayoutCategory.materialize()/>
            </Section>

            <Section title="Relationships">
                <p>"How layout components relate to each other:"</p>

                <ul>
                    <li><strong>"Tabs vs Collapsible"</strong>" \u{2014} Tabs switch between mutually exclusive panels; Collapsible sections can be independently expanded or collapsed."</li>
                    <li><strong>"Stack vs Grid"</strong>" \u{2014} Stack arranges items along a single axis (vertical or horizontal); Grid provides two-dimensional layout with keyboard navigation."</li>
                    <li><strong>"Drawer vs App Bar"</strong>" \u{2014} App Bar provides persistent top-level navigation; Drawer slides in from the side for secondary navigation or settings."</li>
                </ul>
            </Section>
        </DocPage>
    }
}
