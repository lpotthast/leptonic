use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageDataDisplay() -> impl IntoView {
    view! {
        <DocPage title="Data Display">
            <p>
                "Components for presenting structured data in grids, tables, and trees."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::DataDisplay.materialize()/>
            </Section>

        </DocPage>
    }
}
