use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn DisclosureConceptDemo() -> impl IntoView {
    let expanded = RwSignal::new(false);
    let disabled = RwSignal::new(false);

    view! {
        <Collapsible is_expanded=expanded set_expanded=expanded is_disabled=disabled>
            <CollapsibleHeader slot>"Shipping"</CollapsibleHeader>
            <CollapsibleBody slot>"Orders ship within two working days."</CollapsibleBody>
        </Collapsible>
        <p class="demo-status">{move || if expanded.get() { "Expanded." } else { "Collapsed." }}</p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
