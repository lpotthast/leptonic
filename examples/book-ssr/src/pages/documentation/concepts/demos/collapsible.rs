use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn CollapsibleConceptDemo() -> impl IntoView {
    view! {
        <Collapsible>
            <CollapsibleHeader slot>"Click to expand"</CollapsibleHeader>
            <CollapsibleBody slot>"This content is hidden until expanded."</CollapsibleBody>
        </Collapsible>
    }
}
