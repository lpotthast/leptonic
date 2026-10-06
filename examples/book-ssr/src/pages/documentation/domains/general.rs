use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageGeneral() -> impl IntoView {
    view! {
        <DocPage title="General">
            <p>
                "Foundational components and utilities used across the library \u{2014} typography, icons, and callback helpers. "
                "These are building blocks that don\u{2019}t belong to a specific interaction domain. "
                "They typically exist at the component layer only."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::General.materialize()/>
            </Section>

            <Section title="Relationships">
                <ul>
                    <li><strong>"Typography"</strong>" provides text elements ("<code>"H1"</code>"\u{2013}"<code>"H6"</code>", "<code>"P"</code>", "<code>"Code"</code>") used on every documentation page and in most applications."</li>
                    <li><strong>"Icon"</strong>" renders SVG icons that can be placed inside buttons, links, and other interactive elements."</li>
                    <li><strong>"Callback"</strong>" provides the "<code>"Callback<T>"</code>" type used throughout leptonic\u{2019}s hook and component APIs."</li>
                </ul>
            </Section>
        </DocPage>
    }
}
